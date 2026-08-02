//! `#[derive(CgpRecord)]`, the struct-only face of `#[derive(CgpData)]`.
//!
//! `CgpRecord` and `CgpData`-on-a-struct run the same codegen, so this file
//! does not re-snapshot the expansion that `record_derive` already pins. What it
//! does check is that the one derive really delivers all three record slices at
//! once — the per-field getters, the whole-shape field list, and the builder —
//! since that is the reason to reach for it over deriving the three separately.
//!
//! The difference from `CgpData` is at the parser rather than in the output:
//! `CgpRecord` refuses a non-struct item outright, which is what makes it worth
//! writing when the type will always be a struct.
//!
//! See cgp-knowledge-base/cgp/reference/derives/derive_cgp_record.md.

use core::marker::PhantomData;

use cgp::prelude::*;

#[derive(CgpRecord, Clone, Debug, Eq, PartialEq)]
pub struct Person {
    pub first_name: String,
    pub last_name: String,
}

#[test]
fn test_per_field_getters() {
    let mut person = Person {
        first_name: "Alice".to_owned(),
        last_name: "Anderson".to_owned(),
    };

    // The `HasField`/`HasFieldMut` slice.
    assert_eq!(
        person.get_field(PhantomData::<Symbol!("first_name")>),
        "Alice"
    );

    *person.get_field_mut(PhantomData::<Symbol!("last_name")>) = "Baker".to_owned();
    assert_eq!(person.last_name, "Baker");
}

#[test]
fn test_whole_shape_round_trip() {
    // The `HasFields`/`ToFields`/`FromFields` slice.
    let person1 = Person {
        first_name: "Alice".to_owned(),
        last_name: "Anderson".to_owned(),
    };

    let fields = person1.clone().to_fields();
    let person2 = Person::from_fields(fields);

    assert_eq!(person1, person2);
}

#[test]
fn test_incremental_builder() {
    // The `BuildField` slice.
    let person: Person = Person::builder()
        .build_field(PhantomData::<Symbol!("first_name")>, "Alice".to_owned())
        .build_field(PhantomData::<Symbol!("last_name")>, "Anderson".to_owned())
        .finalize_build();

    assert_eq!(
        person,
        Person {
            first_name: "Alice".to_owned(),
            last_name: "Anderson".to_owned(),
        }
    );
}
