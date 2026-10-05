//! `check_components!` with `#[check_providers(...)]` over a *generic* table
//! (`<T> Wrapper<T>`). The check trait takes the context as a parameter, so the
//! table's `T` stays on the generated impls that declare it rather than appearing,
//! undeclared, in the trait's supertrait. The snapshot pins the `impl<T>` impl the
//! generic table produces, and the test checks that the checked provider serves the
//! context at run time.
//!
//! See cgp-knowledge-base-fork/cgp/implementation/entrypoints/check_components.md.

use cgp_fork::prelude::*;
use cgp_fork_macro_test_util::snapshot_check_components;

#[cgp_getter]
pub trait HasName {
    fn name(&self) -> &str;
}

#[derive(HasField)]
pub struct Wrapper<T> {
    pub name: String,
    pub inner: T,
}

delegate_components! {
    <T> Wrapper<T> {
        NameGetterComponent: UseField<Symbol!("name")>,
    }
}

snapshot_check_components! {
    check_components! {
        #[check_trait(CheckWrapperProviders)]
        #[check_providers(UseField<Symbol!("name")>)]
        <T> Wrapper<T> {
            NameGetterComponent,
        }
    }

    expand_generic_check_providers(output) {
        insta::assert_snapshot!(output, @r#"
        trait CheckWrapperProviders<
            __Component__,
            __Context__,
            __Params__: ?Sized,
        >: IsProviderFor<__Component__, __Context__, __Params__> {}
        impl<T> CheckWrapperProviders<NameGetterComponent, Wrapper<T>, ()>
        for UseField<Symbol!("name")> {}
        "#)
    }
}

#[test]
fn test_check_providers_over_a_generic_table() {
    let wrapper = Wrapper {
        name: "wrapped".to_owned(),
        inner: 5u8,
    };
    assert_eq!(wrapper.name(), "wrapped");
    assert_eq!(wrapper.inner, 5);
}
