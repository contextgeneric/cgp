use cgp::prelude::*;

#[cgp_type]
pub trait HasScalarType {
    type Scalar;
}

#[cgp_type]
pub trait HasTypes {
    type Types: HasScalarType;
}

#[cgp_fn]
#[use_type(
    HasTypes::Types,
    @Types::HasScalarType::{Scalar = f64},
)]
pub fn rectangle_area(&self, #[implicit] width: Scalar, #[implicit] height: Scalar) -> Scalar {
    let res: f64 = width * height;
    res
}

pub trait HasFooType {
    type Foo: Ord + Clone;
}

pub trait HasBarType {
    type Bar: HasFooType;
}

#[cgp_fn]
#[use_type(HasFooType::Foo)]
pub fn do_foo(&self) -> Foo {
    todo!()
}

#[cgp_fn]
#[use_type(HasBarType::Bar, @Bar::HasFooType::Foo)]
pub fn do_bar(&self) -> Foo {
    todo!()
}

#[cgp_fn]
#[use_type(
    HasFooType::Foo,
    HasBarType::Bar,
    @Bar::HasFooType::{Foo as BarFoo = Foo},
)]
#[uses(DoFoo, DoBar)]
fn return_foo_or_bar(&self, flag: bool, #[implicit] foo: &Foo, #[implicit] bar: &BarFoo) -> Foo {
    if flag {
        let res: Foo = self.do_foo();
        if &res < foo { res } else { foo.clone() }
    } else {
        let res: BarFoo = self.do_bar();
        if &res < bar { res } else { bar.clone() }
    }
}
