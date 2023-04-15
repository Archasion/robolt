use std::fmt;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct RoboltError {
	pub message: String,
	#[serde(default = "default_error_code")]
	pub code: i8,
	#[serde(skip)]
	kind: RoboltErrorKind,
}

fn default_error_code() -> i8 {
	-1
}

#[derive(Deserialize)]
pub(crate) struct RobloxAPIErrors {
	pub(crate) errors: Vec<RoboltError>,
}

#[derive(Debug, Default)]
pub(crate) enum RoboltErrorKind {
	#[default]
	Api,
	Unknown,
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
	fn from(error: String) -> Self {
		Self {
			kind: RoboltErrorKind::Unknown,
			code: default_error_code(),
			message: error,
		}
	}
}

#[doc(hidden)]
impl From<reqwest::Error> for RoboltError {
	fn from(error: reqwest::Error) -> Self {
		Self {
			kind: RoboltErrorKind::Unknown,
			code: default_error_code(),
			message: error.to_string(),
		}
	}
}
