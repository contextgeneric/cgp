use cgp::prelude::*;

#[cgp_component(FooProvider)]
pub trait CanDoFoo {
    fn foo(&self, value: u32) -> String;
}

pub struct ValueToString;

#[cgp_impl(ValueToString: FooProvider)]
impl<Context> CanDoFoo for Context {
    fn foo(&self, value: u32) -> String {
        value.to_string()
    }
}
