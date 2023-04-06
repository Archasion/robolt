use std::error::Error;
use std::marker::PhantomData;

use reqwest::blocking::Client;
use reqwest::header::{self, HeaderMap};
use reqwest::StatusCode;

use crate::utils::client::{Authenticated, Unauthenticated};
use crate::Robolt;

impl Robolt<Unauthenticated> {
	pub fn authenticate(
		self,
		roblox_cookie: String,
	) -> Result<Robolt<Authenticated>, Box<dyn Error>> {
		let user_agent = format!("{}/{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
		let mut headers = HeaderMap::new();
		let cookie = format!(".ROBLOSECURITY={roblox_cookie}");

		headers.insert(header::CONTENT_LENGTH, "0".parse()?);
		headers.insert(header::COOKIE, cookie.parse()?);

		let res = Client::new()
			.post("https://auth.roblox.com/v2/logout")
			.headers(headers.clone())
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
			.user_agent(user_agent)
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
	pub fn clear_session(self) -> Robolt<Unauthenticated> {
		Robolt {
			state: PhantomData::<Unauthenticated>,
			client: Client::new(),
		}
	}
}