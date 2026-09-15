//! A nested `UseDelegate<new … { … }>` value inside a `for <..> in ..` loop body.
//!
//! A loop body holds ordinary `:` mappings, so its values can open a nested table
//! just as a top-level mapping's can — and the extraction that lifts such a table
//! out has to walk the body's statements, not only its mappings. A mapping-only
//! walk parses the table, names it in the entry's `Delegate`, and never emits it,
//! failing with an `E0425` on the dropped struct.
//!
//! What this file pins is that the table is emitted and carries its entry. It does
//! **not** wire the component through, because the combination is redundant by
//! construction: the loop already yields one provider per key, so a nested table
//! would dispatch the same parameter a second time. The form is worth keeping
//! correct rather than worth recommending.
//!
//! Note the inner table's own generic list. The loop binds a provider variable the
//! entry must mention, or the generated impl leaves it unconstrained (`E0207`); but
//! the loop's variables are not in scope inside the lifted table's impls, so
//! mentioning it only there fails on both counts (`E0207` and an `E0425` for the
//! `Provider` the inner impl cannot see). `new LoopInner<Provider>` is what puts it
//! in both places at once.
//!
//! See cgp-knowledge-base/cgp/implementation/entrypoints/delegate_components.md.

use cgp::prelude::*;

// Incidental: a generic component routed into `DefaultNamespace` under a prefix.
#[cgp_component(FooProvider)]
#[prefix(@test in DefaultNamespace)]
#[derive_delegate(UseDelegate<T>)]
pub trait Foo<T> {
    fn foo(&self, value: &T);
}

#[cgp_impl(new DummyFoo)]
impl<T> FooProvider<T> {
    fn foo(&self, _value: &T) {}
}

// Incidental: the table the loop reads its (key, provider) pairs out of.
cgp_namespace! {
    new FooSources {
        String: DummyFoo,
    }
}

pub struct App;

delegate_components! {
    App {
        namespace DefaultNamespace;

        for <T, Provider> in FooSources {
            @test.FooProviderComponent.T: UseDelegate<new LoopInner<Provider> {
                String: Provider,
            }>,
        }
    }
}

// The lifted table exists and holds the entry the loop body gave it. Naming
// `LoopInner` at all is the regression this file guards: before the extraction
// walked statements, the struct was never declared and this bound would not
// resolve.
pub trait CheckLoopInner<Provider>: DelegateComponent<String, Delegate = Provider> {}

impl<Provider> CheckLoopInner<Provider> for LoopInner<Provider> {}
