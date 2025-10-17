#![allow(unused)]

use core::marker::PhantomData;

use cgp::prelude::*;

#[test]
fn test_basic_delegate_components() {
    pub struct FooKey;
    pub struct FooValue;
    pub struct BarKey;
    pub struct BarValue;
    pub struct BazKey;

    pub struct Components;

    delegate_components! {
        Components {
            FooKey: FooValue,
            [
                BarKey,
                BazKey,
            ]:
                BarValue,
        }
    }

    pub trait CheckDelegates:
        DelegateComponent<FooKey, Delegate = FooValue>
        + DelegateComponent<BarKey, Delegate = BarValue>
        + DelegateComponent<BazKey, Delegate = BarValue>
    {
    }

    impl CheckDelegates for Components {}
}

#[test]
fn test_generic_delegate_components() {
    pub struct FooKey<T>(pub PhantomData<T>);
    pub struct FooValue;
    pub struct BarKey<'a, T>(pub PhantomData<(&'a (), T)>);
    pub struct BarValue<T>(pub PhantomData<T>);
    pub struct BazKey<T1, T2>(pub PhantomData<(T1, T2)>);

    pub struct Components;

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

    pub trait CheckDelegates<'a, T1, T2>:
        DelegateComponent<FooKey<T1>, Delegate = FooValue>
        + DelegateComponent<BarKey<'a, T1>, Delegate = BarValue<T1>>
        + DelegateComponent<BazKey<T1, T2>, Delegate = BarValue<T1>>
    {
    }

    impl<T1, T2> CheckDelegates<'_, T1, T2> for Components where T1: Clone {}
}
