use core::marker::PhantomData;

use cgp::prelude::*;

#[cgp_type {
    provider: FooTypeAt,
    derive_delegate: [UseDelegate<I>],
}]
pub trait HasFooTypeAt<I> {
    type Foo;
}

#[cgp_getter {
    provider: FooGetterAt,
    derive_delegate: UseDelegate<I>,
}]
pub trait HasFooAt<I> {
    fn foo(&self, _tag: PhantomData<I>) -> &str;
}
