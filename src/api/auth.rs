use std::error::Error;
use std::intrinsics::type_name;
use std::marker::PhantomData;

use reqwest::header::{CONTENT_LENGTH, COOKIE};
use reqwest::StatusCode;

use crate::api::routes::RobloxApi;
use crate::utils::client::{default_client_headers, Authenticated, Unauthenticated};
use crate::Robolt;

impl<State> Robolt<State> {
	pub fn is_authenticated(&self) -> bool {
		type_name::<State>() == type_name::<Authenticated>()
	}
}

impl Robolt<Unauthenticated> {
	pub async fn set_cookie(self, roblox_cookie: String) -> Result<Robolt<Authenticated>, Box<dyn Error>> {
		let cookie = format!(".ROBLOSECURITY={roblox_cookie}");
		let mut headers = default_client_headers();

		headers.insert(COOKIE, cookie.parse()?);
		headers.insert(CONTENT_LENGTH, "0".parse()?);

		let res = self
			.http
			.post(format!("https://{}/v2/logout", RobloxApi::Auth.url()))
			.headers(headers.clone())
			.send()
			.await?;

		if !res.status().is_success() && res.status() != StatusCode::FORBIDDEN {
			return Err("Invalid cookie".into());
		}

		let csrf_token = res.headers().get("x-csrf-token").ok_or("No CSRF token found")?;

		Ok(Robolt {
			state: PhantomData::<Authenticated>,
			http: self.http,
			cookie: Some(cookie),
			xcsrf: Some(csrf_token.to_str()?.to_string()),
		})
	}
}

impl Robolt<Authenticated> {
	pub fn remove_cookie(self) -> Robolt<Unauthenticated> {
		Robolt {
			state: PhantomData::<Unauthenticated>,
			http: self.http,
			cookie: None,
			xcsrf: None,
		}
	}
}