//! `#[cgp_impl]`'s `: ComponentType` override targets a component whose marker
//! name does not follow the `{Provider}Component` convention.
//!
//! Here the component is declared with an explicit `name` (`HasFooComponent`)
//! that differs from the provider trait's name plus `Component`
//! (`FooProviderComponent`). A provider for it must name the component with the
//! `#[cgp_impl(new Provider: Component)]` override; without it the macro derives
//! `FooProviderComponent`, which does not exist here, so the generated
//! `IsProviderFor` impl fails to resolve.
//!
//! See cgp-knowledge-base/cgp/reference/macros/cgp_impl.md.

use cgp::prelude::*;
use cgp_macro_test_util::snapshot_cgp_impl;

#[cgp_component {
    name: HasFooComponent,
    provider: FooProvider,
}]
pub trait CanDoFoo {
    fn foo(&self) -> u32;
}

snapshot_cgp_impl! {
    #[cgp_impl(new FortyTwo: HasFooComponent)]
    impl FooProvider {
        fn foo(&self) -> u32 {
            42
        }
    }

    expand_forty_two(output) {
        insta::assert_snapshot!(output, @"
        impl<__Context__> FooProvider<__Context__> for FortyTwo {
            fn foo(__context__: &__Context__) -> u32 {
                42
            }
        }
        impl<__Context__> IsProviderFor<HasFooComponent, __Context__, ()> for FortyTwo {}
        pub struct FortyTwo;
        ")
    }
}

pub struct App;

delegate_components! {
    App {
        HasFooComponent: FortyTwo,
    }
}

#[test]
fn test_component_override() {
    // The `: HasFooComponent` override makes `FortyTwo` a provider for the
    // non-conventionally-named component, so `App` implements `CanDoFoo`.
    assert_eq!(App.foo(), 42);
}
