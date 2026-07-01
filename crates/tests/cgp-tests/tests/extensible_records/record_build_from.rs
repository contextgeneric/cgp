//! Building a record from other records with `build_from`.
//!
//! The extensible builder can absorb a whole smaller record into the partial
//! build in one step: `build_from(foo_bar)` merges every field of `FooBar` into
//! the `FooBarBaz` builder, and `build_from(baz)` merges the remaining field, so
//! once all fields are present `finalize_build` yields the target. This relies on
//! `CanBuildFrom`, which rebuilds a record from a superset of its fields.
//!
//! `#[derive(CgpData)]` here is plain scaffolding; its full expansion is pinned
//! by `record_derive`.
//!
//! See docs/reference/traits/has_builder.md and
//! docs/concepts/extensible-records.md.

use cgp::core::field::impls::CanBuildFrom;
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
