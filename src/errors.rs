use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct RobloxAPIResponseErrorMessage {
    pub message: String
}

#[derive(Deserialize)]
pub(crate) struct RobloxAPIResponseErrors {
    pub errors: Vec<RobloxAPIResponseErrorMessage>,
}