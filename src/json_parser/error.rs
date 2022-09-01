use std::fmt::Display;

/// Error type for parsing function
///
/// holds a string describing the error
#[derive(Debug, PartialEq, Eq)]
pub struct JsonParseError(pub String);

impl Display for JsonParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}


/// result type for parser where error type is [JsonParseError]
pub type Result<T> = std::result::Result<T, JsonParseError>;

impl std::error::Error for JsonParseError {}

impl<S:Into<String>> From<S> for JsonParseError {
    fn from(from_str_ref: S) -> Self {
        JsonParseError(from_str_ref.into())
    }
}