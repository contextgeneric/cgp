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

pub trait CheckBarDelegates:
    DelegateComponent<Index<0>, Delegate = FooComponents>
    + DelegateComponent<Index<1>, Delegate = String>
{
}

impl CheckBarDelegates for BarComponents {}
