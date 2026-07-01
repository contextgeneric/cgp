//! One unit test per file. Each file is self-contained: it defines its own
//! structs, tags, and context types at module scope so that the type-level
//! impls of one test never leak into another.

// `#[derive(HasField)]` snapshots (this concept owns the derive's expansion):
// the distinct field shapes the derive supports — named fields, tuple
// (positional) fields, and lifetime-carrying fields.
pub mod index;
pub mod lifetime_field;

// Nested/chained field access: composing per-field `HasField` impls with
// `ChainGetters` + `UseField` to reach a deeply nested value. The
// `#[derive(HasField)]` snapshots are kept; the incidental `#[cgp_getter]` and
// `delegate_and_check_components!` scaffolding is written plainly.
pub mod chain;
pub mod chain_deeply_nested;
pub mod chain_inner_life;
pub mod chain_outer_life;

// Runtime behavior of the type-level tags themselves (no snapshot): `Symbol!`
// and `Index` `Display`/`StaticString`.
pub mod index_display;
pub mod symbol;
