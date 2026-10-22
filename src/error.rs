//! Error type for the MinBin-based content ID helpers.

use core::fmt;

use minbin::ToFromByteError;

/// Errors returned by MinBin-based content ID helpers.
///
/// Custom [`crate::ToContentId`] implementations choose their own error type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    /// The value could not be serialized by minbin.
    Minbin(ToFromByteError),
}

impl From<ToFromByteError> for Error {
    fn from(error: ToFromByteError) -> Self {
        Self::Minbin(error)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Minbin(error) => write!(f, "minbin serialization failed: {error}"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}
