use core::convert::Infallible;
use std::marker::PhantomData;

use cgp::core::error::ErrorTypeProviderComponent;
use cgp::core::field::impls::CanBuildFrom;
use cgp::extra::dispatch::{BuildAndMerge, BuildAndSetField, BuildWithHandlers};
use cgp::extra::handler::{Computer, Producer, ProducerComponent};
use cgp::prelude::*;

#[derive(Debug, Eq, PartialEq, CgpData)]
pub struct FooBarBaz {
    pub foo: u64,
    pub bar: String,
    pub baz: bool,
}

#[derive(Debug, Eq, PartialEq, CgpData)]
pub struct FooBar {
    pub foo: u64,
    pub bar: String,
}

#[derive(Debug, Eq, PartialEq, CgpData)]
pub struct Baz {
    pub baz: bool,
}

#[test]
fn test_basic_builder() {
    let context: FooBarBaz = FooBarBaz::builder()
        .build_field(PhantomData::<Symbol!("foo")>, 1)
        .build_field(PhantomData::<Symbol!("bar")>, "bar".to_owned())
        .build_field(PhantomData::<Symbol!("baz")>, true)
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

#[cgp_producer]
fn build_foo_bar() -> FooBar {
    FooBar {
        foo: 1,
        bar: "bar".to_owned(),
    }
}

#[cgp_producer]
pub fn build_foo() -> u64 {
    1
}

#[cgp_producer]
pub fn build_bar() -> String {
    "bar".to_owned()
}

#[cgp_producer(BuildBaz)]
pub fn build_baz() -> bool {
    true
}

#[cgp_context]
pub struct App;

delegate_components! {
    AppComponents {
        ErrorTypeProviderComponent: UseType<Infallible>,
    }
}

#[test]
fn test_build_with_handlers() {
    let context = App;
    let code = PhantomData::<()>;

    pub type Handlers =
        Product![BuildAndMerge<BuildFooBar>, BuildAndSetField<Symbol!("baz"), BuildBaz>];

    assert_eq!(
        BuildWithHandlers::<FooBarBaz, Handlers>::compute(&context, code, ()),
        FooBarBaz {
            foo: 1,
            bar: "bar".to_owned(),
            baz: true,
        }
    );

    assert_eq!(
        BuildWithHandlers::<FooBarBaz, Handlers>::try_compute(&context, code, ()),
        Ok(FooBarBaz {
            foo: 1,
            bar: "bar".to_owned(),
            baz: true,
        })
    );
}

#[test]
fn test_build_with_fields() {
    let context = App;
    let code = PhantomData::<()>;

    pub type Handlers = Product![
        BuildAndSetField<Symbol!("baz"), BuildBaz>,
        BuildAndSetField<Symbol!("bar"), BuildBar>,
        BuildAndSetField<Symbol!("foo"), BuildFoo>,
    ];

    assert_eq!(
        BuildWithHandlers::<FooBarBaz, Handlers>::compute(&context, code, ()),
        FooBarBaz {
            foo: 1,
            bar: "bar".to_owned(),
            baz: true,
        }
    );

    assert_eq!(
        BuildWithHandlers::<FooBarBaz, Handlers>::try_compute(&context, code, ()),
        Ok(FooBarBaz {
            foo: 1,
            bar: "bar".to_owned(),
            baz: true,
        })
    );
}
