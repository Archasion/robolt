use serde::Deserialize;

use crate::api::users::PartialUser;
use crate::api::ENDPOINTS;
use crate::errors::RoboltError;
use crate::utils::client::Authenticated;
use crate::Robolt;

impl Robolt<Authenticated> {
	pub async fn privacy(&self, setting: PrivacySetting) -> Result<PrivacyState, RoboltError> {
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
		.await
		.map(|res| res.value)
	}

	pub async fn blocked_users(&self) -> Result<BlockedUsers, RoboltError> {
		self.request_builder(format!(
			"{}/v1/users/get-detailed-blocked-users",
			ENDPOINTS.account_settings
		))
		.send::<BlockedUsers>()
		.await
	}

	pub async fn email(&self) -> Result<Email, RoboltError> {
		self.request_builder(format!("{}/v1/email", ENDPOINTS.account_settings))
			.send::<Email>()
			.await
	}

	pub async fn trade_value(&self) -> Result<TradeValue, RoboltError> {
		self.request_builder(format!("{}/v1/trade-value", ENDPOINTS.account_settings))
			.send::<TradeValueResponse>()
			.await
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
	value: PrivacyState,
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
pub enum PrivacyState {
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