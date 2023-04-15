use serde_repr::Deserialize_repr;

pub mod account_information;
pub mod account_settings;
pub mod avatar;
pub mod badges;
pub mod friends;
pub mod presence;
pub mod routes;
pub mod users;

// Nothing to export
mod auth;
mod economy;
mod points;
mod premium_features;

#[derive(Default, Debug, Deserialize_repr, Clone, PartialEq)]
#[repr(u8)]
/// Limits the number of items returned by the API
///
/// ### Return up to...
///
/// * **10** items (Min)
/// * **25** items (Low)
/// * **50** items (High)
/// * **100** items (Max)
pub enum Limit {
	#[default]
	/// Return up to 10 items
	Min = 10,
	/// Return up to 25 items
	Low = 25,
	/// Return up to 50 items
	High = 50,
	/// Return up to 100 items
	Max = 100,
}