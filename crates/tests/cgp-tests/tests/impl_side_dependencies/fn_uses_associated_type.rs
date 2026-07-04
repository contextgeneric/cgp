//! `#[uses(...)]` accepts a full trait bound, not only the plain `Trait<Params>`
//! form: an associated-type-equality binding such as `HasErrorType<Error = String>`
//! is imported verbatim as `Self: HasErrorType<Error = String>` in the generated
//! impl's `where` clause — an impl-side dependency the consumer trait does not
//! expose. The `#[cgp_fn]` body here is deliberately trivial; what the snapshot
//! pins is that the equality binding lands intact. For pinning an *abstract type*
//! like the error type, the `#[use_type]` equality form
//! (`#[use_type(HasErrorType.{Error = String})]`) remains the preferred spelling;
//! this test exercises the more general bound `#[uses]` now accepts.
//!
//! The `App` context below hand-implements `HasErrorType<Error = String>`, so it
//! satisfies the imported bound and gains `AlwaysTrue` — a wrong pin (a different
//! `Error` type) would make `impl CheckApp for App` fail to compile.
//!
//! See docs/implementation/asts/attributes.md and
//! docs/reference/attributes/uses.md.

use cgp::prelude::*;
use cgp_macro_test_util::snapshot_cgp_fn;

snapshot_cgp_fn! {
    #[cgp_fn]
    #[uses(HasErrorType<Error = String>)]
    pub fn always_true(&self) -> bool {
        true
    }

    expand_always_true(output) {
        insta::assert_snapshot!(output, @"
        pub trait AlwaysTrue {
            fn always_true(&self) -> bool;
        }
        impl<__Context__> AlwaysTrue for __Context__
        where
            Self: HasErrorType<Error = String>,
        {
            fn always_true(&self) -> bool {
                true
            }
        }
        ")
    }
}

pub struct App;

impl HasErrorType for App {
    type Error = String;
}

pub trait CheckApp: AlwaysTrue {}
impl CheckApp for App {}
