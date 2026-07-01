//! Building a record from a pipeline of handlers with `BuildWithHandlers`.
//!
//! Each field (or a merged sub-record) is produced by its own `#[cgp_producer]`
//! and slotted into the target with `BuildAndSetField` / `BuildAndMerge`; a
//! `Product![...]` of these handlers drives the whole record build, run through
//! either `compute` or `try_compute`. The order of the handlers in the product
//! is independent of field order.
//!
//! `#[derive(CgpData)]` here is plain scaffolding; its full expansion is pinned
//! by `record_derive`. The `delegate_components!` wiring is likewise incidental
//! (it only fixes the context's error type); its expansion is owned by
//! `basic_delegation`.
//!
//! See docs/concepts/extensible-records.md and
//! docs/reference/traits/has_builder.md.

use core::convert::Infallible;
use std::marker::PhantomData;

use cgp::core::error::ErrorTypeProviderComponent;
use cgp::extra::dispatch::{BuildAndMerge, BuildAndSetField, BuildWithHandlers};
use cgp::extra::handler::Computer;
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

pub struct App;

delegate_components! {
    App {
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
