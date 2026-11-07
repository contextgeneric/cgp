//! This crate implements CGP's snapshot-testing macros.
#![warn(missing_docs)]

/// The snapshot macros' entry points, one per construct.
pub mod entrypoints;
/// The shared parsing and formatting helpers.
pub mod functions;
/// The keyword markers the snapshot macros parse.
pub mod keywords;
/// The snapshot input types.
pub mod types;
