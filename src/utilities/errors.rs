use std::fmt;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct RoboltError {
    pub message: String,
    #[serde(default = "code_default")]
    pub code: i8,
    #[serde(skip)]
    kind: ErrorKind,
}

fn code_default() -> i8 {
    -1
}

#[derive(Deserialize)]
pub(crate) struct RobloxAPIErrors {
    pub(crate) errors: Vec<RoboltError>,
}

#[derive(Debug)]
pub(crate) enum ErrorKind {
    Unknown,
    Api,
}

#[doc(hidden)]
impl Default for ErrorKind {
    fn default() -> Self {
        ErrorKind::Api
    }
}

#[doc(hidden)]
impl fmt::Display for RoboltError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let kind = match self.kind {
            ErrorKind::Api => format!("API Error (code: {})", self.code),
            ErrorKind::Unknown => "Unknown".to_string(),
        };

        write!(f, "[Robolt] {}: {}", kind, self.message)
    }
}

#[doc(hidden)]
impl From<String> for RoboltError {
    fn from(message: String) -> Self {
        Self {
            kind: ErrorKind::Unknown,
            code: code_default(),
            message,
        }
    }
}