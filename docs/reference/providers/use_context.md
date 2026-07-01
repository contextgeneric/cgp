# `UseContext`

`UseContext` is a zero-sized provider that implements any CGP provider trait by forwarding its methods back to the context's own consumer-trait implementation.

## Purpose

`UseContext` exists to turn a context's existing consumer-trait implementation into a provider that other providers can call. A provider trait is normally implemented by some dedicated provider type, but sometimes the implementation you want is exactly the one the context already supplies through its consumer trait. `UseContext` is that bridge: it is a provider whose method bodies simply call the consumer method on the context, so wiring a component to `UseContext` means "use whatever this context already does for this trait."

This makes `UseContext` the exact dual of the consumer-trait blanket implementation. The blanket impl of a consumer trait such as `CanGreet` runs in the consumer-to-provider direction — a context implements `CanGreet` by delegating to whichever provider implements `Greeter` for it. `UseContext` runs in the opposite direction: it implements the provider trait `Greeter` by delegating to whatever `CanGreet` implementation the context has. One forwards the consumer trait to a provider; the other forwards a provider trait back to the consumer trait.

The pattern matters most for [higher-order providers](../../concepts/higher-order-providers.md), which take another provider as a type parameter. A higher-order provider needs an inner provider to delegate to, and `UseContext` lets that inner provider be "the context's own wiring." When a higher-order provider defaults its inner-provider parameter to `UseContext`, the provider falls back to whatever the main context already wires for that component, rather than forcing the author to name an explicit inner provider.

Like every CGP provider, `UseContext` carries no runtime value. It is a unit struct used purely as a type-level marker; the `self` position of its provider impls is never read, and there is nothing to construct or store.

## Definition

`UseContext` is a unit struct with no fields, defined in `cgp-component`:

```rust
pub struct UseContext;
```

Its only state is its identity as a type. Every implementation `UseContext` carries is generated for it by `#[cgp_component]`, not written by hand, so the struct definition itself is deliberately empty.

## Behavior

`#[cgp_component]` emits a `UseContext` implementation of the provider trait for every component it defines, alongside the consumer blanket impl, the provider blanket impl, the component-name struct, and the [`RedirectLookup`](redirect_lookup.md) impl. The generated impl constrains the context to implement the consumer trait and then forwards each method to it. For a component such as

```rust
#[cgp_component(Greeter)]
pub trait CanGreet {
    fn greet(&self);
}
```

the macro generates the following `UseContext` impl (shown with the macro's real placeholder identifiers):

```rust
impl<__Context__> Greeter<__Context__> for UseContext
where
    __Context__: CanGreet,
{
    fn greet(__context__: &__Context__) {
        __Context__::greet(__context__)
    }
}
```

The provider method `greet` takes the context explicitly and calls the context's own `CanGreet::greet`. Each `UseContext` impl is paired with a matching `IsProviderFor` impl carrying the same `where` clause, so that delegation propagates the dependency and the [check traits](../../concepts/check-traits.md) can report a missing consumer-trait implementation precisely. Supertrait bounds on the consumer trait are reproduced in the `UseContext` impl's `where` clause, so a context must satisfy them before `UseContext` can stand in as a provider.

The cyclic-delegation caveat is the one rule to respect: a context must never delegate a component to `UseContext`. Doing so asks the context to implement the consumer trait by delegating to a provider (`UseContext`) that in turn implements the provider trait by calling the consumer trait — a cycle the trait solver cannot resolve, surfacing as an overflow or unsatisfied-bound error. `UseContext` is meant to be supplied to *another* provider as its inner provider, not wired as a context's own delegate for the same component.

## Examples

The idiomatic use of `UseContext` is as the default inner provider of a higher-order provider, so that the higher-order provider routes through the context's existing wiring unless told otherwise. Consider an area calculator that scales the result of an inner calculator:

```rust
use cgp::prelude::*;

#[cgp_component(AreaCalculator)]
pub trait CanCalculateArea {
    fn area(&self) -> f64;
}

pub struct ScaledArea<InnerCalculator = UseContext>(pub PhantomData<InnerCalculator>);

#[cgp_impl(ScaledArea<InnerCalculator>)]
impl<InnerCalculator> AreaCalculator
where
    InnerCalculator: AreaCalculator<Self>,
{
    fn area(&self, #[implicit] scale_factor: f64) -> f64 {
        InnerCalculator::area(self) * scale_factor * scale_factor
    }
}
```

Because `ScaledArea` defaults `InnerCalculator` to `UseContext`, wiring a context to `ScaledArea` (with no explicit inner provider) makes the inner `area` call resolve to the context's own `CanCalculateArea` implementation. The context computes a base area through whatever `AreaCalculator` it already wires, and `ScaledArea` multiplies that by the scale factor. Overriding the parameter — for example `ScaledArea<RectangleArea>` — instead binds the inner calculation statically to `RectangleArea`, bypassing the context's wiring for that step.

Note that `UseContext` only acts as a default when a higher-order provider's struct definition gives it as the default generic parameter, as `ScaledArea` does above. A provider without such a default has no inner provider to fall back to, and the inner provider must always be named explicitly.

## Related constructs

`UseContext` is the dual of the consumer-trait blanket impl that [`#[cgp_component]`](../macros/cgp_component.md) generates, and the macro emits both for every component. It is most useful with [higher-order providers](../../concepts/higher-order-providers.md) as their default inner provider, and the [dispatch combinators](dispatch_combinators.md) use it the same way — `MatchWithValueHandlers` defaults its per-variant provider to `UseContext`, so each matched payload routes back through the context's own wiring and any one variant's handler can be overridden in the table without disturbing the others, as the [extensible shapes](../../examples/extensible-shapes.md) example shows. It also composes with [`WithProvider`](with_provider.md) through the alias `WithContext = WithProvider<UseContext>`, which adapts a context's foundational getter or type implementation into a component. The closely related [`RedirectLookup`](redirect_lookup.md) provider, also generated by `#[cgp_component]`, routes a lookup through a separate table rather than back to the context. The dependency propagation that keeps `UseContext` honest in [check traits](../../concepts/check-traits.md) flows through [`IsProviderFor`](../traits/is_provider_for.md).

## Source

The struct is defined in [crates/core/cgp-component/src/providers/use_context.rs](../../../crates/core/cgp-component/src/providers/use_context.rs), which also declares the `WithContext` alias. The `UseContext` provider impl is generated by `to_use_context_impl` in [crates/macros/cgp-macro-core/src/types/cgp_component/evaluated/to_use_context_impl.rs](../../../crates/macros/cgp-macro-core/src/types/cgp_component/evaluated/to_use_context_impl.rs), which forwards each provider-trait method to the consumer trait via `trait_items_to_delegated_impl_items`. Expansion snapshots showing the generated `UseContext` impl appear in [crates/tests/cgp-tests/tests/getters/](../../../crates/tests/cgp-tests/tests/getters/) and [crates/tests/cgp-tests/tests/basic_delegation/](../../../crates/tests/cgp-tests/tests/basic_delegation/).
