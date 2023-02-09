#![allow(dead_code)]

use std::error::Error;

use reqwest::{Method, StatusCode};
use reqwest::blocking::Client;
use reqwest::header::{self, HeaderMap};
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::utilities::errors::RobloxAPIResponseErrors;

pub(crate) struct HttpRequest<'a, T: Serialize> {
    pub method: Method,
    pub endpoint: String,
    pub body: Option<&'a T>,
}

pub struct Robolt {
    pub(crate) client: Client,
    authenticated: bool,
}

impl Default for Robolt {
    fn default() -> Self {
        Self::new()
    }
}

impl Robolt {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            authenticated: false,
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

        self.client = Client::builder()
            .default_headers(headers)
            .cookie_store(true)
            .build()?;

        self.authenticated = true;

        Ok(())
    }

    pub fn logout(&mut self) {
        self.client = Client::new();
        self.authenticated = false;
    }

    pub fn is_authenticated(&self) -> bool {
        self.authenticated
    }

    pub(crate) fn request<U, T>(&self, data: HttpRequest<'_, U>) -> Result<T, String>
        where
            T: DeserializeOwned,
            U: Serialize,
    {
        let url = format!("https://{}", data.endpoint);
        let mut builder = self.client.request(data.method, url);

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