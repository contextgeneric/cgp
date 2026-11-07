#![no_std]
//! This crate provides CGP's component machinery: delegation, provider lookup, and the built-in
//! providers.
//!
//! See [`DelegateComponent`](https://contextgeneric.dev/docs/reference/traits/wiring/delegate_component), [`IsProviderFor`](https://contextgeneric.dev/docs/reference/traits/wiring/is_provider_for), [`CanUseComponent`](https://contextgeneric.dev/docs/reference/traits/wiring/can_use_component), [`UseContext`](https://contextgeneric.dev/docs/reference/providers/use_context), [`UseDefault`](https://contextgeneric.dev/docs/reference/providers/use_default), [`UseDelegate`](https://contextgeneric.dev/docs/reference/providers/use_delegate), and [`RedirectLookup`](https://contextgeneric.dev/docs/reference/providers/redirect_lookup).
#![warn(missing_docs)]

/// Re-exports the component machinery that the macros expand to.
pub mod macro_prelude;

mod namespaces;
mod providers;
mod traits;

pub use namespaces::*;
pub use providers::*;
pub use traits::*;
