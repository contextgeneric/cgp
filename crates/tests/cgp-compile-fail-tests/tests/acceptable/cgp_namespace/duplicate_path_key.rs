//! Acceptable failure: two `@`-path prefix-rewrite entries in one `cgp_namespace!`
//! block that map the same path produce two conflicting `MyNamespace<_>` impls
//! (keyed by the same `PathCons<..>` type), rejected with the coherence error
//! E0119 — the namespace analogue of the `delegate_components!`
//! [duplicate_path_key.rs](../delegate_components/duplicate_path_key.rs).
//!
//! This fixture pins the **error span** for a namespace `@`-path key. The key
//! type is a synthesized `PathCons<..>` nest whose own span points at the macro
//! `call_site`; the entry instead carries the span of the path segments the user
//! wrote, so E0119 lands on the duplicated `@foo.bar` leaf segment rather than on
//! the whole `cgp_namespace!` block. If the re-spanning in
//! `build_namespace_impl` (`mapping/eval.rs`) regresses, the caret snaps back to
//! the block and this `.stderr` changes.
//!
//! See docs/errors/wiring/conflicting-wiring.md; error-span mechanics in docs/implementation/entrypoints/cgp_namespace.md.

use cgp::prelude::*;

cgp_namespace! {
    new MyNamespace {
        @foo.bar => @baz,
        @foo.bar => @qux,
    }
}

fn main() {}
