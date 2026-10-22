//! Run with `RUSTFLAGS="-C target-cpu=native" cargo run --manifest-path examples/gxhash/Cargo.toml`.
//! GxHash is isolated in this example package; applications must opt into it.
//! It is non-cryptographic and requires supported CPU instructions.
use contentid::LocalContentHasher;

fn main() {
    let mut hasher = LocalContentHasher::with_hasher(gxhash::GxHasher::with_seed(42));
    hasher.update_value("Position");
    hasher.update_value(&(10_i32, 20_i32));
    // Rust's Hasher interface returns 64 bits, even for GxHash.
    println!("{}", hasher.finalize_u64());
}
