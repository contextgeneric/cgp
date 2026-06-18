use cgp::prelude::*;
use cgp_macro_test_util::assert_delegate_components;
use insta::assert_snapshot;

assert_delegate_components! {
    expand_foo_component(output) {
        assert_snapshot!(output, @"
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

    new FooComponents {
        Index<0>: u64,
        Index<1>: String,
    }
}

delegate_components! {
    new BarComponents {
        Index<0>:
            FooComponents,
        Index<1> ->
            FooComponents,
    }
}

pub trait CheckBarDelegates:
    DelegateComponent<Index<0>, Delegate = FooComponents>
    + DelegateComponent<Index<1>, Delegate = String>
{
}

impl CheckBarDelegates for BarComponents {}
