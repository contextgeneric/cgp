//! Entrypoint for the `auto_dispatch` concept.
//!
//! Covers `#[cgp_auto_dispatch]`, which takes a trait implemented once per payload
//! type and generates its implementation for any extensible-data enum of those
//! payloads, routing each value to the matching variant's impl through a
//! value-handler matcher. One file per method shape: the receiver form, extra
//! arguments, reference lifetimes, generic traits, `async` methods stacked with
//! `#[async_trait]`, and the names the macro generates.
//!
//! This concept owns the `#[cgp_auto_dispatch]` snapshots; other macros used here
//! as incidental scaffolding are written in their plain form.
//!
//! See cgp-knowledge-base/cgp/implementation/entrypoints/cgp_auto_dispatch.md,
//! cgp-knowledge-base/cgp/reference/macros/cgp_auto_dispatch.md, and
//! cgp-knowledge-base/cgp/concepts/dispatching.md.
#![allow(dead_code)]
#![allow(clippy::needless_lifetimes)]

pub mod auto_dispatch;
