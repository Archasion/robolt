use serde::Deserialize;

use crate::api::ENDPOINTS;
use crate::errors::RoboltError;
use crate::utilities::client::Authenticated;
use crate::Robolt;

impl Robolt<Authenticated> {
	pub fn fetch_balance(&self) -> Result<u64, RoboltError> {
		self.request_builder(format!("{}/v1/user/currency", ENDPOINTS.economy))
			.send::<Robux>()
			.map(|res| res.robux)
	}
}

#[derive(Deserialize)]
struct Robux {
	robux: u64,
}