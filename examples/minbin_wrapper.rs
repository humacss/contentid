//! Consumer-owned MinBin serialization for ContentId.
//!
//! Run with `cargo run --example minbin_wrapper`.
//! A local wrapper permits the MinBin implementation under Rust's orphan rules.
//! From conversions move between the wrapper and ContentId; serialization is
//! defined separately. This wire format is exactly 32 digest bytes, with no
//! length prefix. Decoding restores the digest without hashing it again.
//! Use the same pattern with 16 bytes for ContentId128.

use contentid::{ContentId, minbin};
use minbin::{BytesReader, BytesWriter, ToFromByteError, ToFromBytes};

struct WireId(ContentId);

impl From<ContentId> for WireId {
    fn from(id: ContentId) -> Self {
        Self(id)
    }
}

impl From<WireId> for ContentId {
    fn from(id: WireId) -> Self {
        id.0
    }
}

impl<'a> ToFromBytes<'a> for WireId {
    const MAX_BYTES: usize = 32;

    fn to_bytes(&self, writer: &mut BytesWriter<'a>) -> Result<(), ToFromByteError> {
        writer.write_bytes(self.0.as_bytes())
    }

    fn from_bytes(
        buffer: &mut Self,
        reader: &mut BytesReader<'a>,
    ) -> Result<usize, ToFromByteError> {
        let mut digest = [0_u8; 32];
        digest.copy_from_slice(reader.read_bytes(32)?);
        buffer.0 = ContentId::from_hash_bytes(digest);
        Ok(reader.pos)
    }

    fn byte_count(&self) -> usize {
        32
    }
}

fn main() {
    let id = ContentId::from_bytes(b"message contents");
    let bytes = minbin::to_bytes(&WireId::from(id)).unwrap();
    assert_eq!(bytes.as_slice(), id.as_bytes());

    let decoded = minbin::from_bytes(|| WireId(ContentId::ZERO), &bytes).unwrap();
    assert_eq!(ContentId::from(decoded), id);
}
