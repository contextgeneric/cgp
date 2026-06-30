# `#[cgp_new_provider]`

`#[cgp_new_provider]` behaves exactly like [`#[cgp_provider]`](cgp_provider.md) but additionally declares the provider struct, so a provider trait implementation and its `Self` type can be defined in one place.

## Purpose

`#[cgp_new_provider]` exists to save the one line of boilerplate that almost always accompanies a fresh provider: the `pub struct ProviderName;` declaration. A provider struct is a type-level-only marker — it is never instantiated and holds no runtime state — so declaring it separately from the impl that gives it meaning is pure ceremony. `#[cgp_new_provider]` folds that declaration into the impl, producing the struct, the provider impl, and the generated [`IsProviderFor`](../traits/is_provider_for.md) impl together.

Use `#[cgp_new_provider]` when you are introducing a new provider; use [`#[cgp_provider]`](cgp_provider.md) when the struct already exists, for example because it is declared with default generic parameters that the attribute form cannot express, or because several impls share one struct.

## Syntax

`#[cgp_new_provider]` is applied to a provider-trait impl and accepts the same optional component-type argument as [`#[cgp_provider]`](cgp_provider.md). The only requirement beyond `#[cgp_provider]` is that the provider struct must *not* already be declared, since the macro declares it:

```rust
#[cgp_new_provider]
impl<Context> AreaCalculator<Context> for RectangleArea
where
    Context: HasDimensions,
{
    fn area(context: &Context) -> f64 {
        context.width() * context.height()
    }
}
```

As with `#[cgp_provider]`, the impl uses the native provider-trait shape — an explicit leading `Context` type parameter, the provider struct in the `Self` position, and methods taking `context: &Context` rather than `&self`. An optional argument overrides the component type used in the `IsProviderFor` impl, defaulting otherwise to the provider trait's name plus a `Component` suffix.

## Syntax Grammar

The attribute argument of `#[cgp_new_provider]` is the same single optional component type as [`#[cgp_provider]`](cgp_provider.md):

```ebnf
CgpNewProviderArgs -> ComponentType?

ComponentType      -> Type
```

The argument behaves exactly as it does for `#[cgp_provider]` — omitted means the component defaults to the provider trait's name plus a `Component` suffix, and a given `Type` overrides it. The struct declaration that distinguishes this macro is implied by the macro name and is not written in the argument.

## Expansion

`#[cgp_new_provider]` is implemented as `#[cgp_provider]` with the `new` keyword forced on; its expansion is therefore the `#[cgp_provider]` expansion plus a struct declaration. The example above produces:

```rust
impl<Context> AreaCalculator<Context> for RectangleArea
where
    Context: HasDimensions,
{
    fn area(context: &Context) -> f64 {
        context.width() * context.height()
    }
}

impl<Context> IsProviderFor<AreaCalculatorComponent, Context, ()> for RectangleArea
where
    Context: HasDimensions,
{}

pub struct RectangleArea;
```

The provider impl and the derived `IsProviderFor` impl are exactly what [`#[cgp_provider]`](cgp_provider.md) emits — see its Expansion section for how the component type, context type, and `Params` tuple are assembled. The one addition is `pub struct RectangleArea;`.

The struct's shape is taken from the `Self` type of the impl. A plain provider name yields a unit struct as above. A generic provider yields a struct with a `PhantomData` field over its parameters, so the parameters are bound. For instance:

```rust
#[cgp_new_provider]
impl<Context, Code, InCode> Runner<Context, Code> for SpawnAndRun<InCode>
where
    Context: 'static + Send + Clone + CanSendRun<InCode>,
{ /* ... */ }
```

emits, in addition to the provider and `IsProviderFor` impls:

```rust
pub struct SpawnAndRun<InCode>(pub ::core::marker::PhantomData<(InCode)>);
```

## Examples

Defining a complete provider in one block, struct included, is the common idiom for an implementation that has no struct yet:

```rust
use cgp::prelude::*;

#[cgp_component(AreaCalculator)]
pub trait CanCalculateArea {
    fn area(&self) -> f64;
}

#[cgp_auto_getter]
pub trait HasDimensions {
    fn width(&self) -> &f64;
    fn height(&self) -> &f64;
}

#[cgp_new_provider]
impl<Context> AreaCalculator<Context> for RectangleArea
where
    Context: HasDimensions,
{
    fn area(context: &Context) -> f64 {
        context.width() * context.height()
    }
}
```

This is equivalent to writing `pub struct RectangleArea;` followed by the same impl annotated with [`#[cgp_provider]`](cgp_provider.md). In most code, the same provider would be written even more concisely with [`#[cgp_impl(new RectangleArea)]`](cgp_impl.md), whose `new` keyword plays the identical role of declaring the struct while also letting the body use `self`/`Self`.

## Related constructs

`#[cgp_new_provider]` is [`#[cgp_provider]`](cgp_provider.md) plus a struct declaration, and it implements a provider trait generated by [`#[cgp_component]`](cgp_component.md). Its closest relative is [`#[cgp_impl]`](cgp_impl.md) with the `new` keyword, which produces the same three items — struct, provider impl, and `IsProviderFor` impl — from consumer-trait-style syntax; `#[cgp_impl(new ...)]` desugars to `#[cgp_new_provider]`. A provider so defined is wired to a context with [`delegate_components!`](delegate_components.md) and checked with [`check_components!`](check_components.md).

## Source

The macro entry point is `cgp_new_provider` in [crates/macros/cgp-macro-lib/src/cgp_new_provider.rs](../../../crates/macros/cgp-macro-lib/src/cgp_new_provider.rs); it parses the same `ProviderArgs`, sets `new` to enabled, and then runs the identical lowering as [`#[cgp_provider]`](cgp_provider.md). All of the generation logic — including the struct declaration emitted when `new` is set — lives in [crates/macros/cgp-macro-core/src/types/cgp_provider/](../../../crates/macros/cgp-macro-core/src/types/cgp_provider/); the struct shape is built in `item.rs` (`to_provider_struct`). Expansion snapshots that exercise `#[cgp_new_provider]`, including the generic-struct case, are in [crates/tests/cgp-tests](../../../crates/tests/cgp-tests), notably `src/tests/compose.rs` and `src/tests/async/spawn.rs`.
