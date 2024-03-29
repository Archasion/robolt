use std::error::Error;
use std::fmt::{Debug, Display};
use std::marker::PhantomData;

use reqwest::header::{HeaderMap, ACCEPT, CONTENT_LENGTH, CONTENT_TYPE, COOKIE, USER_AGENT};
use reqwest::{Client, Method};
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::api::routes::RobloxApi;
use crate::utils::errors::{RobloxAPIErrors, RoboltError};

impl Default for Robolt {
	fn default() -> Self {
		Self::new()
	}
}

impl Robolt {
	pub fn new() -> Self {
		let client = Client::builder()
			.default_headers(default_client_headers())
			.build()
			.expect("Failed to build client");

		Self {
			state: PhantomData::<Unauthenticated>,
			http: client,
			cookie: None,
			xcsrf: None,
		}
	}

	pub async fn from(roblox_cookie: String) -> Result<Robolt<Authenticated>, Box<dyn Error>> {
		Self::new().set_cookie(roblox_cookie).await
	}
}

impl<State> Robolt<State> {
	pub(crate) fn request<S: ToString + Display>(&self, domain: RobloxApi, path: S) -> RequestBuilder<'_, State> {
		RequestBuilder::new(domain.url(), path, self)
	}

	async fn inner_request<U, T>(
		&self,
		method: Method,
		endpoint: String,
		body: Option<U>,
	) -> Result<T, RoboltError>
	where
		T: DeserializeOwned,
		U: Serialize,
	{
		let builder = {
			let mut builder = self.http.request(method, format!("https://{endpoint}"));
			let mut has_body = false;

			if let Some(body) = &body {
				builder = builder.json(body);
				has_body = true;
			}

			if let (Some(cookie), Some(xcsrf)) = (&self.cookie, &self.xcsrf) {
				let mut headers = HeaderMap::new();
				headers.insert(COOKIE, cookie.parse().unwrap());
				headers.insert("x-csrf-token", xcsrf.parse().unwrap());

				if !has_body {
					headers.insert(CONTENT_LENGTH, "0".parse().unwrap());
				}

				builder = builder.headers(headers);
			}

			builder
		};

		let res = builder.send().await?;
		let status = res.status();

		if !status.is_success() {
			let err_res = res.json::<RobloxAPIErrors>().await?;

			let err = err_res
				.errors
				.into_iter()
				.next()
				.ok_or(RoboltError::from(status.to_string()))?;

			return Err(err);
		}

		let json = res.json::<T>().await?;
		Ok(json)
	}
}

pub(crate) fn default_client_headers() -> HeaderMap {
	let mut headers = HeaderMap::new();
	headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
	headers.insert(ACCEPT, "application/json".parse().unwrap());
	headers.insert(
		USER_AGENT,
		format!("{}/{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"))
			.parse()
			.unwrap(),
	);
	headers
}

impl<'a, State> RequestBuilder<'a, State> {
	fn new<S: ToString + Display>(domain: &str, path: S, robolt: &'a Robolt<State>) -> Self {
		Self {
			method: Method::GET,
			endpoint: format!("{domain}{path}"),
			robolt,
		}
	}

	pub(crate) fn method(mut self, method: Method) -> Self {
		self.method = method;
		self
	}

	pub(crate) async fn send_body<T, U>(self, body: T) -> Result<U, RoboltError>
	where
		T: Serialize,
		U: DeserializeOwned,
	{
		self.robolt.inner_request(self.method, self.endpoint, Some(body)).await
	}

	pub(crate) async fn send<T>(self) -> Result<T, RoboltError>
	where
		T: DeserializeOwned,
	{
		self.robolt
			.inner_request::<(), T>(self.method, self.endpoint, None)
			.await
	}
}

#[derive(Debug, Clone, PartialEq)]
pub struct Unauthenticated;

#[derive(Debug, Clone, PartialEq)]
pub struct Authenticated;

#[derive(Debug, Clone)]
pub struct Robolt<State = Unauthenticated> {
	#[cfg(feature = "http")]
	pub http: Client,
	#[cfg(not(feature = "http"))]
	pub(crate) http: Client,
	pub(crate) state: PhantomData<State>,
	pub(crate) cookie: Option<String>,
	pub(crate) xcsrf: Option<String>,
}

#[derive(Debug, Clone)]
pub(crate) struct RequestBuilder<'a, State> {
	robolt: &'a Robolt<State>,
	method: Method,
	endpoint: String,
}