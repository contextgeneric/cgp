//! One unit test per file. Each file is self-contained: it defines its own
//! components, providers, `#[cgp_fn]` functions, and context types at module
//! scope so that the type-level wiring of one test never leaks into another.

// `#[uses(...)]` on `#[cgp_fn]`: imports a `Self` trait bound so the function can
// call another capability. This concept owns the snapshot showing how `#[uses]`
// lands on the generated impl's `where` clause.
pub mod fn_uses;

// `#[uses(...)]` accepts a full trait bound, not only the plain `Trait<Params>`
// form: this pins the associated-type-equality variant (`HasErrorType<Error = String>`)
// on the `#[cgp_fn]` form, alongside `fn_uses`'s plain-import snapshot.
pub mod fn_uses_associated_type;

// `#[extend(...)]` on `#[cgp_fn]`: adds a supertrait bound to the generated
// trait. This concept owns the snapshot showing how `#[extend]` lands on the
// generated trait definition and impl.
pub mod fn_extend;

// `#[uses(...)]` on a `#[cgp_impl]` provider: imports a `Self` trait bound so the
// provider can call another capability. The provider is written plainly.
pub mod impl_uses;

// `#[uses(...)]` with an associated-type-equality bound on a `#[cgp_impl]`
// provider: pins the context's abstract error type through the imported bound,
// verified by the wiring check.
pub mod impl_uses_associated_type;
