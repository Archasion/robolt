use serde::Deserialize;

use crate::api::routes::RobloxApi;
use crate::errors::RoboltError;
use crate::utils::client::Authenticated;
use crate::Robolt;

impl Robolt<Authenticated> {
	pub async fn balance(&self) -> Result<u64, RoboltError> {
		self.request(RobloxApi::Economy, "/v1/user/currency")
			.send::<UserBalance>()
			.await
			.map(|res| res.robux)
	}
}

#[derive(Deserialize)]
struct UserBalance {
	robux: u64,
}