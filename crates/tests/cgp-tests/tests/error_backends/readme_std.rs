//! The wiring example in `cgp-error-std`'s README, compiled and run as a test. The README marks
//! the block `ignore` because in the crate's own doctests `cgp` names `cgp-core`; the build
//! script turns the block into this module, so the README stays the only copy.
//!
//! See cgp-knowledge-base/projects/error/cgp-error-std/testing.md.

include!(concat!(env!("OUT_DIR"), "/readme_std.rs"));
