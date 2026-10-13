//! An enum may name its variants after the associated types the variant derives implement.
//!
//! `#[derive(CgpData)]` implements traits whose associated types are `Value`, `Remainder`,
//! `Extractor`, `ExtractorRef`, `ExtractorMut`, `Fields`, and `FieldsRef`. Inside an impl for an
//! enum, a bare `Self::Value` could name the associated type or a variant called `Value`, so the
//! derives write each associated type as a qualified projection, such as
//! `<Self as FromVariant<Tag>>::Value`. An enum with all seven names as variants therefore derives
//! the whole family, and each slice works on it.
//!
//! See cgp-knowledge-base/cgp/implementation/entrypoints/derive_extract_field.md and
//! cgp-knowledge-base/cgp/implementation/entrypoints/derive_has_fields.md.

use core::marker::PhantomData;

use cgp::prelude::*;

#[derive(Debug, PartialEq, CgpData)]
pub enum Reserved {
    Value(u32),
    Remainder(u64),
    Extractor(i8),
    ExtractorRef(i16),
    ExtractorMut(i32),
    Fields(bool),
    FieldsRef(char),
}

#[test]
fn test_from_variant_builds_reserved_names() {
    assert_eq!(
        Reserved::from_variant(PhantomData::<Symbol!("Value")>, 1),
        Reserved::Value(1)
    );
    assert_eq!(
        Reserved::from_variant(PhantomData::<Symbol!("FieldsRef")>, 'x'),
        Reserved::FieldsRef('x')
    );
}

#[test]
fn test_extract_field_reaches_reserved_names() {
    let remainder = Reserved::Remainder(2)
        .to_extractor()
        .extract_field(PhantomData::<Symbol!("Value")>)
        .unwrap_err();

    assert_eq!(
        remainder
            .extract_field(PhantomData::<Symbol!("Remainder")>)
            .ok(),
        Some(2)
    );

    assert_eq!(
        Reserved::from_extractor(Reserved::Extractor(3).to_extractor()),
        Reserved::Extractor(3)
    );
}

#[test]
fn test_borrowed_extractors_reach_reserved_names() {
    let reserved = Reserved::ExtractorRef(4);
    let extracted = reserved
        .extractor_ref()
        .extract_field(PhantomData::<Symbol!("ExtractorRef")>);
    assert_eq!(extracted.ok(), Some(&4));

    let mut reserved = Reserved::ExtractorMut(5);
    if let Ok(value) = reserved
        .extractor_mut()
        .extract_field(PhantomData::<Symbol!("ExtractorMut")>)
    {
        *value += 1;
    }
    assert_eq!(reserved, Reserved::ExtractorMut(6));
}

#[test]
fn test_fields_round_trip_reserved_names() {
    let fields = Reserved::Fields(true).to_fields();
    assert_eq!(Reserved::from_fields(fields), Reserved::Fields(true));

    let reserved = Reserved::FieldsRef('y');
    let _fields_ref = reserved.to_fields_ref();
}
