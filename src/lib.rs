#![no_std]

//! Deterministic content IDs for minbin-encoded Rust values.
//!
//! `ContentId::from_bytes` hashes raw bytes directly. The [`ToContentId`]
//! blanket implementation hashes `core::any::type_name::<T>()`, a zero byte,
//! and the value's minbin encoding. That gives Rust-type domain separation for
//! Rust-local value IDs. A Rust type name is not a unique binary identifier
//! and is not guaranteed to remain stable across compiler versions.
//!
//! Shared protocols should pass an explicit stable type identifier into
//! [`id!`] or use [`typed_content_id`] instead of relying on a Rust type path.
//! This keeps IDs reproducible across crates and binaries that use different
//! concrete Rust types for the same protocol payload. Producers must agree on
//! both the identifier and the exact MinBin encoding; matching semantic values
//! alone does not guarantee matching IDs.
//!
//! ```
//! use contentid::{ContentId, ToContentId, typed_content_id};
//!
//! let bytes_id = ContentId::from_bytes(b"example bytes");
//! let protocol_id = typed_content_id("example.Counter.v1", &42_u64);
//! let rust_id = 42_u64.content_id();
//! assert_eq!(protocol_id, contentid::id!(type = "example.Counter.v1", &42_u64));
//! assert_ne!(bytes_id, rust_id);
//! ```
//!
//! With the optional `local` feature, [`LocalContentHasher`] hashes Rust
//! `core::hash::Hash` values with Rust's `DefaultHasher`, or a caller-supplied
//! hasher. Its standard output is 64 bits. Use it for disposable local state;
//! Rust's hashing format and default algorithm are not stable protocols.
//!
//! `ContentId` is the canonical 256-bit ID. Integer forms such as `i64` and
//! `u128` are deterministic, lossy views over fixed digest prefixes decoded
//! with minbin. They are useful for systems that already require numeric keys,
//! but they do not replace the full digest when collision resistance matters.
//!
//! The crate is `no_std` with `alloc`. Callers define their own content formats
//! and choose the appropriate identity scope.

extern crate alloc;
pub extern crate minbin;
#[cfg(feature = "std")]
extern crate std;

mod content_id;
mod error;
mod hasher;
#[cfg(feature = "local")]
mod local;
mod macros;
mod typed;
mod value;

pub use content_id::{ContentId, ContentId128};
pub use error::Error;
pub use hasher::ContentHasher;
#[cfg(feature = "local")]
pub use local::{LocalContentHasher, local_typed_content_id_u64};
pub use typed::{
    try_typed_content_id, try_typed_content_id_128, typed_content_id, typed_content_id_128,
    typed_content_id_i64, typed_content_id_u64, typed_content_id_u128,
};
pub use value::ToContentId;
