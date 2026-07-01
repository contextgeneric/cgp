//! Canonical expansion of `#[cgp_impl]`, the preferred way to write a provider.
//!
//! `#[cgp_impl(new Name)]` lets a provider be written in consumer-style syntax —
//! keeping `self` and the consumer method signatures — and rewrites it into the
//! provider-trait shape, declaring the provider struct and its `IsProviderFor`
//! impl. This file owns that reference snapshot; the component it targets is
//! written with a plain `#[cgp_component]` (already snapshotted in
//! `component_macro`), and a context wires the provider and calls it.
//!
//! See docs/reference/macros/cgp_impl.md.

use cgp::prelude::*;
use cgp_macro_test_util::snapshot_cgp_impl;

#[cgp_component(FooProvider)]
pub trait CanDoFoo {
    fn foo(&self, value: u32) -> String;
}

snapshot_cgp_impl! {
    #[cgp_impl(new ValueToString)]
    impl<Context> FooProvider for Context {
        fn foo(&self, value: u32) -> String {
            value.to_string()
        }
    }

    expand_value_to_string(output) {
        insta::assert_snapshot!(output, @"
        impl<Context> FooProvider<Context> for ValueToString {
            fn foo(__context__: &Context, value: u32) -> String {
                value.to_string()
            }
        }
        impl<Context> IsProviderFor<FooProviderComponent, Context, ()> for ValueToString {}
        pub struct ValueToString;
        ")
    }
}

pub struct App;

delegate_components! {
    App {
        FooProviderComponent: ValueToString,
    }
}

#[test]
fn test_provider_wiring() {
    // Once `App` delegates `FooProviderComponent` to `ValueToString`, it
    // implements the `CanDoFoo` consumer trait.
    assert_eq!(App.foo(42), "42");
}
