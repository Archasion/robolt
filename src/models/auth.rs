use std::error::Error;
use std::marker::PhantomData;

use reqwest::blocking::Client;
use reqwest::header::{self, HeaderMap};
use reqwest::StatusCode;

use crate::Robolt;
use crate::utilities::client::{Authenticated, Unauthenticated};

impl Robolt<Unauthenticated> {
    pub fn login(self, roblox_cookie: String) -> Result<Robolt<Authenticated>, Box<dyn Error>> {
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

        let client = Client::builder()
            .default_headers(headers)
            .cookie_store(true)
            .build()?;

        Ok(Robolt {
            state: PhantomData::<Authenticated>,
            client,
        })
    }
}

impl Robolt<Authenticated> {
    pub fn logout(self) -> Robolt<Unauthenticated> {
        Robolt {
            state: PhantomData::<Unauthenticated>,
            client: Client::new(),
        }
    }
}