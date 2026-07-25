//! `#[use_type]` chaining three foreign imports so a context is resolved through
//! two hops: `#[use_type(HasA.A, HasB.B in A, HasC.C in B)]`.
//!
//! `HasA.A` imports `A` from `Self`, `HasB.B in A` imports `B` from that `A`, and
//! `HasC.C in B` imports `C` from that `B`. Grounding each spec's context up front
//! resolves the chain fully: `A` grounds to `<Self as HasA>::A`, `B` to
//! `<<Self as HasA>::A as HasB>::B`, so the bare `C` rewrites to the three-hop
//! `<<<Self as HasA>::A as HasB>::B as HasC>::C`, and the appended bounds name the
//! same fully-grounded contexts (`<<Self as HasA>::A as HasB>::B: HasC`, not a
//! half-resolved `<A as HasB>::B: HasC`). None of the three abstract-type traits
//! declares a bound on its associated type, so every required bound is supplied by
//! the imports themselves. `CheckApp` asserts a concrete context satisfies the
//! generated trait. The `#[cgp_fn]` snapshot is kept because the multi-hop rewrite
//! is the point.
//!
//! See cgp-knowledge-base/cgp/reference/attributes/use_type.md and
//! cgp-knowledge-base/cgp/concepts/abstract-types.md.

use cgp::prelude::*;
use cgp_macro_test_util::snapshot_cgp_fn;

#[cgp_type]
pub trait HasA {
    type A;
}

#[cgp_type]
pub trait HasB {
    type B;
}

#[cgp_type]
pub trait HasC {
    type C;
}

snapshot_cgp_fn! {
    #[cgp_fn]
    #[use_type(HasA.A, HasB.B in A, HasC.C in B)]
    pub fn deep(&self) -> C {
        todo!()
    }

    expand_deep(output) {
        insta::assert_snapshot!(output, @"
        pub trait Deep: HasA
        where
            <Self as HasA>::A: HasB,
            <<Self as HasA>::A as HasB>::B: HasC,
        {
            fn deep(&self) -> <<<Self as HasA>::A as HasB>::B as HasC>::C;
        }
        impl<__Context__> Deep for __Context__
        where
            Self: HasA,
            <Self as HasA>::A: HasB,
            <<Self as HasA>::A as HasB>::B: HasC,
        {
            fn deep(&self) -> <<<Self as HasA>::A as HasB>::B as HasC>::C {
                todo!()
            }
        }
        ")
    }
}

pub struct Cc;

impl HasC for Cc {
    type C = u32;
}

pub struct Bb;

impl HasB for Bb {
    type B = Cc;
}

pub struct App;

impl HasA for App {
    type A = Bb;
}

pub trait CheckApp: Deep
where
    <Self as HasA>::A: HasB,
    <<Self as HasA>::A as HasB>::B: HasC,
{
}

impl CheckApp for App {}
