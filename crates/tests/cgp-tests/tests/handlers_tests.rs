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
//! See cgp-knowledge-base/cgp/reference/components/computer.md,
//! cgp-knowledge-base/cgp/reference/components/producer.md,
//! cgp-knowledge-base/cgp/reference/components/handler.md, and
//! cgp-knowledge-base/cgp/reference/providers/handler_combinators.md.
#![allow(dead_code)]

pub mod handlers;
