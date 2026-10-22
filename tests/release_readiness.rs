use contentid::{ContentHasher, ContentId, Error, ToContentId};

#[test]
fn numeric_literal_pair_is_a_sequence() {
    let mut expected = ContentHasher::new();
    expected.update_value(&1_u64).unwrap();
    expected.update_value(&2_u64).unwrap();
    assert_eq!(expected.finalize(), contentid::id!(1_u64, 2_u64));
}

#[test]
fn explicit_protocol_syntax_accepts_identifier_expressions() {
    let identifier = "Counter.v1";
    assert_eq!(
        contentid::typed_content_id(identifier, &42_u64),
        contentid::id!(type = identifier, &42_u64)
    );
}

#[test]
fn distributed_id_has_fixed_encoding_and_digest() {
    // MinBin's String prefix is a big-endian u32 byte count, followed by UTF-8.
    // The payload is a big-endian u64. These bytes are the protocol contract.
    let bytes = b"\0\0\0\x0aCounter.v1\0\0\0\0\0\0\0\x2a";
    let id = contentid::typed_content_id("Counter.v1", &42_u64);
    assert_eq!(id, ContentId::from_bytes(bytes));
    assert_eq!(
        id.to_hex(),
        "d12cf4a28094c9ed75060c953b7b2d7b7b8869b5ad0da48fa172c17a2e58c68e"
    );
}

#[test]
fn numeric_views_have_fixed_big_endian_order() {
    let id = ContentId::from_hash_bytes([
        0x80, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
        24, 25, 26, 27, 28, 29, 30, 31,
    ]);
    assert_eq!(u64::from(id), 0x8001_0203_0405_0607);
    assert_eq!(i64::from(id), 0x8001_0203_0405_0607_u64 as i64);
    assert_eq!(u128::from(id), 0x8001_0203_0405_0607_0809_0a0b_0c0d_0e0f);
}

#[test]
fn serialization_errors_propagate_and_leave_builder_unchanged() {
    let oversized = "x".repeat(<String as minbin::ToFromBytes<'_>>::MAX_BYTES);
    let error = Error::Minbin(minbin::ToFromByteError::MaxBytesExceeded);
    assert_eq!(oversized.try_content_id(), Err(error.clone()));
    assert_eq!(oversized.try_content_id_128(), Err(error.clone()));
    assert_eq!(oversized.verify(ContentId::ZERO), Err(error.clone()));
    assert_eq!(
        contentid::try_typed_content_id("Text", &oversized),
        Err(error.clone())
    );
    assert_eq!(
        contentid::try_typed_content_id_128("Text", &oversized),
        Err(error.clone())
    );
    let mut hasher = ContentHasher::new();
    hasher.update_bytes(b"prefix");
    let expected = hasher.clone().finalize();
    assert_eq!(hasher.update_value(&oversized), Err(error));
    assert_eq!(hasher.finalize(), expected);
}
