use serde::Deserialize;

use crate::models::ENDPOINTS;
use crate::Robolt;

impl<State> Robolt<State> {
    pub fn fetch_roblox_badges(&self, user_id: u64) -> Result<Vec<RobloxBadge>, String> {
        self.request_builder(format!(
            "{}/v1/users/{}/roblox-badges",
            ENDPOINTS.account_information, user_id
        ))
            .send()
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RobloxBadge {
    pub id: u8,
    pub name: String,
    pub description: String,
    pub image_url: String,
}