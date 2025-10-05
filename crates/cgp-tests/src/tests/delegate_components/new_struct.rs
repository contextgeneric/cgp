#![allow(unused)]

use core::marker::PhantomData;

use cgp::core::component::UseDelegate;
use cgp::prelude::*;

pub fn test_delegate_components_with_new_struct() {
    struct FooKey;
    struct FooValue;
    struct BarKey;
    struct BarValue;

    delegate_components! {
        new MyComponents {
            FooKey: FooValue,
            BarKey: BarValue,
        }
    }

    trait CheckDelegates:
        DelegateComponent<FooKey, Delegate = FooValue>
        + DelegateComponent<BarKey, Delegate = BarValue>
    {
    }

    impl CheckDelegates for MyComponents {}
}

pub fn test_delegate_components_with_new_generic_struct() {
    struct FooKey<T>(PhantomData<T>);
    struct FooValue;
    struct BarKey;
    struct BarValue<T>(PhantomData<T>);

    delegate_components! {
        <T>
        new MyComponents<T> {
            FooKey<T>: FooValue,
            BarKey: BarValue<T>,
        }
    }

    trait CheckDelegates<T>:
        DelegateComponent<FooKey<T>, Delegate = FooValue>
        + DelegateComponent<BarKey, Delegate = BarValue<T>>
    {
    }

    impl<T> CheckDelegates<T> for MyComponents<T> {}
}

pub fn test_delegate_components_with_new_value() {
    struct FooKey;
    struct FooValue;
    struct BarKey;
    struct BazKey;
    struct BazValue;

    delegate_components! {
        new MyComponents {
            FooKey: FooValue,
            BarKey: UseDelegate<new BarValue {
                BazKey: BazValue,
            }>,
        }
    }

    trait CheckDelegates:
        DelegateComponent<FooKey, Delegate = FooValue>
        + DelegateComponent<BarKey, Delegate = UseDelegate<BarValue>>
    {
    }

    impl CheckDelegates for MyComponents {}

    trait CheckInnerDelegates: DelegateComponent<BazKey, Delegate = BazValue> {}

    impl CheckInnerDelegates for BarValue {}
}

pub fn test_delegate_components_with_generic_new_value() {
    struct FooKey;
    struct FooValue;
    struct BarKey<T>(pub PhantomData<T>);
    struct BazKey;
    struct BazValue<T>(pub PhantomData<T>);

    delegate_components! {
        new MyComponents {
            FooKey: FooValue,
            <T> BarKey<T>: UseDelegate<new BarValue<T> {
                BazKey: BazValue<T>,
            }>,
        }
    }

    trait CheckDelegates<T>:
        DelegateComponent<FooKey, Delegate = FooValue>
        + DelegateComponent<BarKey<T>, Delegate = UseDelegate<BarValue<T>>>
    {
    }

    impl<T> CheckDelegates<T> for MyComponents {}

    trait CheckInnerDelegates<T>: DelegateComponent<BazKey, Delegate = BazValue<T>> {}

    impl<T> CheckInnerDelegates<T> for BarValue<T> {}
}

pub fn test_delegate_new_with_array_key() {
    pub struct FooKey;
    pub struct BarKey;
    pub struct BazKey;

    delegate_components! {
        new MyComponents {
            [
                FooKey,
                BarKey,
                BazKey,
            ]:
                UseDelegate<new InnerComponents {
                    u32: String,
                    u64: bool,
                }>,
        }
    }
}

pub mod test_delegate_new_value_in_preset {
    #[cgp::re_export_imports]
    mod preset {
        use cgp::core::component::UseDelegate;
        use cgp::prelude::{DelegateComponent, *};

        pub struct FooKey;
        pub struct FooValue;
        pub struct BarKey;
        pub struct BazKey;
        pub struct BazValue;

        cgp_preset! {
            PresetWithNewValue {
                FooKey: FooValue,
                BarKey: UseDelegate<new BarValue {
                    BazKey: BazValue,
                }>
            }
        }

        pub trait CheckDelegates:
            DelegateComponent<FooKey, Delegate = FooValue>
            + DelegateComponent<BarKey, Delegate = UseDelegate<BarValue>>
        {
        }

        impl CheckDelegates for PresetWithNewValue::Provider {}

        pub trait CheckInnerDelegates: DelegateComponent<BazKey, Delegate = BazValue> {}

        impl CheckInnerDelegates for BarValue {}
    }
}
