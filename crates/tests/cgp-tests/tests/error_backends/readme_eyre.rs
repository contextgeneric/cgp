//! The wiring example in `cgp-error-eyre`'s README, compiled and run as a test. The README marks
//! the block `ignore` because in the crate's own doctests `cgp` names `cgp-core`; the build
//! script turns the block into this module, so the README stays the only copy.
//!
//! See cgp-knowledge-base/projects/error/cgp-error-eyre/testing.md.

include!(concat!(env!("OUT_DIR"), "/readme_eyre.rs"));
