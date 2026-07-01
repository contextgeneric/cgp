//! Entrypoint for the `async_and_send` concept.
//!
//! Covers CGP's async support and its `Send`-bound story: `#[async_trait]` on
//! `#[cgp_component]` traits, `#[cgp_fn]`, and providers/handlers; the runner
//! components (`Runner`/`CanRun`, `SendRunner`/`CanSendRun`); spawning futures
//! onto an executor that demands `Future: Send + 'static`; and how a concrete
//! context recovers the `Send` bound through a proxy `SendRunner` impl without
//! annotating `Send` throughout the abstract code.
//!
//! See docs/reference/macros/async_trait.md, docs/reference/components/runner.md,
//! and docs/concepts/send-bounds.md.
#![allow(dead_code)]
#![allow(clippy::disallowed_names)]

pub mod async_and_send;
