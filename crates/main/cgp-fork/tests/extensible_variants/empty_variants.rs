//! Variants with no fields, in each of their three forms (`Closed`, `Paused()`,
//! and `Archived {}`), under the variant derives. Each carries the payload `Nil`,
//! the empty product `#[derive(HasFields)]` gives it, so it is built, extracted,
//! cast, and dispatched like any newtype variant.
//!
//! The borrowed extractors hold a reference to `Nil`: a promoted `&Nil` when
//! shared, and a leaked `Box<Nil>` when mutable, since `&mut Nil` is not
//! promoted. That keeps every borrowed matcher working, the mutable ones
//! included. `#[cgp_auto_dispatch]` cannot serve such an enum, because a crate
//! implementing the dispatched trait for the foreign `Nil` conflicts with the
//! blanket impl the macro emits, so the value dispatch here uses providers.
//!
//! See cgp-knowledge-base-fork/cgp/reference/derives/derive_cgp_variant.md and
//! cgp-knowledge-base-fork/cgp/implementation/entrypoints/derive_extract_field.md.

use core::convert::Infallible;
use core::marker::PhantomData;

use cgp_fork::core::error::ErrorTypeProviderComponent;
use cgp_fork::core::field::impls::{CanDowncast, CanUpcast};
use cgp_fork::core::field::traits::{FinalizeExtractResult, StaticString};
use cgp_fork::extra::dispatch::{
    MatchWithFieldHandlers, MatchWithValueHandlers, MatchWithValueHandlersMut,
    MatchWithValueHandlersRef,
};
use cgp_fork::extra::handler::Computer;
use cgp_fork::prelude::*;

#[derive(Debug, PartialEq, CgpVariant)]
pub enum Status {
    Active(Count),
    Closed,
    Paused(),
    Archived {},
}

#[derive(Debug, PartialEq, CgpVariant)]
pub enum OpenStatus {
    Active(Count),
    Paused(),
}

#[derive(Debug, PartialEq)]
pub struct Count(pub u64);

/// A C-like enum: every variant is empty.
#[derive(Debug, PartialEq, CgpVariant)]
pub enum Light {
    Red,
    Amber,
    Green,
}

/// The shape `#[derive(HasFields)]` gives `Status`, written out.
type StatusFields = Sum![
    Field<Symbol!("Active"), Count>,
    Field<Symbol!("Closed"), Nil>,
    Field<Symbol!("Paused"), Nil>,
    Field<Symbol!("Archived"), Nil>,
];

/// The same shape, written as the enum body `Enum!` reads.
type StatusShape = Enum! {
    Active(Count),
    Closed,
    Paused(),
    Archived {},
};

fn assert_same_type<T>(_: PhantomData<T>, _: PhantomData<T>) {}

#[test]
fn test_has_fields_gives_nil_for_every_empty_form() {
    assert_same_type(
        PhantomData::<<Status as HasFields>::Fields>,
        PhantomData::<StatusFields>,
    );
    assert_same_type(PhantomData::<StatusShape>, PhantomData::<StatusFields>);
}

#[test]
fn test_from_variant_builds_each_empty_form() {
    assert_eq!(
        Status::from_variant(PhantomData::<Symbol!("Closed")>, Nil),
        Status::Closed
    );
    assert_eq!(
        Status::from_variant(PhantomData::<Symbol!("Paused")>, Nil),
        Status::Paused()
    );
    assert_eq!(
        Status::from_variant(PhantomData::<Symbol!("Archived")>, Nil),
        Status::Archived {}
    );
    assert_eq!(
        Status::from_variant(PhantomData::<Symbol!("Active")>, Count(3)),
        Status::Active(Count(3))
    );
}

#[test]
fn test_extract_empty_variants_owned() {
    let extracted = Status::Archived {}
        .to_extractor()
        .extract_field(PhantomData::<Symbol!("Archived")>);
    assert_eq!(extracted.ok(), Some(Nil));

    let remainder = Status::Closed
        .to_extractor()
        .extract_field(PhantomData::<Symbol!("Active")>)
        .unwrap_err();
    assert_eq!(
        remainder
            .extract_field(PhantomData::<Symbol!("Closed")>)
            .ok(),
        Some(Nil)
    );

    assert_eq!(
        Status::from_extractor(Status::Paused().to_extractor()),
        Status::Paused()
    );
}

#[test]
fn test_extract_empty_variants_borrowed() {
    let status = Status::Paused();
    let shared: Result<&Nil, _> = status
        .extractor_ref()
        .extract_field(PhantomData::<Symbol!("Paused")>);
    assert_eq!(shared.ok(), Some(&Nil));

    let mut status = Status::Closed;
    let unique: Result<&mut Nil, _> = status
        .extractor_mut()
        .extract_field(PhantomData::<Symbol!("Closed")>);
    assert_eq!(unique.ok(), Some(&mut Nil));
}

/// Name the variant a value is, by extracting every variant in turn and
/// finalizing the empty remainder, which proves the match exhaustive.
fn light_name(light: Light) -> &'static str {
    let remainder = match light
        .to_extractor()
        .extract_field(PhantomData::<Symbol!("Red")>)
    {
        Ok(Nil) => return "red",
        Err(remainder) => remainder,
    };

    let remainder = match remainder.extract_field(PhantomData::<Symbol!("Amber")>) {
        Ok(Nil) => return "amber",
        Err(remainder) => remainder,
    };

    let Nil = remainder
        .extract_field(PhantomData::<Symbol!("Green")>)
        .finalize_extract_result();

    "green"
}

#[test]
fn test_c_like_enum() {
    assert_eq!(light_name(Light::Red), "red");
    assert_eq!(light_name(Light::Amber), "amber");
    assert_eq!(light_name(Light::Green), "green");

    let light = Light::Amber;
    assert!(
        light
            .extractor_ref()
            .extract_field(PhantomData::<Symbol!("Amber")>)
            .is_ok()
    );
}

#[test]
fn test_cast_through_empty_variants() {
    assert_eq!(
        OpenStatus::Paused().upcast(PhantomData::<Status>),
        Status::Paused()
    );

    assert_eq!(
        Status::Paused().downcast(PhantomData::<OpenStatus>).ok(),
        Some(OpenStatus::Paused())
    );

    assert_eq!(
        Status::Closed.downcast(PhantomData::<OpenStatus>).ok(),
        None
    );
}

/// An empty variant beside a generic payload and a borrowed one, so the partial enums carry both a
/// type parameter and the enum's own lifetime alongside the `Nil` payload.
#[derive(Debug, PartialEq, CgpVariant)]
pub enum Lookup<'a, T> {
    Found(T),
    Borrowed(&'a str),
    Missing,
}

#[test]
fn test_empty_variant_beside_generics_and_lifetimes() {
    assert_eq!(
        Lookup::<u64>::from_variant(PhantomData::<Symbol!("Missing")>, Nil),
        Lookup::Missing
    );

    let lookup: Lookup<'_, u64> = Lookup::Missing;
    let shared: Result<&Nil, _> = lookup
        .extractor_ref()
        .extract_field(PhantomData::<Symbol!("Missing")>);
    assert_eq!(shared.ok(), Some(&Nil));

    let mut lookup: Lookup<'_, u64> = Lookup::Missing;
    let unique: Result<&mut Nil, _> = lookup
        .extractor_mut()
        .extract_field(PhantomData::<Symbol!("Missing")>);
    assert_eq!(unique.ok(), Some(&mut Nil));

    let text = String::from("key");
    let lookup: Lookup<'_, u64> = Lookup::Borrowed(&text);
    assert_eq!(
        lookup
            .to_extractor()
            .extract_field(PhantomData::<Symbol!("Borrowed")>)
            .ok(),
        Some("key")
    );
}

/// The operations value dispatch runs on a payload. Value dispatch sees only the
/// payload, so every empty variant reaches the one `Nil` impl.
pub trait Describe {
    fn describe(&self) -> String;

    fn bump(&mut self);

    fn into_count(self) -> u64;
}

impl Describe for Count {
    fn describe(&self) -> String {
        format!("active {}", self.0)
    }

    fn bump(&mut self) {
        self.0 += 1;
    }

    fn into_count(self) -> u64 {
        self.0
    }
}

impl Describe for Nil {
    fn describe(&self) -> String {
        "empty".to_owned()
    }

    fn bump(&mut self) {}

    fn into_count(self) -> u64 {
        0
    }
}

#[cgp_computer]
pub fn describe_value<T: Describe>(value: &T) -> String {
    value.describe()
}

#[cgp_computer]
pub fn bump_value<T: Describe>(value: &mut T) {
    value.bump()
}

#[cgp_computer]
pub fn count_value<T: Describe>(value: T) -> u64 {
    value.into_count()
}

#[test]
fn test_value_dispatch_merges_empty_variants() {
    let code = PhantomData::<()>;

    assert_eq!(
        MatchWithValueHandlersRef::<DescribeValue>::compute(&App, code, &Status::Active(Count(2))),
        "active 2"
    );
    assert_eq!(
        MatchWithValueHandlersRef::<DescribeValue>::compute(&App, code, &Status::Closed),
        "empty"
    );
    assert_eq!(
        MatchWithValueHandlersRef::<DescribeValue>::compute(&App, code, &Status::Archived {}),
        "empty"
    );

    let mut active = Status::Active(Count(2));
    MatchWithValueHandlersMut::<BumpValue>::compute(&App, code, &mut active);
    assert_eq!(active, Status::Active(Count(3)));

    let mut paused = Status::Paused();
    MatchWithValueHandlersMut::<BumpValue>::compute(&App, code, &mut paused);
    assert_eq!(paused, Status::Paused());

    assert_eq!(
        MatchWithValueHandlers::<CountValue>::compute(&App, code, Status::Active(Count(5))),
        5
    );
    assert_eq!(
        MatchWithValueHandlers::<CountValue>::compute(&App, code, Status::Closed),
        0
    );
}

pub struct App;

delegate_components! {
    App {
        ErrorTypeProviderComponent: UseType<Infallible>,
    }
}

/// Field dispatch sees the variant's tag as well as its payload, so it tells the
/// empty variants apart.
#[cgp_computer]
pub fn variant_name<Tag: StaticString, Value>(_field: Field<Tag, Value>) -> &'static str {
    Tag::VALUE
}

#[test]
fn test_field_dispatch_tells_empty_variants_apart() {
    let code = PhantomData::<()>;

    assert_eq!(
        MatchWithFieldHandlers::<VariantName>::compute(&App, code, Status::Closed),
        "Closed"
    );
    assert_eq!(
        MatchWithFieldHandlers::<VariantName>::compute(&App, code, Status::Paused()),
        "Paused"
    );
    assert_eq!(
        MatchWithFieldHandlers::<VariantName>::compute(&App, code, Status::Archived {}),
        "Archived"
    );
}
