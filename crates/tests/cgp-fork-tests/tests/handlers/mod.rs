//! One unit test per file. Each file is self-contained: it defines its own
//! providers, context types, and wiring at module scope so that the type-level
//! setup of one test never leaks into another.

// The `#[cgp_computer]` / `#[cgp_producer]` macros and the `Handler` family:
// runtime tests that a single function definition yields a provider usable
// across the whole computation family (compute / try_compute / produce / handle
// and their async and by-ref variants), including computers whose `#[field]` /
// `#[implicit]` arguments are read from the context.
pub mod computer_context_args;
pub mod computer_macro;
pub mod handler_macro;
pub mod producer_macro;

// The `PipeHandlers` combinator: chaining computers, and chaining handlers with
// the `Promote*` adapters that lift a simpler handler into a more capable one.
pub mod pipe_computers;
pub mod pipe_handlers;

// `#[derive_promote]` lifts a `Computer` into a component provider.
pub mod promote_computer;
pub mod promote_with_input;
