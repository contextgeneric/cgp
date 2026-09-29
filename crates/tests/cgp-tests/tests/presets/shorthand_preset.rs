//! One preset written with the type and getter shorthand expands to the
//! `DelegateComponent` impls a context needs.
//!
//! See cgp-knowledge-base/cgp/reference/macros/cgp_preset.md.

use cgp::prelude::*;

#[cgp_type]
pub trait HasAgeType {
    type Age;
}

#[cgp_getter]
pub trait HasName {
    fn name(&self) -> &str;
}

cgp_preset! {
    PersonPreset {
        type Age = u8,
        getter name,
    }
}

#[derive(HasField)]
pub struct Person {
    pub name: String,
    pub age: u8,
}

delegate_components! {
    Person {
        preset PersonPreset,
    }
}

pub trait CheckAge: HasAgeType<Age = u8> {}

impl CheckAge for Person {}

#[test]
fn preset_entry_wires_every_component() {
    let person = Person {
        name: "Ada".to_owned(),
        age: 36,
    };

    assert_eq!(person.name(), "Ada");
}
