//! The `=>` redirect operator written directly in a `delegate_components!` table.
//!
//! `=>` sets an entry's `Delegate` to a `RedirectLookup` along an `@`-path instead
//! of naming a provider, so the provider is decided wherever the path lands. Two
//! uses appear here. `FooProviderComponent => @FooProviderComponent` roots a
//! component's route at its own name, which is **exactly** what `open
//! FooProviderComponent;` generates — the snapshots below are the equivalence,
//! since `App` and `OpenApp` differ only in which spelling they use and their
//! golden output is identical. And `[BarProviderComponent, BazProviderComponent]
//! => @shared` points two components at one slot, answered by a single
//! `@shared: DummyImpl` entry.
//!
//! This is the only place the suite exercises `=>` outside a `cgp_namespace!`
//! body. The two `delegate_components!` snapshots are the golden output this file
//! owns; the components and providers are incidental scaffolding.
//!
//! See cgp-knowledge-base/cgp/implementation/entrypoints/delegate_components.md and
//! cgp-knowledge-base/cgp/reference/macros/delegate_components.md.

use cgp::prelude::*;
use cgp_macro_test_util::snapshot_delegate_components;

// Incidental: one generic component dispatched per type, and two plain ones
// sharing a redirect slot.
#[cgp_component(FooProvider)]
pub trait Foo<T> {
    fn foo(&self, value: &T);
}

#[cgp_component(BarProvider)]
pub trait Bar {
    fn bar(&self);
}

#[cgp_component(BazProvider)]
pub trait Baz {
    fn baz(&self);
}

// Incidental: plain providers for the three components.
#[cgp_impl(new DummyFoo)]
impl<T> FooProvider<T> {
    fn foo(&self, _value: &T) {}
}

#[cgp_impl(new DummyImpl)]
impl BarProvider {
    fn bar(&self) {}
}

#[cgp_impl(DummyImpl)]
impl BazProvider {
    fn baz(&self) {}
}

pub struct App;

snapshot_delegate_components! {
    delegate_components! {
        App {
            // The spelled-out form of `open FooProviderComponent;`.
            FooProviderComponent =>
                @FooProviderComponent,

            @FooProviderComponent.String:
                DummyFoo,

            // A list key redirecting two components onto one shared slot, which
            // the single entry below answers.
            [BarProviderComponent, BazProviderComponent] =>
                @shared,

            @shared:
                DummyImpl,
        }
    }

    expand_redirect_mapping_app(output) {
        insta::assert_snapshot!(output, @"
        impl DelegateComponent<FooProviderComponent> for App {
            type Delegate = RedirectLookup<App, PathCons<FooProviderComponent, Nil>>;
        }
        impl<
            __Context__,
            __Params__,
        > IsProviderFor<FooProviderComponent, __Context__, __Params__> for App
        where
            RedirectLookup<
                App,
                PathCons<FooProviderComponent, Nil>,
            >: IsProviderFor<FooProviderComponent, __Context__, __Params__>,
        {}
        impl<
            __Wildcard__,
        > DelegateComponent<PathCons<FooProviderComponent, PathCons<String, __Wildcard__>>>
        for App {
            type Delegate = DummyFoo;
        }
        impl<
            __Wildcard__,
            __Context__,
            __Params__,
        > IsProviderFor<
            PathCons<FooProviderComponent, PathCons<String, __Wildcard__>>,
            __Context__,
            __Params__,
        > for App
        where
            DummyFoo: IsProviderFor<
                PathCons<FooProviderComponent, PathCons<String, __Wildcard__>>,
                __Context__,
                __Params__,
            >,
        {}
        impl DelegateComponent<BarProviderComponent> for App {
            type Delegate = RedirectLookup<
                App,
                PathCons<
                    Symbol<
                        6,
                        Chars<
                            's',
                            Chars<'h', Chars<'a', Chars<'r', Chars<'e', Chars<'d', Nil>>>>>,
                        >,
                    >,
                    Nil,
                >,
            >;
        }
        impl<
            __Context__,
            __Params__,
        > IsProviderFor<BarProviderComponent, __Context__, __Params__> for App
        where
            RedirectLookup<
                App,
                PathCons<
                    Symbol<
                        6,
                        Chars<
                            's',
                            Chars<'h', Chars<'a', Chars<'r', Chars<'e', Chars<'d', Nil>>>>>,
                        >,
                    >,
                    Nil,
                >,
            >: IsProviderFor<BarProviderComponent, __Context__, __Params__>,
        {}
        impl DelegateComponent<BazProviderComponent> for App {
            type Delegate = RedirectLookup<
                App,
                PathCons<
                    Symbol<
                        6,
                        Chars<
                            's',
                            Chars<'h', Chars<'a', Chars<'r', Chars<'e', Chars<'d', Nil>>>>>,
                        >,
                    >,
                    Nil,
                >,
            >;
        }
        impl<
            __Context__,
            __Params__,
        > IsProviderFor<BazProviderComponent, __Context__, __Params__> for App
        where
            RedirectLookup<
                App,
                PathCons<
                    Symbol<
                        6,
                        Chars<
                            's',
                            Chars<'h', Chars<'a', Chars<'r', Chars<'e', Chars<'d', Nil>>>>>,
                        >,
                    >,
                    Nil,
                >,
            >: IsProviderFor<BazProviderComponent, __Context__, __Params__>,
        {}
        impl<
            __Wildcard__,
        > DelegateComponent<
            PathCons<
                Symbol<
                    6,
                    Chars<'s', Chars<'h', Chars<'a', Chars<'r', Chars<'e', Chars<'d', Nil>>>>>>,
                >,
                __Wildcard__,
            >,
        > for App {
            type Delegate = DummyImpl;
        }
        impl<
            __Wildcard__,
            __Context__,
            __Params__,
        > IsProviderFor<
            PathCons<
                Symbol<
                    6,
                    Chars<'s', Chars<'h', Chars<'a', Chars<'r', Chars<'e', Chars<'d', Nil>>>>>>,
                >,
                __Wildcard__,
            >,
            __Context__,
            __Params__,
        > for App
        where
            DummyImpl: IsProviderFor<
                PathCons<
                    Symbol<
                        6,
                        Chars<
                            's',
                            Chars<'h', Chars<'a', Chars<'r', Chars<'e', Chars<'d', Nil>>>>>,
                        >,
                    >,
                    __Wildcard__,
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
        FooProviderComponent: String,
        BarProviderComponent,
        BazProviderComponent,
    }
}

// The `open` spelling of the first entry above. Its `DelegateComponent` and
// `IsProviderFor` impls must match `App`'s `FooProviderComponent` pair exactly.
pub struct OpenApp;

snapshot_delegate_components! {
    delegate_components! {
        OpenApp {
            open FooProviderComponent;

            @FooProviderComponent.String:
                DummyFoo,
        }
    }

    expand_redirect_mapping_open_app(output) {
        insta::assert_snapshot!(output, @"
        impl DelegateComponent<FooProviderComponent> for OpenApp {
            type Delegate = RedirectLookup<OpenApp, PathCons<FooProviderComponent, Nil>>;
        }
        impl<
            __Context__,
            __Params__,
        > IsProviderFor<FooProviderComponent, __Context__, __Params__> for OpenApp
        where
            RedirectLookup<
                OpenApp,
                PathCons<FooProviderComponent, Nil>,
            >: IsProviderFor<FooProviderComponent, __Context__, __Params__>,
        {}
        impl<
            __Wildcard__,
        > DelegateComponent<PathCons<FooProviderComponent, PathCons<String, __Wildcard__>>>
        for OpenApp {
            type Delegate = DummyFoo;
        }
        impl<
            __Wildcard__,
            __Context__,
            __Params__,
        > IsProviderFor<
            PathCons<FooProviderComponent, PathCons<String, __Wildcard__>>,
            __Context__,
            __Params__,
        > for OpenApp
        where
            DummyFoo: IsProviderFor<
                PathCons<FooProviderComponent, PathCons<String, __Wildcard__>>,
                __Context__,
                __Params__,
            >,
        {}
        ")
    }
}

check_components! {
    #[check_trait(__CheckOpenApp)]
    OpenApp {
        FooProviderComponent: String,
    }
}
