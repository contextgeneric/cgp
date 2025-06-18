use std::marker::PhantomData;

use cgp::core::macros::Builder;
use cgp::prelude::*;

#[derive(Builder)]
pub struct Context {
    pub foo: u64,
    pub bar: String,
    pub baz: bool,
}

#[test]
fn test_basic_builder() {
    let context = Context::builder()
        .build_field(PhantomData::<symbol!("foo")>, 1)
        .build_field(PhantomData::<symbol!("bar")>, "bar".to_owned())
        .build_field(PhantomData::<symbol!("baz")>, true)
        .finalize_build();

    assert_eq!(context.foo, 1);
    assert_eq!(context.bar, "bar");
    assert_eq!(context.baz, true);
}
