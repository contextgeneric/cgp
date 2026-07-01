//! The `new` keyword in `delegate_components!` declares the provider-table struct
//! as part of the wiring.
//!
//! `delegate_components! { new MyComponents { … } }` defines `struct MyComponents`
//! and its `DelegateComponent` impls together. This is a compile-time check: the
//! `CheckDelegates` bound proves the table resolves as written.
//!
//! See docs/reference/macros/delegate_components.md.

use cgp::prelude::*;

pub struct FooKey;
pub struct FooValue;
pub struct BarKey;
pub struct BarValue;

delegate_components! {
    new MyComponents {
        FooKey: FooValue,
        BarKey: BarValue,
    }
}

pub trait CheckDelegates:
    DelegateComponent<FooKey, Delegate = FooValue> + DelegateComponent<BarKey, Delegate = BarValue>
{
}

impl CheckDelegates for MyComponents {}
