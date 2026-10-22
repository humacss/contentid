use std::collections::{BTreeMap, BTreeSet};

use contentid::{ContentId, minbin, typed_content_id};

#[test]
fn nested_containers_keep_their_protocol_encoding() {
    let value = (Some(0x1234_u16), vec![1_u32, 2_u32]);
    // "Nested.v1" identifier, Some tag + u16, then Vec count + two u32s.
    let bytes = b"\0\0\0\x09Nested.v1\x01\x12\x34\0\0\0\x02\0\0\0\x01\0\0\0\x02";
    assert_eq!(
        typed_content_id("Nested.v1", &value),
        ContentId::from_bytes(bytes)
    );
    assert_ne!(
        typed_content_id("Nested.v1", &value),
        typed_content_id("Nested.v1", &(None::<u16>, vec![1_u32, 2_u32]))
    );
}

#[test]
fn tree_maps_produce_the_same_id_regardless_of_insertion_order() {
    let mut first = BTreeMap::new();
    first.insert(2_u8, 200_u16);
    first.insert(1, 100);
    let mut second = BTreeMap::new();
    second.insert(1_u8, 100_u16);
    second.insert(2, 200);

    // "Map.v1" identifier, entry count, then sorted key/value pairs.
    let bytes = b"\0\0\0\x06Map.v1\0\0\0\x02\x01\0\x64\x02\0\xc8";
    let expected = ContentId::from_bytes(bytes);
    assert_eq!(typed_content_id("Map.v1", &first), expected);
    assert_eq!(typed_content_id("Map.v1", &second), expected);
}

#[test]
fn tree_sets_use_sorted_length_prefixed_elements() {
    let values = BTreeSet::from([300_u16, 1_u16, 300_u16]);
    // "Set.v1" identifier, unique element count, then sorted u16 elements.
    let bytes = b"\0\0\0\x06Set.v1\0\0\0\x02\0\x01\x01\x2c";
    assert_eq!(
        typed_content_id("Set.v1", &values),
        ContentId::from_bytes(bytes)
    );
    assert_eq!(
        minbin::to_bytes(&values).unwrap(),
        b"\0\0\0\x02\0\x01\x01\x2c"
    );
}
