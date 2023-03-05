use serde::Deserialize;

use crate::api::ENDPOINTS;
use crate::api::users::PartialUser;
use crate::errors::RoboltError;
use crate::Robolt;
use crate::utilities::client::Authenticated;

impl Robolt<Authenticated> {
    pub fn my_privacy(&self, setting: PrivacySetting) -> Result<Privacy, RoboltError> {
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

    pub fn my_blocked_users(&self) -> Result<BlockedUsers, RoboltError> {
        self.request_builder(format!(
            "{}/v1/users/get-detailed-blocked-users",
            ENDPOINTS.account_settings
        ))
            .send::<BlockedUsers>()
    }

    pub fn my_email(&self) -> Result<Email, RoboltError> {
        self.request_builder(format!("{}/v1/email", ENDPOINTS.account_settings))
            .send::<Email>()
    }

    pub fn my_trade_value(&self) -> Result<TradeValue, RoboltError> {
        self.request_builder(format!("{}/v1/trade-value", ENDPOINTS.account_settings))
            .send::<TradeValueResponse>()
            .map(|res| res.trade_value)
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct TradeValueResponse {
    trade_value: TradeValue,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub enum TradeValue {
    High,
    Medium,
    Low,
    None,
    Undefined,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Email {
    pub email_address: String,
    pub verified: bool,
    pub can_bypass_password_for_email_update: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockedUsers {
    pub blocked_users: Vec<PartialUser>,
    pub max_blocked_users: u8,
    pub total: u8,
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
    #[serde(alias = "All", alias = "AllUsers")]
    Everyone,
    AllAuthenticatedUsers,
    FriendsFollowingAndFollowers,
    FriendsAndFollowing,
    Followers,
    Following,
    TopFriends,
    Friends,
    NoOne,
    Disabled,
    Undefined,
}