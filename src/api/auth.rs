use std::error::Error;
use std::intrinsics::type_name;
use std::marker::PhantomData;

use reqwest::blocking::Client;
use reqwest::header::{self, HeaderMap};
use reqwest::StatusCode;

use crate::utils::client::{Authenticated, Unauthenticated};
use crate::Robolt;

impl<State> Robolt<State> {
	pub fn is_authenticated(&self) -> bool {
		type_name::<State>() == type_name::<Authenticated>()
	}
}

impl Robolt<Unauthenticated> {
	pub fn login(self, roblox_cookie: String) -> Result<Robolt<Authenticated>, Box<dyn Error>> {
		let user_agent = format!("{}/{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
		let cookie = format!(".ROBLOSECURITY={roblox_cookie}");
		let mut headers = HeaderMap::new();

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
	pub fn logout(self) -> Robolt<Unauthenticated> {
		Robolt {
			state: PhantomData::<Unauthenticated>,
			client: Client::new(),
		}
	}
}