use std::io::Error;

use serde::Deserialize;

use crate::models::ENDPOINTS;
use crate::models::users::PartialUser;
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
            .function("my_privacy")
            .send::<PrivacySettingResponse>()
            .map(|res| res.value)
    }

    pub fn my_blocked_users(&self) -> Result<BlockedUsers, Error> {
        self.request_builder(format!(
            "{}/v1/users/get-detailed-blocked-users",
            ENDPOINTS.account_settings
        ))
            .function("my_blocked_users")
            .send::<BlockedUsers>()
    }
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockedUsers {
    blocked_users: Vec<PartialUser>,
    max_blocked_users: u8,
    total: u8,
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