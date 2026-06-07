use cgp::prelude::*;

#[cgp_type(ProvideFooType)]
pub trait HasFooType<T: ?Sized> {
    type Foo;
}

#[cgp_getter]
pub trait HasFoo<T: ?Sized>: HasFooType<T> {
    fn foo(&self) -> &Self::Foo;
}
