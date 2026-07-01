//! `#[derive(HasFields)]` on a struct with both a lifetime and a type parameter.
//!
//! Fields that borrow (`&'a Name`, `&'a u8`) show how the derive threads the
//! struct's own lifetime through the field list, while `FieldsRef` adds its
//! separate `'__a` borrow on top (`&'__a &'a Name`). The derive expansion is
//! owned by this concept.
//!
//! See docs/reference/derives/derive_has_fields.md.

use core::fmt::Display;

use cgp::prelude::*;
use cgp_macro_test_util::snapshot_derive_has_fields;

snapshot_derive_has_fields! {
    #[derive(HasFields)]
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct Person<'a, Name>
    where
        Name: Display,
    {
        pub name: &'a Name,
        pub age: &'a u8,
    }

    expand_person(output) {
        insta::assert_snapshot!(output, @"
        impl<'a, Name> HasFields for Person<'a, Name>
        where
            Name: Display,
        {
            type Fields = Cons<
                Field<Symbol<4, Chars<'n', Chars<'a', Chars<'m', Chars<'e', Nil>>>>>, &'a Name>,
                Cons<Field<Symbol<3, Chars<'a', Chars<'g', Chars<'e', Nil>>>>, &'a u8>, Nil>,
            >;
        }
        impl<'a, Name> HasFieldsRef for Person<'a, Name>
        where
            Name: Display,
        {
            type FieldsRef<'__a> = Cons<
                Field<
                    Symbol<4, Chars<'n', Chars<'a', Chars<'m', Chars<'e', Nil>>>>>,
                    &'__a &'a Name,
                >,
                Cons<
                    Field<Symbol<3, Chars<'a', Chars<'g', Chars<'e', Nil>>>>, &'__a &'a u8>,
                    Nil,
                >,
            >
            where
                Self: '__a;
        }
        impl<'a, Name> FromFields for Person<'a, Name>
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
        impl<'a, Name> ToFields for Person<'a, Name>
        where
            Name: Display,
        {
            fn to_fields(self) -> Self::Fields {
                Cons(self.name.into(), Cons(self.age.into(), Nil))
            }
        }
        impl<'a, Name> ToFieldsRef for Person<'a, Name>
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
fn test_generic_lifetime_struct() {
    let name = "Alice".to_owned();

    let person1 = Person {
        name: &name,
        age: &32,
    };

    let product = person1.clone().to_fields();
    assert_eq!(product, Cons((&name).into(), Cons((&32).into(), Nil)));

    let product_ref = person1.to_fields_ref();
    assert_eq!(product_ref, Cons((&&name).into(), Cons((&&32).into(), Nil)));

    let person2 = Person::from_fields(product);

    assert_eq!(person1, person2);
}
