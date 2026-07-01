//! `delegate_components!` with a leading generic list, wiring a whole family of
//! keys at once.
//!
//! The `<'a, T1: Clone>` header lifts generics (and their bounds) onto every
//! generated impl, and an entry key may introduce its own extra generics
//! (`<T2> BazKey<T1, T2>`).
//!
//! See docs/reference/macros/delegate_components.md.

use core::marker::PhantomData;

use cgp::prelude::DelegateComponent;
use cgp_macro_test_util::snapshot_delegate_components;

pub struct FooKey<T>(pub PhantomData<T>);
pub struct FooValue;
pub struct BarKey<'a, T>(pub PhantomData<(&'a (), T)>);
pub struct BarValue<T>(pub PhantomData<T>);
pub struct BazKey<T1, T2>(pub PhantomData<(T1, T2)>);

pub struct Components;

snapshot_delegate_components! {
    delegate_components! {
        <'a, T1: Clone>
        Components {
            FooKey<T1>: FooValue,
            [
                BarKey<'a, T1>,
                <T2> BazKey<T1, T2>,
            ]:
                BarValue<T1>,
        }
    }
    expand_components(output) {
        insta::assert_snapshot!(output, @"
        impl<'a, T1: Clone> DelegateComponent<FooKey<T1>> for Components {
            type Delegate = FooValue;
        }
        impl<
            'a,
            T1: Clone,
            __Context__,
            __Params__,
        > IsProviderFor<FooKey<T1>, __Context__, __Params__> for Components
        where
            FooValue: IsProviderFor<FooKey<T1>, __Context__, __Params__>,
        {}
        impl<'a, T1: Clone> DelegateComponent<BarKey<'a, T1>> for Components {
            type Delegate = BarValue<T1>;
        }
        impl<
            'a,
            T1: Clone,
            __Context__,
            __Params__,
        > IsProviderFor<BarKey<'a, T1>, __Context__, __Params__> for Components
        where
            BarValue<T1>: IsProviderFor<BarKey<'a, T1>, __Context__, __Params__>,
        {}
        impl<'a, T1: Clone, T2> DelegateComponent<BazKey<T1, T2>> for Components {
            type Delegate = BarValue<T1>;
        }
        impl<
            'a,
            T1: Clone,
            T2,
            __Context__,
            __Params__,
        > IsProviderFor<BazKey<T1, T2>, __Context__, __Params__> for Components
        where
            BarValue<T1>: IsProviderFor<BazKey<T1, T2>, __Context__, __Params__>,
        {}
        ")
    }
}

pub trait CheckDelegates<'a, T1, T2>:
    DelegateComponent<FooKey<T1>, Delegate = FooValue>
    + DelegateComponent<BarKey<'a, T1>, Delegate = BarValue<T1>>
    + DelegateComponent<BazKey<T1, T2>, Delegate = BarValue<T1>>
{
}

impl<T1, T2> CheckDelegates<'_, T1, T2> for Components where T1: Clone {}
