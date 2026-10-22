//! Explicitly opted-in Rust type-based content IDs with caller-chosen encoding.

use crate::{ContentHasher, ContentId, ContentId128};

/// Builds deterministic content IDs using a caller-chosen serialization format.
///
/// Implement this trait explicitly for each type that needs these methods.
/// MinBin is the default helper: implementations using MinBin can delegate
/// [`write_content`](Self::write_content) to [`ContentHasher::update_value`].
/// Other encoders can use [`ContentHasher::update_bytes`] and return their own
/// error type. Their bytes must be deterministic; cross-platform reproduction
/// also requires a platform-agnostic encoding. Rust type names are not stable
/// protocol identifiers: use an explicit identifier for distributed protocols.
///
/// The default [`try_content_id`](Self::try_content_id) hashes the Rust type
/// name, a zero byte, then the encoded content. Override it to choose a different
/// identity scheme. The other default methods delegate to that result.
///
/// Merely supporting MinBin does not opt a type into this trait:
///
/// ```compile_fail
/// use contentid::ToContentId;
/// let _ = 42_u64.content_id();
/// ```
pub trait ToContentId {
    /// The encoding error returned by this implementation.
    type Error;

    /// Write the encoded content, without a type-name prefix.
    ///
    /// The default ID method adds the prefix before calling this method.
    /// MinBin implementations can call `hasher.update_value(self)`; other
    /// serializers can encode the value and call `hasher.update_bytes(&bytes)`.
    fn write_content(&self, hasher: &mut ContentHasher) -> Result<(), Self::Error>;

    /// Build a 256-bit ID with Rust type-name separation by default.
    ///
    /// Encoding errors are returned unchanged. Override this method to replace
    /// the default identity scheme without a blanket implementation conflict.
    fn try_content_id(&self) -> Result<ContentId, Self::Error> {
        let mut hasher = ContentHasher::new();
        hasher.update_bytes(core::any::type_name::<Self>().as_bytes());
        hasher.update_bytes(&[0]);
        self.write_content(&mut hasher)?;
        Ok(hasher.finalize())
    }

    /// Build a 256-bit content ID, panicking if serialization fails.
    ///
    /// # Panics
    ///
    /// Panics when this implementation's serialization fails. Use
    /// [`try_content_id`](Self::try_content_id) to handle encoding errors.
    fn content_id(&self) -> ContentId {
        self.try_content_id()
            .unwrap_or_else(|_| panic!("content serialization failed for content_id"))
    }

    /// Try to build a 128-bit truncated content ID.
    fn try_content_id_128(&self) -> Result<ContentId128, Self::Error> {
        Ok(ContentId128::from_content_id(self.try_content_id()?))
    }

    /// Build a 128-bit truncated content ID, panicking if serialization fails.
    ///
    /// # Panics
    ///
    /// Panics when this implementation's serialization fails. Use
    /// [`try_content_id_128`](Self::try_content_id_128) to handle encoding errors.
    fn content_id_128(&self) -> ContentId128 {
        self.try_content_id_128()
            .unwrap_or_else(|_| panic!("content serialization failed for content_id_128"))
    }
}
