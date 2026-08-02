//! Failure cases where a CGP macro emits invalid or incorrect Rust.
//!
//! See the entrypoint `invalid_expansion_tests.rs` for the pattern to follow when
//! adding a case.

// The variant derives name their own associated types through `Self::…`, so a variant whose name
// collides with one of them makes the emitted path ambiguous and the expansion does not compile.
pub mod reserved_variant_names;
