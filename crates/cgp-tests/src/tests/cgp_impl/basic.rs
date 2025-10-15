use core::fmt::Display;

use cgp::prelude::*;

#[cgp_component(FooProvider)]
pub trait CanDoFoo {
    fn foo(&self, value: u32) -> String;
}

#[cgp_auto_getter]
pub trait HasName {
    fn name(&self) -> &str;
}

#[cgp_impl(new ValueToString: FooProvider)]
impl<Context> CanDoFoo for Context {
    fn foo(&self, value: u32) -> String {
        value.to_string()
    }
}

#[cgp_impl(new WithNamePrefix: FooProvider)]
impl<Context> CanDoFoo for Context
where
    Context: HasName,
{
    fn foo(&self, value: u32) -> String {
        format!("{}: {}", self.name(), value)
    }
}

pub struct Foo<Tag> {
    pub tag: Tag,
}

#[cgp_impl(new WithFooTag: FooProvider)]
impl<Tag> CanDoFoo for Foo<Tag>
where
    Tag: Display,
{
    fn foo(&self, value: u32) -> String {
        format!("{}: {}", self.tag, value)
    }
}
