use std::marker::PhantomData;

use cgp::core::field::CanBuildFrom;
use cgp::prelude::*;

#[derive(HasFields, BuildField)]
pub struct FooBarBaz {
    pub foo: u64,
    pub bar: String,
    pub baz: bool,
}

#[derive(HasFields, BuildField)]
pub struct FooBar {
    pub foo: u64,
    pub bar: String,
}

#[derive(HasFields, BuildField)]
pub struct Baz {
    pub baz: bool,
}

#[test]
fn test_basic_builder() {
    let context: FooBarBaz = FooBarBaz::builder()
        .build_field(PhantomData::<symbol!("foo")>, 1)
        .build_field(PhantomData::<symbol!("bar")>, "bar".to_owned())
        .build_field(PhantomData::<symbol!("baz")>, true)
        .finalize_build();

    assert_eq!(context.foo, 1);
    assert_eq!(context.bar, "bar");
    assert!(context.baz);
}

#[test]
fn test_build_from() {
    let foo_bar = FooBar {
        foo: 1,
        bar: "bar".to_owned(),
    };

    let baz = Baz { baz: true };

    let foo_bar_baz: FooBarBaz = FooBarBaz::builder()
        .build_from(foo_bar)
        .build_from(baz)
        .finalize_build();

    assert_eq!(foo_bar_baz.foo, 1);
    assert_eq!(foo_bar_baz.bar, "bar");
    assert!(foo_bar_baz.baz);
}
