//! `#[derive_provider(WithProvider)]` generates the `WithProvider` impl for a
//! `#[cgp_component]` trait. The struct already exists; the macro only emits
//! the impl. An abstract type bridges through `TypeProvider`, a getter through
//! `FieldGetter`.
//!
//! See cgp-knowledge-base-fork/cgp/reference/macros/derive_provider.md.

use cgp_fork::prelude::*;

#[derive_provider(WithProvider)]
#[cgp_component(NameTypeProvider)]
pub trait HasNameType {
    type Name;
}

pub struct Named;

delegate_components! {
    Named {
        NameTypeProviderComponent: WithProvider<UseType<&'static str>>,
    }
}

pub trait CheckName: HasNameType<Name = &'static str> {}

impl CheckName for Named {}

#[derive_provider(WithProvider)]
#[cgp_component(NameGetter)]
pub trait HasName {
    fn name(&self) -> &str;
}

#[derive(HasField)]
pub struct Person {
    pub name: String,
}

delegate_components! {
    Person {
        NameGetterComponent: WithProvider<UseField<Symbol!("name")>>,
    }
}

#[test]
fn derived_with_provider_reads_the_field() {
    let person = Person {
        name: "Ada".to_owned(),
    };

    assert_eq!(person.name(), "Ada");
}
