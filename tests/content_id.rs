use contentid::{ContentId, ContentId128, ToContentId};

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
struct TestPayload {
    account_id: u128,
    time: i64,
    label: String,
}

minbin::minbin_struct! { TestPayload [
    self.account_id: u128,
    self.time: i64,
    self.label: String
] }

#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct SameBytesDifferentType {
    account_id: u128,
    time: i64,
    label: String,
}

minbin::minbin_struct! { SameBytesDifferentType [
    self.account_id: u128,
    self.time: i64,
    self.label: String
] }

impl ToContentId for TestPayload {
    type Error = contentid::Error;

    fn write_content(&self, hasher: &mut contentid::ContentHasher) -> Result<(), Self::Error> {
        hasher.update_value(self)
    }
}

impl ToContentId for SameBytesDifferentType {
    type Error = contentid::Error;

    fn write_content(&self, hasher: &mut contentid::ContentHasher) -> Result<(), Self::Error> {
        hasher.update_value(self)
    }
}

#[test]
fn content_id_is_deterministic_for_same_type_and_data() {
    let payload = TestPayload {
        account_id: 7,
        time: 42,
        label: "hello".to_string(),
    };

    assert_eq!(
        payload.try_content_id().unwrap(),
        payload.try_content_id().unwrap()
    );
    assert_eq!(payload.content_id(), payload.try_content_id().unwrap());
}

#[test]
fn content_id_changes_when_data_changes() {
    let first = TestPayload {
        account_id: 7,
        time: 42,
        label: "hello".to_string(),
    };
    let second = TestPayload {
        label: "goodbye".to_string(),
        ..first.clone()
    };

    assert_ne!(first.content_id(), second.content_id());
}

#[test]
fn content_id_includes_type_name_domain_separation() {
    let first = TestPayload {
        account_id: 7,
        time: 42,
        label: "hello".to_string(),
    };
    let second = SameBytesDifferentType {
        account_id: 7,
        time: 42,
        label: "hello".to_string(),
    };

    assert_ne!(first.content_id(), second.content_id());
}

#[test]
fn content_id_hashes_raw_bytes() {
    let id = ContentId::from_bytes(b"abc");

    assert_eq!(
        "6437b3ac38465133ffb63b75273a8db548c558465d79db03fd359c6cd5bd9d85",
        id.to_hex()
    );
    assert_eq!(id.as_bytes().len(), 32);
}

#[test]
fn content_id_128_truncates_256_bit_id() {
    let id = ContentId::from_bytes(b"abc");
    let id128 = ContentId128::from_content_id(id);

    assert_eq!(&id.as_bytes()[..16], id128.as_bytes());
    assert_eq!(u128::from_be_bytes(*id128.as_bytes()), id128.as_u128());
    assert_eq!("6437b3ac38465133ffb63b75273a8db5", id128.to_hex());
}

#[test]
fn content_id_converts_to_common_integers_with_minbin_digest_prefixes() {
    let bytes = [
        0x80, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x88, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e,
        0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d,
        0x1e, 0x1f,
    ];
    let id = ContentId::from_hash_bytes(bytes);
    let expected_i64 = minbin::from_bytes(i64::default, &bytes[..8]).unwrap();
    let expected_u64 = minbin::from_bytes(u64::default, &bytes[..8]).unwrap();
    let expected_u128 = minbin::from_bytes(u128::default, &bytes[..16]).unwrap();

    assert_eq!(expected_i64, i64::from(id));
    assert_eq!(expected_i64, i64::from(&id));
    assert_eq!(expected_u64, u64::from(id));
    assert_eq!(expected_u64, u64::from(&id));
    assert_eq!(expected_u128, u128::from(id));
    assert_eq!(expected_u128, u128::from(&id));
}

#[test]
fn content_hasher_integer_helpers_match_content_id_conversions() {
    let mut hasher = contentid::ContentHasher::new();
    hasher.update_value(&"TestPayload".to_string()).unwrap();
    hasher.update_value(&7u128).unwrap();
    hasher.update_value(&42i64).unwrap();
    let id = hasher.finalize();

    let mut i64_hasher = contentid::ContentHasher::new();
    i64_hasher.update_value(&"TestPayload".to_string()).unwrap();
    i64_hasher.update_value(&7u128).unwrap();
    i64_hasher.update_value(&42i64).unwrap();

    let mut u64_hasher = contentid::ContentHasher::new();
    u64_hasher.update_value(&"TestPayload".to_string()).unwrap();
    u64_hasher.update_value(&7u128).unwrap();
    u64_hasher.update_value(&42i64).unwrap();

    let mut u128_hasher = contentid::ContentHasher::new();
    u128_hasher
        .update_value(&"TestPayload".to_string())
        .unwrap();
    u128_hasher.update_value(&7u128).unwrap();
    u128_hasher.update_value(&42i64).unwrap();

    assert_eq!(i64::from(id), i64_hasher.finalize_i64());
    assert_eq!(u64::from(id), u64_hasher.finalize_u64());
    assert_eq!(u128::from(id), u128_hasher.finalize_u128());
}

#[test]
fn id_macro_hashes_tuple_values_to_canonical_content_id() {
    type Result<T> = std::result::Result<T, &'static str>;

    let first = contentid::id!("TestPayload".to_string(), 7u128, 42i64);
    let second = contentid::id!("TestPayload".to_string(), 7u128, 42i64);
    let changed = contentid::id!("TestPayload".to_string(), 7u128, 43i64);
    let _: Result<()> = Ok(());

    assert_eq!(first, second);
    assert_ne!(first, changed);
}

#[test]
fn id_macro_hashes_explicit_type_identifier_and_value_content() {
    let value = TestPayload {
        account_id: 7,
        time: 42,
        label: "Mira".to_string(),
    };
    let id = contentid::id!(type = "ChatMessage", &value);
    let manual = contentid::typed_content_id("ChatMessage", &value);

    assert_eq!(manual, id);
    assert_ne!(contentid::id!(type = "OtherMessage", &value), id);
}

#[test]
fn typed_content_id_integer_functions_match_content_id_conversions() {
    let value = TestPayload {
        account_id: 7,
        time: 42,
        label: "Mira".to_string(),
    };
    let id = contentid::typed_content_id("Record.v1", &value);

    assert_eq!(
        i64::from(id),
        contentid::typed_content_id_i64("Record.v1", &value)
    );
    assert_eq!(
        u64::from(id),
        contentid::typed_content_id_u64("Record.v1", &value)
    );
    assert_eq!(
        u128::from(id),
        contentid::typed_content_id_u128("Record.v1", &value)
    );
}

#[test]
fn content_hasher_matches_tuple_id_macro() {
    let mut hasher = contentid::ContentHasher::new();
    hasher.update_value(&"TestPayload".to_string()).unwrap();
    hasher.update_value(&7u128).unwrap();
    hasher.update_value(&42i64).unwrap();

    assert_eq!(
        hasher.finalize(),
        contentid::id!("TestPayload".to_string(), 7u128, 42i64)
    );
}

#[cfg(feature = "local")]
#[test]
fn local_content_id_is_stable_for_the_same_hash_content() {
    let payload = TestPayload {
        account_id: 7,
        time: 42,
        label: "hello".to_string(),
    };
    let changed = TestPayload {
        label: "goodbye".to_string(),
        ..payload.clone()
    };

    let first = contentid::local_typed_content_id_u64("TestPayload", &payload);
    let second = contentid::local_typed_content_id_u64("TestPayload", &payload);
    let changed = contentid::local_typed_content_id_u64("TestPayload", &changed);

    assert_eq!(first, second);
    assert_ne!(first, changed);
}

#[cfg(feature = "local")]
#[test]
fn local_content_id_accepts_hash_only_content() {
    #[derive(Hash)]
    struct HashOnlyPayload {
        account_id: u128,
        time: i64,
    }

    let payload = HashOnlyPayload {
        account_id: 7,
        time: 42,
    };
    let changed = HashOnlyPayload {
        account_id: 7,
        time: 43,
    };

    let first = contentid::local_typed_content_id_u64("HashOnlyPayload", &payload);
    let second = contentid::local_typed_content_id_u64("HashOnlyPayload", &payload);
    let changed = contentid::local_typed_content_id_u64("HashOnlyPayload", &changed);

    assert_eq!(first, second);
    assert_ne!(first, changed);
}

#[cfg(feature = "local")]
#[test]
fn local_ids_use_rust_default_hasher() {
    use std::hash::{DefaultHasher, Hash, Hasher};
    let mut expected = DefaultHasher::new();
    "Counter".hash(&mut expected);
    42_u64.hash(&mut expected);
    assert_eq!(
        expected.finish(),
        contentid::local_typed_content_id_u64("Counter", &42_u64)
    );
}

#[cfg(feature = "local")]
#[test]
fn local_ids_accept_a_caller_supplied_hasher() {
    use std::hash::{DefaultHasher, Hash, Hasher};
    let state = std::collections::hash_map::RandomState::new();
    use std::hash::BuildHasher;
    let mut expected = state.build_hasher();
    "Counter".hash(&mut expected);
    42_u64.hash(&mut expected);
    let mut actual = contentid::LocalContentHasher::with_hasher(state.build_hasher());
    actual.update_value("Counter");
    actual.update_value(&42_u64);
    assert_eq!(expected.finish(), actual.finalize_u64());

    let mut expected = DefaultHasher::new();
    expected.write(b"hello");
    let mut actual = contentid::LocalContentHasher::new();
    actual.update_bytes(b"hello");
    assert_eq!(expected.finish(), actual.finalize_u64());
}

#[test]
fn protocol_ids_agree_for_differently_named_types_with_identical_encoding() {
    let first = TestPayload {
        account_id: 7,
        time: 42,
        label: "hello".into(),
    };
    let second = SameBytesDifferentType {
        account_id: 7,
        time: 42,
        label: "hello".into(),
    };
    assert_eq!(
        minbin::to_bytes(&first).unwrap(),
        minbin::to_bytes(&second).unwrap()
    );
    assert_eq!(
        contentid::typed_content_id("Message.v1", &first),
        contentid::typed_content_id("Message.v1", &second)
    );
    assert_ne!(
        contentid::typed_content_id("Message.v1", &first),
        contentid::typed_content_id("Message.v2", &second)
    );
}

#[test]
fn explicit_minbin_opt_in_preserves_the_type_name_and_encoding_hash_format() {
    let value = TestPayload {
        account_id: 7,
        time: -42,
        label: "hello".into(),
    };
    let mut bytes = core::any::type_name::<TestPayload>().as_bytes().to_vec();
    bytes.push(0);
    bytes.extend(minbin::to_bytes(&value).unwrap());
    assert_eq!(value.content_id(), ContentId::from_bytes(&bytes));
}
