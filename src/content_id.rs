//! Content ID storage types and deterministic numeric views.

use alloc::{format, string::String};
use core::fmt;

use minbin::ToFromBytes;

const I64_DIGEST_BYTES: usize = 8;
const U64_DIGEST_BYTES: usize = 8;
const U128_DIGEST_BYTES: usize = 16;

/// A 256-bit BLAKE3 content ID.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ContentId([u8; 32]);

impl ContentId {
    /// The all-zero content ID.
    pub const ZERO: Self = Self([0; 32]);

    /// Hash raw bytes with BLAKE3-256.
    pub fn from_bytes(data: &[u8]) -> Self {
        let hash = blake3::hash(data);
        Self(*hash.as_bytes())
    }

    /// Create an ID from an already-computed 32-byte digest.
    pub const fn from_hash_bytes(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    /// Borrow the raw 32-byte digest.
    pub const fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }

    /// Return the lower-case hex encoding.
    pub fn to_hex(&self) -> String {
        format!("{self}")
    }
}

impl fmt::Debug for ContentId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ContentId({self})")
    }
}

impl fmt::Display for ContentId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write_hex(f, &self.0)
    }
}

/// Decode the first 8 digest bytes as an `i64` with minbin's primitive format.
impl From<&ContentId> for i64 {
    fn from(id: &ContentId) -> Self {
        decode_digest_prefix(i64::default, &id.as_bytes()[..I64_DIGEST_BYTES])
    }
}

impl From<ContentId> for i64 {
    fn from(id: ContentId) -> Self {
        Self::from(&id)
    }
}

/// Decode the first 8 digest bytes as a `u64` with minbin's primitive format.
impl From<&ContentId> for u64 {
    fn from(id: &ContentId) -> Self {
        decode_digest_prefix(u64::default, &id.as_bytes()[..U64_DIGEST_BYTES])
    }
}

impl From<ContentId> for u64 {
    fn from(id: ContentId) -> Self {
        Self::from(&id)
    }
}

/// Decode the first 16 digest bytes as a `u128` with minbin's primitive format.
impl From<&ContentId> for u128 {
    fn from(id: &ContentId) -> Self {
        decode_digest_prefix(u128::default, &id.as_bytes()[..U128_DIGEST_BYTES])
    }
}

impl From<ContentId> for u128 {
    fn from(id: ContentId) -> Self {
        Self::from(&id)
    }
}

/// A 128-bit convenience truncation of a 256-bit content ID.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ContentId128([u8; 16]);

impl ContentId128 {
    /// The all-zero 128-bit content ID.
    pub const ZERO: Self = Self([0; 16]);

    /// Truncate a 256-bit content ID to its first 16 bytes.
    pub fn from_content_id(id: ContentId) -> Self {
        let mut bytes = [0; 16];
        bytes.copy_from_slice(&id.as_bytes()[..16]);
        Self(bytes)
    }

    /// Hash raw bytes with BLAKE3-256 and truncate to 128 bits.
    pub fn from_bytes(data: &[u8]) -> Self {
        Self::from_content_id(ContentId::from_bytes(data))
    }

    /// Create an ID from an already-computed 16-byte digest.
    pub const fn from_hash_bytes(bytes: [u8; 16]) -> Self {
        Self(bytes)
    }

    /// Borrow the raw 16-byte digest.
    pub const fn as_bytes(&self) -> &[u8; 16] {
        &self.0
    }

    /// Return this ID as a big-endian `u128`.
    pub const fn as_u128(&self) -> u128 {
        u128::from_be_bytes(self.0)
    }

    /// Return the lower-case hex encoding.
    pub fn to_hex(&self) -> String {
        format!("{self}")
    }
}

impl fmt::Debug for ContentId128 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ContentId128({self})")
    }
}

impl fmt::Display for ContentId128 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write_hex(f, &self.0)
    }
}

fn decode_digest_prefix<'a, T>(init: impl FnOnce() -> T, bytes: &'a [u8]) -> T
where
    T: ToFromBytes<'a>,
{
    minbin::from_bytes(init, bytes)
        .expect("fixed-size primitive digest prefixes must be valid minbin values")
}

fn write_hex(f: &mut fmt::Formatter<'_>, bytes: &[u8]) -> fmt::Result {
    for byte in bytes {
        write!(f, "{byte:02x}")?;
    }
    Ok(())
}
