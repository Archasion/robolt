use serde::Deserialize;

use crate::models::ENDPOINTS;
use crate::Robolt;
use crate::utilities::client::Authenticated;

impl<State> Robolt<State> {
    pub fn fetch_roblox_badges(&self, user_id: u64) -> Result<Vec<RobloxBadge>, String> {
        self.request_builder(format!(
            "{}/v1/users/{}/roblox-badges",
            ENDPOINTS.account_information, user_id
        ))
            .send()
    }

    pub fn fetch_user_socials(&self, user_id: u64) -> Result<UserSocials, String> {
        self.request_builder(format!(
            "{}/v1/users/{}/promotion-channels",
            ENDPOINTS.account_information, user_id
        ))
            .send()
    }
}

impl Robolt<Authenticated> {
    pub fn my_socials(&self) -> Result<AuthenticatedUserSocials, String> {
        self.request_builder(format!(
            "{}/v1/promotion-channels",
            ENDPOINTS.account_information
        ))
            .send()
    }
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RobloxBadge {
    pub id: u8,
    pub name: String,
    pub description: String,
    pub image_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserSocials {
    pub facebook: Option<String>,
    pub twitter: Option<String>,
    pub youtube: Option<String>,
    pub twitch: Option<String>,
    pub guilded: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthenticatedUserSocials {
    #[serde(rename = "promotionChannelsVisibilityPrivacy")]
    pub visibility: String,
    #[serde(flatten)]
    pub connections: UserSocials,
}