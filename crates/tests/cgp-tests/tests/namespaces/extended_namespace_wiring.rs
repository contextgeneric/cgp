//! Wiring a context through the path-rewriting `ExtendedNamespace`.
//!
//! `App` opens the inheriting `ExtendedNamespace` (defined in the sibling
//! `extended` module), which rewrites the `cgp.core.error` prefix onto `app`, so
//! the error components can be wired under the shorter `@app.*` path while a
//! component outside that prefix (`@cgp.extra.handler.TryComputerComponent`) is
//! still reachable by its full path. Exercises the namespace/`@`-path form of
//! `delegate_components!` (kept as a snapshot) and confirms the wiring compiles
//! via the `CheckApp` supertrait bundle.
//!
//! See docs/reference/macros/cgp_namespace.md and
//! docs/reference/macros/delegate_components.md.

use cgp::core::error::{ErrorRaiserComponent, ErrorTypeProviderComponent, ErrorWrapperComponent};
use cgp::extra::error::RaiseFrom;
use cgp::extra::handler::CanTryCompute;
use cgp::prelude::*;
use cgp_macro_test_util::snapshot_delegate_components;

// The path-rewriting namespace lives in the sibling `extended` module (moved here
// from `cgp_tests::namespaces::extended`).
use crate::namespaces::extended::ExtendedNamespace;

pub struct App;

snapshot_delegate_components! {
    delegate_components! {
        App {
            namespace ExtendedNamespace;

            @app.ErrorTypeProviderComponent:
                UseType<String>,
            @app.{
                ErrorRaiserComponent.{&'static str, String},
                ErrorWrapperComponent,
            }:
                RaiseFrom,
            @cgp.extra.handler.TryComputerComponent:
                Foo,
        }
    }

    expand_extended_ns_app(output) {
        insta::assert_snapshot!(output, @"
        impl<__Key__, __Value__> DelegateComponent<__Key__> for App
        where
            __Key__: ExtendedNamespace<App, Delegate = __Value__>,
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
            __Key__: ExtendedNamespace<App, Delegate = __Value__>,
            __Value__: IsProviderFor<__Key__, __Context__, __Params__>,
        {}
        impl<
            __Wildcard__,
        > DelegateComponent<
            PathCons<
                Symbol<3, Chars<'a', Chars<'p', Chars<'p', Nil>>>>,
                PathCons<ErrorTypeProviderComponent, __Wildcard__>,
            >,
        > for App {
            type Delegate = UseType<String>;
        }
        impl<
            __Wildcard__,
            __Context__,
            __Params__,
        > IsProviderFor<
            PathCons<
                Symbol<3, Chars<'a', Chars<'p', Chars<'p', Nil>>>>,
                PathCons<ErrorTypeProviderComponent, __Wildcard__>,
            >,
            __Context__,
            __Params__,
        > for App
        where
            UseType<
                String,
            >: IsProviderFor<
                PathCons<
                    Symbol<3, Chars<'a', Chars<'p', Chars<'p', Nil>>>>,
                    PathCons<ErrorTypeProviderComponent, __Wildcard__>,
                >,
                __Context__,
                __Params__,
            >,
        {}
        impl<
            __Wildcard__,
        > DelegateComponent<
            PathCons<
                Symbol<3, Chars<'a', Chars<'p', Chars<'p', Nil>>>>,
                PathCons<ErrorRaiserComponent, PathCons<&'static str, __Wildcard__>>,
            >,
        > for App {
            type Delegate = RaiseFrom;
        }
        impl<
            __Wildcard__,
            __Context__,
            __Params__,
        > IsProviderFor<
            PathCons<
                Symbol<3, Chars<'a', Chars<'p', Chars<'p', Nil>>>>,
                PathCons<ErrorRaiserComponent, PathCons<&'static str, __Wildcard__>>,
            >,
            __Context__,
            __Params__,
        > for App
        where
            RaiseFrom: IsProviderFor<
                PathCons<
                    Symbol<3, Chars<'a', Chars<'p', Chars<'p', Nil>>>>,
                    PathCons<ErrorRaiserComponent, PathCons<&'static str, __Wildcard__>>,
                >,
                __Context__,
                __Params__,
            >,
        {}
        impl<
            __Wildcard__,
        > DelegateComponent<
            PathCons<
                Symbol<3, Chars<'a', Chars<'p', Chars<'p', Nil>>>>,
                PathCons<ErrorRaiserComponent, PathCons<String, __Wildcard__>>,
            >,
        > for App {
            type Delegate = RaiseFrom;
        }
        impl<
            __Wildcard__,
            __Context__,
            __Params__,
        > IsProviderFor<
            PathCons<
                Symbol<3, Chars<'a', Chars<'p', Chars<'p', Nil>>>>,
                PathCons<ErrorRaiserComponent, PathCons<String, __Wildcard__>>,
            >,
            __Context__,
            __Params__,
        > for App
        where
            RaiseFrom: IsProviderFor<
                PathCons<
                    Symbol<3, Chars<'a', Chars<'p', Chars<'p', Nil>>>>,
                    PathCons<ErrorRaiserComponent, PathCons<String, __Wildcard__>>,
                >,
                __Context__,
                __Params__,
            >,
        {}
        impl<
            __Wildcard__,
        > DelegateComponent<
            PathCons<
                Symbol<3, Chars<'a', Chars<'p', Chars<'p', Nil>>>>,
                PathCons<ErrorWrapperComponent, __Wildcard__>,
            >,
        > for App {
            type Delegate = RaiseFrom;
        }
        impl<
            __Wildcard__,
            __Context__,
            __Params__,
        > IsProviderFor<
            PathCons<
                Symbol<3, Chars<'a', Chars<'p', Chars<'p', Nil>>>>,
                PathCons<ErrorWrapperComponent, __Wildcard__>,
            >,
            __Context__,
            __Params__,
        > for App
        where
            RaiseFrom: IsProviderFor<
                PathCons<
                    Symbol<3, Chars<'a', Chars<'p', Chars<'p', Nil>>>>,
                    PathCons<ErrorWrapperComponent, __Wildcard__>,
                >,
                __Context__,
                __Params__,
            >,
        {}
        impl<
            __Wildcard__,
        > DelegateComponent<
            PathCons<
                Symbol<3, Chars<'c', Chars<'g', Chars<'p', Nil>>>>,
                PathCons<
                    Symbol<5, Chars<'e', Chars<'x', Chars<'t', Chars<'r', Chars<'a', Nil>>>>>>,
                    PathCons<
                        Symbol<
                            7,
                            Chars<
                                'h',
                                Chars<
                                    'a',
                                    Chars<
                                        'n',
                                        Chars<'d', Chars<'l', Chars<'e', Chars<'r', Nil>>>>,
                                    >,
                                >,
                            >,
                        >,
                        PathCons<TryComputerComponent, __Wildcard__>,
                    >,
                >,
            >,
        > for App {
            type Delegate = Foo;
        }
        impl<
            __Wildcard__,
            __Context__,
            __Params__,
        > IsProviderFor<
            PathCons<
                Symbol<3, Chars<'c', Chars<'g', Chars<'p', Nil>>>>,
                PathCons<
                    Symbol<5, Chars<'e', Chars<'x', Chars<'t', Chars<'r', Chars<'a', Nil>>>>>>,
                    PathCons<
                        Symbol<
                            7,
                            Chars<
                                'h',
                                Chars<
                                    'a',
                                    Chars<
                                        'n',
                                        Chars<'d', Chars<'l', Chars<'e', Chars<'r', Nil>>>>,
                                    >,
                                >,
                            >,
                        >,
                        PathCons<TryComputerComponent, __Wildcard__>,
                    >,
                >,
            >,
            __Context__,
            __Params__,
        > for App
        where
            Foo: IsProviderFor<
                PathCons<
                    Symbol<3, Chars<'c', Chars<'g', Chars<'p', Nil>>>>,
                    PathCons<
                        Symbol<
                            5,
                            Chars<'e', Chars<'x', Chars<'t', Chars<'r', Chars<'a', Nil>>>>>,
                        >,
                        PathCons<
                            Symbol<
                                7,
                                Chars<
                                    'h',
                                    Chars<
                                        'a',
                                        Chars<
                                            'n',
                                            Chars<'d', Chars<'l', Chars<'e', Chars<'r', Nil>>>>,
                                        >,
                                    >,
                                >,
                            >,
                            PathCons<TryComputerComponent, __Wildcard__>,
                        >,
                    >,
                >,
                __Context__,
                __Params__,
            >,
        {}
        ")
    }
}

#[cgp_computer]
fn foo(x: u64) -> Result<u64, String> {
    Ok(x * 2)
}

pub trait CheckApp: HasErrorType + CanRaiseError<&'static str> + CanTryCompute<(), u64> {}

impl CheckApp for App {}
