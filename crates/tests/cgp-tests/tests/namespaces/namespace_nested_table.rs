//! A nested `UseDelegate<new … { … }>` value inside a `cgp_namespace!` body.
//!
//! A namespace body is parsed by the same `DelegateEntries` type a
//! `delegate_components!` table is, so it accepts the legacy nested-table value —
//! and `cgp_namespace!` must lift that inner table out into its own struct and
//! `DelegateComponent` impls exactly as `delegate_components!` does. Drop that and
//! the entry's `Delegate` names a `FooTable` nothing declares, failing with an
//! `E0425` that reads like a typo rather than a dropped table — the regression
//! this file guards.
//!
//! Putting the dispatch table in the namespace rather than on the context is the
//! point of the form: `AppA` and `AppB` join `NestedNs` and both get the per-type
//! dispatch without either one spelling it out. The `cgp_namespace!` snapshot is
//! the golden output this file owns — it pins the lifted `FooTable` struct and its
//! two entries beside the namespace's own impl — and the component and provider
//! are incidental scaffolding.
//!
//! See cgp-knowledge-base/cgp/implementation/entrypoints/cgp_namespace.md and
//! cgp-knowledge-base/cgp/reference/macros/cgp_namespace.md.

use cgp::prelude::*;
use cgp_macro_test_util::snapshot_cgp_namespace;

// Incidental: a generic component whose dispatch the nested table drives. The
// legacy nested-table form resolves through `#[derive_delegate]`, unlike `open`.
#[cgp_component(FooProvider)]
#[derive_delegate(UseDelegate<T>)]
pub trait Foo<T> {
    fn foo(&self, value: &T);
}

// Incidental: one per-value provider, shared by both dispatch entries.
#[cgp_impl(new DummyFoo)]
impl<T> FooProvider<T> {
    fn foo(&self, _value: &T) {}
}

snapshot_cgp_namespace! {
    cgp_namespace! {
        new NestedNs {
            FooProviderComponent:
                UseDelegate<new FooTable {
                    String: DummyFoo,
                    u64: DummyFoo,
                }>,
        }
    }

    expand_nested_table_namespace(output) {
        insta::assert_snapshot!(output, @"
        pub struct __NestedNsComponents;
        pub trait NestedNs<__Table__> {
            type Delegate;
        }
        pub struct FooTable;
        impl<__Table__> NestedNs<__Table__> for FooProviderComponent {
            type Delegate = UseDelegate<FooTable>;
        }
        impl DelegateComponent<String> for FooTable {
            type Delegate = DummyFoo;
        }
        impl<__Context__, __Params__> IsProviderFor<String, __Context__, __Params__> for FooTable
        where
            DummyFoo: IsProviderFor<String, __Context__, __Params__>,
        {}
        impl DelegateComponent<u64> for FooTable {
            type Delegate = DummyFoo;
        }
        impl<__Context__, __Params__> IsProviderFor<u64, __Context__, __Params__> for FooTable
        where
            DummyFoo: IsProviderFor<u64, __Context__, __Params__>,
        {}
        ")
    }
}

pub struct AppA;

delegate_components! {
    AppA {
        namespace NestedNs;
    }
}

check_components! {
    AppA {
        FooProviderComponent: [String, u64],
    }
}

// A second context joining the same namespace inherits the same dispatch table,
// which is what putting it in the namespace bought.
pub struct AppB;

delegate_components! {
    AppB {
        namespace NestedNs;
    }
}

check_components! {
    #[check_trait(__CheckAppB)]
    AppB {
        FooProviderComponent: [String, u64],
    }
}
