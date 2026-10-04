//! `#[cgp_auto_dispatch]` on a `&self` method whose return borrow is written
//! with an *explicit* lifetime (`fn call<'a>(&'a self) -> &'a str`).
//!
//! Companion to `self_ref_return_implicit_ref`, which writes the
//! same shape with an elided lifetime.
//!
//! See cgp-knowledge-base/cgp/implementation/entrypoints/cgp_auto_dispatch.md and
//! cgp-knowledge-base/cgp/reference/macros/cgp_auto_dispatch.md.

use cgp::prelude::*;
use cgp_macro_test_util::snapshot_cgp_auto_dispatch;

use super::types::{Bar, Foo, FooBar};

snapshot_cgp_auto_dispatch! {
    #[cgp_auto_dispatch]
    pub trait CanCall {
        fn call<'a>(&'a self) -> &'a str;
    }

    expand_explicit_return_lifetime(output) {
        insta::assert_snapshot!(output, @"
        pub trait CanCall {
            fn call<'a>(&'a self) -> &'a str;
        }
        impl<__Variants__> CanCall for __Variants__
        where
            MatchWithValueHandlersRef<
                ComputeCall,
            >: for<'a> Computer<(), (), &'a __Variants__, Output = &'a str>,
            __Variants__: HasExtractor,
        {
            fn call<'a>(&'a self) -> &'a str {
                <MatchWithValueHandlersRef<
                    ComputeCall,
                > as Computer<_, _, _>>::compute(&(), ::core::marker::PhantomData::<()>, self)
            }
        }
        #[cgp_computer(ComputeCall)]
        fn __compute_call__<'a, __Variants__: CanCall>(
            __Variants__: &'a __Variants__,
        ) -> &'a str {
            __Variants__.call()
        }
        ")
    }
}

impl CanCall for Foo {
    fn call(&self) -> &str {
        "foo"
    }
}

impl CanCall for Bar {
    fn call(&self) -> &str {
        "bar"
    }
}

pub trait CheckCanCallFooBar: CanCall {}
impl CheckCanCallFooBar for FooBar {}

#[test]
fn test_call_self_ref_return_explicit_ref() {
    assert_eq!(FooBar::Foo(Foo).call(), "foo");
}
