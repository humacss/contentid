//! Public ID construction macros.

/// Build a canonical [`ContentId`](crate::ContentId) from one or more minbin
/// values.
///
/// Use `id!(value)` for a single struct or payload value. Use multiple
/// arguments for tuple-like IDs where every value is intentionally part of the
/// content. The `id!(type = "ProtocolName", &value)` form is reserved for explicit
/// stable protocol identifiers and matches
/// [`typed_content_id`](crate::typed_content_id).
///
/// # Panics
///
/// Panics when any input value cannot be serialized with minbin. Use
/// [`ContentHasher`](crate::ContentHasher) directly when serialization failure
/// must be handled.
#[macro_export]
macro_rules! id {
    (type = $type_identifier:expr, $value:expr $(,)?) => {{
        $crate::typed_content_id($type_identifier, $value)
    }};
    ($($item:expr),+ $(,)?) => {{
        let mut hasher = $crate::ContentHasher::new();
        $(
            hasher
                .update_value(&$item)
                .expect("minbin serialization failed for contentid::id");
        )+
        hasher.finalize()
    }};
}
