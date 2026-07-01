//! Entrypoint for the `basic_delegation` concept.
//!
//! Covers the fundamentals of the CGP component pattern: defining a
//! consumer/provider component pair with `#[cgp_component]`, writing providers
//! with `#[cgp_impl]`, wiring a context to providers with `delegate_components!`,
//! and implementing a consumer trait directly on a context.
//!
//! See docs/concepts/consumer-and-provider-traits.md and
//! docs/reference/macros/delegate_components.md.
#![allow(dead_code)]

pub mod basic_delegation;
