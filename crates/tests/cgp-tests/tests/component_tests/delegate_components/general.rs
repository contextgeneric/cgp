#![allow(unused)]

mod test_basic_delegate_components {
    use cgp::prelude::DelegateComponent;
    use cgp_macro_test_util::snapshot_delegate_components;
    use insta::assert_snapshot;

    pub struct FooKey;
    pub struct FooValue;
    pub struct BarKey;
    pub struct BarValue;
    pub struct BazKey;

    pub struct Components;

    snapshot_delegate_components! {
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

        expand_components(output) {
            insta::assert_snapshot!(output, @"
            impl DelegateComponent<FooKey> for Components {
                type Delegate = FooValue;
            }
            impl<__Context__, __Params__> IsProviderFor<FooKey, __Context__, __Params__>
            for Components
            where
                FooValue: IsProviderFor<FooKey, __Context__, __Params__>,
            {}
            impl DelegateComponent<BarKey> for Components {
                type Delegate = BarValue;
            }
            impl<__Context__, __Params__> IsProviderFor<BarKey, __Context__, __Params__>
            for Components
            where
                BarValue: IsProviderFor<BarKey, __Context__, __Params__>,
            {}
            impl DelegateComponent<BazKey> for Components {
                type Delegate = BarValue;
            }
            impl<__Context__, __Params__> IsProviderFor<BazKey, __Context__, __Params__>
            for Components
            where
                BarValue: IsProviderFor<BazKey, __Context__, __Params__>,
            {}
            ")
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

mod test_generic_delegate_components {
    use core::marker::PhantomData;

    use cgp::prelude::DelegateComponent;
    use cgp_macro_test_util::snapshot_delegate_components;
    use insta::assert_snapshot;

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
}
