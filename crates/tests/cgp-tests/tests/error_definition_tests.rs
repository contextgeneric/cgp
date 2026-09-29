//! Entrypoint for the `error_definition` concept.
//!
//! `#[cgp_auto_error]` builds `HasErrorType`, `CanRaiseError`, and `CanWrapError`
//! providers from one inherent impl. The traits themselves stay the ones in
//! `cgp-error`; the macro only supplies their wiring.
//!
//! See cgp-knowledge-base/cgp/reference/macros/cgp_auto_error.md.
#![allow(dead_code)]

pub mod error_definition;
