use serde::Deserialize;

#[derive(Deserialize)]
pub struct RobloxAPIResponseErrorMessage {
    pub message: String,
}

#[derive(Deserialize)]
pub struct RobloxAPIResponseErrors {
    pub errors: Vec<RobloxAPIResponseErrorMessage>,
}
