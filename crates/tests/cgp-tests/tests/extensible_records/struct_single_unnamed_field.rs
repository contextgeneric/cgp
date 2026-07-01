//! `#[derive(HasFields)]` on a newtype (single unnamed field).
//!
//! A one-element tuple struct is a special case: its `Fields` is the inner type
//! directly (not a `Cons`/`Nil` list), and `from_fields`/`to_fields` pass the
//! single value straight through. The derive expansion is owned by this concept.
//!
//! See docs/reference/derives/derive_has_fields.md.

use cgp::prelude::*;
use cgp_macro_test_util::snapshot_derive_has_fields;

snapshot_derive_has_fields! {
    #[derive(HasFields)]
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct Person(String);

    expand_person(output) {
        insta::assert_snapshot!(output, @"
        impl HasFields for Person {
            type Fields = String;
        }
        impl HasFieldsRef for Person {
            type FieldsRef<'__a> = &'__a String where Self: '__a;
        }
        impl FromFields for Person {
            fn from_fields(field: Self::Fields) -> Self {
                Self(field)
            }
        }
        impl ToFields for Person {
            fn to_fields(self) -> Self::Fields {
                self.0
            }
        }
        impl ToFieldsRef for Person {
            fn to_fields_ref<'__a>(&'__a self) -> Self::FieldsRef<'__a>
            where
                Self: '__a,
            {
                &self.0
            }
        }
        ")
    }
}

#[test]
fn test_single_unnamed_field() {
    let name = "Alice".to_owned();

    let person1 = Person(name.clone());

    let product = person1.clone().to_fields();
    assert_eq!(product, name.clone());

    let product_ref = person1.to_fields_ref();
    assert_eq!(product_ref, &name);

    let person2 = Person::from_fields(product);

    assert_eq!(person1, person2);
}
