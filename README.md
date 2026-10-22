# contentid

Deterministic, platform-agnostic, content-derived, decentralized ID generator.

`contentid` is a deterministic content ID generator for Rust. When two systems
work with the same content, they can independently calculate the same ID.
Neither needs to ask the other which ID it generated or coordinate with a
central service first.

The ID comes from the content itself. That makes it useful for recognizing
the same message, identifying shared data, or checking whether a value has
changed. You decide which data defines the identity; `contentid` calculates
the ID from that data.

The main purpose of this library is **distributed IDs**: letting independent
programs agree on identity from content, across machines and platforms.

It also provides local IDs for projects that need both. You can keep ID
generation in one library and follow the same pattern for each: give the
content an identifier, hash the value, and use the result where it belongs.

## Distributed IDs

Sometimes different programs or platforms need to agree on the ID without
coordinating with each other.

Distributed IDs use [minbin](https://github.com/humacss/minbin) to produce a
platform-agnostic binary representation, then hash those bytes with
BLAKE3. Give the content an explicit identifier such as `"ChatMessage.v1"`,
and every producer using that identifier and the same encoded bytes can
calculate the same 256-bit `ContentId`.

The Rust types can have different names in each program. What needs to agree
is the identifier and the encoding: which fields are included, their order,
and how their values are represented. MinBin lets you define that format;
`contentid` does not infer a shared format from different Rust types.

Distributed IDs are available with the default features. If you already have
a shared byte representation, you can hash those bytes directly instead.

## Local IDs

Local IDs are an additional convenience. If you only need local IDs, a Rust
hasher you are happy with may be all you need. If you already use `contentid`
for distributed IDs, you can use the same library for your local IDs too.

Sometimes the ID only matters inside the process that is currently running.
You might want to recognize a value you have already processed or use its
content as a cache key. Other machines do not need to reproduce that ID, and
you can discard it when the process ends.

Local IDs hash Rust values directly through the `Hash` trait, without first
serializing them. They use Rust's `DefaultHasher` by default and return a
**64-bit `u64`** because that is the output exposed by Rust's `Hasher` trait.
This limit applies to the local API, including caller-supplied hashers; it is
not the default size of distributed IDs. Enable the `local` feature to use it.
You can supply your own hasher when you need a different algorithm or seed.

Keep the hash inputs, implementation, and seed consistent wherever you compare
local IDs. There is no compatibility promise across builds, Rust versions,
or platforms. These IDs are for disposable local state.

“Local” describes where you can rely on the ID, not a random identity assigned
to the process. The default hasher starts from a fixed state, so separate
runs can produce the same IDs too.

## Which should I use?

Start with distributed IDs when another program or platform needs to arrive
at the same ID from the same content. Use local IDs for disposable state when
only your running process needs to reproduce them.

The APIs follow the same identifier-plus-value pattern:

```rust
let value = 42_u64;
let distributed = contentid::typed_content_id("Counter.v1", &value);
let local = contentid::local_typed_content_id_u64("Counter.v1", &value);

println!("Distributed: {distributed}");
println!("Local: {local}");
```

This example requires the `local` feature. The distributed result is a
`ContentId`; the local result is a `u64`. They are separate identities for
different scopes and are not interchangeable.

Both are hashes, so collisions are possible. Deterministic means the same
inputs produce the same ID; it does not guarantee that every different input
has a unique ID. The full BLAKE3 digest provides much stronger collision
resistance than a 64-bit local ID.

## Usage

Add `contentid` to your project:

```toml
[dependencies]
contentid = "0.1.0"
```

Version 0.1.0 is being prepared for its first release and requires MinBin 1.0.
Until both are published to crates.io, use a local checkout as described in
[the release guide](https://github.com/humacss/contentid/blob/main/RELEASE.md).
The supported Rust version is 1.90 or later.

### Distributed IDs

Use the default dependency shown above; the `local` feature is not needed.
Define the encoding and give the content an explicit identifier:

```rust
use contentid::minbin;

#[derive(Default)]
struct Message {
    sender: u128,
    text: String,
}

minbin::minbin_struct! { Message [
    self.sender: u128,
    self.text: String
] }

let message = Message {
    sender: 7,
    text: "Hello".to_string(),
};

let id = contentid::id!(type = "ChatMessage.v1", &message);
let reproduced = contentid::typed_content_id("ChatMessage.v1", &message);
assert_eq!(id, reproduced);
println!("{id}");
```

The identifier is part of the hash input. Keep it the same wherever you want
the same identity. Introduce a new identifier when the meaning or encoding
changes and you want that change reflected in the ID.

Matching field names or matching values alone is not enough. All producers
need the same identifier and exact encoded bytes, including field order and
primitive representations.

Another program can reproduce this ID using the same `"ChatMessage.v1"`
identifier and MinBin bytes, even if its Rust struct has a different name.
An ID produced by the local `Hash` path will not match this distributed ID.

### Local IDs

For disposable local state, enable the `local` feature:

```toml
[dependencies]
contentid = { version = "0.1.0", features = ["local"] }
```

Then use a value that implements Rust's `Hash` trait:

```rust
#[derive(Hash)]
struct Position {
    x: i32,
    y: i32,
}

let position = Position { x: 10, y: 20 };
let id = contentid::local_typed_content_id_u64("Position", &position);
assert_eq!(id, contentid::local_typed_content_id_u64("Position", &position));
```

This uses Rust's `DefaultHasher` without MinBin serialization or an intermediate
byte allocation. Its output is 64 bits. Use it for state you can discard and
rebuild; the algorithm and Rust's `Hash` format can change between Rust releases.
`DefaultHasher::new()` uses a fixed initial state, rather than the randomized
seed used by a default `HashMap`.

You can supply your own hasher, including a seed you control:

```rust
use std::hash::{BuildHasher, RandomState};
use contentid::LocalContentHasher;

let state = RandomState::new();
let mut hasher = LocalContentHasher::with_hasher(state.build_hasher());
hasher.update_value("Position");
hasher.update_value(&(10_i32, 20_i32));
let id = hasher.finalize_u64();
println!("{id}");
```

Reuse the same `RandomState` when you need comparable IDs; a new state uses a
new random seed. Your choice of algorithm, seed, and `Hash` implementations
determines the guarantees your local IDs can provide.

For an explicit GxHash example, see [the GxHash example](https://github.com/humacss/contentid/blob/main/examples/gxhash/src/main.rs).
On a machine with the required AES instructions, run:

```sh
RUSTFLAGS="-C target-cpu=native" cargo run --manifest-path examples/gxhash/Cargo.toml
```

GxHash is isolated in a separate example package and is not a dependency of
this crate. Applications that want it must add it themselves. It is
non-cryptographic and requires supported CPU instructions. The standard Rust `Hasher` interface exposes 64 bits, including
when you supply GxHash.

### IDs tied to a Rust type

If both sides share the same Rust type, `ToContentId` is another convenience:
it uses BLAKE3 over the Rust type name and MinBin bytes. It is available with
the default features and is separate from the configurable local hasher.

Implement MinBin serialization for your type, then call `content_id()`:

```rust
use contentid::{ToContentId, minbin};

#[derive(Default)]
struct Message {
    sender: u128,
    text: String,
}

minbin::minbin_struct! { Message [
    self.sender: u128,
    self.text: String
] }

let message = Message {
    sender: 7,
    text: "Hello".to_string(),
};

let id = message.content_id();
assert_eq!(id, message.content_id());
assert!(message.verify(id).is_ok());
```

The Rust type name is part of this ID. Two differently named types can encode
to the same bytes and still produce different IDs through `ToContentId`.

### Raw bytes

If you already have the bytes, there is no need to serialize them again:

```rust
use contentid::ContentId;

let id = ContentId::from_bytes(b"file contents");
assert_eq!(id, ContentId::from_bytes(b"file contents"));
```

## ID sizes

| API | Default output |
| --- | --- |
| Distributed: `typed_content_id` or `id!(type = "TypeIdentifier", &value)` | 256-bit `ContentId` |
| Rust type-based: `value.content_id()` | 256-bit `ContentId` |
| Local: `local_typed_content_id_u64` or `LocalContentHasher::finalize_u64` | 64-bit `u64` |

The local API uses the standard Rust `Hasher` interface, so supplying a
wider-output algorithm does not expose its extra bits through this API.
Converting that `u64` to a larger integer would not add collision resistance.

`ContentId` holds the full 256-bit BLAKE3 digest. `ContentId128` keeps its first
128 bits. Both support comparison, ordering, hashing, and hexadecimal display.

Shorter distributed IDs are explicit choices. If an existing interface needs
a number, you can convert a `ContentId` to
`i64`, `u64`, or `u128`:

```rust
let id = contentid::typed_content_id("Counter.v1", &42_u64);
let numeric_id = u64::from(id);
assert_eq!(numeric_id, contentid::typed_content_id_u64("Counter.v1", &42_u64));
```

These conversions decode the first 8 or 16 digest bytes using MinBin's
primitive format. `ContentId128::as_u128()` explicitly uses big-endian order.
Shorter IDs have less collision resistance. Keep the full digest when you
need the full BLAKE3-256 identity.

## Building an ID in steps

`ContentHasher` lets you add bytes or MinBin values before producing an ID.
`id!(first, second, third)` is a convenience for hashing several MinBin values
in sequence. Neither automatically includes their Rust type names.

Raw byte updates form one continuous stream. Writing `b"ab"` once produces
the same result as writing `b"a"` and `b"b"` separately. You own the input
format, so add lengths or other boundaries where your format needs them.

`LocalContentHasher` passes writes directly to the selected Rust hasher.
Write-boundary behavior depends on that implementation; add explicit framing
when hashing multiple raw byte fields.

The convenience functions panic if MinBin serialization fails. Use
`try_content_id`, `try_typed_content_id`, or `ContentHasher::update_value` when
you need to handle that error.

## Features

| Feature | What it enables |
| --- | --- |
| `std` (default) | Standard library support and `alloc` |
| `alloc` | MinBin allocation support; the crate requires allocation even with default features disabled |
| `serde` | Serde serialization and deserialization for the ID storage types |
| `local` | Rust `Hash` local IDs with a configurable hasher; enables `std` |

The crate supports `no_std` with an allocator.

## Release status

Version 0.1.0 is the initial release candidate. Its API covers raw bytes, Rust
values, explicit protocol identifiers, and optional local hashing. The
[release guide](https://github.com/humacss/contentid/blob/main/RELEASE.md)
tracks the remaining publication steps. MinBin 1.0 must be published first.

Distributed IDs depend on the identifier and exact MinBin encoding. The fixed
protocol vectors in the tests protect the current encoding; keep those vectors
unchanged when updating dependencies. Local IDs make no compatibility promise
across builds, Rust releases, or platforms.

## Development

Protocol macro calls use `id!(type = identifier, &value)`. The earlier
`id!("Identifier", &value)` shorthand has been replaced so ordinary sequences
such as `id!(1_u64, 2_u64)` are unambiguous. This syntax change preserves the
protocol identifier's encoding and the resulting ID.

Run checks from this repository:

```sh
cargo fmt --check
cargo test
cargo test --all-features
cargo check --no-default-features
cargo clippy --all-targets --all-features -- -D warnings
cargo doc --no-deps --all-features
```

## License

MIT. See [LICENSE](https://github.com/humacss/contentid/blob/main/LICENSE).
