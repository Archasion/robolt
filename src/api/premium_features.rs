use std::io::Error;

use crate::api::ENDPOINTS;
use crate::Robolt;
use crate::utilities::client::Authenticated;

impl Robolt<Authenticated> {
    pub fn has_premium(&self, user_id: u64) -> Result<bool, Error> {
        self.request_builder(format!(
            "{}/v1/users/{}/validate-membership",
            ENDPOINTS.premium_features, user_id
        ))
            .function("has_premium")
            .send()
    }
}