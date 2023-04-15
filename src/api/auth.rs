use std::error::Error;
use std::intrinsics::type_name;
use std::marker::PhantomData;

use reqwest::header::{self, HeaderMap};
use reqwest::StatusCode;

use crate::utils::client::{default_client_headers, Authenticated, Unauthenticated};
use crate::Robolt;

impl<State> Robolt<State> {
	pub fn is_authenticated(&self) -> bool {
		type_name::<State>() == type_name::<Authenticated>()
	}
}

impl Robolt<Unauthenticated> {
	pub async fn login(
		self,
		roblox_cookie: String,
	) -> Result<Robolt<Authenticated>, Box<dyn Error>> {
		let cookie = format!(".ROBLOSECURITY={roblox_cookie}");
		let mut headers = HeaderMap::from(default_client_headers());

		headers.insert(header::COOKIE, cookie.parse()?);

		let res = self
			.client
			.post("https://auth.roblox.com/v2/logout")
			.headers(headers.clone())
			.send()
			.await?;

		if !res.status().is_success() && res.status() != StatusCode::FORBIDDEN {
			return Err("Invalid cookie".into());
		}

		let csrf_token = res
			.headers()
			.get("x-csrf-token")
			.ok_or("No CSRF token found")?;

		Ok(Robolt {
			state: PhantomData::<Authenticated>,
			client: self.client,
			cookie: Some(cookie),
			xcsrf: Some(csrf_token.to_str()?.to_string()),
		})
	}
}

impl Robolt<Authenticated> {
	pub fn logout(self) -> Robolt<Unauthenticated> {
		Robolt::new()
	}
}