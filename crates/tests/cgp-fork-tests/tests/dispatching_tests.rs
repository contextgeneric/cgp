//! Entrypoint for the `dispatching` concept.
//!
//! Covers CGP's dispatch machinery: the `#[cgp_auto_dispatch]` macro (which
//! turns a per-variant trait into a handler that routes an extensible-data enum
//! to the matching variant impl), the `UseDelegate` dispatch provider and its
//! `UseDelegate`-table form of `delegate_components!`, and composing
//! handler/computer providers.
//!
//! This concept owns the `UseDelegate`-table snapshots of `delegate_components!`
//! and `delegate_and_check_components!`; other macros used here as incidental
//! scaffolding are written in their plain form (their expansions are pinned in
//! their own owning targets).
//!
//! See cgp-knowledge-base/cgp/reference/macros/cgp_auto_dispatch.md,
//! cgp-knowledge-base/cgp/reference/providers/use_delegate.md,
//! cgp-knowledge-base/cgp/reference/providers/dispatch_combinators.md, and
//! cgp-knowledge-base/cgp/concepts/dispatching.md.
#![allow(dead_code)]
#![allow(clippy::needless_lifetimes)]

pub mod dispatching;
