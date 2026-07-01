//! A `#[cgp_component]` trait may carry a default method body and a supertrait.
//!
//! The default body is copied into the generated provider trait, so a provider
//! declared with an empty `#[cgp_impl]` (here `UseDefault`) inherits it. This is
//! the distinct component-expansion variant with a supertrait bound and a default
//! method, kept alongside the plain snapshot in `component_macro`.
//!
//! See docs/implementation/entrypoints/cgp_component.md (Snapshots) for this
//! supertrait-plus-default-method variant, and docs/reference/macros/cgp_component.md
//! and docs/reference/attributes/extend.md for the user-facing semantics.

use cgp::prelude::*;
use cgp_macro_test_util::snapshot_cgp_component;

#[cgp_getter]
pub trait HasName {
    fn name(&self) -> &str {
        "John"
    }
}

snapshot_cgp_component! {
    #[cgp_component(Greeter)]
    pub trait CanGreet: HasName {
        fn greet(&self) -> String {
            format!("Hello, {}!", self.name())
        }
    }

    expand_can_greet(output) {
        insta::assert_snapshot!(output, @r#"
        pub trait CanGreet: HasName {
            fn greet(&self) -> String {
                format!("Hello, {}!", self.name())
            }
        }
        impl<__Context__> CanGreet for __Context__
        where
            __Context__: HasName,
            __Context__: Greeter<__Context__>,
        {
            fn greet(&self) -> String {
                __Context__::greet(self)
            }
        }
        pub trait Greeter<__Context__>: IsProviderFor<GreeterComponent, __Context__, ()>
        where
            __Context__: HasName,
        {
            fn greet(__context__: &__Context__) -> String {
                format!("Hello, {}!", __context__.name())
            }
        }
        impl<__Provider__, __Context__> Greeter<__Context__> for __Provider__
        where
            __Context__: HasName,
            __Provider__: DelegateComponent<GreeterComponent>
                + IsProviderFor<GreeterComponent, __Context__, ()>,
            <__Provider__ as DelegateComponent<
                GreeterComponent,
            >>::Delegate: Greeter<__Context__>,
        {
            fn greet(__context__: &__Context__) -> String {
                <__Provider__ as DelegateComponent<
                    GreeterComponent,
                >>::Delegate::greet(__context__)
            }
        }
        pub struct GreeterComponent;
        impl<__Context__> Greeter<__Context__> for UseContext
        where
            __Context__: HasName,
            __Context__: CanGreet,
        {
            fn greet(__context__: &__Context__) -> String {
                __Context__::greet(__context__)
            }
        }
        impl<__Context__> IsProviderFor<GreeterComponent, __Context__, ()> for UseContext
        where
            __Context__: HasName,
            __Context__: CanGreet,
        {}
        impl<__Context__, __Components__, __Path__> Greeter<__Context__>
        for RedirectLookup<__Components__, __Path__>
        where
            __Context__: HasName,
            __Components__: DelegateComponent<__Path__>,
            <__Components__ as DelegateComponent<__Path__>>::Delegate: Greeter<__Context__>,
        {
            fn greet(__context__: &__Context__) -> String {
                <__Components__ as DelegateComponent<__Path__>>::Delegate::greet(__context__)
            }
        }
        impl<
            __Context__,
            __Components__,
            __Path__,
        > IsProviderFor<GreeterComponent, __Context__, ()>
        for RedirectLookup<__Components__, __Path__>
        where
            __Context__: HasName,
            __Components__: DelegateComponent<__Path__>,
            <__Components__ as DelegateComponent<
                __Path__,
            >>::Delegate: IsProviderFor<GreeterComponent, __Context__, ()>
                + Greeter<__Context__>,
        {}
        "#)
    }
}

pub struct UseDefault;

// Empty provider impls pick up the default method bodies from the provider trait.
#[cgp_impl(UseDefault)]
impl<Context> NameGetter for Context {}

#[cgp_impl(UseDefault)]
impl<Context: HasName> Greeter for Context {}

#[test]
fn test_default_method_impl() {
    pub struct App;

    delegate_components! {
        App {
            [
                NameGetterComponent,
                GreeterComponent,
            ]:
                UseDefault,
        }
    }

    assert_eq!(App.greet(), "Hello, John!");
}
