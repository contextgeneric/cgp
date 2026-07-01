# `#[cgp_component]` — implementation

`#[cgp_component]` turns one consumer trait into a full component by parsing the trait, deriving the provider trait and the blanket impls that route between the two sides, and emitting the standard provider impls. This document covers how that works internally; for the accepted syntax and the complete expansion a user sees, read the reference document [reference/macros/cgp_component.md](../../reference/macros/cgp_component.md).

## Entry point

The macro is driven by the thin `cgp_component` function in [cgp-macro-lib/src/cgp_component.rs](../../../crates/macros/cgp-macro-lib/src/cgp_component.rs), which follows the canonical entry-point shape: it parses the attribute into a `CgpComponentArgs` and the item into a `syn::ItemTrait`, then runs the pipeline and emits the result.

```rust
let item_cgp_component = ItemCgpComponent { args, item_trait };
let derived = item_cgp_component.preprocess()?.eval()?.to_items()?;
```

All real logic lives in `cgp-macro-core`. Two failures surface here: a malformed attribute is rejected while parsing `CgpComponentArgs` (the argument grammar is enforced by its `Parse` impl), and applying the macro to a non-trait item fails at `syn::parse2::<ItemTrait>`.

## Pipeline

The macro moves through three stages, each a method on the AST type the previous one produced; the [`cgp_component` AST stack](../asts/cgp_component.md) documents those types in full.

- **preprocess** strips the CGP modifier attributes (`#[derive_delegate]`, `#[prefix]`, and the rest) off the trait, separating them from the plain trait the later stages transform.
- **eval** is the core derivation: it builds the provider trait, the consumer and provider blanket impls, and the component marker struct.
- **to_items** renders everything into the final `Vec<syn::Item>`, appending the standard provider impls (`UseContext`, `RedirectLookup`, and one `UseDelegate` or prefix impl per modifier attribute).

## Generated items

The macro emits five core items in a fixed order — consumer trait, consumer blanket impl, provider trait, provider blanket impl, component struct — followed by the standard provider impls. The order is what the canonical snapshot pins, so it is a contract rather than an incidental detail.

The interesting work is deriving the provider trait from the consumer trait: the original `Self` becomes an explicit leading context parameter, `self`/`Self` in every signature are rewritten to the context (via the `replace_self` visitors), and the trait's sole supertrait becomes an [`IsProviderFor`](../../reference/traits/is_provider_for.md) bound that captures the component and its parameters. A method that took `&self` ends up taking the context by value-name:

```rust
// consumer trait
pub trait CanCalculateArea {
    fn area(&self) -> f64;
}

// derived provider trait
pub trait AreaCalculator<__Context__>:
    IsProviderFor<AreaCalculatorComponent, __Context__, ()>
{
    fn area(__context__: &__Context__) -> f64;
}
```

The two blanket impls are the routing machinery and are forwarding shells: the consumer impl makes any context that implements the provider trait for itself gain the consumer trait, and the provider impl lets any provider that delegates the component (via [`DelegateComponent`](../../reference/traits/delegate_component.md)) inherit the provider trait from its delegate. Both forward each method body to the chosen type through the shared [delegated-impl helpers](../functions/derive/delegated_impls.md), which is why their bodies all read as `<delegate>::method(context, …)`. The `UseContext` and `RedirectLookup` provider impls are two more forwarding shells over the same helpers — one routing back through the context's own consumer impl, the other along a namespace lookup path.

## Behavior and corner cases

A **supertrait** on the consumer trait is not kept as a supertrait on the provider trait; it is lowered to a bound on the context, and the same context bound is threaded onto every generated impl. So `pub trait CanGreet: HasName` produces:

```rust
pub trait Greeter<__Context__>: IsProviderFor<GreeterComponent, __Context__, ()>
where
    __Context__: HasName,
{ /* … */ }
```

A **default method body** is preserved into the provider trait, because the provider trait is a clone of the consumer trait with only `self`/`Self` rewritten. This is what lets an empty `#[cgp_impl]` provider inherit the default and satisfy the component with no method of its own.

**Generic parameters** on the component are appended after the context in the provider trait and collected into the `IsProviderFor` params tuple. A lifetime stays ahead of the context (lifetimes must precede type parameters) and is lifted into `Life<'a>` in that tuple, so `HasReference<'a, T>` yields `IsProviderFor<…, (Life<'a>, T)>`. Only *type* parameters, not lifetimes or const parameters, extend the `RedirectLookup` lookup path.

The **reserved identifiers** appear literally in the output: the context parameter is `__Context__` (unless the `context` key overrides it), the provider parameter is `__Provider__`, and the `RedirectLookup` impl introduces `__Components__` and `__Path__`. These names are chosen so they never clash with a user's own type parameters.

## Known issues

A const generic parameter on the component causes a **panic** rather than a clean error: building the params tuple hits an `unimplemented!` for const parameters (see [parse_is_provider_params](../functions/parse/is_provider_params.md)). Applying `#[cgp_component]` to a trait with a const generic therefore aborts expansion with a panic instead of a spanned `syn::Error`; the correct behavior would be a clean rejection or genuine const support. Because the macro produces no output, this case has no expansion snapshot and is a candidate failure case for `cgp-macro-tests`.

## Snapshots

Every `snapshot_cgp_component!` invocation across the suite is indexed here, since these snapshots all belong to this entrypoint:

- [basic_delegation/component_macro.rs](../../../crates/tests/cgp-tests/tests/basic_delegation/component_macro.rs) — the canonical plain expansion (one method, no parameters).
- [basic_delegation/default_methods.rs](../../../crates/tests/cgp-tests/tests/basic_delegation/default_methods.rs) — a supertrait lowered to a context `where`-bound plus a default method body copied into the provider trait.
- [generic_components/component_lifetime.rs](../../../crates/tests/cgp-tests/tests/generic_components/component_lifetime.rs) — a lifetime kept ahead of `__Context__`, lifted to `Life<'a>`, with a type parameter extending the `RedirectLookup` path via `ConcatPath`.
- [namespaces/namespace_basic.rs](../../../crates/tests/cgp-tests/tests/namespaces/namespace_basic.rs), [namespaces/namespace_symbol_path.rs](../../../crates/tests/cgp-tests/tests/namespaces/namespace_symbol_path.rs), [namespaces/namespace_type_path.rs](../../../crates/tests/cgp-tests/tests/namespaces/namespace_type_path.rs), [namespaces/namespace_multi.rs](../../../crates/tests/cgp-tests/tests/namespaces/namespace_multi.rs), [namespaces/redirect_lookup.rs](../../../crates/tests/cgp-tests/tests/namespaces/redirect_lookup.rs), [namespaces/default_impls.rs](../../../crates/tests/cgp-tests/tests/namespaces/default_impls.rs), [namespaces/prefix_default_namespace.rs](../../../crates/tests/cgp-tests/tests/namespaces/prefix_default_namespace.rs) — the namespace and prefix-impl variants.

Two variants have no snapshot yet: the `UseDelegate` impl a `#[derive_delegate]` attribute adds to a bare component (exercised through the error and handler families instead), and a component carrying a type parameter but no lifetime, distinct from the combined lifetime-and-type case above.

## Tests

The behavioral tests confirm the generated wiring works:

- [basic_delegation/default_methods.rs](../../../crates/tests/cgp-tests/tests/basic_delegation/default_methods.rs) checks at run time that an empty provider impl inherits the default bodies and `App.greet()` returns the expected string.
- [generic_components/component_lifetime.rs](../../../crates/tests/cgp-tests/tests/generic_components/component_lifetime.rs) wires the lifetime-carrying component and passes `check_components!`.
- [cgp-macro-tests/tests/parser_rejections/cgp_component.rs](../../../crates/tests/cgp-macro-tests/tests/parser_rejections/cgp_component.rs) asserts the macro rejects a non-trait item at parse time.

## Source

- Entry point: `cgp_component` in [cgp-macro-lib/src/cgp_component.rs](../../../crates/macros/cgp-macro-lib/src/cgp_component.rs).
- Pipeline and AST types: [cgp-macro-core/src/types/cgp_component/](../../../crates/macros/cgp-macro-core/src/types/cgp_component/), documented in [asts/cgp_component.md](../asts/cgp_component.md).
- Forwarding method bodies: the [delegated-impl helpers](../functions/derive/delegated_impls.md).
- `IsProviderFor` params tuple: [parse_is_provider_params](../functions/parse/is_provider_params.md).
- Fragment construction: [parse_internal!](../macros/parse_internal.md).
