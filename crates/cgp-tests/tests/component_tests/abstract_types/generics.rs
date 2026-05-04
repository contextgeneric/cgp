use cgp::prelude::*;

#[cgp_component(FooProvider)]
pub trait Foo<T> {
    fn foo(&self, value: &T);
}

// Test that the `Error` parameter in `FooProvider<Error>`
// is desugared correctly into `Context::Error` and not `Self::Error`
#[cgp_impl(new FooError)]
#[use_type(HasErrorType::Error)]
impl FooProvider<Error> {
    fn foo(&self, _value: &Error) {}
}
