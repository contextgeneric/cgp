//! A table entry value may itself be a nested `UseDelegate<new … { … }>` table.
//!
//! This builds a two-level dispatch table inline: `BarKey` delegates to
//! `UseDelegate<BarValue>`, and `BarValue` is its own table keyed by `BazKey`.
//!
//! See docs/reference/macros/delegate_components.md and
//! docs/reference/providers/use_delegate.md.

use cgp::core::component::UseDelegate;
use cgp::prelude::*;

pub struct FooKey;
pub struct FooValue;
pub struct BarKey;
pub struct BazKey;
pub struct BazValue;

delegate_components! {
    new MyComponents {
        FooKey: FooValue,
        BarKey: UseDelegate<new BarValue {
            BazKey: BazValue,
        }>,
    }
}

pub trait CheckDelegates:
    DelegateComponent<FooKey, Delegate = FooValue>
    + DelegateComponent<BarKey, Delegate = UseDelegate<BarValue>>
{
}

impl CheckDelegates for MyComponents {}

pub trait CheckInnerDelegates: DelegateComponent<BazKey, Delegate = BazValue> {}

impl CheckInnerDelegates for BarValue {}
