//! Wiring a context through per-type default impls and the `for <..> in ..` loop.
//!
//! Each context opens `DefaultNamespace` (or the inheriting `ExtendedNamespace`)
//! and populates the `ShowImpl` component from a default-impl source: `AppA` and
//! `AppC` iterate `DefaultImpls1<ShowImplComponent>` (the per-type registry filled
//! by `#[default_impl]` in the `default_impls` module) and override `u64`; `AppB`
//! iterates the `DefaultShowComponents` namespace table. This exercises the
//! namespace/`for … in`/`@`-path forms of `delegate_components!` (kept as
//! snapshots) against the reusable namespaces defined in the sibling
//! `default_impls` module.
//!
//! See docs/reference/traits/default_namespace.md and
//! docs/reference/macros/delegate_components.md.

use cgp::core::component::DefaultImpls1;
use cgp::prelude::*;
use cgp_macro_test_util::snapshot_delegate_components;

// The reusable namespaces and providers live in the sibling `default_impls`
// module (moved here from `cgp_tests::namespaces::default_impls`).
use crate::namespaces::default_impls::{
    DefaultShowComponents, ExtendedNamespace, ShowImplComponent, ShowWithDisplay,
};

pub struct AppA;

snapshot_delegate_components! {
    delegate_components! {
        AppA {
            namespace DefaultNamespace;

            for <T, Provider> in DefaultImpls1<ShowImplComponent> {
                @test.ShowImplComponent.T: Provider,
            }

            @test.ShowImplComponent.u64:
                ShowWithDisplay,
        }
    }

    expand_default_impls_app_a(output) {
        insta::assert_snapshot!(output, @"
        impl<__Key__, __Value__> DelegateComponent<__Key__> for AppA
        where
            __Key__: DefaultNamespace<AppA, Delegate = __Value__>,
        {
            type Delegate = __Value__;
        }
        impl<
            __Key__,
            __Value__,
            __Context__,
            __Params__,
        > IsProviderFor<__Key__, __Context__, __Params__> for AppA
        where
            __Key__: DefaultNamespace<AppA, Delegate = __Value__>,
            __Value__: IsProviderFor<__Key__, __Context__, __Params__>,
        {}
        impl<
            __Wildcard__,
            T,
            Provider,
        > DelegateComponent<
            PathCons<
                Symbol<4, Chars<'t', Chars<'e', Chars<'s', Chars<'t', Nil>>>>>,
                PathCons<ShowImplComponent, PathCons<T, __Wildcard__>>,
            >,
        > for AppA
        where
            T: DefaultImpls1<ShowImplComponent, AppA, Delegate = Provider>,
        {
            type Delegate = Provider;
        }
        impl<
            __Wildcard__,
            T,
            Provider,
            __Context__,
            __Params__,
        > IsProviderFor<
            PathCons<
                Symbol<4, Chars<'t', Chars<'e', Chars<'s', Chars<'t', Nil>>>>>,
                PathCons<ShowImplComponent, PathCons<T, __Wildcard__>>,
            >,
            __Context__,
            __Params__,
        > for AppA
        where
            T: DefaultImpls1<ShowImplComponent, AppA, Delegate = Provider>,
            Provider: IsProviderFor<
                PathCons<
                    Symbol<4, Chars<'t', Chars<'e', Chars<'s', Chars<'t', Nil>>>>>,
                    PathCons<ShowImplComponent, PathCons<T, __Wildcard__>>,
                >,
                __Context__,
                __Params__,
            >,
        {}
        impl<
            __Wildcard__,
        > DelegateComponent<
            PathCons<
                Symbol<4, Chars<'t', Chars<'e', Chars<'s', Chars<'t', Nil>>>>>,
                PathCons<ShowImplComponent, PathCons<u64, __Wildcard__>>,
            >,
        > for AppA {
            type Delegate = ShowWithDisplay;
        }
        impl<
            __Wildcard__,
            __Context__,
            __Params__,
        > IsProviderFor<
            PathCons<
                Symbol<4, Chars<'t', Chars<'e', Chars<'s', Chars<'t', Nil>>>>>,
                PathCons<ShowImplComponent, PathCons<u64, __Wildcard__>>,
            >,
            __Context__,
            __Params__,
        > for AppA
        where
            ShowWithDisplay: IsProviderFor<
                PathCons<
                    Symbol<4, Chars<'t', Chars<'e', Chars<'s', Chars<'t', Nil>>>>>,
                    PathCons<ShowImplComponent, PathCons<u64, __Wildcard__>>,
                >,
                __Context__,
                __Params__,
            >,
        {}
        ")
    }
}

check_components! {
    AppA {
        ShowImplComponent: [
            String,
            u64,
        ]
    }
}

pub struct AppB;

snapshot_delegate_components! {
    delegate_components! {
        AppB {
            namespace DefaultNamespace;

            for <T, Provider> in DefaultShowComponents {
                @test.ShowImplComponent.T: Provider,
            }
        }
    }

    expand_default_impls_app_b(output) {
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
            T,
            Provider,
        > DelegateComponent<
            PathCons<
                Symbol<4, Chars<'t', Chars<'e', Chars<'s', Chars<'t', Nil>>>>>,
                PathCons<ShowImplComponent, PathCons<T, __Wildcard__>>,
            >,
        > for AppB
        where
            T: DefaultShowComponents<AppB, Delegate = Provider>,
        {
            type Delegate = Provider;
        }
        impl<
            __Wildcard__,
            T,
            Provider,
            __Context__,
            __Params__,
        > IsProviderFor<
            PathCons<
                Symbol<4, Chars<'t', Chars<'e', Chars<'s', Chars<'t', Nil>>>>>,
                PathCons<ShowImplComponent, PathCons<T, __Wildcard__>>,
            >,
            __Context__,
            __Params__,
        > for AppB
        where
            T: DefaultShowComponents<AppB, Delegate = Provider>,
            Provider: IsProviderFor<
                PathCons<
                    Symbol<4, Chars<'t', Chars<'e', Chars<'s', Chars<'t', Nil>>>>>,
                    PathCons<ShowImplComponent, PathCons<T, __Wildcard__>>,
                >,
                __Context__,
                __Params__,
            >,
        {}
        ")
    }
}

check_components! {
    AppB {
        ShowImplComponent: [
            String,
            u64,
        ]
    }
}

pub struct AppC;

snapshot_delegate_components! {
    delegate_components! {
        AppC {
            namespace ExtendedNamespace;

            for <T, Provider> in DefaultImpls1<ShowImplComponent> {
                @test.ShowImplComponent.T: Provider,
            }

            @test.ShowImplComponent.u64:
                ShowWithDisplay,
        }
    }

    expand_default_impls_app_c(output) {
        insta::assert_snapshot!(output, @"
        impl<__Key__, __Value__> DelegateComponent<__Key__> for AppC
        where
            __Key__: ExtendedNamespace<AppC, Delegate = __Value__>,
        {
            type Delegate = __Value__;
        }
        impl<
            __Key__,
            __Value__,
            __Context__,
            __Params__,
        > IsProviderFor<__Key__, __Context__, __Params__> for AppC
        where
            __Key__: ExtendedNamespace<AppC, Delegate = __Value__>,
            __Value__: IsProviderFor<__Key__, __Context__, __Params__>,
        {}
        impl<
            __Wildcard__,
            T,
            Provider,
        > DelegateComponent<
            PathCons<
                Symbol<4, Chars<'t', Chars<'e', Chars<'s', Chars<'t', Nil>>>>>,
                PathCons<ShowImplComponent, PathCons<T, __Wildcard__>>,
            >,
        > for AppC
        where
            T: DefaultImpls1<ShowImplComponent, AppC, Delegate = Provider>,
        {
            type Delegate = Provider;
        }
        impl<
            __Wildcard__,
            T,
            Provider,
            __Context__,
            __Params__,
        > IsProviderFor<
            PathCons<
                Symbol<4, Chars<'t', Chars<'e', Chars<'s', Chars<'t', Nil>>>>>,
                PathCons<ShowImplComponent, PathCons<T, __Wildcard__>>,
            >,
            __Context__,
            __Params__,
        > for AppC
        where
            T: DefaultImpls1<ShowImplComponent, AppC, Delegate = Provider>,
            Provider: IsProviderFor<
                PathCons<
                    Symbol<4, Chars<'t', Chars<'e', Chars<'s', Chars<'t', Nil>>>>>,
                    PathCons<ShowImplComponent, PathCons<T, __Wildcard__>>,
                >,
                __Context__,
                __Params__,
            >,
        {}
        impl<
            __Wildcard__,
        > DelegateComponent<
            PathCons<
                Symbol<4, Chars<'t', Chars<'e', Chars<'s', Chars<'t', Nil>>>>>,
                PathCons<ShowImplComponent, PathCons<u64, __Wildcard__>>,
            >,
        > for AppC {
            type Delegate = ShowWithDisplay;
        }
        impl<
            __Wildcard__,
            __Context__,
            __Params__,
        > IsProviderFor<
            PathCons<
                Symbol<4, Chars<'t', Chars<'e', Chars<'s', Chars<'t', Nil>>>>>,
                PathCons<ShowImplComponent, PathCons<u64, __Wildcard__>>,
            >,
            __Context__,
            __Params__,
        > for AppC
        where
            ShowWithDisplay: IsProviderFor<
                PathCons<
                    Symbol<4, Chars<'t', Chars<'e', Chars<'s', Chars<'t', Nil>>>>>,
                    PathCons<ShowImplComponent, PathCons<u64, __Wildcard__>>,
                >,
                __Context__,
                __Params__,
            >,
        {}
        ")
    }
}

check_components! {
    AppC {
        ShowImplComponent: [
            String,
            u64,
            u32,
        ]
    }
}
