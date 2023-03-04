use std::io::Error;

use serde::Deserialize;

use crate::api::ENDPOINTS;
use crate::Robolt;
use crate::utilities::client::Authenticated;

impl Robolt<Authenticated> {
    pub fn fetch_balance(&self) -> Result<u64, Error> {
        self.request_builder(format!("{}/v1/user/currency", ENDPOINTS.economy))
            .function("fetch_balance")
            .send::<Robux>()
            .map(|res| res.robux)
    }
}

#[derive(Deserialize)]
struct Robux {
    robux: u64,
}