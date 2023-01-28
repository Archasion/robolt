#![allow(dead_code)]

use std::cell::RefCell;
use std::error::Error;
use std::rc::Rc;

use reqwest::{Method, StatusCode};
use reqwest::blocking::Client;
use reqwest::header::{self, HeaderMap};
use serde::de::DeserializeOwned;

use crate::models::users::UserBuilder;
use crate::utilities::errors::RobloxAPIResponseErrors;

pub(crate) struct HttpRequest {
    pub method: Method,
    pub endpoint: String,
    pub body: Option<String>,
}

pub struct Robolt {
    pub users: UserBuilder,
    client: Rc<RefCell<RoboltClient>>,
}

pub(crate) struct RoboltClient {
    inner: Client,
    authenticated: bool,
}

impl Default for Robolt {
    fn default() -> Self {
        Self::new()
    }
}

impl Robolt {
    pub fn new() -> Self {
        let client = Rc::new(RefCell::new(RoboltClient {
            inner: Client::new(),
            authenticated: false,
        }));

        let client_ref = Rc::clone(&client);

        Self {
            users: UserBuilder::new(client_ref),
            client,
        }
    }

    pub fn login(&mut self, cookie: String) -> Result<(), Box<dyn Error>> {
        let mut headers = HeaderMap::new();
        let cookie = format!(".ROBLOSECURITY={cookie}");

        headers.insert(header::COOKIE, cookie.parse()?);

        let res = Client::new()
            .post("https://auth.roblox.com/v2/logout")
            .headers(headers.clone())
            .body("")
            .send()?;

        if !res.status().is_success() && res.status() != StatusCode::FORBIDDEN {
            return Err("Invalid cookie".into());
        }

        let csrf_token = res
            .headers()
            .get("x-csrf-token")
            .ok_or("No CSRF token found")?;

        headers.insert("X_CSRF_TOKEN", csrf_token.clone());

        let authenticated_client = Client::builder()
            .default_headers(headers)
            .cookie_store(true)
            .build()?;

        let client = RoboltClient {
            inner: authenticated_client,
            authenticated: true,
        };

        self.client.replace(client);
        Ok(())
    }

    pub fn logout(&mut self) {
        let client = RoboltClient {
            inner: Client::new(),
            authenticated: false,
        };

        self.client.replace(client);
    }
}

pub(crate) trait RoboltClientExt {
    fn request<T>(&self, data: HttpRequest) -> Result<T, String>
        where
            T: DeserializeOwned;
}

impl RoboltClientExt for RefCell<RoboltClient> {
    fn request<T>(&self, data: HttpRequest) -> Result<T, String>
        where
            T: DeserializeOwned,
    {
        let url = format!("https://{}", data.endpoint);
        let res = self
            .borrow()
            .inner
            .request(data.method, url)
            .body(data.body.unwrap_or_default())
            .send()
            .map_err(|e| e.to_string())?;

        let status = res.status();
        if !status.is_success() {
            let err_res = res
                .json::<RobloxAPIResponseErrors>()
                .map_err(|e| e.to_string())?;

            let err = err_res.errors.first().ok_or(status.to_string())?;

            return Err(err.message.clone());
        }

        let json = res.json::<T>().map_err(|e| e.to_string())?;

        Ok(json)
    }
}

#[cfg(test)]
mod tests {
    use dotenv::dotenv;
    use tokio_test::assert_ok;

    use super::*;

    #[test]
    fn login() {
        dotenv().ok();
        let mut client = Robolt::new();
        let cookie = std::env::var("ROBLOX_COOKIE").unwrap();

        assert_ok!(client.login(cookie));
    }
}