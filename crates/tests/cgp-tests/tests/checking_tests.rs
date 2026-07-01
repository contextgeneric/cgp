//! Entrypoint for the `checking` concept.
//!
//! Covers CGP's compile-time wiring verification: `check_components!` (a
//! standalone assertion that a context can use a set of components, generating a
//! `CanUseComponent`-supertraited check trait) and `delegate_and_check_components!`
//! (which wires a context and checks the same wiring in one step). This concept
//! owns the canonical macro-expansion snapshots for both, exercising the
//! generic-parameter forms (`#[check_params]`, per-entry parameter lists, array
//! keys), the `#[check_trait(...)]` name override, the `#[check_providers(...)]`
//! form that checks providers directly, and the generic-context/lifetime forms.
//!
//! See docs/reference/macros/check_components.md,
//! docs/reference/macros/delegate_and_check_components.md,
//! docs/reference/traits/can_use_component.md, and docs/concepts/check-traits.md.
#![allow(dead_code)]

pub mod checking;
