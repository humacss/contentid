# Changelog

## 0.1.0 — Unreleased

Initial release candidate.

- Distributed 256-bit BLAKE3 content IDs over raw bytes or MinBin-encoded values.
- Explicit protocol identifiers and Rust type-name-based identity.
- Optional 128-bit and numeric views of distributed IDs.
- Incremental hashing and fallible serialization APIs.
- Optional 64-bit local IDs using Rust's `DefaultHasher` or a caller-supplied hasher.
- Optional Serde support for ID storage types.
- `no_std` support with allocation.

Requires Rust 1.90 or later and MinBin 1.0. Distributed compatibility is checked
with fixed encoding and digest vectors. Local IDs are disposable and carry no
cross-build or cross-platform compatibility guarantee.
