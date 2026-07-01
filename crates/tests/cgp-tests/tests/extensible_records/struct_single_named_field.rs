//! Canonical expansion of `#[derive(HasFields)]` for a struct with one named
//! field.
//!
//! Unlike `#[derive(CgpData)]`, `#[derive(HasFields)]` derives *only* the field
//! list: `HasFields`/`HasFieldsRef` (the type-level `Cons`/`Nil` spine) plus the
//! `FromFields`/`ToFields`/`ToFieldsRef` round-trip — no per-field `HasField`
//! access and no builder. This is the reference snapshot for that derive on a
//! struct; the other `struct_*` files reuse it for the remaining field shapes.
//!
//! See docs/reference/derives/derive_has_fields.md and
//! docs/reference/traits/has_fields.md.

use cgp::prelude::*;
use cgp_macro_test_util::snapshot_derive_has_fields;

snapshot_derive_has_fields! {
    #[derive(HasFields)]
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct Person {
        pub name: String,
    }

    expand_person(output) {
        insta::assert_snapshot!(output, @"
        impl HasFields for Person {
            type Fields = Cons<
                Field<Symbol<4, Chars<'n', Chars<'a', Chars<'m', Chars<'e', Nil>>>>>, String>,
                Nil,
            >;
        }
        impl HasFieldsRef for Person {
            type FieldsRef<'__a> = Cons<
                Field<
                    Symbol<4, Chars<'n', Chars<'a', Chars<'m', Chars<'e', Nil>>>>>,
                    &'__a String,
                >,
                Nil,
            >
            where
                Self: '__a;
        }
        impl FromFields for Person {
            fn from_fields(Cons(name, Nil): Self::Fields) -> Self {
                Self { name: name.value }
            }
        }
        impl ToFields for Person {
            fn to_fields(self) -> Self::Fields {
                Cons(self.name.into(), Nil)
            }
        }
        impl ToFieldsRef for Person {
            fn to_fields_ref<'__a>(&'__a self) -> Self::FieldsRef<'__a>
            where
                Self: '__a,
            {
                Cons((&self.name).into(), Nil)
            }
        }
        ")
    }
}

#[test]
fn test_single_named_field() {
    let name = "Alice".to_owned();

    let person1 = Person { name: name.clone() };

    let product = person1.clone().to_fields();
    assert_eq!(product, Cons(name.clone().into(), Nil));

    let product_ref = person1.to_fields_ref();
    assert_eq!(product_ref, Cons((&name).into(), Nil));

    let person2 = Person::from_fields(product);

    assert_eq!(person1, person2);
}
