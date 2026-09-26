//! `#[cgp_auto_dispatch]` in a module that imports the computer consumer traits.
//!
//! The generated method calls the matcher through its provider trait, so importing
//! `CanCompute` and `CanComputeAsync` beside the prelude's `Computer` and
//! `AsyncComputer` leaves the call unambiguous for a synchronous method with an
//! argument and for an async `&self` method, both dispatched over `FooBar`.
//!
//! See cgp-knowledge-base/cgp/implementation/entrypoints/cgp_auto_dispatch.md.

// Only their presence in scope matters: the generated calls must not resolve to them.
#[allow(unused_imports)]
use cgp::extra::handler::{CanCompute, CanComputeAsync};
use cgp::prelude::*;
use futures::executor::block_on;

use super::types::{Bar, Foo, FooBar};

#[cgp_auto_dispatch]
pub trait CanGreet {
    fn greet(&self, name: &str) -> String;
}

impl CanGreet for Foo {
    fn greet(&self, name: &str) -> String {
        format!("foo greets {name}")
    }
}

impl CanGreet for Bar {
    fn greet(&self, name: &str) -> String {
        format!("bar greets {name}")
    }
}

#[cgp_auto_dispatch]
#[async_trait]
pub trait CanCall {
    async fn call(&self) -> &'static str;
}

impl CanCall for Foo {
    async fn call(&self) -> &'static str {
        "foo"
    }
}

impl CanCall for Bar {
    async fn call(&self) -> &'static str {
        "bar"
    }
}

#[test]
fn test_consumer_traits_in_scope() {
    assert_eq!(FooBar::Bar(Bar).greet("ann"), "bar greets ann");
    assert_eq!(block_on(FooBar::Foo(Foo).call()), "foo");
}
