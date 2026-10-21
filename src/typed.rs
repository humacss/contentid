//! Explicit stable type-identifier APIs for shared protocol IDs.

use alloc::string::ToString;

use minbin::ToFromBytes;

use crate::{ContentHasher, ContentId, ContentId128, Error};

/// Try to build a 256-bit content ID from an explicit stable type identifier
/// plus one value's minbin content.
pub fn try_typed_content_id<T>(type_identifier: &str, value: &T) -> Result<ContentId, Error>
where
    T: for<'a> ToFromBytes<'a>,
{
    let mut hasher = ContentHasher::new();
    hasher.update_value(&type_identifier.to_string())?;
    hasher.update_value(value)?;
    Ok(hasher.finalize())
}

/// Build a 256-bit content ID from an explicit stable type identifier plus one
/// value's minbin content.
///
/// # Panics
///
/// Panics when the type identifier or value cannot be serialized with minbin.
/// Use [`try_typed_content_id`] when serialization failure must be handled.
pub fn typed_content_id<T>(type_identifier: &str, value: &T) -> ContentId
where
    T: for<'a> ToFromBytes<'a>,
{
    try_typed_content_id(type_identifier, value)
        .expect("minbin serialization failed for typed_content_id")
}

/// Try to build a 128-bit content ID from an explicit stable type identifier
/// plus one value's minbin content.
pub fn try_typed_content_id_128<T>(type_identifier: &str, value: &T) -> Result<ContentId128, Error>
where
    T: for<'a> ToFromBytes<'a>,
{
    Ok(ContentId128::from_content_id(try_typed_content_id(
        type_identifier,
        value,
    )?))
}

/// Build a 128-bit content ID from an explicit stable type identifier plus one
/// value's minbin content.
///
/// # Panics
///
/// Panics when the type identifier or value cannot be serialized with minbin.
/// Use [`try_typed_content_id_128`] when serialization failure must be handled.
pub fn typed_content_id_128<T>(type_identifier: &str, value: &T) -> ContentId128
where
    T: for<'a> ToFromBytes<'a>,
{
    try_typed_content_id_128(type_identifier, value)
        .expect("minbin serialization failed for typed_content_id_128")
}

/// Build an `i64` ID from an explicit stable type identifier plus one value's
/// minbin content.
///
/// # Panics
///
/// Panics when the type identifier or value cannot be serialized with minbin.
/// Use [`try_typed_content_id`] and convert the returned [`ContentId`] when
/// serialization failure must be handled.
pub fn typed_content_id_i64<T>(type_identifier: &str, value: &T) -> i64
where
    T: for<'a> ToFromBytes<'a>,
{
    i64::from(typed_content_id(type_identifier, value))
}

/// Build a `u64` ID from an explicit stable type identifier plus one value's
/// minbin content.
///
/// # Panics
///
/// Panics when the type identifier or value cannot be serialized with minbin.
/// Use [`try_typed_content_id`] and convert the returned [`ContentId`] when
/// serialization failure must be handled.
pub fn typed_content_id_u64<T>(type_identifier: &str, value: &T) -> u64
where
    T: for<'a> ToFromBytes<'a>,
{
    u64::from(typed_content_id(type_identifier, value))
}

/// Build a `u128` ID from an explicit stable type identifier plus one value's
/// minbin content.
///
/// # Panics
///
/// Panics when the type identifier or value cannot be serialized with minbin.
/// Use [`try_typed_content_id`] and convert the returned [`ContentId`] when
/// serialization failure must be handled.
pub fn typed_content_id_u128<T>(type_identifier: &str, value: &T) -> u128
where
    T: for<'a> ToFromBytes<'a>,
{
    u128::from(typed_content_id(type_identifier, value))
}
