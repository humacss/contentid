use core::convert::Infallible;

use contentid::{ContentHasher, ContentId, ContentId128, ToContentId};

struct CustomEncoded(u64);

impl ToContentId for CustomEncoded {
    type Error = Infallible;

    fn write_content(&self, hasher: &mut ContentHasher) -> Result<(), Self::Error> {
        hasher.update_bytes(&self.0.to_be_bytes());
        Ok(())
    }
}

#[test]
fn custom_encoding_needs_no_minbin_implementation_and_includes_type_name() {
    let value = CustomEncoded(42);
    let mut expected = ContentHasher::new();
    expected.update_bytes(core::any::type_name::<CustomEncoded>().as_bytes());
    expected.update_bytes(&[0]);
    expected.update_bytes(&42_u64.to_be_bytes());
    assert_eq!(value.try_content_id(), Ok(expected.finalize()));
    assert_ne!(value.content_id(), CustomEncoded(43).content_id());
    assert_eq!(
        value.content_id_128(),
        ContentId128::from_content_id(value.content_id())
    );
}

#[derive(PartialEq, Eq)]
struct EncodeFailure;

struct Failing;

impl ToContentId for Failing {
    type Error = EncodeFailure;

    fn write_content(&self, _: &mut ContentHasher) -> Result<(), Self::Error> {
        Err(EncodeFailure)
    }
}

#[test]
fn serializer_errors_propagate_without_requiring_error_traits() {
    assert!(matches!(Failing.try_content_id(), Err(EncodeFailure)));
    assert!(matches!(Failing.try_content_id_128(), Err(EncodeFailure)));
}

#[test]
#[should_panic(expected = "content serialization failed")]
fn panicking_convenience_reports_custom_serializer_failure() {
    Failing.content_id();
}

struct Overridden;

impl ToContentId for Overridden {
    type Error = Infallible;

    fn write_content(&self, _: &mut ContentHasher) -> Result<(), Self::Error> {
        panic!("overridden ID generation must not call the default encoder")
    }

    fn try_content_id(&self) -> Result<ContentId, Self::Error> {
        Ok(ContentId::from_bytes(b"custom identity"))
    }
}

#[test]
fn custom_id_override_is_used_by_all_default_conveniences() {
    let expected = ContentId::from_bytes(b"custom identity");
    assert_eq!(Overridden.try_content_id(), Ok(expected));
    assert_eq!(Overridden.content_id(), expected);
    assert_eq!(
        Overridden.content_id_128(),
        ContentId128::from_content_id(expected)
    );
    assert_eq!(
        Overridden.try_content_id_128(),
        Ok(ContentId128::from_content_id(expected))
    );
}

#[test]
fn custom_encoded_payload_can_reproduce_a_distributed_id() {
    let value = CustomEncoded(42);
    let mut hasher = ContentHasher::new();
    hasher.update_value(&"Counter.v1".to_string()).unwrap();
    value.write_content(&mut hasher).unwrap();
    assert_eq!(
        hasher.finalize(),
        contentid::typed_content_id("Counter.v1", &42_u64)
    );
}
