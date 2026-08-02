//! The type-level product operations: `AppendProduct`, `ConcatProduct`, and `MapFields`.
//!
//! These three are the list algebra the structural machinery is built from — growing a
//! field product by one entry, splicing two products together, and rewriting every entry
//! through a `MapType` marker. They are pure type-level functions evaluated by the trait
//! solver, so the assertions below are type equalities rather than runtime comparisons:
//! each `fn` body coerces the computed `Output`/`Mapped` type to the type the operation is
//! documented to produce, and the test compiles only if the two are the same type.
//!
//! `MapFields` is the one defined over *both* spines, so it is exercised over a `Cons`/`Nil`
//! product and an `Either`/`Void` sum, with the same marker applied to each.
//!
//! None of the three is in the prelude; they are imported from `cgp::core::field::traits`,
//! as is the `IsOptional` marker's home in `cgp::core::field::impls`.
//!
//! See cgp-knowledge-base/cgp/reference/traits/product_ops.md.

use cgp::core::field::impls::IsOptional;
use cgp::core::field::traits::{AppendProduct, ConcatProduct, MapFields};
use cgp::prelude::*;

/// `AppendProduct` adds one entry at the end of a product, keeping the existing entries in
/// order.
#[test]
fn test_append_product() {
    type Base = Product![Field<Symbol!("host"), String>];

    type WithPort = <Base as AppendProduct<Field<Symbol!("port"), u16>>>::Output;

    fn assert_appended(
        fields: WithPort,
    ) -> Product![Field<Symbol!("host"), String>, Field<Symbol!("port"), u16>] {
        fields
    }

    let appended = assert_appended(Cons(
        "localhost".to_owned().into(),
        Cons(8080_u16.into(), Nil),
    ));

    assert_eq!(appended.0.value, "localhost");
}

/// Appending to the empty product yields a one-entry product.
#[test]
fn test_append_to_the_empty_product() {
    type One = <Nil as AppendProduct<Field<Symbol!("host"), String>>>::Output;

    fn assert_one(fields: One) -> Product![Field<Symbol!("host"), String>] {
        fields
    }

    let one = assert_one(Cons("localhost".to_owned().into(), Nil));

    assert_eq!(one.0.value, "localhost");
}

/// `ConcatProduct` splices a whole product onto the end of another, which is what makes
/// append the single-entry special case of concat.
#[test]
fn test_concat_product() {
    type Left = Product![Field<Symbol!("host"), String>, Field<Symbol!("port"), u16>];
    type Right = Product![Field<Symbol!("tls"), bool>];

    type Joined = <Left as ConcatProduct<Right>>::Output;

    fn assert_joined(
        fields: Joined,
    ) -> Product![
        Field<Symbol!("host"), String>,
        Field<Symbol!("port"), u16>,
        Field<Symbol!("tls"), bool>,
    ] {
        fields
    }

    let joined = assert_joined(Cons(
        "localhost".to_owned().into(),
        Cons(8080_u16.into(), Cons(true.into(), Nil)),
    ));

    assert!(joined.1.1.0.value);
}

/// Concatenating onto the empty product returns the other product unchanged, and
/// concatenating the empty product onto one leaves it unchanged — the two identity cases.
#[test]
fn test_concat_identities() {
    type Fields = Product![Field<Symbol!("host"), String>];

    fn assert_left_identity(fields: <Nil as ConcatProduct<Fields>>::Output) -> Fields {
        fields
    }

    fn assert_right_identity(fields: <Fields as ConcatProduct<Nil>>::Output) -> Fields {
        fields
    }

    let left = assert_left_identity(Cons("a".to_owned().into(), Nil));
    let right = assert_right_identity(Cons("b".to_owned().into(), Nil));

    assert_eq!(left.0.value, "a");
    assert_eq!(right.0.value, "b");
}

/// `MapFields` leaves a product's length and order alone and rewrites each entry type
/// through the marker's `Map`. `IsPresent` is therefore the identity.
#[test]
fn test_map_fields_over_a_product() {
    type Fields = Product![String, u16, bool];

    fn assert_optional(
        fields: <Fields as MapFields<IsOptional>>::Mapped,
    ) -> Product![Option<String>, Option<u16>, Option<bool>,] {
        fields
    }

    fn assert_nothing(fields: <Fields as MapFields<IsNothing>>::Mapped) -> Product![(), (), ()] {
        fields
    }

    fn assert_present(fields: <Fields as MapFields<IsPresent>>::Mapped) -> Fields {
        fields
    }

    let optional = assert_optional(Cons(
        Some("a".to_owned()),
        Cons(Some(1), Cons(Some(true), Nil)),
    ));
    assert_eq!(optional.0, Some("a".to_owned()));

    let _nothing = assert_nothing(Cons((), Cons((), Cons((), Nil))));
    let present = assert_present(Cons("a".to_owned(), Cons(1, Cons(true, Nil))));
    assert_eq!(present.0, "a");
}

/// Mapping the empty product is the empty product, whatever the marker.
#[test]
fn test_map_fields_over_the_empty_product() {
    fn assert_empty(fields: <Nil as MapFields<IsOptional>>::Mapped) -> Nil {
        fields
    }

    assert_eq!(assert_empty(Nil), Nil);
}

/// The same marker applies over the `Either`/`Void` sum spine, which is what lets one
/// operation produce both a partial record and a partial enum.
#[test]
fn test_map_fields_over_a_sum() {
    type Variants = Sum![String, u16];

    fn assert_optional(
        variants: <Variants as MapFields<IsOptional>>::Mapped,
    ) -> Sum![Option<String>, Option<u16>] {
        variants
    }

    let mapped = assert_optional(Either::Left(Some("a".to_owned())));

    assert_eq!(mapped, Either::Left(Some("a".to_owned())));
}

/// Mapping the empty sum is the empty sum. `Void` is uninhabited, so this is a type-level
/// assertion with no value to build — the function existing is the whole check.
#[allow(dead_code)]
fn assert_map_fields_over_the_empty_sum(variants: <Void as MapFields<IsOptional>>::Mapped) -> Void {
    variants
}
