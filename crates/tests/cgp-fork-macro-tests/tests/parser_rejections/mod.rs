//! Failure cases: inputs the CGP macros must reject.
//!
//! A rejection test drives a `cgp-fork-macro-lib` entrypoint (or a `cgp-fork-macro-core`
//! parser) with an invalid input and asserts it returns `Err` rather than
//! producing tokens. This is how we pin down which code CGP deliberately refuses,
//! and catch regressions where a macro starts accepting something it should not.
//!
//! To add a case:
//! 1. call the entrypoint, e.g. `cgp_fork_macro_lib::cgp_component(attr, body)`;
//! 2. assert the result is `Err` with [`assert_macro_rejects`];
//! 3. if the rejection corresponds to a documented limitation, note it in the
//!    owning reference document's `## Known issues` section and link to it here.

use proc_macro2::TokenStream;

/// Assert that a macro entrypoint rejects its input. `run` is the entrypoint call
/// (for example `|| cgp_fork_macro_lib::cgp_component(attr.clone(), body.clone())`).
#[track_caller]
pub fn assert_macro_rejects(label: &str, run: impl FnOnce() -> syn::Result<TokenStream>) {
    if let Ok(tokens) = run() {
        panic!("expected `{label}` to be rejected, but it expanded to:\n{tokens}");
    }
}

/// Assert that a macro entrypoint rejects its input with exactly `message`.
///
/// Pinning the message, not just the `Err`, keeps a rejection test honest across
/// a refactor: an input that starts failing for a different reason (an internal
/// parse error, say) fails the test instead of silently passing it.
#[track_caller]
pub fn assert_macro_rejects_with(
    label: &str,
    message: &str,
    run: impl FnOnce() -> syn::Result<TokenStream>,
) {
    match run() {
        Ok(tokens) => {
            panic!("expected `{label}` to be rejected, but it expanded to:\n{tokens}")
        }
        Err(error) => assert_eq!(
            error.to_string(),
            message,
            "`{label}` was rejected with an unexpected message"
        ),
    }
}

pub mod blanket_trait;
pub mod cgp_auto_dispatch;
pub mod cgp_auto_error;
pub mod cgp_auto_impl;
pub mod cgp_auto_log;
pub mod cgp_component;
pub mod cgp_computer;
pub mod cgp_fn;
pub mod cgp_impl;
pub mod cgp_namespace;
pub mod cgp_preset;
pub mod cgp_producer;
pub mod cgp_provider;
pub mod cgp_type;
pub mod check_components;
pub mod delegate_and_check_components;
pub mod delegate_components;
pub mod derive_cgp_data;
pub mod derive_from_variant;
pub mod derive_provider;
pub mod getters;
pub mod use_provider;
pub mod use_type;
