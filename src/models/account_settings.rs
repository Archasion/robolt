use std::io::Error;

use serde::Deserialize;

use crate::models::ENDPOINTS;
use crate::Robolt;
use crate::utilities::client::Authenticated;

impl Robolt<Authenticated> {
    pub fn my_privacy(&self, setting: PrivacySetting) -> Result<Privacy, Error> {
        self.request_builder(format!(
            "{}/v1/{}",
            ENDPOINTS.account_settings,
            match setting {
                PrivacySetting::AppChat => "app-chat-privacy",
                PrivacySetting::GameChat => "game-chat-privacy",
                PrivacySetting::Trade => "trade-privacy",
                PrivacySetting::Inventory => "inventory-privacy",
                PrivacySetting::PrivateMessage => "private-message-privacy",
            }
        ))
            .send::<PrivacySettingResponse>()
            .map(|res| res.value)
    }
}

#[derive(Deserialize)]
struct PrivacySettingResponse {
    #[serde(
    alias = "appChatPrivacy",
    alias = "gameChatPrivacy",
    alias = "tradePrivacy",
    alias = "inventoryPrivacy",
    alias = "privateMessagePrivacy"
    )]
    value: Privacy,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub enum PrivacySetting {
    AppChat,
    GameChat,
    Trade,
    Inventory,
    PrivateMessage,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub enum Privacy {
    #[serde(alias = "All")]
    AllUsers,
    AllAuthenticatedUsers,
    #[serde(alias = "Followers")]
    FriendsFollowingAndFollowers,
    #[serde(alias = "Following")]
    FriendsAndFollowing,
    TopFriends,
    Friends,
    NoOne,
    Disabled,
    Undefined,
}