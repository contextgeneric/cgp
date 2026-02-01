use core::fmt::Display;

use cgp::prelude::*;

#[cgp_getter]
pub trait HasName {
    type Name: Display;

    fn name(&self) -> &Self::Name;
}

#[derive(HasField)]
pub struct Person {
    pub first_name: String,
}

delegate_components! {
    Person {
        NameGetterComponent:
            UseField<Symbol!("first_name")>,
    }
}

pub trait CheckHasName: HasName<Name = String> {}
impl CheckHasName for Person {}
