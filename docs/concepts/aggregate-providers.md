# Aggregate providers

An aggregate provider is a zero-sized provider whose implementation is a delegation table rather than a method body: it dispatches each component to a sub-provider, so a group of component wirings can be bundled once and reused by many contexts as a single unit.

## Wiring a table that is not a context

The [`delegate_components!`](../reference/macros/delegate_components.md) macro usually defines the wiring of a concrete context — the type a caller holds and invokes methods on. But its target need not be a context at all. With a leading `new` keyword, the macro defines a fresh zero-sized struct and gives *it* a delegation table:

```rust
delegate_components! {
    new GeometryComponents {
        AreaCalculatorComponent: RectangleArea,
        PerimeterCalculatorComponent: RectanglePerimeter,
    }
}
```

`GeometryComponents` is an **aggregate provider**: a marker type, never instantiated, whose only content is the [`DelegateComponent`](../reference/traits/delegate_component.md) table that maps `AreaCalculatorComponent` to `RectangleArea` and `PerimeterCalculatorComponent` to `RectanglePerimeter`. It aggregates a set of related component wirings under one name so they can be adopted together, the way a small library of behaviors is packaged for reuse.

## How a context uses one

A context adopts the whole bundle by delegating a group of components to the aggregate provider in one entry. Because the aggregate provider carries its own table for those components, the context does not repeat the individual wirings:

```rust
delegate_components! {
    Rectangle {
        [AreaCalculatorComponent, PerimeterCalculatorComponent]: GeometryComponents,
    }
}
```

After this, `Rectangle` implements both `CanCalculateArea` and `CanCalculatePerimeter`, and each call resolves through `Rectangle`'s table to `GeometryComponents`, then through `GeometryComponents`' table to the leaf provider. Swapping every context that uses the bundle to a different set of behaviors is a matter of editing the bundle once, not each context. This is the same benefit a [namespace](namespaces.md) gives, reached by a lighter mechanism: an aggregate provider is delegated to explicitly, component by component, rather than joined and inherited by path.

## A provider, not a context

The defining property of an aggregate provider — and the one that governs how it is checked — is that it is a **provider**, occupying the delegate position in a wiring, and is *never its own context*. Tracing a call makes this concrete. When `rect.area()` resolves, the context type stays `Rectangle` through the entire chain:

- `Rectangle: CanCalculateArea` holds because `Rectangle: AreaCalculator<Rectangle>` (the consumer blanket impl).
- `Rectangle: AreaCalculator<Rectangle>` holds because `Rectangle` delegates `AreaCalculatorComponent` to `GeometryComponents` and `GeometryComponents: AreaCalculator<Rectangle>` (the provider blanket impl).
- `GeometryComponents: AreaCalculator<Rectangle>` holds because `GeometryComponents` delegates `AreaCalculatorComponent` to `RectangleArea` and `RectangleArea: AreaCalculator<Rectangle>`.

The context argument is `Rectangle` at every step; `GeometryComponents` appears only in the `Self`/delegate position, never as the `Context`. So a leaf provider like `RectangleArea` reads its fields and capabilities from `Rectangle`, the real context, not from the bundle. `GeometryComponents` has no fields and never implements a provider trait with itself in the context position — it is pure routing. This is why an aggregate provider is a *provider* in the exact sense of the [consumer/provider duality](consumer-and-provider-traits.md): it is something a context delegates *to*, not something that is used *as* a context.

## Composing aggregate providers

Aggregate providers nest, because a delegate is allowed to be another `DelegateComponent` carrier rather than a leaf. One bundle can delegate a component to a second bundle, which delegates it onward, and the resolution walks each table in turn — all while the context argument stays fixed on the real context at the end of the chain. The [`IsProviderFor`](../reference/traits/is_provider_for.md) forwarding impl that each `delegate_components!` entry generates chains the same way, so a dependency left unmet several bundles deep still propagates back to the context where the component is finally checked, rather than vanishing at a bundle boundary. Layering bundles this way is how a base set of behaviors is extended or specialized without rewriting the base.

## Why an aggregate provider is never checked directly

An aggregate provider must be wired with plain [`delegate_components!`](../reference/macros/delegate_components.md), never [`delegate_and_check_components!`](../reference/macros/delegate_and_check_components.md) — and this is a correctness rule, not merely a stylistic one. The check that `delegate_and_check_components!` derives asserts [`CanUseComponent`](../reference/traits/can_use_component.md) on the *target*, which asks whether the target can use each component *as a context*: whether the target both delegates the component and satisfies the chosen provider's real dependencies for itself. An aggregate provider satisfies neither half in a meaningful way — it is never its own context, has no fields, and never stands in the context position — so the assertion cannot hold, and the fused macro would report failures that describe a situation that never occurs at runtime.

An aggregate provider is verified indirectly instead, at the place where it is actually used. Checking a real context that delegates a component to the bundle exercises the whole chain, because [`check_components!`](../reference/macros/check_components.md) on that context walks through the bundle's `IsProviderFor` forwarding down to the leaf provider's bounds. When a bundle needs verifying on its own — to localize which layer of a nested stack is broken — the `#[check_providers(...)]` form of `check_components!` asserts `IsProviderFor` directly on the named aggregate provider *for a concrete context*, which is the provider-side check the bundle's role calls for, rather than the context-side `CanUseComponent` check. Both routes verify the bundle through a real context; neither treats the bundle as a context itself. See [check traits](check-traits.md) for the mechanics of both check forms.

## Aggregate providers versus namespaces

An aggregate provider and a [namespace](namespaces.md) are the two ways CGP packages reusable wiring, and they differ in how a context adopts the package. A context adopts an aggregate provider by *delegating* named components to it explicitly — `[A, B]: TheBundle` — so the context spells out which components come from the bundle. A context adopts a namespace by *joining* it with a `namespace` header, inheriting every entry the namespace resolves and overriding only what it names. The aggregate provider is the more direct, table-to-table mechanism and is the natural choice for a small, explicitly-delegated bundle of behaviors; the namespace adds path-keyed lookup and inheritance-with-override for preset-style configuration that scales across many components. Both are resolved entirely at compile time through `DelegateComponent`, and both are providers-not-contexts in the checking sense.

## Related constructs

An aggregate provider is defined and consumed through [`delegate_components!`](../reference/macros/delegate_components.md), whose `new` keyword emits the bundle struct and table. Its table is the [`DelegateComponent`](../reference/traits/delegate_component.md) trait, and each entry also generates the [`IsProviderFor`](../reference/traits/is_provider_for.md) forwarding that keeps dependencies diagnosable across nested bundles. Because it is a provider and not a context, it is checked through a real context with [`check_components!`](../reference/macros/check_components.md) or its `#[check_providers(...)]` form rather than with [`delegate_and_check_components!`](../reference/macros/delegate_and_check_components.md); see [check traits](check-traits.md). It is the lighter sibling of the [namespace](namespaces.md) as a reusable-wiring mechanism, and it rests on the [consumer/provider duality](consumer-and-provider-traits.md) that distinguishes what is delegated to from what is used as a context.

## Source

The `new` keyword that defines a bundle struct alongside its table is handled in the delegation-table parser at [crates/macros/cgp-macro-core/src/types/delegate_component/](../../crates/macros/cgp-macro-core/src/types/delegate_component/) (the table and `new` handling in `table/main.rs`, the `DelegateComponent`/`IsProviderFor` impl construction in `mapping/eval.rs`). The `DelegateComponent` trait an aggregate provider carries is defined in [crates/core/cgp-component/src/traits/delegate_component.rs](../../crates/core/cgp-component/src/traits/delegate_component.rs), and the `IsProviderFor` marker whose forwarding chains through bundles is in [crates/core/cgp-component/src/traits/is_provider.rs](../../crates/core/cgp-component/src/traits/is_provider.rs).
