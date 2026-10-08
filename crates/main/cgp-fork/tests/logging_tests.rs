//! Entrypoint for the `logging` concept.
//!
//! `CanLog` is the logger consumer trait. `#[cgp_auto_log]` blanket-implements a
//! narrower trait by packing each method's arguments into a detail struct and
//! calling `log`, the same shape as `#[cgp_auto_getter]` over `HasField`.
//!
//! See cgp-knowledge-base-fork/cgp/reference/macros/cgp_auto_log.md.
#![allow(dead_code)]

pub mod logging;
