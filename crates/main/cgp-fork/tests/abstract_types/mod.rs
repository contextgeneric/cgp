//! One unit test per file. Each file is self-contained: it defines its own
//! abstract-type components, providers, and context types at module scope so that
//! the type-level wiring of one test never leaks into another.

// Canonical `#[cgp_type]` macro-expansion snapshots (this concept owns them): the
// full expansion plus its genuinely distinct variants — a self-referential
// associated-type bound, an unsized generic parameter with a custom provider
// name, and an associated type bounded by another abstract-type component.
pub mod cgp_type_bounded;
pub mod cgp_type_macro;
pub mod cgp_type_self_referential;
pub mod cgp_type_unsized;

// The `#[use_type]` attribute (and the `#[extend]` alternative) importing an
// abstract type into a `#[cgp_component]`/`#[cgp_impl]`/`#[cgp_auto_getter]`.
pub mod extend_component;
pub mod use_type_auto_getter;
pub mod use_type_component;
pub mod use_type_foreign;
pub mod use_type_generic_param;
pub mod use_type_path_qualified;
pub mod use_type_trait_arg_component;

// The `#[use_type]` attribute rewriting abstract types inside `#[cgp_fn]`: the
// bare alias, alias renaming, type-equality bounds, foreign/nested type sources
// (including a two-hop foreign chain), the grounding of an alias that appears in
// the imported trait's own generic arguments, and the arrangements resolution must
// be indifferent to — a reversed chain, a shared context, and specs split across
// stacked attributes. These keep the `#[cgp_fn]` snapshot because the abstract-type
// rewrite is the point (the `#[cgp_fn]` expansion itself is owned by
// `implicit_arguments`).
pub mod use_type_fn_alias;
pub mod use_type_fn_deep_foreign;
pub mod use_type_fn_equality;
pub mod use_type_fn_equality_cross_trait;
pub mod use_type_fn_equality_nested;
pub mod use_type_fn_expr_path;
pub mod use_type_fn_extend;
pub mod use_type_fn_foreign;
pub mod use_type_fn_foreign_equality;
pub mod use_type_fn_foreign_equality_cross_trait;
pub mod use_type_fn_generic_trait_equality;
pub mod use_type_fn_nested_foreign;
pub mod use_type_fn_reverse_order;
pub mod use_type_fn_shared_context;
pub mod use_type_fn_stacked_attributes;
pub mod use_type_fn_trait_arg_alias;
pub mod use_type_foreign_getter;
pub mod use_type_uses_supertrait;
