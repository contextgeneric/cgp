# `#[extend]` — the AST stack

`#[extend(Trait)]` adds *supertrait* bounds to a generated trait, widening the trait's public interface rather than adding a hidden impl-side dependency. It is a modifier attribute collected by a host macro; this page covers what it parses into and what it injects, and the shared collection mechanism lives in the [attribute-modifier overview](README.md). For the user-facing syntax and expansion, read the reference document [reference/attributes/extend.md](../../../reference/attributes/extend.md).

## What it parses into

`#[extend]` has no dedicated AST type: it parses directly into a `Vec<TypeParamBound>` field on its host collector, populated by `Punctuated::<TypeParamBound, Comma>::parse_terminated`. On `#[cgp_fn]` that field is `FunctionAttributes::extend`; on `#[cgp_component]` it is `CgpComponentAttributes::extend`. Each bound is a full `syn::TypeParamBound`, so the same wide grammar `#[uses]` accepts parses here.

## What the hosts inject

`#[extend]` is accepted on `#[cgp_component]` and `#[cgp_fn]`, and the two hosts treat it differently because a `#[cgp_component]` trait can already declare supertraits natively while a `#[cgp_fn]` trait cannot:

- On **`#[cgp_component]`**, `preprocess` appends the bounds to the consumer trait's supertraits (`item_trait.supertraits.extend(attributes.extend.clone())`) before the later stages transform the trait. It is the preferred way to add a *non-type* capability supertrait; an abstract-type supertrait should instead use [`#[use_type]`](use_type.md), which adds the bound *and* rewrites the type.
- On **`#[cgp_fn]`**, the bounds are pushed onto *both* the generated trait's supertraits and the impl's `where` clause. This dual placement exists because it is the only way to add a supertrait to a `#[cgp_fn]` trait — a `#[cgp_fn]`'s own `where` clauses are reserved for impl-side dependencies, so there is no other channel through which a supertrait can reach the generated trait.

The contrast with [`#[uses]`](uses.md) is the reason both exist: `#[uses]` lands its bound on the impl's `Self` alone, hidden from callers, while `#[extend]` makes the bound a supertrait that every caller sees. And the contrast with [`#[extend_where]`](extend_where.md) is placement: `#[extend]` adds a *supertrait* (a bound on the trait's own `Self`), while `#[extend_where]` adds a full `where` predicate that may bound any type.

## Tests

The behavioral tests exercise both hosts and the interaction with getters and `#[use_type]`:

- [impl_side_dependencies/fn_extend.rs](../../../../crates/tests/cgp-tests/tests/impl_side_dependencies/fn_extend.rs) pins the `#[cgp_fn]` supertrait form.
- [abstract_types/extend_component.rs](../../../../crates/tests/cgp-tests/tests/abstract_types/extend_component.rs) exercises it on a component, and [abstract_types/use_type_fn_extend.rs](../../../../crates/tests/cgp-tests/tests/abstract_types/use_type_fn_extend.rs) alongside `#[use_type]`.
- [getters/abstract_type_extend.rs](../../../../crates/tests/cgp-tests/tests/getters/abstract_type_extend.rs) uses it with a getter.

## Source

- The `extend` field is on `FunctionAttributes` in [function.rs](../../../../crates/macros/cgp-macro-core/src/types/attributes/function.rs) and on `CgpComponentAttributes` in [cgp_component_attributes.rs](../../../../crates/macros/cgp-macro-core/src/types/attributes/cgp_component_attributes.rs).
- The hosts that drive it: [entrypoints/cgp_component.md](../../entrypoints/cgp_component.md) and [entrypoints/cgp_fn.md](../../entrypoints/cgp_fn.md).
