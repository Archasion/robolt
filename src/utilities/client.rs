#![allow(dead_code)]

use reqwest::blocking::Client;
use reqwest::Method;
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
    pub(crate) authenticated: bool,
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

    pub(crate) fn request<U, T>(&self, data: HttpRequest<U>) -> Result<T, String>
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
                .map_err(|_| status.to_string())?;

            let err = err_res.errors.first().ok_or(status.to_string())?;
            return Err(err.message.clone());
        }

        let json = res.json::<T>().map_err(|e| e.to_string())?;

        Ok(json)
    }
}