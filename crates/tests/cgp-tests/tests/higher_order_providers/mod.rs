//! One unit test per file, each self-contained at module scope.

// `#[use_provider]` snapshots (this concept owns them).
pub mod use_provider_fn;
pub mod use_provider_impl;

// The scaling pattern end-to-end: an outer calculator wraps an inner one.
pub mod rectangle_or_circle;
pub mod scaled_area;

// The inner-provider bound of a lifetime-carrying component, which gets no
// `IsProviderFor` counterpart.
pub mod lifetime_inner_provider;
