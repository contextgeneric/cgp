//! One unit test per file. Each file is self-contained: it defines its own
//! components, providers, namespaces, and context types at module scope so that
//! the type-level wiring of one test never leaks into another.

// `cgp_namespace!` expansion snapshots (this concept owns the macro): the basic
// form, the two path-segment shapes (`Type` vs. `symbol`), and multiple
// namespaces attached to one component.
pub mod namespace_basic;
pub mod namespace_multi;
pub mod namespace_symbol_path;
pub mod namespace_type_path;

// `#[prefix(...)]` + `namespace`/`@`-path wiring snapshots (this concept owns the
// namespace forms of `delegate_components!`): attaching components to
// `DefaultNamespace` and wiring a context through `@`-paths, `RedirectLookup`
// redirection, `open` per-value dispatch, and array/group namespace keys.
pub mod multi_param_namespace;
pub mod multi_param_open;
pub mod namespace_group;
pub mod open_dispatch;
pub mod prefix_default_namespace;
pub mod redirect_lookup;

// Namespace inheritance and per-type default impls. `default_impls` and
// `extended` define reusable namespaces/providers (with `cgp_namespace!`,
// `#[prefix]`, and `#[default_impl]` snapshots); the `*_wiring` modules consume
// them from sibling modules to exercise the `for <..> in ..` loop, `DefaultImpls1`,
// and namespace inheritance in `delegate_components!`.
pub mod default_impls;
pub mod default_impls_wiring;
pub mod extended;
pub mod extended_namespace_wiring;
