//! Disposable local IDs over Rust [`Hash`] values.
//!
//! Uses [`DefaultHasher`] unless the caller supplies another [`Hasher`].
//! The built-in ID helpers and [`LocalContentHasher::finalize_u64`] return the
//! 64 bits exposed by [`Hasher::finish`]. To obtain a custom hasher's wider
//! output, pass it by mutable reference, let the builder go out of scope, and
//! call the hasher's own finishing method. Distributed IDs use the separate
//! 256-bit [`crate::ContentId`] API.
//! Neither Rust's hashing format nor its default algorithm is a stable protocol.

use core::hash::{Hash, Hasher};
use std::hash::DefaultHasher;

/// Incremental local ID builder using a caller-selected Rust hasher.
///
/// [`finalize_u64`](Self::finalize_u64) returns the standard 64-bit output.
/// Borrow a custom hasher through [`with_hasher`](Self::with_hasher) to retain
/// access to its algorithm-specific finishing methods after the builder goes
/// out of scope, including wider outputs where supported.
#[derive(Clone)]
pub struct LocalContentHasher<H = DefaultHasher>(H);

impl LocalContentHasher {
    /// Create a builder with Rust's default hasher and its fixed initial state.
    ///
    /// This is not a randomly seeded `HashMap` hasher. Results are disposable
    /// local identities, with no compatibility promise across Rust releases.
    pub fn new() -> Self {
        Self(DefaultHasher::new())
    }
}

impl<H: Hasher> LocalContentHasher<H> {
    /// Use the supplied hasher, preserving its seed and existing state.
    ///
    /// Pass `&mut hasher` to borrow it. Once the builder goes out of scope,
    /// use the original hasher's own finishing method for custom output types.
    pub fn with_hasher(hasher: H) -> Self {
        Self(hasher)
    }

    /// Pass bytes directly to the selected hasher.
    ///
    /// Write-boundary behavior is determined by that hasher. Callers are
    /// responsible for framing multiple fields to avoid ambiguous inputs.
    pub fn update_bytes(&mut self, bytes: &[u8]) {
        self.0.write(bytes);
    }

    /// Hash a Rust value without an intermediate serialization.
    ///
    /// This does not automatically include the Rust type name.
    pub fn update_value<T: Hash + ?Sized>(&mut self, value: &T) {
        value.hash(&mut self.0);
    }

    /// Finish using the selected hasher's standard 64-bit output.
    pub fn finalize_u64(self) -> u64 {
        self.0.finish()
    }
}

impl Default for LocalContentHasher {
    fn default() -> Self {
        Self::new()
    }
}

/// Build a disposable local 64-bit ID with Rust's default hasher.
///
/// Hashes the identifier followed by the value. Use [`LocalContentHasher::with_hasher`]
/// to choose a different hashing algorithm or seed.
pub fn local_typed_content_id_u64<T: Hash + ?Sized>(type_identifier: &str, value: &T) -> u64 {
    let mut hasher = LocalContentHasher::new();
    hasher.update_value(type_identifier);
    hasher.update_value(value);
    hasher.finalize_u64()
}
