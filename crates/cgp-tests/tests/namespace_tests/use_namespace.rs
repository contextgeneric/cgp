use cgp::core::component::CoreComponents;
use cgp::prelude::*;

pub struct MyComponents;

#[cgp_component(FooProvider)]
#[use_namespace(DefaultNamespace: CoreComponents.MyComponents)]
pub trait CanDoFoo {
    fn foo(&self);
}
