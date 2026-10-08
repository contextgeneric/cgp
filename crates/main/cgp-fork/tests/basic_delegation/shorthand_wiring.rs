//! A context wired with the type and getter shorthand reads the same values as
//! one wired with explicit `UseType` and `UseField`.
//!
//! See cgp-knowledge-base-fork/cgp/reference/macros/delegate_components.md.

use cgp_fork::prelude::*;

#[cgp_type]
pub trait HasAgeType {
    type Age;
}

#[cgp_getter]
pub trait HasName {
    fn name(&self) -> &str;
}

#[derive(HasField)]
pub struct ShorthandPerson {
    pub name: String,
    pub age: u8,
}

delegate_components! {
    ShorthandPerson {
        type Age = u8,
        getter name,
    }
}

#[derive(HasField)]
pub struct ExplicitPerson {
    pub name: String,
    pub age: u8,
}

delegate_components! {
    ExplicitPerson {
        AgeTypeProviderComponent: UseType<u8>,
        NameGetterComponent: UseField<Symbol!("name")>,
    }
}

pub trait CheckAge: HasAgeType<Age = u8> {}

impl CheckAge for ShorthandPerson {}
impl CheckAge for ExplicitPerson {}

#[test]
fn shorthand_matches_explicit_wiring() {
    let shorthand = ShorthandPerson {
        name: "Ada".to_owned(),
        age: 36,
    };
    let explicit = ExplicitPerson {
        name: "Ada".to_owned(),
        age: 36,
    };

    assert_eq!(shorthand.name(), explicit.name());
    assert_eq!(shorthand.age, explicit.age);
}
