//! One unit test per file. Each file is self-contained: it defines its own
//! components, providers, and context types at module scope so that the
//! type-level wiring of one test never leaks into another.

// A runtime test showing that a future can be spawned onto an executor that
// requires `Future: Send + 'static`, even though the abstract types and
// interfaces carry no explicit `Send` bounds. The Send bound is recovered by a
// proxy `SendRunner` impl on the concrete context.
pub mod spawn;

// Async `#[cgp_fn]` expansion: this concept owns the async variant of the
// `#[cgp_fn]` macro snapshot (the `#[async_trait]` handling is the feature).
pub mod cgp_fn_async;
