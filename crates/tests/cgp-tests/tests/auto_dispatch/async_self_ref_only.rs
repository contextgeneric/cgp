//! `#[cgp_auto_dispatch]` combined with `#[async_trait]`: a `&self` async
//! method, dispatched over `FooBar`.
//!
//! See cgp-knowledge-base/cgp/implementation/entrypoints/cgp_auto_dispatch.md and
//! cgp-knowledge-base/cgp/reference/macros/cgp_auto_dispatch.md.

use cgp::prelude::*;
use cgp_macro_test_util::snapshot_cgp_auto_dispatch;
use futures::executor::block_on;

use super::types::{Bar, Foo, FooBar};

snapshot_cgp_auto_dispatch! {
    #[cgp_auto_dispatch]
    #[async_trait]
    pub trait CanCall {
        async fn call(&self) -> &'static str;
    }

    expand_async_self_ref_only(output) {
        insta::assert_snapshot!(output, @"
        #[async_trait]
        pub trait CanCall {
            async fn call(&self) -> &'static str;
        }
        impl<__Variants__> CanCall for __Variants__
        where
            MatchWithValueHandlersRef<
                ComputeCall,
            >: for<'__a__> AsyncComputer<(), (), &'__a__ __Variants__, Output = &'static str>,
            __Variants__: HasExtractor,
        {
            async fn call(&self) -> &'static str {
                <MatchWithValueHandlersRef<
                    ComputeCall,
                > as AsyncComputer<
                    _,
                    _,
                    _,
                >>::compute_async(&(), ::core::marker::PhantomData::<()>, self)
                    .await
            }
        }
        #[cgp_computer(ComputeCall)]
        async fn __compute_call__<'__a__, __Variants__: CanCall>(
            __Variants__: &'__a__ __Variants__,
        ) -> &'static str {
            __Variants__.call().await
        }
        ")
    }
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

pub trait CheckCanCallFooBar: CanCall {}
impl CheckCanCallFooBar for FooBar {}

#[test]
fn test_call_async_self_ref_only() {
    assert_eq!(block_on(FooBar::Foo(Foo).call()), "foo");
}
