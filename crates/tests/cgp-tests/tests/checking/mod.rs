//! One unit test per file. Each file is self-contained: it defines its own
//! components, getters, and context types at module scope so that the type-level
//! wiring of one test never leaks into another.

// `delegate_and_check_components!` snapshots (this concept owns the macro's
// expansion): the basic wire-and-check step, its generic-context form, and the
// generic-parameter `#[check_params]` / array-key form.
pub mod delegate_and_check_basic;
pub mod delegate_and_check_generic;
pub mod delegate_and_check_params;

// `check_components!` snapshots (this concept owns the macro's expansion): the
// standalone check with `#[check_trait(...)]` overrides and per-entry parameter
// lists, the `#[check_providers(...)]` form, and the generic-context/lifetime form.
pub mod check_generic;
pub mod check_providers;
pub mod check_trait;
