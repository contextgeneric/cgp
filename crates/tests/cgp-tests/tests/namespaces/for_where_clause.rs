//! The optional `where` clause on a `for <..> in ..` loop is merged into every
//! impl the loop generates. Before the fix the clause was parsed and silently
//! dropped; the snapshot pins the loop's `where T: Clone` predicate landing on
//! each emitted impl, alongside the namespace bound the loop reconstructs.
//!
//! `Show`/`WhereShowComponents` are incidental scaffolding (written plainly, not
//! snapshotted — the owning targets pin their expansions); the
//! `delegate_components!` `for`-loop expansion is what this file captures.
//!
//! See cgp-knowledge-base/cgp/implementation/entrypoints/cgp_namespace.md (Tests) and
//! cgp-knowledge-base/cgp/reference/macros/cgp_namespace.md.

use cgp::prelude::*;
use cgp_macro_test_util::snapshot_delegate_components;

// Incidental: a generic component so `@test.ShowImplComponent.T` names a real key.
#[cgp_component(ShowImpl)]
pub trait Show<T> {
    fn show(&self, value: &T) -> String;
}

// Incidental: a namespace table for the loop to iterate.
cgp_namespace! {
    new WhereShowComponents {
        [String, u64]: UseContext,
    }
}

pub struct WhereApp;

snapshot_delegate_components! {
    delegate_components! {
        WhereApp {
            for <T, Provider> in WhereShowComponents where T: Clone {
                @test.ShowImplComponent.T: Provider,
            }
        }
    }

    expand_for_where(output) {
        insta::assert_snapshot!(output, @"
        impl<
            __Wildcard__,
            T,
            Provider,
        > DelegateComponent<
            PathCons<
                Symbol<4, Chars<'t', Chars<'e', Chars<'s', Chars<'t', Nil>>>>>,
                PathCons<ShowImplComponent, PathCons<T, __Wildcard__>>,
            >,
        > for WhereApp
        where
            T: Clone,
            T: WhereShowComponents<WhereApp, Delegate = Provider>,
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
        > for WhereApp
        where
            T: Clone,
            T: WhereShowComponents<WhereApp, Delegate = Provider>,
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
