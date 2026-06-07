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
pub trait HasFoo: HasFooType {
    fn foo(&self) -> &Self::Foo;
}

#[cgp_getter]
pub trait HasBar: HasBarType {
    fn bar(&self) -> &Self::Bar;
}
