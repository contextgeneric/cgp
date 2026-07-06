# `#[uses]` — the AST stack

`#[uses(TraitA, TraitB<Param>)]` imports `Self` trait bounds onto a provider's generated impl, reading like a `use` statement that lists the capabilities the provider depends on. It is a modifier attribute collected by a host macro rather than a standalone macro; this page covers its AST type and what it injects, and the shared collection mechanism lives in the [attribute-modifier overview](README.md). For the user-facing syntax and expansion, read the reference document [reference/attributes/uses.md](../../../reference/attributes/uses.md).

## `UsesAttributes`

The attribute parses into `UsesAttributes`, which holds a single `Vec<TypeParamBound>` — one bound per imported capability. Each entry is a full [`syn::TypeParamBound`](https://docs.rs/syn/latest/syn/enum.TypeParamBound.html), so the parser accepts any bound a `where` clause accepts, not only the idiomatic `Trait<Params>` form: an associated-type-equality binding (`HasErrorType<Error = AppError>`), a higher-ranked bound (`for<'a> Trait<'a>`), or a lifetime bound all parse. The plain `Trait<Params>` form is the one to prefer, and the equality form is better expressed through [`#[use_type]`](use_type.md) when the trait is an abstract-type component; the wider grammar is simply what a `TypeParamBound` permits.

The type is deliberately thin — it carries the bounds and hands them back through its `ToTypeParamBounds` impl, whose `to_type_param_bounds` clones the imports into a `Punctuated<TypeParamBound, Plus>`. The host is what appends them; `UsesAttributes` only holds and yields them.

## What the hosts inject

`#[uses]` is accepted on `#[cgp_impl]` and `#[cgp_fn]`, and both land the bounds on the generated impl's `where` clause — on `Self` — never on the consumer trait, which is what keeps the dependency hidden from callers. The two hosts store the parsed bounds slightly differently, but the effect is the same:

- On **`#[cgp_impl]`**, the bounds are parsed into the `UsesAttributes` held by `CgpImplAttributes` (its `uses.imports` field). `ItemCgpImpl::lower` adds them to the provider impl's `where` clause as `Self`-keyed predicates, alongside the bounds contributed by `#[use_type]` and `#[use_provider]`.
- On **`#[cgp_fn]`**, the bounds are parsed straight into a `Vec<TypeParamBound>` field on `FunctionAttributes` (a field that mirrors `#[extend]`'s), and `preprocess` pushes each as a `Self: Trait` predicate onto the impl only.

The contrast with [`#[extend]`](extend.md) is the crux and worth holding in mind: `#[uses]` adds an impl-side `Self` bound that the trait interface does not expose, whereas `#[extend]` makes its bound a *supertrait* of the generated trait, visible to every caller. `#[uses]` is therefore the way to declare an impl-side dependency, and `#[extend]` the way to widen the interface.

## Tests

The behavioral tests exercise `#[uses]` on both hosts and alongside generics:

- [impl_side_dependencies/fn_uses.rs](../../../../crates/tests/cgp-tests/tests/impl_side_dependencies/fn_uses.rs) pins the `#[cgp_fn]` form — a `Self` trait bound imported as an impl-side dependency.
- [impl_side_dependencies/impl_uses.rs](../../../../crates/tests/cgp-tests/tests/impl_side_dependencies/impl_uses.rs) pins the `#[cgp_impl]` form.
- [generic_components/fn_impl_generics.rs](../../../../crates/tests/cgp-tests/tests/generic_components/fn_impl_generics.rs) exercises it alongside generic parameters.
- [impl_side_dependencies/fn_uses_associated_type.rs](../../../../crates/tests/cgp-tests/tests/impl_side_dependencies/fn_uses_associated_type.rs) pins the associated-type-equality bound (`HasErrorType<Error = ...>`) that `#[uses]` also accepts, on the `#[cgp_fn]` form.
- [impl_side_dependencies/impl_uses_associated_type.rs](../../../../crates/tests/cgp-tests/tests/impl_side_dependencies/impl_uses_associated_type.rs) exercises the same equality bound end-to-end on the `#[cgp_impl]` form.

## Source

- `UsesAttributes` and its `ToTypeParamBounds` impl are in [cgp-macro-core/src/types/attributes/uses.rs](../../../../crates/macros/cgp-macro-core/src/types/attributes/uses.rs).
- The `#[cgp_impl]` collector is `CgpImplAttributes` in [cgp_impl_attributes.rs](../../../../crates/macros/cgp-macro-core/src/types/attributes/cgp_impl_attributes.rs); the `#[cgp_fn]` collector is `FunctionAttributes` in [function.rs](../../../../crates/macros/cgp-macro-core/src/types/attributes/function.rs).
- The hosts that drive it: [entrypoints/cgp_impl.md](../../entrypoints/cgp_impl.md) and [entrypoints/cgp_fn.md](../../entrypoints/cgp_fn.md).
