//! Entrypoint for the `parser_rejections` failure-case target.
//!
//! This target collects inputs that the CGP macros must **reject** — malformed
//! syntax, or forms CGP deliberately disallows. Each case asserts that the
//! relevant `cgp-macro-core` parser (or a `cgp-macro-lib` entrypoint) returns an
//! error rather than silently accepting the input.
//!
//! See crates/tests/CLAUDE.md ("Adding a failure case").
#![allow(dead_code)]

pub mod parser_rejections;
