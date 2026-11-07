#![no_std]
//! This crate provides CGP's handler family: the computation components and the combinators that
//! compose them.
//!
//! See [`CanCompute`](https://contextgeneric.dev/docs/reference/components/handler/computer), [`CanHandle`](https://contextgeneric.dev/docs/reference/components/handler/handler), [`PipeHandlers`](https://contextgeneric.dev/docs/reference/providers/handler/pipe_handlers), and [`Promote`](https://contextgeneric.dev/docs/reference/providers/handler/promote).
#![warn(missing_docs)]

mod components;
mod providers;
mod types;

pub use components::*;
pub use providers::*;
pub use types::*;
