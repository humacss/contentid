//! Incremental hashing over caller-encoded bytes and MinBin values.

use minbin::ToFromBytes;

use crate::{ContentId, ContentId128, Error};

/// Incremental content ID builder.
#[derive(Clone)]
pub struct ContentHasher(blake3::Hasher);

impl ContentHasher {
    /// Create an empty BLAKE3 content hasher.
    pub fn new() -> Self {
        Self(blake3::Hasher::new())
    }

    /// Add raw bytes to the hash stream.
    pub fn update_bytes(&mut self, bytes: &[u8]) {
        self.0.update(bytes);
    }

    /// Serialize a value with MinBin, the default encoding helper.
    ///
    /// For another serializer, encode the value first and pass its bytes to
    /// [`update_bytes`](Self::update_bytes).
    pub fn update_value<T>(&mut self, value: &T) -> Result<(), Error>
    where
        T: for<'a> ToFromBytes<'a>,
    {
        self.update_bytes(&minbin::to_bytes(value)?);
        Ok(())
    }

    /// Finish as a 256-bit content ID.
    pub fn finalize(self) -> ContentId {
        ContentId::from_hash_bytes(*self.0.finalize().as_bytes())
    }

    /// Finish as a 128-bit truncated content ID.
    pub fn finalize_128(self) -> ContentId128 {
        ContentId128::from_content_id(self.finalize())
    }

    /// Finish as a deterministic signed 64-bit view of the content ID.
    pub fn finalize_i64(self) -> i64 {
        i64::from(self.finalize())
    }

    /// Finish as a deterministic unsigned 64-bit view of the content ID.
    pub fn finalize_u64(self) -> u64 {
        u64::from(self.finalize())
    }

    /// Finish as a deterministic unsigned 128-bit view of the content ID.
    pub fn finalize_u128(self) -> u128 {
        u128::from(self.finalize())
    }
}

impl Default for ContentHasher {
    fn default() -> Self {
        Self::new()
    }
}
