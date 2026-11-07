//! `Self::CONST` naming an associated const the block declares survives the
//! `Self` rewrite, in a `#[cgp_impl]` body and in a component's default method.
//!
//! The rewrite turns `Self` into the context, but a `Self::LIMIT` where the block
//! itself declares `const LIMIT` means the provider's own const, as with a local
//! associated type. The macros collect the consts a block declares beside its
//! associated types and leave such a path alone, so `AllowUnderLimit` reads its
//! own `LIMIT` and the provider trait's default body reads the wired provider's.
//! In a `#[cgp_impl]` body the path is also qualified through the provider
//! trait, which the snapshot pins.
//!
//! See cgp-knowledge-base/cgp/implementation/entrypoints/cgp_impl.md (Behavior and corner cases)
//! and cgp-knowledge-base/cgp/reference/macros/cgp_impl.md.

use cgp::prelude::*;
use cgp_macro_test_util::snapshot_cgp_impl;

#[cgp_component(RateLimiter)]
pub trait CanRateLimit {
    const LIMIT: u32;

    fn allow(&self, count: u32) -> bool;

    // The default body is copied into the provider trait, where `Self::LIMIT`
    // must stay the provider's const rather than become the unbounded context's.
    fn remaining(&self, count: u32) -> u32 {
        Self::LIMIT.saturating_sub(count)
    }
}

// The provider's `Self::LIMIT` becomes the qualified path through its provider
// trait, in plain code and inside a macro body alike: left as `Self::LIMIT` on
// the provider, it would be ambiguous with the consumer trait's `LIMIT`, which
// the consumer blanket impl also gives the provider.
snapshot_cgp_impl! {
    #[cgp_impl(new AllowUnderLimit)]
    impl RateLimiter {
        const LIMIT: u32 = 10;

        fn allow(&self, count: u32) -> bool {
            count < Self::LIMIT && matches!(count, 0..Self::LIMIT)
        }
    }

    expand_allow_under_limit(output) {
        insta::assert_snapshot!(output, @"
        impl<__Context__> RateLimiter<__Context__> for AllowUnderLimit {
            const LIMIT: u32 = 10;
            fn allow(__context__: &__Context__, count: u32) -> bool {
                count < <AllowUnderLimit as RateLimiter<__Context__>>::LIMIT
                    && matches!(
                        count, 0.. < AllowUnderLimit as RateLimiter < __Context__ > > ::LIMIT
                    )
            }
        }
        impl<__Context__> IsProviderFor<RateLimiterComponent, __Context__, ()>
        for AllowUnderLimit {}
        ///Provider `AllowUnderLimit` for [`RateLimiter`].
        pub struct AllowUnderLimit;
        ")
    }
}

pub struct App;

delegate_components! {
    App {
        RateLimiterComponent: AllowUnderLimit,
    }
}

#[test]
fn test_self_local_assoc_const() {
    assert!(App.allow(9));
    assert!(!App.allow(10));
    assert_eq!(App.remaining(4), 6);
}
