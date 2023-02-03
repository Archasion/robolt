#![allow(dead_code)]

use std::cell::RefCell;
use std::error::Error;
use std::rc::Rc;

use reqwest::{Method, StatusCode};
use reqwest::blocking::Client;
use reqwest::header::{self, HeaderMap};
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::models::users::UserBuilder;
use crate::utilities::errors::RobloxAPIResponseErrors;

pub(crate) struct HttpRequest<'a, T: Serialize> {
    pub method: Method,
    pub endpoint: String,
    pub body: Option<&'a T>,
}

pub struct Robolt {
    client: Rc<RefCell<Client>>,
}

impl Default for Robolt {
    fn default() -> Self {
        Self::new()
    }
}

impl Robolt {
    pub fn new() -> Self {
        Self {
            client: Rc::new(RefCell::new(Client::new())),
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

        self.client.replace(authenticated_client);
        Ok(())
    }

    pub fn logout(&mut self) {
        self.client.replace(Client::new());
    }

    pub fn users(&self) -> UserBuilder {
        UserBuilder::from(Rc::clone(&self.client))
    }
}

pub(crate) trait BorrowClient {
    fn request<U, T>(&self, data: HttpRequest<'_, U>) -> Result<T, String>
        where
            T: DeserializeOwned,
            U: Serialize;
}

impl BorrowClient for RefCell<Client> {
    fn request<U, T>(&self, data: HttpRequest<'_, U>) -> Result<T, String>
        where
            T: DeserializeOwned,
            U: Serialize,
    {
        let url = format!("https://{}", data.endpoint);
        let mut builder = self.borrow().request(data.method, url);

        if let Some(body) = data.body {
            builder = builder.json(body);
        }

        let res = builder.send().map_err(|e| e.to_string())?;

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