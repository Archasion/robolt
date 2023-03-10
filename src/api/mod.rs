use serde::Deserialize;

pub(crate) mod account_information;
pub(crate) mod account_settings;
mod auth;
pub(crate) mod badges;
mod economy;
pub(crate) mod friends;
mod points;
mod premium_features;
pub(crate) mod presence;
mod users;

struct RobloxAPIEndpoints<T = &'static str> {
	account_information: T,
	account_settings:    T,
	premium_features:    T,
	presence:            T,
	economy:             T,
	friends:             T,
	points:              T,
	badges:              T,
	users:               T,
	base:                T,
}

#[derive(Debug, Deserialize)]
struct DataResponse<T> {
	data: Vec<T>,
}

#[derive(Deserialize)]
struct CountResponse<T> {
	count: T,
}

const ENDPOINTS: RobloxAPIEndpoints = RobloxAPIEndpoints {
	account_information: "accountinformation.roblox.com",
	account_settings:    "accountsettings.roblox.com",
	premium_features:    "premiumfeatures.roblox.com",
	presence:            "presence.roblox.com",
	economy:             "economy.roblox.com",
	friends:             "friends.roblox.com",
	points:              "points.roblox.com",
	badges:              "badges.roblox.com",
	users:               "users.roblox.com",
	base:                "api.roblox.com",
};
