//! One unit test per file. Each file is self-contained: it defines its own
//! traits and context types at module scope so that the type-level wiring of
//! one test never leaks into another.
//!
//! These files own the canonical `#[blanket_trait]` expansion snapshots, one
//! per genuinely distinct variant of the macro.

// Plain supertrait-only blanket trait.
pub mod basic;

// Blanket trait carrying a default method.
pub mod with_method;

// Blanket trait re-exporting an associated type from a supertrait.
pub mod associated_type;

// Same, but the associated type carries a bound (`: Clone`).
pub mod associated_type_bounded;
