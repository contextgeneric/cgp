//! Entrypoint for parser corner-case tests of the `IdentWithTypeArgs` /
//! `IdentWithTypeGenerics` / `PathWithTypeArgs` grammars in `cgp-macro-core`.
//!
//! These call the `cgp-macro-core` parsers directly to pin what they accept,
//! reject, and round-trip.
#![allow(dead_code)]

pub mod ident_with_type_params;
