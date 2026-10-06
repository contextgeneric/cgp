//! A value of a `Struct!` type moves in and out of the struct it describes
//! through `ToFields` and `FromFields`, and a value built by hand with
//! `product!` rebuilds the struct, which is what makes the macro usable for
//! naming a shape in code that converts values.
//!
//! See cgp-knowledge-base/cgp/implementation/entrypoints/struct.md.

use cgp::prelude::*;

#[derive(Debug, PartialEq, HasFields)]
pub struct Person {
    pub name: String,
    pub age: u8,
}

#[derive(Debug, PartialEq, HasFields)]
pub struct Pair(pub u64, pub String);

#[test]
fn a_named_struct_round_trips_through_its_shape() {
    let person = Person {
        name: "Alice".to_owned(),
        age: 30,
    };

    let fields: Struct! { name: String, age: u8 } = person.to_fields();

    let Cons(name, Cons(age, Nil)) = &fields;
    assert_eq!(name.value, "Alice");
    assert_eq!(age.value, 30);

    assert_eq!(
        Person::from_fields(fields),
        Person {
            name: "Alice".to_owned(),
            age: 30,
        }
    );
}

#[test]
fn a_tuple_struct_round_trips_through_its_shape() {
    let fields: Struct!(u64, String) = Pair(7, "seven".to_owned()).to_fields();

    assert_eq!(Pair::from_fields(fields), Pair(7, "seven".to_owned()));
}

#[test]
fn a_value_built_by_hand_rebuilds_the_struct() {
    let fields: Struct! { name: String, age: u8 } = product!["Bob".to_owned().into(), 40u8.into()];

    assert_eq!(
        Person::from_fields(fields),
        Person {
            name: "Bob".to_owned(),
            age: 40,
        }
    );
}
