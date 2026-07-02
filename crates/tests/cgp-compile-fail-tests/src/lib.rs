//! Compile-fail tests for the CGP macros.
//!
//! Every test here is a `compile_fail` doctest: a ```` ```rust,compile_fail ````
//! block containing a real CGP macro invocation that must **not** compile.
//! rustdoc compiles each block and the test passes only if compilation fails, so
//! this crate exists as a library — doctests are collected only from a library
//! target, never from an integration (`tests/`) target. Run the suite with
//! `cargo test --doc -p cgp-compile-fail-tests` (note that `cargo nextest` does
//! not run doctests).
//!
//! # What belongs here
//!
//! A `compile_fail` doctest is reserved for input that a CGP macro **accepts** but
//! whose **expansion** then fails to compile — the failure lands on the emitted
//! Rust, not inside the macro. This is the right tool for a documented bug or
//! known limitation, and for the cases a macro cannot reject because it lacks the
//! whole-program view the borrow/coherence check needs: two separate
//! `delegate_components!` blocks that delegate the same key, or generic
//! `delegate_components!` entries that expand to overlapping impls, both of which
//! the macro defers to the Rust compiler. Pair each probe with a companion
//! ```` ```rust ```` block that compiles once the offending element is removed, so
//! the test proves *which* element causes the failure, and comment on why it must
//! not compile.
//!
//! Input that a macro **rejects** during expansion (it returns `Err`) does not
//! belong here — test it by driving the entrypoint function directly in
//! `cgp-macro-tests` with the `assert_macro_rejects` helper, which is enough to
//! pin a rejection and gives a precise check of the macro's own diagnostic.
//!
//! # Organization
//!
//! Tests are grouped by CGP concept, mirroring the layout of the main `cgp-tests`
//! suite: one subdirectory per concept under `src/` (`basic_delegation/`,
//! `dispatching/`, …), and within each, one module file per category of
//! compile-fail case. Register each concept as a `pub mod` below and each category
//! as a `pub mod` in the concept's `mod.rs`.
//!
//! No cases are enumerated yet; a future agent adds a concept subdirectory and its
//! category modules alongside the first case it captures.
