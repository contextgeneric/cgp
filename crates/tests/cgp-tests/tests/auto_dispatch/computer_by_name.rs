//! The per-variant computer `#[cgp_auto_dispatch]` generates is a public provider
//! with a predictable name, `Compute` plus the method name in PascalCase, and it
//! can be wired by name.
//!
//! Code outside the macro relies on that name: the extensible-shapes example in
//! the knowledge base wires `ComputeArea` into a matcher by hand. This test pins
//! both uses, calling `ComputeCall` on a payload directly and running it through
//! the same value-handler matcher the generated blanket impl uses.
//!
//! See cgp-knowledge-base/cgp/implementation/entrypoints/cgp_auto_dispatch.md.

use cgp::extra::handler::Computer;
use cgp::prelude::*;

use super::types::{Bar, Foo, FooBar};

#[cgp_auto_dispatch]
pub trait CanCall {
    fn call(&self) -> &'static str;
}

impl CanCall for Foo {
    fn call(&self) -> &'static str {
        "foo"
    }
}

impl CanCall for Bar {
    fn call(&self) -> &'static str {
        "bar"
    }
}

#[test]
fn test_computer_by_name() {
    assert_eq!(
        <ComputeCall as Computer<(), (), &Bar>>::compute(&(), PhantomData, &Bar),
        "bar",
    );

    assert_eq!(
        <MatchWithValueHandlersRef<ComputeCall> as Computer<(), (), &FooBar>>::compute(
            &(),
            PhantomData,
            &FooBar::Foo(Foo),
        ),
        "foo",
    );
}
