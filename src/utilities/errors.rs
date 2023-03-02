#![allow(dead_code)]

use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct RobloxAPIError {
    pub(crate) message: String,
}

#[derive(Deserialize)]
pub(crate) struct RobloxAPIErrors {
    pub(crate) errors: Vec<RobloxAPIError>,
}