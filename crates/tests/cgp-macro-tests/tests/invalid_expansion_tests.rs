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
//!    section (per cgp-knowledge-base/cgp/AGENTS.md), and link from the test to that document.
//!
//! One case is captured so far: `reserved_variant_names`, where the variant derives name their
//! own associated types through `Self::…` and so cannot be applied to an enum with a variant of
//! a colliding name.
#![allow(dead_code)]

pub mod invalid_expansion;
