# Consumer and provider traits

The defining idea of CGP is that one trait definition becomes two traits — a consumer trait that callers use and a provider trait that implementers write — so that many independent implementations of the same capability can coexist without violating Rust's coherence rules.

## The problem

Rust conflates using a capability with implementing it, and its coherence rules then permit only one implementation per type. A plain `trait CanCalculateArea` is implemented by the same type that callers invoke `.area()` on, so a crate can supply at most one `area` behavior for a given context, and an implementation written in a downstream crate runs into the orphan rule. This is exactly the wall a programmer hits when they want two interchangeable strategies for the same operation, or want to define a behavior for a type they do not own.

CGP's answer is to split the single trait into a pair. The capability is still declared once, but the declaration is compiled into two related traits whose roles are kept apart: one is what code *calls*, the other is what code *implements*. Pulling these two roles into separate traits is what lets an unlimited number of implementations live side by side, because the type that carries each implementation is no longer the context.

## The duality

A consumer trait is the caller's view: it keeps the original `Self` receiver, so a caller writes `context.area()` exactly as with an ordinary trait. A provider trait is the implementer's view: the original `Self` is moved out into an explicit leading `Context` type parameter, and every `self`/`Self` becomes `context`/`Context`. The provider trait is then implemented not for the context but for a dedicated, zero-sized **provider** struct — a type-level-only name like `RectangleArea` that is never instantiated and carries no runtime value.

Moving `Self` to a parameter is what sidesteps coherence. Because a provider implements `AreaCalculator<Context>` for *its own* struct over a generic `Context`, rather than implementing a trait for the context, the orphan and overlap rules do not bite: a crate can define `RectangleArea`, `CircleArea`, and any number of further providers for the same component, each a distinct `Self` type, all valid at once. The cost is that the provider-trait shape reads inside-out, which is why providers are normally written through the sugar described below rather than by hand.

These two traits are produced from one definition by [`#[cgp_component]`](../reference/macros/cgp_component.md), the macro that turns an ordinary trait into a full CGP **component** — the consumer trait, the provider trait, a zero-sized component-name struct used as a key, and the blanket impls that connect them.

## How the two traits connect

Two generated blanket impls bridge the consumer and provider sides, and together they make `context.area()` resolve to a chosen provider without the caller naming it. The **consumer blanket impl** says that any context which implements the provider trait *for itself* automatically gets the consumer trait, forwarding `context.area()` to `Context::area(self)`. The **provider blanket impl** says that any provider which delegates the component inherits the provider trait from whatever it delegates to, looked up through [`DelegateComponent`](../reference/traits/delegate_component.md).

Wiring is what supplies that delegation. A concrete context picks one provider per component by becoming a type-level table whose entry for `AreaCalculatorComponent` names the chosen provider; [`delegate_components!`](../reference/macros/delegate_components.md) writes that table. The two blanket impls then chain through it: `context.area()` resolves through the consumer impl to the context implementing the provider trait for itself, which resolves through the provider impl to the table entry, landing on the selected provider's `area`. Selecting a different provider in the table is the only change needed to swap the behavior — no caller is touched.

The marker [`IsProviderFor`](../reference/traits/is_provider_for.md) rides along on the provider trait as a supertrait, capturing each provider's dependencies so that a missing one surfaces as a readable compiler error rather than a bare "trait not implemented". It is wiring's bookkeeping, not something a provider author writes; it is generated for them.

## In practice

Most code never sees the raw provider-trait shape, because the sugar restores the familiar look of an ordinary trait impl. A provider is written with [`#[cgp_impl]`](../reference/macros/cgp_impl.md), which lets the author keep `self`, `Self`, and the consumer-trait signatures while the macro mechanically rewrites the block into the provider-trait form and emits the `IsProviderFor` impl. The lower-level [`#[cgp_provider]`](../reference/macros/cgp_provider.md) is the same machinery without the `self`/`Self` rewrite, for when the inside-out shape is wanted explicitly.

A complete component ties the pieces together: define the trait, write a provider, wire it, call it.

```rust
use cgp::prelude::*;

#[cgp_component(AreaCalculator)]
pub trait CanCalculateArea {
    fn area(&self) -> f64;
}

#[cgp_impl(new RectangleArea)]
impl AreaCalculator {
    fn area(&self, #[implicit] width: f64, #[implicit] height: f64) -> f64 {
        width * height
    }
}

#[derive(HasField)]
pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

delegate_components! {
    Rectangle {
        AreaCalculatorComponent: RectangleArea,
    }
}

fn print_area(rect: &Rectangle) {
    println!("area = {}", rect.area()); // CanCalculateArea, via RectangleArea
}
```

A context can also implement a consumer trait directly, exactly as it would a vanilla Rust trait, when code reuse is not the goal. The consumer/provider split is a superset of ordinary traits, not a replacement: the provider machinery is what you opt into when a capability needs more than one implementation, and skipping it costs nothing for the simple case.

A second provider mirrors the consumer relationship in reverse. [`UseContext`](../reference/providers/use_context.md) is a built-in provider that implements the provider trait *by routing back through the context's own consumer-trait implementation* — the dual of the consumer blanket impl, and the hook that lets a higher-order provider fall back to whatever the context already has wired.

## Related constructs

[`#[cgp_component]`](../reference/macros/cgp_component.md) is the macro that generates the whole component — both traits, the component-name struct, and the blanket impls — and is the doc to read for the exact expansion. Providers are written with [`#[cgp_impl]`](../reference/macros/cgp_impl.md) (consumer-trait-style sugar) or its lower layer [`#[cgp_provider]`](../reference/macros/cgp_provider.md); [`#[cgp_fn]`](../reference/macros/cgp_fn.md) is the lighter alternative when only a single implementation is ever needed and no wiring is wanted.

The connective tissue is documented in the traits and provider sections. [`DelegateComponent`](../reference/traits/delegate_component.md) is the type-level table the provider blanket impl reads, written by [`delegate_components!`](../reference/macros/delegate_components.md); [`IsProviderFor`](../reference/traits/is_provider_for.md) is the dependency-tracking marker on the provider trait; and [`UseContext`](../reference/providers/use_context.md) is the provider that closes the loop back to the consumer side. For the dependency-injection idea that providers express in their `where` clauses, see [impl-side dependencies](impl-side-dependencies.md); for why the trait split is needed in the first place and how local wiring restores coherent use, see [bypassing coherence](coherence.md).
