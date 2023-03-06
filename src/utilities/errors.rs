use std::fmt;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct RoboltError {
	pub message: String,
	#[serde(default = "code_default")]
	pub code:    i8,
	#[serde(skip)]
	kind:        RoboltErrorKind,
}

fn code_default() -> i8 {
	-1
}

#[derive(Deserialize)]
pub(crate) struct RobloxAPIErrors {
	pub(crate) errors: Vec<RoboltError>,
}

#[derive(Debug)]
pub(crate) enum RoboltErrorKind {
	Unknown,
	Api,
}

#[doc(hidden)]
impl Default for RoboltErrorKind {
	fn default() -> Self {
		RoboltErrorKind::Api
	}
}

#[doc(hidden)]
impl fmt::Display for RoboltError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let kind = match self.kind {
			RoboltErrorKind::Api => format!("API Error (code: {})", self.code),
			RoboltErrorKind::Unknown => "Unknown".to_string(),
		};

		write!(f, "[Robolt] {}: {}", kind, self.message)
	}
}

#[doc(hidden)]
impl From<String> for RoboltError {
	fn from(message: String) -> Self {
		Self {
			kind: RoboltErrorKind::Unknown,
			code: code_default(),
			message,
		}
	}
}