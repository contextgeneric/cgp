//! Entrypoint for the `handlers` concept.
//!
//! Covers CGP's computation family: defining `Computer`/`Producer` providers
//! from functions with `#[cgp_computer]`/`#[cgp_producer]`, the automatic
//! promotion between the synchronous `Computer`, the input-free `Producer`, and
//! the async, fallible `Handler`, and composing handlers into pipelines with the
//! `PipeHandlers` combinator.
//!
//! This concept does *not* own `#[cgp_component]`, `#[cgp_provider]`,
//! `check_components!`, or `delegate_components!` snapshots — those live in their
//! owning targets — so the scaffolding here uses the plain macros.
//!
//! See docs/reference/components/computer.md,
//! docs/reference/components/producer.md, docs/reference/components/handler.md,
//! and docs/reference/providers/handler_combinators.md.
#![allow(dead_code)]

pub mod handlers;
