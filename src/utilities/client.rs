#![allow(dead_code)]

use std::io::{Error, ErrorKind};
use std::marker::PhantomData;

use reqwest::{header, Method};
use reqwest::blocking::Client;
use reqwest::header::HeaderMap;
use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;

use crate::utilities::errors::RobloxAPIErrors;

pub(crate) struct RequestBuilder<'a, State> {
    pub(crate) robolt: &'a Robolt<State>,
    pub(crate) method: Method,
    pub(crate) endpoint: String,
    pub(crate) function: &'static str,
}

pub struct Unauthenticated;

pub struct Authenticated;

#[derive(Deserialize)]
pub(crate) struct EmptyResponse {}

pub struct Robolt<State = Unauthenticated> {
    pub(crate) client: Client,
    pub(crate) state: PhantomData<State>,
}

impl Default for Robolt {
    fn default() -> Self {
        Self::new()
    }
}

impl Robolt {
    pub fn new() -> Self {
        let user_agent = format!("{}/{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
        let mut headers = HeaderMap::new();

        headers.insert(header::CONTENT_LENGTH, "0".parse().unwrap());

        let client = Client::builder()
            .user_agent(user_agent)
            .default_headers(headers)
            .build()
            .expect("Failed to build client");

        Self {
            state: PhantomData::<Unauthenticated>,
            client,
        }
    }
}

impl<State> Robolt<State> {
    pub(crate) fn request_builder(&self, endpoint: String) -> RequestBuilder<'_, State> {
        RequestBuilder::new(endpoint, self)
    }

    fn request<U, T>(
        &self,
        method: Method,
        endpoint: String,
        function: &str,
        body: Option<U>,
    ) -> Result<T, Error>
        where
            T: DeserializeOwned,
            U: Serialize,
    {
        let url = format!("https://{endpoint}");
        let builder = {
            let mut builder = self.client.request(method, url);

            if let Some(body) = &body {
                builder = builder.json(body);
            }

            builder
        };

        let unknown_error = |err: String| {
            Error::new(
                ErrorKind::Other,
                format!(
                    "An unknown error has occurred while executing {}() | {}",
                    function, err
                ),
            )
        };

        let res = builder
            .send()
            .map_err(|err| unknown_error(err.to_string()))?;

        let status = res.status();

        if !status.is_success() {
            let err_res = res
                .json::<RobloxAPIErrors>()
                .map_err(|_| Error::new(ErrorKind::Other, status.to_string()))?;

            let err = err_res
                .errors
                .first()
                .ok_or_else(|| Error::new(ErrorKind::Other, status.to_string()))?;

            return Err(Error::new(
                ErrorKind::Other,
                format!("RobloxAPIError: {}", err.message.clone()),
            ));
        }

        let json = res
            .json::<T>()
            .map_err(|err| unknown_error(err.to_string()))?;

        Ok(json)
    }
}

impl<'a, State> RequestBuilder<'a, State> {
    fn new(endpoint: String, robolt: &'a Robolt<State>) -> Self {
        Self {
            method: Method::GET,
            function: "[Unknown Function]",
            endpoint,
            robolt,
        }
    }

    pub(crate) fn method(mut self, method: Method) -> Self {
        self.method = method;
        self
    }

    pub(crate) fn function(mut self, function: &'static str) -> Self {
        self.function = function;
        self
    }

    pub(crate) fn send_body<T, U>(self, body: T) -> Result<U, Error>
        where
            T: Serialize,
            U: DeserializeOwned,
    {
        self.robolt
            .request(self.method, self.endpoint, self.function, Some(body))
    }

    pub(crate) fn send<T>(self) -> Result<T, Error>
        where
            T: DeserializeOwned,
    {
        self.robolt
            .request::<(), T>(self.method, self.endpoint, self.function, None)
    }
}