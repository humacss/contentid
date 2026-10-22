# contentid Agent Rules

This crate owns deterministic content IDs only. Keep serialization, hashing,
and ID representation focused and independently usable.

## Boundary

- Depend on `minbin` for canonical bytes and `blake3` for hashing.
- Keep the public API small, no_std-friendly, and auditable.
- Use `ToContentId` type-name domain separation for local Rust value IDs where
  both producer and consumer share the exact Rust type.
- Use explicit string identifiers through `id!(type = "TypeIdentifier", &value)` or
  the fallible `try_typed_content_id` function for shared protocol IDs that
  must be reproduced across crate boundaries.
- Keep derive macros out of v0.1 unless the user explicitly expands the scope.

## Verification

Run focused checks from this crate:

```sh
cargo test --manifest-path Cargo.toml
```
