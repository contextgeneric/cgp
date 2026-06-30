# Abstract types

An abstract type is a trait carrying a single associated type that generic code refers to as `Self::Foo` without committing to any concrete type, so that the concrete choice is supplied by wiring and can differ from one context to another.

## The idea

An abstract type lets generic code name a type it does not fix. Instead of hard-coding `f64` or `String`, a CGP trait declares an associated type — `trait HasScalarType { type Scalar; }` — and code written against it refers to `Self::Scalar`, leaving the actual type open. The trait is the abstraction; the associated type is the slot a context fills in. This is the type-level analogue of dependency injection: just as a getter trait lets a context supply a *value* the provider needs, an abstract-type trait lets a context supply a *type* the provider builds on.

The reason this matters is the same reason behavior is made swappable in CGP. A provider written in terms of `Self::Scalar` works unchanged whether a context chooses `f32`, `f64`, or a fixed-point type, and two contexts can make different choices from the same generic code. An abstract type is, in the end, nothing more than an ordinary Rust trait with one associated type — generic functions constrain `Context: HasScalarType` and use `Context::Scalar` exactly as they would any associated type. CGP adds machinery to make declaring and wiring these traits cheap, but the underlying construct is plain Rust, and a context can always implement the trait directly with `impl HasScalarType for App { type Scalar = f64; }`.

## Making a type swappable with `#[cgp_type]`

The [`#[cgp_type]`](../reference/macros/cgp_type.md) macro turns an abstract-type trait into a full CGP component, so the concrete type can be chosen through wiring rather than a hand-written impl. Applied to a trait with exactly one associated type, it produces everything [`#[cgp_component]`](../reference/macros/cgp_component.md) would — the consumer trait, the provider trait, the blanket impls, the component marker — but specialized to forward an associated type rather than a method:

```rust
#[cgp_type]
pub trait HasScalarType {
    type Scalar: Copy;
}
```

The default provider name is keyed off the *associated type* name, not the trait name, so `Scalar` yields `ScalarTypeProvider` and the component `ScalarTypeProviderComponent`. A bound on the associated type, such as `Copy` above, is carried everywhere the type appears in the expansion and enforced on whatever concrete type a context chooses. The decisive extra construct `#[cgp_type]` generates is a blanket impl for `UseType`, described next, which is what lets a context pick a concrete type without writing a provider of its own.

## Wiring a concrete type with `UseType`

A context binds an abstract type to a concrete one by wiring its provider component to [`UseType<T>`](../reference/providers/use_type.md). Because every abstract-type provider has the same trivial shape — "the associated type *is* this concrete type" — `#[cgp_type]` generates that shape once as a blanket impl of the provider trait for `UseType<Scalar>`, setting the associated type to the generic parameter. A context then names the concrete type directly in its delegation table:

```rust
use cgp::prelude::*;

#[cgp_type]
pub trait HasScalarType {
    type Scalar: Copy;
}

pub struct App;

delegate_components! {
    App {
        ScalarTypeProviderComponent: UseType<f64>,
    }
}
```

Wiring `ScalarTypeProviderComponent` to `UseType<f64>` makes `App` implement `HasScalarType` with `Scalar = f64`, with no bespoke provider, and the `Copy` bound is checked against `f64` at the wiring site. This is the type-level mirror of how `UseField` supplies a value-level getter. The `UseType` *provider struct* should not be confused with the [`#[use_type]`](../reference/attributes/use_type.md) *attribute*: the provider, covered here, wires a concrete type into a context; the attribute imports an abstract type into a definition and rewrites bare mentions of it into fully-qualified form. They are complementary but different things.

## Sharing a type across contexts

The point of an abstract type compounds when several pieces of generic code share one. Because the type lives on a trait the context implements, every provider and trait that needs a `Scalar` refers to the *same* `Self::Scalar`, so a context fixes the choice once and all of them agree. This is most valuable when the main target of a trait is a generic parameter rather than the context itself — for example a `CanCalculateAreaOfShape<Shape>` trait implemented by a common `Context` for many `Shape` types. The shapes need not each declare a scalar type or coordinate on a common one; the shared context supplies a single `Scalar`, and every shape interoperates through it:

```rust
#[cgp_type]
pub trait HasScalarType {
    type Scalar: Copy;
}

#[cgp_component(AreaOfShapeCalculator)]
pub trait CanCalculateAreaOfShape<Shape>: HasScalarType {
    fn area(&self, shape: &Shape) -> Self::Scalar;
}
```

Here `Rectangle` and `Circle` carry no scalar type of their own; the context that calculates their areas does, via its `HasScalarType` wiring, and switching the context's `UseType<f32>` to `UseType<f64>` changes the scalar for every shape at once. The same arrangement is how CGP shares an error type across an entire application, which is the canonical example.

## The canonical example: `HasErrorType`

CGP's most-used abstract type is [`HasErrorType`](../reference/components/has_error_type.md), which supplies one `Error` type that an entire context's code agrees on. It is defined with `#[cgp_type]` exactly as above:

```rust
#[cgp_type]
pub trait HasErrorType {
    type Error: Debug;
}
```

A context wires `ErrorTypeProviderComponent` to `UseType<anyhow::Error>` (or any concrete error), and from then on every fallible provider in that context refers to the same `Self::Error`, so errors compose without each component declaring its own error type or converting between several. The `Debug` bound lets that abstract error be unwrapped and logged. `HasErrorType` shows the pattern at its most useful: a single associated type, supplied once by the context through `UseType`, shared by arbitrarily much generic code — and built on by the further error machinery (`CanRaiseError`, `CanWrapError`) that constrains `Self: HasErrorType` and works against the abstract `Error` throughout.

## Related constructs

Abstract types are defined with [`#[cgp_type]`](../reference/macros/cgp_type.md), the abstract-type specialization of [`#[cgp_component]`](../reference/macros/cgp_component.md). They are built on CGP's foundational [`HasType`/`TypeProvider`](../reference/components/has_type.md) component, the built-in abstract-type machinery that `#[cgp_type]` adapts. A context binds a concrete type through the [`UseType` provider](../reference/providers/use_type.md), and other definitions import an abstract type and rewrite bare mentions of it with the [`#[use_type]` attribute](../reference/attributes/use_type.md) — a different construct from the provider despite the shared name. [`HasErrorType`](../reference/components/has_error_type.md) is the canonical abstract type, supplying a shared `Error` type across a context's code. Abstract-type components are wired with [`delegate_components!`](../reference/macros/delegate_components.md) and verified with [`check_components!`](../reference/macros/check_components.md) like any other component.

## Source

The runtime `HasType`, `TypeProvider`, and `UseType` definitions are in [crates/core/cgp-type/src/](../../crates/core/cgp-type/src/); `HasErrorType` is in [crates/core/cgp-error/src/traits/has_error_type.rs](../../crates/core/cgp-error/src/traits/has_error_type.rs); the `#[cgp_type]` codegen is in [crates/macros/cgp-macro-core/src/types/cgp_type/](../../crates/macros/cgp-macro-core/src/types/cgp_type/).
