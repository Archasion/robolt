use std::error::Error;

use reqwest::blocking::Client;
use reqwest::header::{self, HeaderMap};
use reqwest::StatusCode;

use crate::Robolt;

impl Robolt {
    pub fn login(&mut self, roblox_cookie: String) -> Result<(), Box<dyn Error>> {
        let mut headers = HeaderMap::new();
        let cookie = format!(".ROBLOSECURITY={roblox_cookie}");

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

        headers.insert("X-CSRF-TOKEN", csrf_token.clone());

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
}
