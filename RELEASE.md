# Releasing contentid

Publication is manual. The repository's CI does not publish crates, create
release tags, or require a crates.io token.

## Release order

1. Prepare and publish MinBin 1.0.0 from its own repository.
2. Wait until Cargo can resolve `minbin = "1.0.0"` from crates.io.
3. Run the registry-only checks below before publishing Content ID 0.1.0.

Content ID targets the proposed MinBin 1.0 API based on its current `main`,
including in-place `ToFromBytes` deserialization and the struct macros.
MinBin release preparation is separate work; this repository does not change
or publish MinBin. The older published MinBin 0.1.1 is not an interchangeable dependency. A version bump alone does
not establish release readiness for MinBin; check its own tests and package.

## Developing before MinBin 1.0 is published

For local testing with a sibling `minbin` checkout whose package version is
1.0.0, supply an explicit Cargo override:

```sh
cargo test --all-features --config 'patch.crates-io.minbin.path="../minbin"'
```

This override is local testing only. Do not add it to the published manifest
or CI. Tests using an override do not prove that the registry-only package
builds.

## Prepare the release commit

Once MinBin 1.0.0 is published, remove any local Cargo overrides. Delete any
local lockfiles generated during development, then run the validation commands
below against crates.io. Both local lockfiles can be removed with:

```sh
rm -f Cargo.lock examples/gxhash/Cargo.lock
```

Lockfiles are ignored and are not committed. CI starts without them and
resolves compatible versions from the manifests. Cargo may generate local
lockfiles and include a generated lockfile in a package archive; this does not
pin dependencies for applications using the library. Those applications have
their own dependency resolution and lockfiles.

Confirm the crate name and version are available on crates.io. The preparation
check found no registered `contentid` crate; this does not reserve the name.
Update the 0.1.0 changelog heading with the release date and remove the
pre-release availability note from the README. Review and commit the release
changes, then ensure the complete CI run for that exact commit succeeds.

## Validate without uploading

Run from the Content ID repository with a clean working tree:

```sh
cargo fmt --check
cargo test
cargo test --all-features
cargo test --no-default-features
cargo clippy --all-targets --all-features -- -D warnings
cargo doc --no-deps --all-features
cargo check --target wasm32-unknown-unknown --no-default-features
cargo check --target wasm32-unknown-unknown --no-default-features --features serde
cargo package --list
cargo publish --dry-run --all-features
```

Install the WebAssembly target with `rustup target add wasm32-unknown-unknown`
if needed. CI also checks the declared Rust 1.90 minimum.

Inspect `target/package/contentid-0.1.0.crate`. The package should contain the
manifest, source, tests, benchmark, README, changelog, license, and
Cargo-generated metadata (which may include a generated lockfile). Agent guidance, CI files, release instructions,
local paths, credentials, and the separate GxHash example package must not be
included. Read the normalized `Cargo.toml` inside the archive and confirm that
all dependencies resolve through crates.io.

A successful dry run builds the packaged crate without uploading it. A
successful test run with the local MinBin override is not a substitute.

## Publish manually

The maintainer performs these steps after reviewing the dry run and CI:

1. Configure crates.io authentication outside the repository.
2. Run `cargo publish` from the validated release commit.
3. Confirm the version is visible on crates.io and docs.rs builds successfully.
4. Tag the published commit `v0.1.0` and push the tag.

Do not store registry tokens in repository files or paste them into issues,
logs, or documentation. No token is needed for the preparation work.
