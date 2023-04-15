use crate::api::ENDPOINTS;
use crate::errors::RoboltError;
use crate::utils::client::Authenticated;
use crate::Robolt;

impl Robolt<Authenticated> {
	pub async fn has_premium(&self, user_id: u64) -> Result<bool, RoboltError> {
		self.request_builder(format!(
			"{}/v1/users/{}/validate-membership",
			ENDPOINTS.premium_features, user_id
		))
		.send()
		.await
	}
}