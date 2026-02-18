use core::fmt::Display;

use cgp::prelude::*;

pub trait HasFooType<T> {
    type Foo;
}

pub trait HasBarType {
    type Bar;

    type Baz;
}

#[allow(unused)]
#[cgp_fn]
#[use_type(<HasFooType<X>>::{Foo as FooX}, <HasFooType<Y>>::{Foo as FooY}, HasBarType::{Bar, Baz})]
pub fn do_foo_bar<X, Y>(
    &self,
    x: X,
    #[implicit] foo_x: &FooX,
    #[implicit] foo_y: &FooY,
    #[implicit] bar: &Bar,
    y: Y,
) -> Option<Baz>
where
    FooX: Display,
{
    None
}
