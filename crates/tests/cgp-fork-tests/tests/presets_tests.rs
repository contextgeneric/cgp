//! Entrypoint for the `presets` concept.
//!
//! A preset bundles wiring entries into one provider. One preset entry in
//! `delegate_components!` expands to a `DelegateComponent` impl per component,
//! and two presets combine by inheritance or by listing both.
//!
//! See cgp-knowledge-base-fork/cgp/reference/macros/cgp_preset.md.
#![allow(dead_code)]

pub mod presets;
