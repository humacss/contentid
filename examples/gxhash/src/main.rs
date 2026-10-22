//! Run with `RUSTFLAGS="-C target-cpu=native" cargo run --manifest-path examples/gxhash/Cargo.toml`.
//! GxHash is isolated in this example package; applications must opt into it.
//! It is non-cryptographic and requires supported CPU instructions.
use contentid::LocalContentHasher;

fn main() {
    let mut gx = gxhash::GxHasher::with_seed(42);
    {
        // Borrow the hasher so its algorithm-specific output remains accessible.
        let mut hasher = LocalContentHasher::with_hasher(&mut gx);
        hasher.update_value("Position");
        hasher.update_value(&(10_i32, 20_i32));
    }

    // Finish through GxHash itself to obtain all 128 bits.
    let id: u128 = gx.finish_u128();
    println!("{id:032x}");
}
