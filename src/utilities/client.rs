#![allow(dead_code)]

use reqwest::blocking::Client;
use reqwest::Method;
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::utilities::errors::RobloxAPIResponseErrors;

pub(crate) struct RequestBuilder<'a> {
    pub(crate) robolt: &'a Robolt,
    pub(crate) method: Method,
    pub(crate) endpoint: String,
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

    pub(crate) fn request_builder(&self, endpoint: String) -> RequestBuilder<'_> {
        RequestBuilder::new(endpoint, self)
    }

    fn request<U, T>(&self, method: Method, endpoint: String, body: Option<U>) -> Result<T, String>
        where
            T: DeserializeOwned,
            U: Serialize,
    {
        let url = format!("https://{endpoint}");
        let mut builder = self.client.request(method, url);

        if let Some(body) = &body {
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

impl<'a> RequestBuilder<'a> {
    fn new(endpoint: String, robolt: &'a Robolt) -> Self {
        Self {
            method: Method::GET,
            endpoint,
            robolt,
        }
    }

    pub(crate) fn method(mut self, method: Method) -> Self {
        self.method = method;
        self
    }

    pub(crate) fn send_body<T, U>(self, body: Option<T>) -> Result<U, String>
        where
            T: Serialize,
            U: DeserializeOwned,
    {
        self.robolt.request(self.method, self.endpoint, body)
    }

    pub(crate) fn send<T>(self) -> Result<T, String>
        where
            T: DeserializeOwned,
    {
        self.robolt.request::<(), T>(self.method, self.endpoint, None)
    }
}