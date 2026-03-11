use cgp::prelude::*;

pub struct MyComponents;

#[cgp_component(FooProvider)]
#[use_namespace(DefaultNamespace: MyComponents)]
pub trait CanDoFoo {
    fn foo(&self);
}
