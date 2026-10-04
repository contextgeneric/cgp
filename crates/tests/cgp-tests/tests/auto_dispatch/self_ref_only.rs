//! `#[cgp_auto_dispatch]` on a `&self` method with no extra arguments.
//!
//! The generated handler routes a `&FooBar` reference to the matching variant
//! impl of `CanCall`.
//!
//! See cgp-knowledge-base/cgp/implementation/entrypoints/cgp_auto_dispatch.md and
//! cgp-knowledge-base/cgp/reference/macros/cgp_auto_dispatch.md.

use cgp::prelude::*;
use cgp_macro_test_util::snapshot_cgp_auto_dispatch;

use super::types::{Bar, Foo, FooBar};

snapshot_cgp_auto_dispatch! {
    #[cgp_auto_dispatch]
    pub trait CanCall {
        fn call(&self) -> &'static str;
    }

    expand_self_ref_only(output) {
        insta::assert_snapshot!(output, @"
        pub trait CanCall {
            fn call(&self) -> &'static str;
        }
        impl<__Variants__> CanCall for __Variants__
        where
            MatchWithValueHandlersRef<
                ComputeCall,
            >: for<'__a__> Computer<(), (), &'__a__ __Variants__, Output = &'static str>,
            __Variants__: HasExtractor,
        {
            fn call(&self) -> &'static str {
                <MatchWithValueHandlersRef<
                    ComputeCall,
                > as Computer<_, _, _>>::compute(&(), ::core::marker::PhantomData::<()>, self)
            }
        }
        #[cgp_computer(ComputeCall)]
        fn __compute_call__<'__a__, __Variants__: CanCall>(
            __Variants__: &'__a__ __Variants__,
        ) -> &'static str {
            __Variants__.call()
        }
        ")
    }
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

pub trait CheckCanCallFooBar: CanCall {}
impl CheckCanCallFooBar for FooBar {}

#[test]
fn test_call_self_ref_only() {
    assert_eq!(FooBar::Foo(Foo).call(), "foo");
}
