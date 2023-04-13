use serde::Deserialize;
use serde_repr::Deserialize_repr;

pub mod account_information;
pub mod account_settings;
mod auth;
pub mod avatar;
pub mod badges;
pub mod economy;
pub mod friends;
pub mod points;
pub mod premium_features;
pub mod presence;
pub mod users;

struct RobloxAPIEndpoints<T = &'static str> {
	account_information: T,
	account_settings: T,
	premium_features: T,
	presence: T,
	economy: T,
	friends: T,
	points: T,
	badges: T,
	avatar: T,
	users: T,
}

#[derive(Default, Debug, Deserialize_repr)]
#[repr(u8)]
pub enum Limit {
	#[default]
	Min = 10,
	Low = 25,
	High = 50,
	Max = 100,
}

#[derive(Deserialize)]
struct DataResponse<T> {
	data: Vec<T>,
}

#[derive(Deserialize)]
struct CountResponse<T> {
	count: T,
}

#[derive(Deserialize)]
struct EmptyResponse {}

const ENDPOINTS: RobloxAPIEndpoints = RobloxAPIEndpoints {
	account_information: "accountinformation.roblox.com",
	account_settings: "accountsettings.roblox.com",
	premium_features: "premiumfeatures.roblox.com",
	presence: "presence.roblox.com",
	economy: "economy.roblox.com",
	friends: "friends.roblox.com",
	points: "points.roblox.com",
	badges: "badges.roblox.com",
	avatar: "avatar.roblox.com",
	users: "users.roblox.com",
};
