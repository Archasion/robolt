use serde::Deserialize;

use crate::api::routes::RobloxApi;
use crate::errors::RoboltError;
use crate::utils::client::Authenticated;
use crate::utils::response::CountResponse;
use crate::Robolt;

impl Robolt<Authenticated> {
	pub async fn chat_settings(&self) -> Result<ChatSettings, RoboltError> {
		self.request(RobloxApi::Chat, "/v2/chat-settings").send().await
	}

	pub async fn conversations(&self, conversation_ids: Vec<u64>) -> Result<Vec<Conversation>, RoboltError> {
		let mut query = String::new();
		for id in conversation_ids {
			query.push_str(&format!("conversationIds={id}&"));
		}
		self.request(RobloxApi::Chat, format!("/v2/get-conversations?{query}"))
			.send()
			.await
	}

	pub async fn messages(
		&self,
		conversation_id: u64,
		page_size: u64,
		exclusive_start_message_id: Option<String>,
	) -> Result<Vec<ChatMessage>, RoboltError> {
		let mut query = format!("conversationId={}&pageSize={}", conversation_id, page_size);
		if let Some(id) = exclusive_start_message_id {
			query.push_str(&format!("&exclusiveStartMessageId={id}"));
		}

		self.request(RobloxApi::Chat, format!("/v2/get-messages?{query}"))
			.send()
			.await
	}

	pub async fn rollout_settings(&self, feature_names: Vec<String>) -> Result<RolloutFeatures, RoboltError> {
		let mut query = String::new();
		for name in feature_names {
			query.push_str(&format!("featureNames={name}&"));
		}

		self.request(RobloxApi::Chat, format!("/v2/get-rollout-settings?{query}"))
			.send()
			.await
	}

	pub async fn unread_conversation_count(&self) -> Result<u64, RoboltError> {
		self.request(RobloxApi::Chat, "/v2/get-unread-conversation-count")
			.send::<CountResponse<u64>>()
			.await
			.map(|res| res.count)
	}

	pub async fn unread_messages(
		&self,
		conversation_ids: Vec<u64>,
		page_size: u64,
	) -> Result<Vec<UnreadMessages>, RoboltError> {
		let mut query = format!("pageSize={page_size}");
		for id in conversation_ids {
			query.push_str(&format!("&conversationIds={id}"));
		}

		self.request(RobloxApi::Chat, format!("/v2/get-unread-messages?{query}"))
			.send()
			.await
	}
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UnreadMessages {
	conversation_id: u64,
	#[serde(rename = "chatMessages")]
	messages: Vec<ChatMessage>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RolloutSettings {
	feature_name: String,
	is_rollout_enabled: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RolloutFeatures {
	rollout_features: Vec<RolloutSettings>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatMessage {
	pub id: String,
	pub sender_type: ChatParticipantType,
	pub sent: String,
	pub read: bool,
	pub message_type: ChatMessageType,
	pub decorators: Vec<String>,
	pub sender_target_id: u64,
	pub content: String,
	pub link: Option<Link>,
	pub event_based: Option<EventBased>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub enum ChatMessageType {
	PlainText,
	Link,
	EventBased,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Link {
	#[serde(rename = "type")]
	pub link_type: String,
	pub game: GameLink,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameLink {
	universe_id: u64,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EventBased {
	#[serde(rename = "type")]
	pub event_type: String,
	pub set_conversation_universe: SetConversationUniverseEventBased,
}

pub enum ChatMessageEventType {
	SetConversationUniverse,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetConversationUniverseEventBased {
	pub actor_user_id: u64,
	pub universe_id: u64,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Conversation {
	pub id: u64,
	pub title: String,
	pub initiator: ChatParticipant,
	pub has_unread_messages: bool,
	pub participants: Vec<ChatParticipant>,
	pub conversation_type: ConversationType,
	pub conversation_title: ConversationTitle,
	pub last_updated: String,
	pub conversation_universe: Option<ConversationUniverse>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatParticipant {
	#[serde(rename = "type")]
	pub participant_type: ChatParticipantType,
	pub target_id: u64,
	pub name: String,
	pub display_name: String,
	pub has_verified_badge: bool,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConversationUniverse {
	universe_id: String,
	root_place_id: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConversationTitle {
	title_for_viewer: String,
	is_default_title: bool,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub enum ChatParticipantType {
	User,
	System,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub enum ConversationType {
	#[serde(rename = "OneToOneConversation")]
	OneToOne,
	#[serde(rename = "MultiUserConversation")]
	MultiUser,
	#[serde(rename = "CloudEditConversation")]
	CloudEdit,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatSettings {
	#[serde(rename = "chatEnabled")]
	pub is_enabled: bool,
	pub is_active_chat_user: bool,
	pub is_connect_tab_enabled: bool,
}