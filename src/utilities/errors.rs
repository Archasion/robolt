#![allow(dead_code)]

use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct RobloxAPIResponseErrorMessage {
    pub(crate) message: String,
}

#[derive(Deserialize)]
pub(crate) struct RobloxAPIResponseErrors {
    pub(crate) errors: Vec<RobloxAPIResponseErrorMessage>,
}
