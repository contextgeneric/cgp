//! `RedirectLookup` re-routing a component lookup along a `#[prefix]` path.
//!
//! `#[prefix(@bar.baz in DefaultNamespace)]` makes `CanDoFoo`'s default lookup
//! redirect through `RedirectLookup<Components, Path!["bar", "baz", FooProviderComponent]>`,
//! so its snapshot carries the `DefaultNamespace<..> for FooProviderComponent` impl
//! that names that path. A context then wires the whole `@bar.baz` prefix to a
//! provider in one entry, and the provider is reached via the redirect. The
//! `#[prefix]`-component and namespace `delegate_components!` snapshots are the
//! canonical golden output; the `TestProvider` impl is incidental scaffolding.
//!
//! See docs/reference/providers/redirect_lookup.md and
//! docs/reference/traits/default_namespace.md.

use cgp::prelude::*;
use cgp_macro_test_util::{snapshot_cgp_component, snapshot_delegate_components};

snapshot_cgp_component! {
    #[cgp_component(FooProvider)]
    #[prefix(@bar.baz in DefaultNamespace)]
    pub trait CanDoFoo {
        fn foo();
    }

    expand_redirect_foo(output) {
        insta::assert_snapshot!(output, @"
        pub trait CanDoFoo {
            fn foo();
        }
        impl<__Context__> CanDoFoo for __Context__
        where
            __Context__: FooProvider<__Context__>,
        {
            fn foo() {
                __Context__::foo()
            }
        }
        pub trait FooProvider<
            __Context__,
        >: IsProviderFor<FooProviderComponent, __Context__, ()> {
            fn foo();
        }
        impl<__Provider__, __Context__> FooProvider<__Context__> for __Provider__
        where
            __Provider__: DelegateComponent<FooProviderComponent>
                + IsProviderFor<FooProviderComponent, __Context__, ()>,
            <__Provider__ as DelegateComponent<
                FooProviderComponent,
            >>::Delegate: FooProvider<__Context__>,
        {
            fn foo() {
                <__Provider__ as DelegateComponent<FooProviderComponent>>::Delegate::foo()
            }
        }
        pub struct FooProviderComponent;
        impl<__Context__> FooProvider<__Context__> for UseContext
        where
            __Context__: CanDoFoo,
        {
            fn foo() {
                __Context__::foo()
            }
        }
        impl<__Context__> IsProviderFor<FooProviderComponent, __Context__, ()> for UseContext
        where
            __Context__: CanDoFoo,
        {}
        impl<__Context__, __Components__, __Path__> FooProvider<__Context__>
        for RedirectLookup<__Components__, __Path__>
        where
            __Components__: DelegateComponent<__Path__>,
            <__Components__ as DelegateComponent<__Path__>>::Delegate: FooProvider<__Context__>,
        {
            fn foo() {
                <__Components__ as DelegateComponent<__Path__>>::Delegate::foo()
            }
        }
        impl<
            __Context__,
            __Components__,
            __Path__,
        > IsProviderFor<FooProviderComponent, __Context__, ()>
        for RedirectLookup<__Components__, __Path__>
        where
            __Components__: DelegateComponent<__Path__>,
            <__Components__ as DelegateComponent<
                __Path__,
            >>::Delegate: IsProviderFor<FooProviderComponent, __Context__, ()>
                + FooProvider<__Context__>,
        {}
        impl<__Components__> DefaultNamespace<__Components__> for FooProviderComponent {
            type Delegate = RedirectLookup<
                __Components__,
                PathCons<
                    Symbol<3, Chars<'b', Chars<'a', Chars<'r', Nil>>>>,
                    PathCons<
                        Symbol<3, Chars<'b', Chars<'a', Chars<'z', Nil>>>>,
                        PathCons<FooProviderComponent, Nil>,
                    >,
                >,
            >;
        }
        ")
    }
}

pub struct BarComponent;

pub struct BazComponent;

// Incidental: a plain provider reached through the redirect.
#[cgp_impl(new TestProvider)]
impl FooProvider {
    fn foo() {}
}

pub struct App;

snapshot_delegate_components! {
    delegate_components! {
        App {
            namespace DefaultNamespace;

            // @bar: TestProvider,

            @bar.baz: TestProvider,

            // @bar.baz.FooProviderComponent: TestProvider,
        }
    }

    expand_redirect_app(output) {
        insta::assert_snapshot!(output, @"
        impl<__Key__, __Value__> DelegateComponent<__Key__> for App
        where
            __Key__: DefaultNamespace<App, Delegate = __Value__>,
        {
            type Delegate = __Value__;
        }
        impl<
            __Key__,
            __Value__,
            __Context__,
            __Params__,
        > IsProviderFor<__Key__, __Context__, __Params__> for App
        where
            __Key__: DefaultNamespace<App, Delegate = __Value__>,
            __Value__: IsProviderFor<__Key__, __Context__, __Params__>,
        {}
        impl<
            __Wildcard__,
        > DelegateComponent<
            PathCons<
                Symbol<3, Chars<'b', Chars<'a', Chars<'r', Nil>>>>,
                PathCons<Symbol<3, Chars<'b', Chars<'a', Chars<'z', Nil>>>>, __Wildcard__>,
            >,
        > for App {
            type Delegate = TestProvider;
        }
        impl<
            __Wildcard__,
            __Context__,
            __Params__,
        > IsProviderFor<
            PathCons<
                Symbol<3, Chars<'b', Chars<'a', Chars<'r', Nil>>>>,
                PathCons<Symbol<3, Chars<'b', Chars<'a', Chars<'z', Nil>>>>, __Wildcard__>,
            >,
            __Context__,
            __Params__,
        > for App
        where
            TestProvider: IsProviderFor<
                PathCons<
                    Symbol<3, Chars<'b', Chars<'a', Chars<'r', Nil>>>>,
                    PathCons<Symbol<3, Chars<'b', Chars<'a', Chars<'z', Nil>>>>, __Wildcard__>,
                >,
                __Context__,
                __Params__,
            >,
        {}
        ")
    }
}

check_components! {
    App {
        FooProviderComponent,
    }
}
