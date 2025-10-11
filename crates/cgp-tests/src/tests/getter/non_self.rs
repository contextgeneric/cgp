use cgp::prelude::*;

#[cgp_type]
pub trait HasFooType {
    type Foo;
}

#[cgp_type]
pub trait HasBarType {
    type Bar;
}

#[cgp_getter]
pub trait HasFooBar: HasFooType + HasBarType {
    fn foo_bar(foo: &Self::Foo) -> &Self::Bar;
}
