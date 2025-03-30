use core::fmt::Display;

use cgp::prelude::*;

#[test]
fn test_single_named_field() {
    #[derive(HasFields)]
    pub struct Person {
        pub name: String,
    }

    let name = "Alice".to_owned();

    let person = Person { name: name.clone() };

    let product = person.to_fields();
    assert_eq!(product, Cons(name.clone().into(), Nil));
}

#[test]
fn test_two_named_field() {
    #[derive(HasFields)]
    pub struct Person {
        pub name: String,
        pub age: u8,
    }

    let name = "Alice".to_owned();

    let person = Person {
        name: name.clone(),
        age: 32,
    };

    let product = person.to_fields();
    assert_eq!(product, Cons(name.clone().into(), Cons(32.into(), Nil)));
}

#[test]
fn test_generic_struct() {
    #[derive(HasFields)]
    pub struct Person<Name>
    where
        Name: Display,
    {
        pub name: Name,
        pub age: u8,
    }

    let name = "Alice".to_owned();

    let person = Person {
        name: name.clone(),
        age: 32,
    };

    let product = person.to_fields();
    assert_eq!(product, Cons(name.clone().into(), Cons(32.into(), Nil)));
}
