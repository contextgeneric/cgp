//! `#[derive(HasFields)]` for a struct with two named fields.
//!
//! Shows the two-field `Cons<_, Cons<_, Nil>>` field list and the matching
//! `from_fields`/`to_fields` round-trip. The derive expansion is owned by this
//! concept; `struct_single_named_field` pins the canonical reference.
//!
//! See docs/reference/derives/derive_has_fields.md.

use cgp::prelude::*;
use cgp_macro_test_util::snapshot_derive_has_fields;

snapshot_derive_has_fields! {
    #[derive(HasFields)]
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct Person {
        pub name: String,
        pub age: u8,
    }

    expand_person(output) {
        insta::assert_snapshot!(output, @"
        impl HasFields for Person {
            type Fields = Cons<
                Field<Symbol<4, Chars<'n', Chars<'a', Chars<'m', Chars<'e', Nil>>>>>, String>,
                Cons<Field<Symbol<3, Chars<'a', Chars<'g', Chars<'e', Nil>>>>, u8>, Nil>,
            >;
        }
        impl HasFieldsRef for Person {
            type FieldsRef<'__a> = Cons<
                Field<
                    Symbol<4, Chars<'n', Chars<'a', Chars<'m', Chars<'e', Nil>>>>>,
                    &'__a String,
                >,
                Cons<Field<Symbol<3, Chars<'a', Chars<'g', Chars<'e', Nil>>>>, &'__a u8>, Nil>,
            >
            where
                Self: '__a;
        }
        impl FromFields for Person {
            fn from_fields(Cons(name, Cons(age, Nil)): Self::Fields) -> Self {
                Self {
                    name: name.value,
                    age: age.value,
                }
            }
        }
        impl ToFields for Person {
            fn to_fields(self) -> Self::Fields {
                Cons(self.name.into(), Cons(self.age.into(), Nil))
            }
        }
        impl ToFieldsRef for Person {
            fn to_fields_ref<'__a>(&'__a self) -> Self::FieldsRef<'__a>
            where
                Self: '__a,
            {
                Cons((&self.name).into(), Cons((&self.age).into(), Nil))
            }
        }
        ")
    }
}

#[test]
fn test_two_named_field() {
    let name = "Alice".to_owned();

    let person1 = Person {
        name: name.clone(),
        age: 32,
    };

    let product = person1.clone().to_fields();
    assert_eq!(product, Cons(name.clone().into(), Cons(32.into(), Nil)));

    let product_ref = person1.to_fields_ref();
    assert_eq!(product_ref, Cons((&name).into(), Cons((&32).into(), Nil)));

    let person2 = Person::from_fields(product);

    assert_eq!(person1, person2);
}
