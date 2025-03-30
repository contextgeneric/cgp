use core::fmt::Display;

use cgp::prelude::*;

#[test]
fn test_single_named_field() {
    #[derive(Clone, Debug, Eq, PartialEq, HasFields)]
    pub struct Person {
        pub name: String,
    }

    let name = "Alice".to_owned();

    let person1 = Person { name: name.clone() };

    let product = person1.clone().to_fields();
    assert_eq!(product, Cons(name.clone().into(), Nil));

    let product_ref = person1.to_fields_ref();
    assert_eq!(product_ref, Cons((&name).into(), Nil));

    let person2 = Person::from_fields(product);

    assert_eq!(person1, person2);
}

#[test]
fn test_two_named_field() {
    #[derive(Clone, Debug, Eq, PartialEq, HasFields)]
    pub struct Person {
        pub name: String,
        pub age: u8,
    }

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

#[test]
fn test_generic_struct() {
    #[derive(Clone, Debug, Eq, PartialEq, HasFields)]
    pub struct Person<Name>
    where
        Name: Display,
    {
        pub name: Name,
        pub age: u8,
    }

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

// #[test]
// fn test_single_unnamed_field() {
//     #[derive(HasFields)]
//     pub struct Person(String);

//     let name = "Alice".to_owned();

//     let person = Person { 0: name.clone() };

//     let product = person.to_fields();
//     assert_eq!(product, Cons(name.clone().into(), Nil));
// }
