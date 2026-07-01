//! A nested `UseDelegate` value may introduce its own generics via a per-entry
//! `<T>` list.
//!
//! `<T> BarKey<T>: UseDelegate<new BarValue<T> { … }>` threads the entry generic
//! `T` through both the outer key and the inner generated table struct.
//!
//! See docs/reference/macros/delegate_components.md.

use core::marker::PhantomData;

use cgp::core::component::UseDelegate;
use cgp::prelude::*;

pub struct FooKey;
pub struct FooValue;
pub struct BarKey<T>(pub PhantomData<T>);
pub struct BazKey;
pub struct BazValue<T>(pub PhantomData<T>);

delegate_components! {
    new MyComponents {
        FooKey: FooValue,
        <T> BarKey<T>: UseDelegate<new BarValue<T> {
            BazKey: BazValue<T>,
        }>,
    }
}

pub trait CheckDelegates<T>:
    DelegateComponent<FooKey, Delegate = FooValue>
    + DelegateComponent<BarKey<T>, Delegate = UseDelegate<BarValue<T>>>
{
}

impl<T> CheckDelegates<T> for MyComponents {}

pub trait CheckInnerDelegates<T>: DelegateComponent<BazKey, Delegate = BazValue<T>> {}

impl<T> CheckInnerDelegates<T> for BarValue<T> {}
