//! Canonical expansion of `delegate_components!` for a standalone provider table.
//!
//! `delegate_components!` builds the type-level wiring table: for each entry it
//! emits a `DelegateComponent` impl (mapping a component key to its provider) and
//! an `IsProviderFor` impl that forwards the provider's dependencies. This file
//! owns the reference snapshot, including the `->` forwarding form that delegates
//! to another table's entry.
//!
//! See docs/reference/macros/delegate_components.md.

use cgp::prelude::*;
use cgp_macro_test_util::snapshot_delegate_components;

snapshot_delegate_components! {
    delegate_components! {
        new FooComponents {
            Index<0>: u64,
            Index<1>: String,
        }
    }

    expand_foo_component(output) {
        insta::assert_snapshot!(output, @"
        pub struct FooComponents;
        impl DelegateComponent<Index<0>> for FooComponents {
            type Delegate = u64;
        }
        impl<__Context__, __Params__> IsProviderFor<Index<0>, __Context__, __Params__>
        for FooComponents
        where
            u64: IsProviderFor<Index<0>, __Context__, __Params__>,
        {}
        impl DelegateComponent<Index<1>> for FooComponents {
            type Delegate = String;
        }
        impl<__Context__, __Params__> IsProviderFor<Index<1>, __Context__, __Params__>
        for FooComponents
        where
            String: IsProviderFor<Index<1>, __Context__, __Params__>,
        {}
        ")
    }
}

// The `->` form forwards a key to another table's delegate instead of assigning
// a provider directly.
snapshot_delegate_components! {
    delegate_components! {
        new BarComponents {
            Index<0>:
                FooComponents,
            Index<1> ->
                FooComponents,
        }
    }

    expand_bar_component(output) {
        insta::assert_snapshot!(output, @"
        pub struct BarComponents;
        impl DelegateComponent<Index<0>> for BarComponents {
            type Delegate = FooComponents;
        }
        impl<__Context__, __Params__> IsProviderFor<Index<0>, __Context__, __Params__>
        for BarComponents
        where
            FooComponents: IsProviderFor<Index<0>, __Context__, __Params__>,
        {}
        impl DelegateComponent<Index<1>> for BarComponents
        where
            FooComponents: DelegateComponent<Index<1>>,
        {
            type Delegate = <FooComponents as DelegateComponent<Index<1>>>::Delegate;
        }
        impl<__Context__, __Params__> IsProviderFor<Index<1>, __Context__, __Params__>
        for BarComponents
        where
            FooComponents: DelegateComponent<Index<1>>,
            <FooComponents as DelegateComponent<
                Index<1>,
            >>::Delegate: IsProviderFor<Index<1>, __Context__, __Params__>,
        {}
        ")
    }
}

// Compile-time proof that the table resolves to the expected delegates.
pub trait CheckBarDelegates:
    DelegateComponent<Index<0>, Delegate = FooComponents>
    + DelegateComponent<Index<1>, Delegate = String>
{
}

impl CheckBarDelegates for BarComponents {}
