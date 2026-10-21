//! Blanket trait for Rust-local value content IDs.

use minbin::ToFromBytes;

use crate::{ContentId, ContentId128, Error};

/// Builds deterministic content IDs from values.
pub trait ToContentId {
    /// Try to build a 256-bit content ID.
    fn try_content_id(&self) -> Result<ContentId, Error>;

    /// Build a 256-bit content ID, panicking if serialization fails.
    ///
    /// # Panics
    ///
    /// Panics when this value cannot be serialized with minbin. Use
    /// [`try_content_id`](Self::try_content_id) when serialization failure must
    /// be handled.
    fn content_id(&self) -> ContentId {
        self.try_content_id()
            .expect("minbin serialization failed for content_id")
    }

    /// Try to build a 128-bit truncated content ID.
    fn try_content_id_128(&self) -> Result<ContentId128, Error> {
        Ok(ContentId128::from_content_id(self.try_content_id()?))
    }

    /// Build a 128-bit truncated content ID, panicking if serialization fails.
    ///
    /// # Panics
    ///
    /// Panics when this value cannot be serialized with minbin. Use
    /// [`try_content_id_128`](Self::try_content_id_128) when serialization
    /// failure must be handled.
    fn content_id_128(&self) -> ContentId128 {
        self.try_content_id_128()
            .expect("minbin serialization failed for content_id_128")
    }

    /// Verify that this value currently has the expected content ID.
    fn verify(&self, expected: ContentId) -> Result<(), Error> {
        if self.try_content_id()? == expected {
            Ok(())
        } else {
            Err(Error::Mismatch)
        }
    }
}

impl<T> ToContentId for T
where
    T: for<'a> ToFromBytes<'a>,
{
    fn try_content_id(&self) -> Result<ContentId, Error> {
        let bytes = minbin::to_bytes(self)?;
        let mut hasher = blake3::Hasher::new();
        hasher.update(core::any::type_name::<T>().as_bytes());
        hasher.update(&[0]);
        hasher.update(&bytes);
        Ok(ContentId::from_hash_bytes(*hasher.finalize().as_bytes()))
    }
}
