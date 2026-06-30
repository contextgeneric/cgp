# `#[derive_delegate]`

`#[derive_delegate]` is an attribute on a `#[cgp_component]` trait that generates a `UseDelegate`-style provider, which dispatches the component's implementation on one of the trait's generic type parameters using an inner type-level table.

> **Legacy:** `#[derive_delegate]` and the [`UseDelegate`](../providers/use_delegate.md) provider it generates are a legacy dispatch mechanism. A component no longer needs this attribute to be dispatched on a generic parameter: the `open` statement of [`delegate_components!`](../macros/delegate_components.md) achieves the same per-type dispatch through the [`RedirectLookup`](../providers/redirect_lookup.md) impl that every [`#[cgp_component]`](../macros/cgp_component.md) already generates, wiring the per-value entries directly into the context's own table with better ergonomics and no separate inner table. Prefer `open` for new code, and add `#[derive_delegate]` only when the legacy `UseDelegate<new ...>` nested-table wiring is specifically wanted. The attribute is retained for compatibility and is expected to be deprecated, and eventually removed, once the namespace-based form is shown to cover every dispatch case.

## Purpose

`#[derive_delegate]` solves the problem of choosing a different provider per value of a generic parameter. A component with a generic parameter, such as `CanCalculateArea<Shape>`, often wants `Rectangle` to be handled by one provider and `Circle` by another. Without help, the author would have to write a dispatcher provider by hand — an impl of the provider trait that looks up the right delegate based on the `Shape` type and forwards every method to it. That impl is mechanical and identical in shape across every component, differing only in which generic parameter is the dispatch key.

The attribute generates that dispatcher for you. Adding `#[derive_delegate(UseDelegate<Shape>)]` to the component emits an implementation of the provider trait for [`UseDelegate`](../providers/use_delegate.md) that treats its inner `Components` type as a type-level table keyed on `Shape`, looks up the delegate for each concrete `Shape`, and forwards the call. A context then wires the component to `UseDelegate<SomeTable>` and fills `SomeTable` with one provider per shape, getting per-type dispatch without writing the dispatcher.

A component may need to dispatch on more than one parameter, and `#[derive_delegate]` supports this by accepting several attributes, each naming its own dispatcher type and key. The `UseDelegate` type CGP provides is the default dispatcher, but the same machinery works for any wrapper type, so a component can dispatch on one parameter through `UseDelegate` and on another through a custom dispatcher such as `UseInputDelegate`. Only the parameter named in the dispatcher's angle brackets is used as the lookup key; the others flow through unchanged.

## Syntax

`#[derive_delegate]` is applied as an outer attribute on a `#[cgp_component]` trait and takes a wrapper type parameterized by the generic parameter to dispatch on. The single form names one dispatcher and one key:

```rust
#[cgp_component(AreaCalculator)]
#[derive_delegate(UseDelegate<Shape>)]
pub trait CanCalculateArea<Shape> {
    fn area(&self, shape: &Shape) -> f64;
}
```

`UseDelegate` is the wrapper type that will carry the lookup table, and `Shape` is the trait generic parameter used as the dispatch key. The key may also be a parenthesized tuple of parameters, `UseDelegate<(A, B)>`, when the table should be keyed on more than one parameter at once.

To dispatch on several parameters independently, repeat the attribute, one per dispatcher. Each names a distinct wrapper type and the single parameter it keys on:

```rust
use core::marker::PhantomData;

#[cgp_component(Computer)]
#[derive_delegate(UseDelegate<Code>)]
#[derive_delegate(UseInputDelegate<Input>)]
pub trait CanCompute<Code, Input> {
    type Output;

    fn compute(&self, _code: PhantomData<Code>, input: Input) -> Self::Output;
}
```

Here the default `UseDelegate` dispatches on `Code`, while the custom `UseInputDelegate` dispatches on `Input`. The custom dispatcher is an ordinary struct the user defines — `pub struct UseInputDelegate<Components>(pub PhantomData<Components>);` — with the same single-type-parameter shape as `UseDelegate`.

## Expansion

Each `#[derive_delegate]` attribute emits one additional provider impl alongside everything else `#[cgp_component]` generates. The impl is for the named wrapper applied to a fresh `Components` table type, and it follows the same forwarding shape as a normal provider blanket impl, except that the lookup key is the dispatch parameter rather than the component name. Starting from the single-form example:

```rust
#[cgp_component(AreaCalculator)]
#[derive_delegate(UseDelegate<Shape>)]
pub trait CanCalculateArea<Shape> {
    fn area(&self, shape: &Shape) -> f64;
}
```

the attribute generates, in addition to the consumer trait, provider trait, and blanket impls, the following dispatcher impl:

```rust
impl<Context, Shape, Components, Delegate> AreaCalculator<Context, Shape>
    for UseDelegate<Components>
where
    Components: DelegateComponent<(Shape), Delegate = Delegate>,
    Delegate: AreaCalculator<Context, Shape>,
{
    fn area(context: &Context, shape: &Shape) -> f64 {
        Delegate::area(context, shape)
    }
}
```

The dispatch key is wrapped in a tuple, `DelegateComponent<(Shape), ...>`, so that a multi-parameter key composes uniformly. The `Components` type is the inner table: when a context looks up the entry for a concrete `Shape`, `DelegateComponent` yields the `Delegate` provider, and the impl forwards `area` to it. The provider trait's other parameters — here just `Context` — pass through to the delegate unchanged. The generic added for the table is named `__Components__` and the looked-up delegate `__Delegate__` in the real output, alongside the provider trait's own context generic `__Context__`; the shorter names are used here for readability.

When the component carries supertraits, they ride along into the dispatcher. The provider trait records each supertrait as a `Context: Supertrait` predicate in its `where` clause, and because the dispatcher impl reuses the provider trait's generics, that predicate appears on the generated `UseDelegate` impl as well — so a `#[derive_delegate]` on a trait like `CanRaiseError<SourceError>: HasErrorType` produces a dispatcher whose `where` clause also requires `Context: HasErrorType`.

When several `#[derive_delegate]` attributes are present, one such impl is generated per attribute, each keyed on its own parameter. The `CanCompute` component above produces one impl for `UseDelegate<Components>` keyed on `(Code)` and a second for `UseInputDelegate<Components>` keyed on `(Input)`, both forwarding `compute` to the looked-up delegate. The two dispatchers are independent, so a context can pick which parameter to dispatch on, or compose them by nesting one table inside another.

The inner table is built with [`delegate_components!`](../macros/delegate_components.md), and its nested-table syntax pairs naturally with this provider. A context wires the component to `UseDelegate<SomeTable>` and defines `SomeTable`'s entries in the same breath:

```rust
delegate_components! {
    MyApp {
        AreaCalculatorComponent:
            UseDelegate<new AreaCalculatorComponents {
                Rectangle: RectangleArea,
                Circle: CircleArea,
            }>,
    }
}
```

The keys in the inner table — `Rectangle`, `Circle` — are the concrete `Shape` types the generated impl looks up, and the values are the providers each one dispatches to.

## Examples

A complete dispatching component connects the attribute to a working wiring. The component declares the dispatcher, and two providers implement it for different shapes:

```rust
use cgp::prelude::*;

#[cgp_component(AreaCalculator)]
#[derive_delegate(UseDelegate<Shape>)]
pub trait CanCalculateArea<Shape> {
    fn area(&self, shape: &Shape) -> f64;
}

pub struct Rectangle { pub width: f64, pub height: f64 }
pub struct Circle { pub radius: f64 }

#[cgp_new_provider]
impl<Context> AreaCalculator<Context, Rectangle> for RectangleArea {
    fn area(_context: &Context, shape: &Rectangle) -> f64 {
        shape.width * shape.height
    }
}

#[cgp_new_provider]
impl<Context> AreaCalculator<Context, Circle> for CircleArea {
    fn area(_context: &Context, shape: &Circle) -> f64 {
        core::f64::consts::PI * shape.radius * shape.radius
    }
}
```

A context wires the component to `UseDelegate` with an inner table mapping each shape to its provider:

```rust
pub struct MyApp;

delegate_components! {
    MyApp {
        AreaCalculatorComponent:
            UseDelegate<new AreaCalculatorComponents {
                Rectangle: RectangleArea,
                Circle: CircleArea,
            }>,
    }
}
```

Now `MyApp` implements `CanCalculateArea<Rectangle>` through `RectangleArea` and `CanCalculateArea<Circle>` through `CircleArea`. The generated `UseDelegate` impl performs the lookup: for a `Rectangle` it reads the `Rectangle` entry from `AreaCalculatorComponents`, finds `RectangleArea`, and forwards `area` to it.

## Related constructs

`#[derive_delegate]` is an attribute on [`#[cgp_component]`](../macros/cgp_component.md) and only makes sense for components that carry generic parameters. It generates an impl for the [`UseDelegate`](../providers/use_delegate.md) provider (or a user-defined dispatcher of the same shape), whose role and behavior that document covers in full. The inner lookup table it dispatches through is populated with [`delegate_components!`](../macros/delegate_components.md), whose nested-table syntax is the idiomatic way to define `UseDelegate<...>` wirings in place.

## Source

The attribute is parsed by `DeriveDelegateAttribute::parse` in [crates/macros/cgp-macro-core/src/types/attributes/derive_delegate/attribute.rs](../../../crates/macros/cgp-macro-core/src/types/attributes/derive_delegate/attribute.rs), which reads the wrapper identifier and the angle-bracketed key (a single identifier or a parenthesized tuple). The dispatcher impl is built by the same file's `to_provider_impl`, which appends `__Components__` and `__Delegate__` generics, emits the `DelegateComponent<(params), Delegate = __Delegate__>` and `__Delegate__: ProviderTrait` bounds, and forwards each method through `trait_items_to_delegated_impl_items`. The attribute is collected in `types/attributes/cgp_component_attributes.rs` and emitted by `to_use_delegate_impls` in [crates/macros/cgp-macro-core/src/types/cgp_component/evaluated/item.rs](../../../crates/macros/cgp-macro-core/src/types/cgp_component/evaluated/item.rs). The `UseDelegate` provider and a worked single-key expansion are documented in [crates/core/cgp-component/src/providers/use_delegate.rs](../../../crates/core/cgp-component/src/providers/use_delegate.rs); the multi-attribute form with a custom `UseInputDelegate` is used in [crates/extra/cgp-handler/src/components/computer.rs](../../../crates/extra/cgp-handler/src/components/computer.rs).
