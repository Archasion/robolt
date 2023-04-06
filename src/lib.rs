pub use api::account_information::{Gender, SocialsVisibility};
pub use api::account_settings::{Privacy, PrivacySetting, TradeValue};
pub use api::avatar::{
	AvatarItemFilter,
	AvatarItemType,
	AvatarScale,
	AvatarType,
	BodyColors,
	BrickColor,
	Outfit,
	OutfitType,
};
pub use api::friends::FriendshipStatus;
pub use api::presence::UserPresenceType;
pub use api::Limit;
pub use utils::client::Robolt;
pub use utils::errors;

mod api;
mod utils;