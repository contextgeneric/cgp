use cgp::prelude::*;

#[cgp_component(FooProvider)]
pub trait CanDoFoo {
    fn foo(&self, value: u32) -> String;
}

#[cgp_auto_getter]
pub trait HasName {
    fn name(&self) -> &str;
}

#[cgp_impl(new ValueToString)]
impl FooProvider {
    fn foo(&self, value: u32) -> String {
        value.to_string()
    }
}

pub mod inner {
    use core::fmt::Display;

    use cgp::prelude::*;

    use super::{FooProvider, FooProviderComponent, HasName};

    #[cgp_impl(new WithNamePrefix)]
    impl FooProvider
    where
        Self: HasName,
    {
        fn foo(&self, value: u32) -> String {
            format!("{}: {}", self.name(), value)
        }
    }

    pub struct Foo<Tag> {
        pub tag: Tag,
    }

    #[cgp_impl(new WithFooTag: FooProviderComponent)]
    impl<Tag> FooProvider for Foo<Tag>
    where
        Tag: Display,
    {
        fn foo(&self, value: u32) -> String {
            format!("{}: {}", self.tag, value)
        }
    }
}
