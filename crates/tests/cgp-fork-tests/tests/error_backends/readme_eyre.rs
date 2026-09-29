//! The wiring example in `cgp-fork-error-eyre`'s README, compiled and run as a test. The README marks
//! the block `ignore` because in the crate's own doctests `cgp_fork` names `cgp-fork-core`; the build
//! script turns the block into this module, so the README stays the only copy.
//!
//! See cgp-knowledge-base/projects/error/cgp-fork-error-eyre/testing.md.

include!(concat!(env!("OUT_DIR"), "/readme_eyre.rs"));
