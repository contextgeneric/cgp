//! The two field derives on a **unit struct**, the degenerate record shape.
//!
//! A unit struct has no fields, and the two derives answer that differently
//! rather than both erroring. `#[derive(HasField)]` emits *nothing at all*,
//! since there is no field to key an accessor on. `#[derive(HasFields)]` still
//! emits all five representation impls, with `Fields` being the empty product
//! `Nil` — so a unit struct is a valid, if trivial, extensible record and
//! round-trips through `to_fields`/`from_fields`.
//!
//! See cgp-knowledge-base/cgp/reference/derives/derive_has_field.md and
//! cgp-knowledge-base/cgp/reference/derives/derive_has_fields.md.

use cgp::prelude::*;
use cgp_macro_test_util::{snapshot_derive_has_field, snapshot_derive_has_fields};

snapshot_derive_has_field! {
    #[derive(HasField)]
    pub struct NoFields;

    expand_has_field(output) {
        insta::assert_snapshot!(output, @"")
    }
}

snapshot_derive_has_fields! {
    #[derive(HasFields)]
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct Unit;

    expand_has_fields(output) {
        insta::assert_snapshot!(output, @"
        impl HasFields for Unit {
            type Fields = Nil;
        }
        impl HasFieldsRef for Unit {
            type FieldsRef<'__a> = Nil where Self: '__a;
        }
        impl FromFields for Unit {
            fn from_fields(Nil: Self::Fields) -> Self {
                Self
            }
        }
        impl ToFields for Unit {
            fn to_fields(self) -> Self::Fields {
                Nil
            }
        }
        impl ToFieldsRef for Unit {
            fn to_fields_ref<'__a>(&'__a self) -> Self::FieldsRef<'__a>
            where
                Self: '__a,
            {
                Nil
            }
        }
        ")
    }
}

#[test]
fn test_unit_struct_round_trip() {
    let unit1 = Unit;

    let fields = unit1.clone().to_fields();
    assert_eq!(fields, Nil);

    let fields_ref = unit1.to_fields_ref();
    assert_eq!(fields_ref, Nil);

    let unit2 = Unit::from_fields(fields);
    assert_eq!(unit1, unit2);
}
