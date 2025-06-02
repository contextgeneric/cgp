use core::marker::PhantomData;

use cgp::prelude::*;

#[cgp_getter {
    provider: FooGetterAt,
    use_delegate: I,
}]
pub trait HasFooAt<I> {
    fn foo(&self, _tag: PhantomData<I>) -> &str;
}
