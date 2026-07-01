//! Failure cases where a CGP macro emits invalid or incorrect Rust.
//!
//! See the entrypoint `invalid_expansion_tests.rs` for the pattern to follow when
//! adding a case. This module is intentionally empty until the first genuine
//! invalid-expansion case is captured; keeping the target in place means the
//! harness is ready and a future agent only has to add a `pub mod <case>;` line
//! and the snapshot file.
