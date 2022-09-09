use std::fmt::Display;

/// Error type for parsing function
///
/// holds a string describing the error
#[derive(Debug, PartialEq, Eq)]
pub struct FuzJsonParseError(pub String);

impl Display for FuzJsonParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// result type for parser where error type is [FuzJsonParseError]
pub type Result<T> = std::result::Result<T, FuzJsonParseError>;

impl std::error::Error for FuzJsonParseError {}

impl<S: Into<String>> From<S> for FuzJsonParseError {
    fn from(from_str_ref: S) -> Self {
        FuzJsonParseError(from_str_ref.into())
    }
}
