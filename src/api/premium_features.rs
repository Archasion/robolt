use crate::api::ENDPOINTS;
use crate::errors::RoboltError;
use crate::Robolt;
use crate::utilities::client::Authenticated;

impl Robolt<Authenticated> {
    pub fn has_premium(&self, user_id: u64) -> Result<bool, RoboltError> {
        self.request_builder(format!(
            "{}/v1/users/{}/validate-membership",
            ENDPOINTS.premium_features, user_id
        ))
            .send()
    }
}