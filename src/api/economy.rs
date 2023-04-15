use serde::Deserialize;

use crate::api::ENDPOINTS;
use crate::errors::RoboltError;
use crate::utils::client::Authenticated;
use crate::Robolt;

impl Robolt<Authenticated> {
	pub async fn balance(&self) -> Result<u64, RoboltError> {
		self.request_builder(format!("{}/v1/user/currency", ENDPOINTS.economy))
			.send::<UserBalance>()
			.await
			.map(|res| res.robux)
	}
}

#[derive(Deserialize)]
struct UserBalance {
	robux: u64,
}