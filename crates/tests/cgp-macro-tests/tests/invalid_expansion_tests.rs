//! Entrypoint for the `invalid_expansion` failure-case target.
//!
//! This target captures cases where a CGP macro currently emits **invalid or
//! wrong Rust** — code that a user would reasonably expect to work but that the
//! macro mishandles. Because the captured expansion is stored as a *string*
//! `insta` snapshot, the test compiles even though the code it describes would
//! not.
//!
//! To add a case:
//! 1. produce the expansion by calling the matching `cgp-macro-lib` entrypoint,
//!    pretty-printing it (see `cgp-macro-test-util-lib`'s `pretty_format`), and
//!    asserting it against an inline `insta` snapshot;
//! 2. add a code comment explaining **why** the output is wrong and **what the
//!    correct output should be**;
//! 3. record the limitation in the owning reference document's `## Known issues`
//!    section (per docs/CLAUDE.md), and link from the test to that document.
//!
//! No cases are enumerated yet; see crates/tests/CLAUDE.md ("Migration status").
#![allow(dead_code)]

pub mod invalid_expansion;
