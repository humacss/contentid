//! Error type for content ID construction and verification.

use core::fmt;

use minbin::ToFromByteError;

/// Content ID construction errors.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    /// The value could not be serialized by minbin.
    Minbin(ToFromByteError),
    /// The computed ID did not match the expected ID.
    Mismatch,
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
            Self::Mismatch => write!(f, "content ID mismatch"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}
