//! One unit test per file. Each file is self-contained: it defines its own
//! traits, providers, and context types at module scope so that the type-level
//! wiring of one test never leaks into another.

// The `UseDelegate` dispatch provider and the `UseDelegate`-table form of
// `delegate_components!` (this concept owns those snapshots).
pub mod use_delegate_getter;

// Composing handler/computer providers.
pub mod compose;

// Table structs declared by `delegate_components!` that carry a `const` parameter:
// a const-generic `new` target and a nested inner table with a const generic list.
pub mod const_generic_tables;
