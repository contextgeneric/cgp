//! `new` with a leading generic list declares a generic provider-table struct.
//!
//! `delegate_components! { <T> new MyComponents<T> { … } }` defines
//! `struct MyComponents<T>` and wires a family of tables at once.
//!
//! See docs/reference/macros/delegate_components.md.

use core::marker::PhantomData;

use cgp::prelude::*;

pub struct FooKey<T>(PhantomData<T>);
pub struct FooValue;
pub struct BarKey;
pub struct BarValue<T>(PhantomData<T>);

delegate_components! {
    <T>
    new MyComponents<T> {
        FooKey<T>: FooValue,
        BarKey: BarValue<T>,
    }
}

pub trait CheckDelegates<T>:
    DelegateComponent<FooKey<T>, Delegate = FooValue>
    + DelegateComponent<BarKey, Delegate = BarValue<T>>
{
}

impl<T> CheckDelegates<T> for MyComponents<T> {}
