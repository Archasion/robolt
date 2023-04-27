use serde::Deserialize;

use crate::api::routes::RobloxApi;
use crate::errors::RoboltError;
use crate::utils::client::Authenticated;
use crate::Robolt;

impl Robolt<Authenticated> {
	pub async fn chat_settings(&self) -> Result<ChatSettings, RoboltError> {
		self.request(RobloxApi::Chat, "/v2/chat-settings").send().await
	}
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatSettings {
	#[serde(rename = "chatEnabled")]
	pub is_enabled: bool,
	pub is_active_chat_user: bool,
	pub is_connect_tab_enabled: bool,
}