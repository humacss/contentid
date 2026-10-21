# contentid

Deterministic content IDs for minbin-encoded Rust values.

`contentid` is intentionally small and infrastructure-only:

- `ContentId`: 256-bit BLAKE3 ID.
- `ContentId128`: first 128 bits of a `ContentId`.
- `ToContentId`: blanket implementation for `minbin::ToFromBytes` values.
- `id!`: canonical ID from a single value, an explicit stable type string plus
  one value, or a tuple-like list of values.

With the optional `local` feature, `LocalContentHasher` and
`local_typed_content_id_u128` provide faster GxHash-128 IDs directly from
Rust `Hash` implementations, without MinBin serialization or an intermediate
byte allocation. These IDs are intended only for process-local state that is
discarded with its owner. They are not persistence or protocol identities.

The crate is `no_std` with `alloc`.

## Raw bytes

Raw bytes can be hashed directly:

```rust
let id = contentid::ContentId::from_bytes(b"example bytes");
```

This is the right API when the caller already owns a canonical byte
representation.

## Rust-local value IDs

Rust values use type-name domain separation plus their minbin bytes:

```rust
use contentid::ToContentId;

let id = payload.content_id();
payload.verify(id)?;
# Ok::<(), contentid::Error>(())
```

Use this only when the producer and consumer share the same Rust type path.
`core::any::type_name::<T>()` is intentionally part of the hash input.

## Shared protocol IDs

Shared protocol values should use an explicit string identifier instead of the
Rust type path:

```rust
let id = contentid::id!("Record.v1", &value);
```

That identifier is part of the content. Keep it stable across crates,
processes, and protocol versions.

## Integer views

`ContentId` is the canonical ID form. Integer IDs are deterministic lossy views
over fixed digest prefixes, decoded with `minbin` so the result is
platform-agnostic:

```rust
let full = contentid::typed_content_id("Record.v1", &record);
let record_id = i64::from(full);

let record_id = contentid::typed_content_id_i64("Record.v1", &record);
```

Use integer IDs only where a protocol or storage layer needs that
shape. The full `ContentId` keeps the complete BLAKE3-256 digest.

Current integer views:

- `i64` and `u64`: first 8 digest bytes decoded as the target primitive.
- `u128`: first 16 digest bytes decoded as the target primitive.

## Boundaries

This crate owns deterministic content identity. Callers define the data format
and choose whether to use `ContentId` or a numeric view.
