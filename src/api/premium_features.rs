use crate::api::routes::RobloxApi;
use crate::errors::RoboltError;
use crate::utils::client::Authenticated;
use crate::Robolt;

impl Robolt<Authenticated> {
	pub async fn has_premium(&self, user_id: u64) -> Result<bool, RoboltError> {
		self.request(
			RobloxApi::PremiumFeatures,
			format!("/v1/users/{user_id}/validate-membership"),
		)
		.send()
		.await
	}
}