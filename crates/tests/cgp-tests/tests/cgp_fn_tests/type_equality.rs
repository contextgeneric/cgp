use std::fmt::Display;

use cgp::prelude::*;

#[cgp_type]
pub trait HasScalarType {
    type Scalar;
}

#[cgp_fn]
#[use_type(HasScalarType::{Scalar = f64})]
pub fn rectangle_area(&self, #[implicit] width: Scalar, #[implicit] height: Scalar) -> Scalar {
    let res: f64 = width * height;
    res
}

pub trait HasFooType {
    // The `Ord + Clone` bounds are visible to both `Foo` and `Bar` because of `Bar = Foo` below
    type Foo: Ord + Clone;
}

pub trait HasBarType {
    // The `Display` bounds are hidden because of `Bar = Foo` below
    type Bar: Display;
}

#[cgp_fn]
#[use_type(HasFooType::Foo)]
pub fn do_foo(&self) -> Foo {
    todo!()
}

#[cgp_fn]
#[use_type(HasBarType::Bar)]
pub fn do_bar(&self) -> Bar {
    todo!()
}

#[cgp_fn]
#[use_type(HasBarType::{Bar as Baz = Foo}, HasFooType::Foo)]
#[uses(DoFoo, DoBar)]
fn return_foo_or_bar(&self, flag: bool, #[implicit] foo: &Foo, #[implicit] bar: &Baz) -> Foo {
    if flag {
        let res: Foo = self.do_foo();
        if &res < foo { res } else { foo.clone() }
    } else {
        let res: Baz = self.do_bar();
        if &res < bar { res } else { bar.clone() }
    }
}
