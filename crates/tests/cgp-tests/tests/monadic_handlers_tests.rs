//! Entrypoint for the `monadic_handlers` concept.
//!
//! Covers CGP's monadic handler combinators: composing `Computer` handlers
//! through a monad with `PipeMonadic`, and the `BindOk`/`BindErr` binders that
//! sequence a handler within the ok/err monads. These are runtime tests — a
//! passing test is an actual assertion on the computed value, not just
//! successful compilation.
//!
//! See docs/concepts/monadic-handlers.md and
//! docs/reference/providers/monad_providers.md.
#![allow(dead_code)]

pub mod monadic_handlers;
