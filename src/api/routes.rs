use std::ops::Deref;

pub enum RobloxApi {
	Presence,
	Users,
	AccountInformation,
	AccountSettings,
	Economy,
	Friends,
	Points,
	Badges,
	Avatar,
	PremiumFeatures,
	Auth,
	Custom(&'static str),
}

impl RobloxApi {
	pub(crate) fn url(&self) -> &str {
		match self {
			RobloxApi::Presence => "presence.roblox.com",
			RobloxApi::Users => "users.roblox.com",
			RobloxApi::AccountInformation => "accountinformation.roblox.com",
			RobloxApi::AccountSettings => "accountsettings.roblox.com",
			RobloxApi::Economy => "economy.roblox.com",
			RobloxApi::Friends => "friends.roblox.com",
			RobloxApi::Points => "points.roblox.com",
			RobloxApi::Badges => "badges.roblox.com",
			RobloxApi::Avatar => "avatar.roblox.com",
			RobloxApi::PremiumFeatures => "premiumfeatures.roblox.com",
			RobloxApi::Auth => "auth.roblox.com",
			RobloxApi::Custom(s) => s.deref(),
		}
	}
}