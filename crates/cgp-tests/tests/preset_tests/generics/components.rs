use core::marker::PhantomData;

use cgp::prelude::*;

#[cgp_type]
pub trait HasFooType {
    type Foo;
}

#[cgp_type]
pub trait HasBarType {
    type Bar;
}

#[cgp_getter {
    name: FooGetterComponent<I>,
    provider: FooGetter,
}]
pub trait HasFooAt<I>: HasFooType {
    fn foo(&self, _tag: PhantomData<I>) -> &Self::Foo;
}

#[cgp_getter {
    provider: BarGetter,
}]
pub trait HasBar: HasBarType {
    fn bar(&self) -> &Self::Bar;
}
