//! Fast process-local IDs over Rust [`Hash`] values.

use core::hash::{Hash, Hasher};

const LOCAL_HASH_SEED: i64 = 0x517c_c1b7_2722_0a95;

/// Incremental GxHash-128 builder for non-persistent local IDs.
#[derive(Clone)]
pub struct LocalContentHasher(LocalHasher);

#[derive(Clone)]
struct LocalHasher(gxhash::GxHasher);

impl Hasher for LocalHasher {
    fn finish(&self) -> u64 {
        self.0.finish()
    }

    fn write(&mut self, bytes: &[u8]) {
        self.0.write_usize(bytes.len());
        let mut chunks = bytes.chunks_exact(16);
        for chunk in &mut chunks {
            self.0
                .write_u128(u128::from_le_bytes(chunk.try_into().unwrap()));
        }
        let remainder = chunks.remainder();
        if !remainder.is_empty() {
            let mut tail = [0; 16];
            tail[..remainder.len()].copy_from_slice(remainder);
            self.0.write_u128(u128::from_le_bytes(tail));
        }
    }

    fn write_u8(&mut self, value: u8) {
        self.0.write_u8(value);
    }

    fn write_u16(&mut self, value: u16) {
        self.0.write_u16(value);
    }

    fn write_u32(&mut self, value: u32) {
        self.0.write_u32(value);
    }

    fn write_u64(&mut self, value: u64) {
        self.0.write_u64(value);
    }

    fn write_u128(&mut self, value: u128) {
        self.0.write_u128(value);
    }

    fn write_usize(&mut self, value: usize) {
        self.0.write_usize(value);
    }

    fn write_i8(&mut self, value: i8) {
        self.0.write_i8(value);
    }

    fn write_i16(&mut self, value: i16) {
        self.0.write_i16(value);
    }

    fn write_i32(&mut self, value: i32) {
        self.0.write_i32(value);
    }

    fn write_i64(&mut self, value: i64) {
        self.0.write_i64(value);
    }

    fn write_i128(&mut self, value: i128) {
        self.0.write_i128(value);
    }

    fn write_isize(&mut self, value: isize) {
        self.0.write_isize(value);
    }
}

impl LocalContentHasher {
    /// Create an empty local content hasher.
    pub fn new() -> Self {
        Self(LocalHasher(gxhash::GxHasher::with_seed(LOCAL_HASH_SEED)))
    }

    /// Add raw bytes to the hash stream.
    pub fn update_bytes(&mut self, bytes: &[u8]) {
        self.0.write(bytes);
    }

    /// Add a typed Rust value to the hash stream without an intermediate encoding.
    pub fn update_value<T>(&mut self, value: &T)
    where
        T: Hash + ?Sized,
    {
        value.hash(&mut self.0);
    }

    /// Finish as a non-persistent local `u128` ID.
    pub fn finalize_u128(self) -> u128 {
        self.0.0.finish_u128()
    }
}

impl Default for LocalContentHasher {
    fn default() -> Self {
        Self::new()
    }
}

/// Build a local `u128` ID from a type identifier and typed Rust value.
pub fn local_typed_content_id_u128<T>(type_identifier: &str, value: &T) -> u128
where
    T: Hash + ?Sized,
{
    let mut hasher = LocalContentHasher::new();
    hasher.update_value(type_identifier);
    hasher.update_value(value);
    hasher.finalize_u128()
}
