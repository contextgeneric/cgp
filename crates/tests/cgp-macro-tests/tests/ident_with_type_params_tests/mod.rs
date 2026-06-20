pub mod new_ident_with_type_args;
pub mod new_ident_with_type_generics;
pub mod path_with_type_args;

use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::parse::Parse;
use syn::parse2;

/// Assert that `tokens` parse successfully as `T`.
#[track_caller]
pub fn assert_parses<T: Parse>(tokens: TokenStream) {
    if let Err(err) = parse2::<T>(tokens.clone()) {
        panic!(
            "expected `{tokens}` to parse as `{}`, but parsing failed: {err}",
            core::any::type_name::<T>(),
        );
    }
}

/// Assert that `tokens` are rejected (fail to parse) as `T`.
#[track_caller]
pub fn assert_rejects<T: Parse>(tokens: TokenStream) {
    if parse2::<T>(tokens.clone()).is_ok() {
        panic!(
            "expected `{tokens}` to be rejected as `{}`, but it parsed successfully",
            core::any::type_name::<T>(),
        );
    }
}

/// Assert that re-emitting the parsed value and parsing it again yields the
/// same token stream. This verifies that `parse` then `to_tokens` is a stable
/// round trip, while ignoring purely cosmetic spacing differences against the
/// original source.
#[track_caller]
pub fn assert_idempotent<T: Parse + ToTokens>(tokens: TokenStream) {
    let first: T =
        parse2(tokens.clone()).unwrap_or_else(|e| panic!("parse failed for `{tokens}`: {e}"));
    let emitted = first.to_token_stream();

    let second: T = parse2(emitted.clone())
        .unwrap_or_else(|e| panic!("re-parse failed for `{emitted}` (from `{tokens}`): {e}"));

    assert_eq!(
        emitted.to_string(),
        second.to_token_stream().to_string(),
        "emission is not idempotent for `{tokens}`",
    );
}
