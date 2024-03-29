use serde::Deserialize;
use serde_repr::Deserialize_repr;

use crate::api::routes::RobloxApi;
use crate::errors::RoboltError;
use crate::utils::client::Authenticated;
use crate::utils::response::CountResponse;
use crate::Robolt;

impl<State> Robolt<State> {
	pub async fn roblox_badges(&self, user_id: u64) -> Result<Vec<RobloxBadge>, RoboltError> {
		self.request(
			RobloxApi::AccountInformation,
			format!("/v1/users/{user_id}/roblox-badges"),
		)
		.send()
		.await
	}

	pub async fn user_socials(&self, user_id: u64) -> Result<UserSocials, RoboltError> {
		self.request(
			RobloxApi::AccountInformation,
			format!("/v1/users/{user_id}/promotion-channels"),
		)
		.send()
		.await
	}
}

impl Robolt<Authenticated> {
	pub async fn socials_auth(&self) -> Result<AuthenticatedUserSocials, RoboltError> {
		self.request(RobloxApi::AccountInformation, "/v1/promotion-channels")
			.send()
			.await
	}

	pub async fn birthdate(&self) -> Result<Birthdate, RoboltError> {
		self.request(RobloxApi::AccountInformation, "/v1/birthdate")
			.send()
			.await
	}

	pub async fn blurb(&self) -> Result<String, RoboltError> {
		self.request(RobloxApi::AccountInformation, "/v1/description")
			.send::<ProfileDescription>()
			.await
			.map(|res| res.description)
	}

	pub async fn gender(&self) -> Result<Gender, RoboltError> {
		self.request(RobloxApi::AccountInformation, "/v1/gender")
			.send::<GenderResponse>()
			.await
			.map(|res| res.gender)
	}

	pub async fn consecutive_xbox_logins(&self) -> Result<u16, RoboltError> {
		self.request(RobloxApi::AccountInformation, "/v1/xbox-live/consecutive-login-days")
			.send::<CountResponse<u16>>()
			.await
			.map(|res| res.count)
	}

	pub async fn phone_number(&self) -> Result<PhoneNumber, RoboltError> {
		self.request(RobloxApi::AccountInformation, "/v1/phone").send().await
	}
}

#[derive(Debug, Clone, PartialEq, Deserialize_repr)]
#[repr(u8)]
pub enum Gender {
	Unknown = 1,
	Male = 2,
	Female = 3,
}

#[derive(Deserialize)]
struct GenderResponse {
	gender: Gender,
}

#[derive(Deserialize)]
struct ProfileDescription {
	description: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PhoneNumber {
	pub country_code: String,
	pub prefix: String,
	pub phone: String,
	pub is_verified: bool,
	pub verification_code_length: u8,
	pub can_bypass_password_for_phone_update: bool,
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

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthenticatedUserSocials {
	#[serde(rename = "promotionChannelsVisibilityPrivacy")]
	pub visibility: SocialsVisibility,
	#[serde(flatten)]
	pub connections: UserSocials,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub enum SocialsVisibility {
	AllUsers,
	FriendsFollowingAndFollowers,
	FriendsAndFollowing,
	Friends,
	NoOne,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Birthdate {
	#[serde(rename = "birthYear")]
	pub year: u16,
	#[serde(rename = "birthMonth")]
	pub month: u8,
	#[serde(rename = "birthDay")]
	pub day: u8,
}