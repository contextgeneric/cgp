use cgp::extra::field::impls::{
    CanFinalizeWithDefault, FinalizeOptional, HasOptionalBuilder, SetOptional,
};
use cgp::prelude::*;

#[derive(HasFields, BuildField)]
pub struct Context {
    pub foo: String,
    pub bar: u64,
}

#[test]
fn test_optional_fields() {
    let builder = Context::optional_builder();

    let builder = builder
        .set(PhantomData::<Symbol!("foo")>, "foo".to_owned())
        .set(PhantomData::<Symbol!("bar")>, 42);

    let (replaced, builder) = builder.set_optional(PhantomData::<Symbol!("foo")>, "bar".to_owned());
    assert_eq!(replaced, Some("foo".to_owned()));

    let context = builder.finalize_optional().unwrap();
    assert_eq!(context.foo, "bar");
    assert_eq!(context.bar, 42);
}

#[test]
fn test_optional_and_default_fields() {
    let builder = Context::optional_builder();

    let builder = builder.set(PhantomData::<Symbol!("foo")>, "foo".to_owned());

    let context = builder.finalize_with_default();

    assert_eq!(context.foo, "foo");
    assert_eq!(context.bar, 0);
}
