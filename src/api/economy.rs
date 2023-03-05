use serde::Deserialize;

use crate::api::ENDPOINTS;
use crate::errors::RoboltError;
use crate::Robolt;
use crate::utilities::client::Authenticated;

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