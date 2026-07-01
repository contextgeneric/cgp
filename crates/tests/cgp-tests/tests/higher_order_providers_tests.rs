//! Entrypoint for the `higher_order_providers` concept.
//!
//! A higher-order provider takes another provider as a generic parameter and
//! constrains it with a provider-trait bound via `#[use_provider]`, so its inner
//! behavior is chosen by wiring. This concept owns the `#[use_provider]` snapshots
//! (on both `#[cgp_fn]` and `#[cgp_impl]`) and exercises the scaling pattern where
//! an outer calculator wraps an inner one.
//!
//! See docs/reference/attributes/use_provider.md and
//! docs/concepts/higher-order-providers.md.
#![allow(dead_code)]

pub mod higher_order_providers;
