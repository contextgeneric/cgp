# `#[use_provider]` — the AST stack

`#[use_provider(Inner: AreaCalculator)]` completes an inner provider's bound for a higher-order provider: the one thing it does is finish the bound by inserting the context as its leading type argument, so the user's `: AreaCalculator` becomes `AreaCalculator<Self>`, and move the completed bound onto the impl's `where` clause. It is a modifier attribute collected by a host macro; this page covers its AST types and what it injects, and the shared collection mechanism lives in the [attribute-modifier overview](README.md). For the user-facing syntax and expansion, read the reference document [reference/attributes/use_provider.md](../../../reference/attributes/use_provider.md).

## `UseProviderAttribute`

The attribute parses into a `UseProviderAttribute` per entry: a `context_type` (always `Self`), a `provider_type` (the inner provider parameter, e.g. `Inner`), a colon, and a `+`-separated list of provider-trait paths as `provider_trait_bounds` (each a `PathWithTypeArgs`). Parsing is straightforward — the context is fixed to `Self`, then the provider type, the colon, and the terminated `+`-list of bounds.

The completion happens in two methods. `to_type_param_bounds(context_type)` walks each provider-trait bound, clones it, and **inserts the context at index 0 of the bound's angle-bracketed arguments**, so `AreaCalculator` becomes `AreaCalculator<Self>` and a bound that already carries parameters keeps them after the context. Position 0 sits ahead of any lifetime argument, which would be invalid Rust on its own; the method re-parses each completed bound through `parse_internal!`, and that `syn` round-trip re-emits lifetimes first, normalizing the order (see [Generic-parameter insertion and lifetime ordering](../../README.md#generic-parameter-insertion-and-lifetime-ordering)). `to_provider_bounds(context_type)` then wraps the completed bounds into a single `provider_type: bounds` `where` predicate:

```rust
// #[use_provider(Inner: AreaCalculator)]  becomes the where-predicate:
Inner: AreaCalculator<Self>
```

## `UseProviderAttributes`

`UseProviderAttributes` holds the `Vec<UseProviderAttribute>` a host collected and applies them through its `AddTypeParamBounds` impl. `add_type_param_bounds(self_type, generics)` returns early when there are no entries, otherwise pushes each entry's `to_provider_bounds(self_type)` predicate onto the impl generics' `where` clause. The `self_type` passed in is the impl's context type, which is what fills the inserted leading argument.

## What the hosts inject

`#[use_provider]` is accepted on `#[cgp_impl]` (collected into `CgpImplAttributes`) and `#[cgp_fn]` (collected into `FunctionAttributes`), and on both it contributes only the completed `where` predicate on the provider parameter. There is no call-site rewriting: the body still calls the inner provider explicitly through the associated-function form (`Inner::area(self)`), so the attribute's whole job is the bound, not the invocation.

## Tests

The behavioral tests exercise both hosts and a full higher-order provider:

- [higher_order_providers/use_provider_fn.rs](../../../../crates/tests/cgp-tests/tests/higher_order_providers/use_provider_fn.rs) pins the `#[cgp_fn]` form.
- [higher_order_providers/use_provider_impl.rs](../../../../crates/tests/cgp-tests/tests/higher_order_providers/use_provider_impl.rs) pins the `#[cgp_impl]` form.
- [higher_order_providers/scaled_area.rs](../../../../crates/tests/cgp-tests/tests/higher_order_providers/scaled_area.rs) wires a full higher-order provider through it.

## Source

- The `use_provider/` submodule in [cgp-macro-core/src/types/attributes/use_provider/](../../../../crates/macros/cgp-macro-core/src/types/attributes/use_provider/): `attribute.rs` holds `UseProviderAttribute` and the bound completion, `attributes.rs` holds `UseProviderAttributes` and its `AddTypeParamBounds` impl.
- The hosts that drive it: [entrypoints/cgp_impl.md](../../entrypoints/cgp_impl.md) and [entrypoints/cgp_fn.md](../../entrypoints/cgp_fn.md).
