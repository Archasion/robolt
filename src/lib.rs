pub use api::account_information::{Gender, SocialsVisibility};
pub use api::account_settings::{Privacy, PrivacySetting, TradeValue};
pub use api::friends::FriendshipStatus;
pub use api::presence::UserPresenceType;
pub use utilities::client::Robolt;
pub use utilities::errors;

mod api;
mod utilities;
