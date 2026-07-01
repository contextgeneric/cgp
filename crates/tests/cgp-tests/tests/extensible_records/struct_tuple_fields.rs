//! `#[derive(HasFields)]` on a multi-field tuple struct.
//!
//! With more than one unnamed field, the derive keys the field list by `Index<N>`
//! rather than treating it as a bare newtype, producing the usual `Cons`/`Nil`
//! spine. The derive expansion is owned by this concept.
//!
//! See docs/reference/derives/derive_has_fields.md.

use cgp::prelude::*;
use cgp_macro_test_util::snapshot_derive_has_fields;

snapshot_derive_has_fields! {
    #[derive(HasFields)]
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct Person(String, u8);

    expand_person(output) {
        insta::assert_snapshot!(output, @"
        impl HasFields for Person {
            type Fields = Cons<Field<Index<0>, String>, Cons<Field<Index<1>, u8>, Nil>>;
        }
        impl HasFieldsRef for Person {
            type FieldsRef<'__a> = Cons<
                Field<Index<0>, &'__a String>,
                Cons<Field<Index<1>, &'__a u8>, Nil>,
            >
            where
                Self: '__a;
        }
        impl FromFields for Person {
            fn from_fields(Cons(field_1, Cons(field_0, Nil)): Self::Fields) -> Self {
                Self(field_1.value, field_0.value)
            }
        }
        impl ToFields for Person {
            fn to_fields(self) -> Self::Fields {
                Cons(self.0.into(), Cons(self.1.into(), Nil))
            }
        }
        impl ToFieldsRef for Person {
            fn to_fields_ref<'__a>(&'__a self) -> Self::FieldsRef<'__a>
            where
                Self: '__a,
            {
                Cons((&self.0).into(), Cons((&self.1).into(), Nil))
            }
        }
        ")
    }
}

#[test]
fn test_single_unnamed_multi_field() {
    let name = "Alice".to_owned();

    let person1 = Person(name.clone(), 32);

    let product = person1.clone().to_fields();
    assert_eq!(product, Cons(name.clone().into(), Cons(32.into(), Nil)));

    let product_ref = person1.to_fields_ref();
    assert_eq!(product_ref, Cons((&name).into(), Cons((&32).into(), Nil)));

    let person2 = Person::from_fields(product);

    assert_eq!(person1, person2);
}
