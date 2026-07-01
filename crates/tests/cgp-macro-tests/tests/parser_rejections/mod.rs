//! Failure cases: inputs the CGP macros must reject.
//!
//! A rejection test drives a `cgp-macro-lib` entrypoint (or a `cgp-macro-core`
//! parser) with an invalid input and asserts it returns `Err` rather than
//! producing tokens. This is how we pin down which code CGP deliberately refuses,
//! and catch regressions where a macro starts accepting something it should not.
//!
//! To add a case:
//! 1. call the entrypoint, e.g. `cgp_macro_lib::cgp_component(attr, body)`;
//! 2. assert the result is `Err` with [`assert_macro_rejects`];
//! 3. if the rejection corresponds to a documented limitation, note it in the
//!    owning reference document's `## Known issues` section and link to it here.

use proc_macro2::TokenStream;

/// Assert that a macro entrypoint rejects its input. `run` is the entrypoint call
/// (for example `|| cgp_macro_lib::cgp_component(attr.clone(), body.clone())`).
#[track_caller]
pub fn assert_macro_rejects(label: &str, run: impl FnOnce() -> syn::Result<TokenStream>) {
    if let Ok(tokens) = run() {
        panic!("expected `{label}` to be rejected, but it expanded to:\n{tokens}");
    }
}

pub mod cgp_component;
