//! Regression: `#[default_impl]` on a provider that carries a `#[use_type]`
//! abstract-type dependency (and an `#[implicit]` field read).
//!
//! `#[default_impl]` builds its namespace-registration impl (`impl Namespace<..>
//! for PathKey { type Delegate = Provider; }`) from the provider impl's generics,
//! which by this stage already carry the impl-side bounds that `#[use_type]`,
//! `#[uses]`, `#[implicit]`, and `#[use_provider]` push onto `Self`. Those bounds
//! must be dropped: the registration impl's `Self` is the path key
//! (`PathCons<..>`), so a leaked `Self: HasNameType` would demand `PathCons<..>:
//! HasNameType` and never resolve, silently breaking every context that joins the
//! namespace. The snapshot pins that the registration impl carries only
//! `__Components__` and no `where` clause; the wiring below proves a context
//! resolves the provider (and its abstract-type dependency) through the namespace.
//!
//! See docs/implementation/asts/attributes.md (`#[default_impl]`) and
//! docs/reference/traits/default_namespace.md.

use cgp::prelude::*;
use cgp_macro_test_util::snapshot_cgp_impl;

#[cgp_type]
#[prefix(@app.types in DefaultNamespace)]
pub trait HasNameType {
    type Name: core::fmt::Display;
}

#[cgp_component(Greeter)]
#[prefix(@app.core in DefaultNamespace)]
#[use_type(HasNameType.Name)]
pub trait CanGreet {
    fn greet(&self) -> String;
}

snapshot_cgp_impl! {
    #[cgp_impl(new GreetByName)]
    #[default_impl(@app.core.GreeterComponent in AppNamespace)]
    #[use_type(HasNameType.Name)]
    impl Greeter {
        fn greet(&self, #[implicit] name: &Name) -> String {
            format!("Hello, {name}!")
        }
    }

    expand_greet_by_name(output) {
        insta::assert_snapshot!(output, @r#"
        impl<__Context__> Greeter<__Context__> for GreetByName
        where
            __Context__: HasField<
                Symbol<4, Chars<'n', Chars<'a', Chars<'m', Chars<'e', Nil>>>>>,
                Value = <__Context__ as HasNameType>::Name,
            >,
            __Context__: HasNameType,
        {
            fn greet(__context__: &__Context__) -> String {
                let name: &<__Context__ as HasNameType>::Name = __context__
                    .get_field(
                        ::core::marker::PhantomData::<
                            Symbol<4, Chars<'n', Chars<'a', Chars<'m', Chars<'e', Nil>>>>>,
                        >,
                    );
                format!("Hello, {name}!")
            }
        }
        impl<__Context__> IsProviderFor<GreeterComponent, __Context__, ()> for GreetByName
        where
            __Context__: HasField<
                Symbol<4, Chars<'n', Chars<'a', Chars<'m', Chars<'e', Nil>>>>>,
                Value = <__Context__ as HasNameType>::Name,
            >,
            __Context__: HasNameType,
        {}
        pub struct GreetByName;
        impl<__Components__> AppNamespace<__Components__>
        for PathCons<
            Symbol<3, Chars<'a', Chars<'p', Chars<'p', Nil>>>>,
            PathCons<
                Symbol<4, Chars<'c', Chars<'o', Chars<'r', Chars<'e', Nil>>>>>,
                PathCons<GreeterComponent, Nil>,
            >,
        > {
            type Delegate = GreetByName;
        }
        "#)
    }
}

cgp_namespace! {
    new AppNamespace: DefaultNamespace {
        @app.types.NameTypeProviderComponent:
            UseType<String>,
    }
}

#[derive(HasField)]
pub struct App {
    pub name: String,
}

delegate_components! {
    App {
        namespace AppNamespace;
    }
}

check_components! {
    App {
        NameTypeProviderComponent,
        GreeterComponent,
    }
}

#[test]
fn test_default_impl_use_type() {
    let app = App {
        name: "World".to_owned(),
    };

    assert_eq!(app.greet(), "Hello, World!");
}
