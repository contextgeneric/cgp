use cgp::prelude::*;

pub trait HasFooType {
    type Foo;
}

pub trait HasBarType {
    type Bar;
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
#[use_type(HasFooType::{Foo as Foo}, HasBarType::{Bar as Foo})]
#[uses(DoFoo, DoBar)]
fn return_foo_or_bar(&self, flag: bool) -> Foo {
    if flag { self.do_foo() } else { self.do_bar() }
}
