#![allow(unused)]

use lazy_static::lazy_static;
use reqwest::blocking::Client;
use reqwest::cookie::{CookieStore, Jar};
use reqwest::header::{self, HeaderMap, HeaderValue};
use reqwest::{Method, Url};
use serde::de::DeserializeOwned;
use serde::Deserialize;
use std::sync::RwLock;

use crate::utilities::errors::RobloxAPIResponseErrors;

lazy_static! {
    pub static ref HTTP: RwLock<HttpClient> = {
        let client = HttpClient::new();
        RwLock::new(client)
    };
}

pub struct HttpRequest {
    pub method: Method,
    pub url: String,
    pub headers: Option<HeaderMap>,
    pub body: Option<String>,
    pub response: bool,
}

pub struct HttpClient {
    pub client: Client,
}

impl Default for HttpClient {
    fn default() -> Self {
        Self::new()
    }
}

impl HttpClient {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }
}

pub trait HttpClientExt {
    fn send<T>(&self, data: HttpRequest) -> Result<Option<T>, String>
    where
        T: DeserializeOwned;

    fn set_cookie(&self, cookie: &str) -> Result<(), &str>;
    fn remove_cookie(&self);
}

impl HttpClientExt for RwLock<HttpClient> {
    fn send<T>(&self, data: HttpRequest) -> Result<Option<T>, String>
    where
        T: DeserializeOwned,
    {
        let res = self
            .read()
            .expect("Failed to read HTTP client")
            .client
            .request(data.method, format!("https://{}", data.url))
            .body(data.body.unwrap_or_default())
            .headers(data.headers.unwrap_or_default())
            .send()
            .map_err(|e| e.to_string())?;

        let status = res.status();

        if !status.is_success() {
            let err_res = res.json::<RobloxAPIResponseErrors>();

            return match err_res {
                Ok(body) => {
                    let error = body.errors.first().expect("Unknown error");
                    Err(error.message.to_string())
                }
                Err(_) => Err(status.to_string()),
            };
        }

        if !data.response {
            return Ok(None);
        }

        res.json::<T>().map_err(|e| e.to_string()).map(Some)
    }

    fn set_cookie(&self, cookie: &str) -> Result<(), &str> {
        let mut headers = HeaderMap::new();
        headers.insert(header::COOKIE, HeaderValue::from_str(cookie).unwrap());
        headers.insert(header::CONTENT_TYPE, HeaderValue::from_static("text/json"));

        let res = Client::new()
            .post("https://auth.roblox.com/v2/logout")
            .body("")
            .headers(headers.clone())
            .send()
            .unwrap();

        if !res.status().is_success() && res.status().as_u16() != 403 {
            return Err("Invalid cookie");
        }

        let csrf = res.headers().get("x-csrf-token");

        if csrf.is_none() {
            return Err("Failed to fetch X-CSRF-TOKEN");
        }

        headers.insert("X-CSRF-TOKEN", csrf.unwrap().to_owned());
        self.write().expect("Failed to modify HTTP client").client = Client::builder()
            .default_headers(headers)
            .cookie_store(true)
            .build()
            .expect("Failed to build HTTP client");

        Ok(())
    }

    fn remove_cookie(&self) {
        self.write().expect("Failed to modify HTTP client").client = Client::new();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Value;
    use tokio_test::{assert_err, assert_ok};

    const ENDPOINT_GET: &str = "httpbin.org/get";
    const ENDPOINT_404: &str = "httpbin.org/status/404";
    const ENDPOINT_ROBLOX: &str = "users.roblox.com/v1/users/0"; // Intentionally invalid user ID

    #[test]
    fn ok_req() {
        let req = HttpRequest {
            method: Method::GET,
            url: ENDPOINT_GET.to_string(),
            headers: None,
            body: None,
            response: false,
        };

        let res = HTTP.send::<serde_json::Value>(req);
        assert_ok!(res);
    }

    #[test]
    fn err_req() {
        let req = HttpRequest {
            method: Method::GET,
            url: ENDPOINT_404.to_string(),
            headers: None,
            body: None,
            response: false,
        };

        let res = HTTP.send::<serde_json::Value>(req);
        assert_err!(res);
    }

    #[test]
    fn roblox_err() {
        let req = HttpRequest {
            method: Method::GET,
            url: ENDPOINT_ROBLOX.to_string(),
            headers: None,
            body: None,
            response: true,
        };

        let res = HTTP.send::<serde_json::Value>(req);

        assert_err!(&res);
        assert_eq!(res.unwrap_err(), "The user id is invalid.");
    }
}