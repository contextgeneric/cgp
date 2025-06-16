use cgp::prelude::*;

delegate_components! {
    new FooComponents {
        Index<0>: u64,
        Index<1>: String,
    }
}

delegate_components! {
    new BarComponents {
        Index<0>: FooComponents,
        Index<1> -> FooComponents,
    }
}

pub trait CheckBarDelegates:
    DelegateComponent<Index<0>, Delegate = FooComponents>
    + DelegateComponent<Index<1>, Delegate = String>
{
}

impl CheckBarDelegates for BarComponents {}
