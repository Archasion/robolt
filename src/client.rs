#![allow(unused)]

use lazy_static::lazy_static;
use serde::de::DeserializeOwned;
use reqwest::cookie::{CookieStore, Jar};
use reqwest::header::{HeaderMap, HeaderValue, self};
use reqwest::blocking::Client;
use reqwest::{Method, Url};
use std::sync::RwLock;

use crate::errors::RobloxAPIResponseErrors;

lazy_static! {
    pub(crate) static ref HTTP: RwLock<HttpClient> = {
        let client = HttpClient::new();
        RwLock::new(client)
    };
}

pub(crate) struct HttpClient {
    client: Client
}

impl Default for HttpClient {
    fn default() -> Self {
        Self::new()
    }
}

impl HttpClient {
    pub(crate) fn new() -> Self {
        let user_agent = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

        Self {
            client: Client::builder()
                .user_agent(user_agent)
                .cookie_store(true)
                .build()
                .expect("Failed to build reqwest client"),
        }
    }
}

pub(crate) trait HttpClientExt {
    fn set_cookie(&self, cookie: &str) -> Result<(), &str>;
    fn req<T>(&self, method: Method, url: &str, headers: Option<HeaderMap>) -> Result<T, String>
        where T: DeserializeOwned;
}

impl HttpClientExt for RwLock<HttpClient> {
    fn set_cookie(&self, cookie: &str) -> Result<(), &str> {
        let mut headers = HeaderMap::new();
        headers.insert(header::COOKIE, HeaderValue::from_str(cookie).unwrap());

        let mut res = Client::new()
            .get("https://auth.roblox.com/v2/logout")
            .headers(headers.clone())
            .send()
            .unwrap();

        if !res.status().is_success() {
            return Err("Invalid cookie");
        }

        let csrf = res.headers().get("x-csrf-token");

        if csrf.is_none() {
            return Err("Failed to fetch X-CSRF-TOKEN");
        }

        headers.insert("X-CSRF-TOKEN", csrf.unwrap().to_owned());
        self.write().unwrap().client = Client::builder()
            .default_headers(headers)
            .cookie_store(true)
            .build()
            .expect("Failed to build reqwest client");

        Ok(())
    }

    fn req<T>(&self, method: Method, url: &str, mut headers: Option<HeaderMap>) -> Result<T, String>
        where T: DeserializeOwned
    {
        let res = self
            .read()
            .unwrap()
            .client
            .request(method, format!("https://{url}"))
            .headers(headers.unwrap_or_default())
            .send();

        match res {
            Ok(res) => {
                let status = res.status();

                if status.is_success() {
                    let body = res.json::<T>();
                    match body {
                        Ok(body) => Ok(body),
                        Err(err) => Err(err.to_string()),
                    }
                } else {
                    let body = res.json::<RobloxAPIResponseErrors>();
                    match body {
                        Ok(body) => {
                            let errors = body.errors;
                            let error = errors.first().unwrap();
                            Err(error.message.to_string())
                        }
                        Err(_) => Err(status.to_string())
                    }
                }
            },
            Err(err) => Err(err.to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::Value;
    use super::*;

    const ENDPOINT_GET: &str = "httpbin.org/get";
    const ENDPOINT_POST: &str = "httpbin.org/post";
    const ENDPOINT_404: &str = "httpbin.org/status/404";
    const ENDPOINT_ROBLOX: &str = "users.roblox.com/v1/users/0"; // Intentionally invalid user ID

    #[test]
    fn ok_get_req() {
        let res = HTTP.req::<Value>(Method::GET, ENDPOINT_GET, None);
        assert!(res.is_ok());
    }

    #[test]
    fn ok_post_req() {
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", "application/json".parse().unwrap());

        let res = HTTP.req::<Value>(Method::POST, ENDPOINT_POST, Some(headers));
        assert!(res.is_ok());
    }

    #[test]
    fn err_get_req() {
        let res = HTTP.req::<Value>(Method::GET, ENDPOINT_404, None);
        assert!(res.is_err());
    }

    #[test]
    fn err_post_req() {
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", "application/json".parse().unwrap());

        let res = HTTP.req::<Value>(Method::POST, ENDPOINT_404, Some(headers));
        assert_eq!(res.unwrap_err(), "404 Not Found");
    }

    #[test]
    fn roblox_err_res() {
        let res = HTTP.req::<String>(Method::GET, ENDPOINT_ROBLOX, None);

        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), "The user id is invalid.");
    }
}