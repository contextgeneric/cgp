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
fn test_builder() {
    let _context: PartialContext<IsPresent, IsPresent, IsPresent> = Context::builder()
        .build_field(PhantomData::<symbol!("foo")>, 1)
        .build_field(PhantomData::<symbol!("bar")>, "bar".to_owned())
        .build_field(PhantomData::<symbol!("baz")>, true);
}
