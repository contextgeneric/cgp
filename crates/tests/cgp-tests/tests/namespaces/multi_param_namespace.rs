//! Namespace `@`-path wiring for a component with a lifetime and many parameters.
//!
//! `Foo<'a, T, U>` attaches to `DefaultNamespace` via `#[prefix(@app in ..)]`, and
//! a context wires it through full paths that pin the leading path parameters,
//! including a generic key (`@app.FooProviderComponent.bool.<T> T: DummyFoo`). This
//! is the namespace counterpart to `multi_param_open`, dispatching the same
//! multi-parameter component through a joined namespace instead of `open`. The
//! namespace `delegate_components!` snapshot is the canonical golden output; the
//! component and provider are incidental scaffolding.
//!
//! See docs/reference/traits/default_namespace.md and
//! docs/reference/macros/delegate_components.md.

use cgp::prelude::*;
use cgp_macro_test_util::snapshot_delegate_components;

// Incidental component, attached to `DefaultNamespace` so it can be wired by path.
#[cgp_component(FooProvider)]
#[prefix(@app in DefaultNamespace)]
pub trait Foo<'a, T, U> {
    fn foo(&self, first: &'a T, second: U);
}

// Incidental: a plain per-value provider.
#[cgp_impl(new DummyFoo)]
impl<'a, T, U> FooProvider<'a, T, U> {
    fn foo(&self, _first: &'a T, _second: U) {}
}

pub struct AppB;

snapshot_delegate_components! {
    delegate_components! {
        AppB {
            namespace DefaultNamespace;

            @app.FooProviderComponent.String.u64:
                DummyFoo,
            @app.FooProviderComponent.bool.<T> T:
                DummyFoo,
        }
    }

    expand_multi_param_app_b(output) {
        insta::assert_snapshot!(output, @"
        impl<__Key__, __Value__> DelegateComponent<__Key__> for AppB
        where
            __Key__: DefaultNamespace<AppB, Delegate = __Value__>,
        {
            type Delegate = __Value__;
        }
        impl<
            __Key__,
            __Value__,
            __Context__,
            __Params__,
        > IsProviderFor<__Key__, __Context__, __Params__> for AppB
        where
            __Key__: DefaultNamespace<AppB, Delegate = __Value__>,
            __Value__: IsProviderFor<__Key__, __Context__, __Params__>,
        {}
        impl<
            __Wildcard__,
        > DelegateComponent<
            PathCons<
                Symbol<3, Chars<'a', Chars<'p', Chars<'p', Nil>>>>,
                PathCons<FooProviderComponent, PathCons<String, PathCons<u64, __Wildcard__>>>,
            >,
        > for AppB {
            type Delegate = DummyFoo;
        }
        impl<
            __Wildcard__,
            __Context__,
            __Params__,
        > IsProviderFor<
            PathCons<
                Symbol<3, Chars<'a', Chars<'p', Chars<'p', Nil>>>>,
                PathCons<FooProviderComponent, PathCons<String, PathCons<u64, __Wildcard__>>>,
            >,
            __Context__,
            __Params__,
        > for AppB
        where
            DummyFoo: IsProviderFor<
                PathCons<
                    Symbol<3, Chars<'a', Chars<'p', Chars<'p', Nil>>>>,
                    PathCons<FooProviderComponent, PathCons<String, PathCons<u64, __Wildcard__>>>,
                >,
                __Context__,
                __Params__,
            >,
        {}
        impl<
            T,
            __Wildcard__,
        > DelegateComponent<
            PathCons<
                Symbol<3, Chars<'a', Chars<'p', Chars<'p', Nil>>>>,
                PathCons<FooProviderComponent, PathCons<bool, PathCons<T, __Wildcard__>>>,
            >,
        > for AppB {
            type Delegate = DummyFoo;
        }
        impl<
            T,
            __Wildcard__,
            __Context__,
            __Params__,
        > IsProviderFor<
            PathCons<
                Symbol<3, Chars<'a', Chars<'p', Chars<'p', Nil>>>>,
                PathCons<FooProviderComponent, PathCons<bool, PathCons<T, __Wildcard__>>>,
            >,
            __Context__,
            __Params__,
        > for AppB
        where
            DummyFoo: IsProviderFor<
                PathCons<
                    Symbol<3, Chars<'a', Chars<'p', Chars<'p', Nil>>>>,
                    PathCons<FooProviderComponent, PathCons<bool, PathCons<T, __Wildcard__>>>,
                >,
                __Context__,
                __Params__,
            >,
        {}
        ")
    }
}

check_components! {
    <'a> AppB {
        FooProviderComponent: [
            (Life<'a>, String, u64),
            (Life<'a>, bool, String),
        ],
    }
}
