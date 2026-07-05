//! Acceptable failure: two `@`-path entries in one `delegate_components!` block
//! that map the same namespace path produce two conflicting `DelegateComponent`
//! impls (keyed by the same `PathCons<..>` type) for `App`, rejected with the
//! coherence error E0119 — the `@`-path analogue of [duplicate_key_same_block.rs].
//!
//! This fixture pins the **error span** for an `@`-path key. The key type is a
//! synthesized `PathCons<..>` nest whose own span points at the macro
//! `call_site`; the entry instead carries the span of the path segments the user
//! wrote, so E0119 lands on the duplicated `ErrorTypeProviderComponent` segment
//! (the path's leaf) rather than on the whole block. If the path-key span
//! threading in `key/path.rs` regresses, the caret snaps back to the block and
//! this `.stderr` changes.
//!
//! See docs/errors/wiring/conflicting-wiring.md; error-span mechanics in docs/implementation/entrypoints/delegate_components.md.

use cgp::core::error::ErrorTypeProviderComponent;
use cgp::prelude::*;

pub struct App;

delegate_components! {
    App {
        namespace DefaultNamespace;

        @cgp.core.error.ErrorTypeProviderComponent: UseType<String>,
        @cgp.core.error.ErrorTypeProviderComponent: UseType<String>,
    }
}

fn main() {}
