//! `Self::LIMIT` in a `#[cgp_impl]` block names the nearest `LIMIT`: a nested item's own, then the
//! block's own, before a const of the same name that a `#[uses]` trait gives the context.
//!
//! `Self` in the block means the context, but a `Self::LIMIT` whose `LIMIT` the block declares
//! names the provider's own const, whatever the context's traits declare. Here the context also
//! has `HasLimit::LIMIT`, imported through `#[uses]`, so `allow` reads the block's 10 and
//! `context_allows` reads the context's 100 through the qualified `<Self as HasLimit>`. A local
//! `Floor` inside `allow` has its own `Self` and its own `LIMIT`, which the rewrite leaves alone,
//! and `Floor::LIMIT` names `Floor`'s const although the block declares one of the same name. A
//! `Self::CEILING` naming a const only the context has is rewritten to the context, in code and
//! in a macro body alike.
//!
//! See cgp-knowledge-base/cgp/implementation/entrypoints/cgp_impl.md (Behavior and corner cases)
//! and cgp-knowledge-base/cgp/reference/macros/cgp_impl.md.

use cgp::prelude::*;

pub trait HasLimit {
    const LIMIT: u32;
}

pub trait HasCeiling {
    const CEILING: u32;
}

#[cgp_component(RateLimiter)]
pub trait CanRateLimit {
    const LIMIT: u32;

    fn allow(&self, count: u32) -> bool;

    fn context_allows(&self, count: u32) -> bool;
}

#[cgp_impl(new AllowUnderLimit)]
#[uses(HasLimit, HasCeiling)]
impl RateLimiter {
    const LIMIT: u32 = 10;

    fn allow(&self, count: u32) -> bool {
        struct Floor;

        impl Floor {
            const LIMIT: u32 = 1;

            fn limit() -> u32 {
                Self::LIMIT
            }
        }

        count >= Floor::LIMIT && count >= Floor::limit() && count < Self::LIMIT
    }

    fn context_allows(&self, count: u32) -> bool {
        count < <Self as HasLimit>::LIMIT
            && count < Self::CEILING
            && matches!(Self::CEILING, ceiling if count < ceiling)
    }
}

pub struct App;

impl HasLimit for App {
    const LIMIT: u32 = 100;
}

impl HasCeiling for App {
    const CEILING: u32 = 1000;
}

delegate_components! {
    App {
        RateLimiterComponent: AllowUnderLimit,
    }
}

#[test]
fn test_self_local_assoc_const_scope() {
    assert!(!App.allow(0));
    assert!(App.allow(5));
    assert!(!App.allow(50));
    assert!(App.context_allows(50));
}
