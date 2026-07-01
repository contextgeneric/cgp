//! `#[derive(HasFields)]` on a *generic* struct with a `where` clause.
//!
//! Each generated impl carries the struct's `Name` parameter and forwards its
//! `where Name: Display` bound, so the field-list derive works on parameterized
//! structs. The derive expansion is owned by this concept.
//!
//! See docs/reference/derives/derive_has_fields.md.

use core::fmt::Display;

use cgp::prelude::*;
use cgp_macro_test_util::snapshot_derive_has_fields;

snapshot_derive_has_fields! {
    #[derive(HasFields)]
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct Person<Name>
    where
        Name: Display,
    {
        pub name: Name,
        pub age: u8,
    }

    expand_person(output) {
        insta::assert_snapshot!(output, @"
        impl<Name> HasFields for Person<Name>
        where
            Name: Display,
        {
            type Fields = Cons<
                Field<Symbol<4, Chars<'n', Chars<'a', Chars<'m', Chars<'e', Nil>>>>>, Name>,
                Cons<Field<Symbol<3, Chars<'a', Chars<'g', Chars<'e', Nil>>>>, u8>, Nil>,
            >;
        }
        impl<Name> HasFieldsRef for Person<Name>
        where
            Name: Display,
        {
            type FieldsRef<'__a> = Cons<
                Field<
                    Symbol<4, Chars<'n', Chars<'a', Chars<'m', Chars<'e', Nil>>>>>,
                    &'__a Name,
                >,
                Cons<Field<Symbol<3, Chars<'a', Chars<'g', Chars<'e', Nil>>>>, &'__a u8>, Nil>,
            >
            where
                Self: '__a;
        }
        impl<Name> FromFields for Person<Name>
        where
            Name: Display,
        {
            fn from_fields(Cons(name, Cons(age, Nil)): Self::Fields) -> Self {
                Self {
                    name: name.value,
                    age: age.value,
                }
            }
        }
        impl<Name> ToFields for Person<Name>
        where
            Name: Display,
        {
            fn to_fields(self) -> Self::Fields {
                Cons(self.name.into(), Cons(self.age.into(), Nil))
            }
        }
        impl<Name> ToFieldsRef for Person<Name>
        where
            Name: Display,
        {
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
fn test_generic_struct() {
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
