//! Entrypoint for the `error_backends` concept.
//!
//! Covers the standalone error backends `cgp-error-anyhow`, `cgp-error-eyre`, and
//! `cgp-error-std`: wiring each as a context's abstract error type, raising standard and
//! non-standard source errors through their providers, wrapping details, and the output each
//! produces. These crates are documented as a project of their own in the knowledge base.
//!
//! See cgp-knowledge-base/projects/error/README.md and
//! cgp-knowledge-base/cgp/concepts/modular-error-handling.md.
#![allow(dead_code)]

pub mod error_backends;
