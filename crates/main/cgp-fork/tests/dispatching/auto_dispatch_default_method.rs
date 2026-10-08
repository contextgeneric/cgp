//! A dispatch trait method with a default body.
//!
//! The macro keeps the trait unchanged, so a payload impl may rely on the default
//! while another overrides it. The enum-level blanket impl always dispatches, so
//! the enum calls whichever body its current variant's payload uses.
//!
//! See cgp-knowledge-base-fork/cgp/implementation/entrypoints/cgp_auto_dispatch.md.

use cgp_fork::prelude::*;

use super::types::{Bar, Foo, FooBar};

#[cgp_auto_dispatch]
pub trait CanDescribe {
    fn describe(&self) -> String {
        "default".to_owned()
    }
}

impl CanDescribe for Foo {}

impl CanDescribe for Bar {
    fn describe(&self) -> String {
        "bar".to_owned()
    }
}

#[test]
fn test_default_method() {
    assert_eq!(FooBar::Foo(Foo).describe(), "default");
    assert_eq!(FooBar::Bar(Bar).describe(), "bar");
}
