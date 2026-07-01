# `#[cgp_type]`

`#[cgp_type]` defines an abstract-type component — a trait carrying a single associated type — by extending [`#[cgp_component]`](cgp_component.md) and generating the extra constructs that let a context choose the concrete type through wiring, most notably a [`UseType`](../providers/use_type.md) blanket impl.

## Purpose

`#[cgp_type]` exists to make associated types swappable across contexts the same way `#[cgp_component]` makes behavior swappable. An abstract type in CGP is just a trait with one associated type — `trait HasScalarType { type Scalar; }` — that lets generic code refer to `Self::Scalar` without committing to a concrete type. On its own such a trait is wired like any other component, but choosing the concrete type would otherwise mean writing a provider impl by hand for every type you want to plug in. `#[cgp_type]` removes that friction.

The macro's value is the additional generated constructs layered on top of the component expansion. Because every abstract-type provider follows the same trivial shape — "the associated type *is* this concrete type" — `#[cgp_type]` can generate that shape once and for all as a [`UseType`](../providers/use_type.md) blanket impl. A context then names the concrete type directly in its wiring (`UseType<String>`) instead of defining a bespoke provider. This is the same convenience relationship that `#[cgp_getter]` has to `UseField`: a general-purpose provider parameterized by the thing the context wants to supply.

Direct implementation remains available and is often the clearest choice. An abstract type can always be implemented straight on a concrete context through its consumer trait, which is barely more verbose than wiring `UseType` and is the most transparent way to show that a CGP abstract type is nothing more than a vanilla Rust trait with an associated type.

## Syntax

The macro is applied to a trait that contains exactly one associated type and no methods. The associated type may carry bounds, but it must not be generic or have a `where` clause. The simplest form takes no argument:

```rust
#[cgp_type]
pub trait HasScalarType {
    type Scalar;
}
```

Like `#[cgp_component]`, the component needs a provider trait name, and `#[cgp_type]` derives one from the associated type's name by default. The default provider name is the associated type name with a `TypeProvider` suffix, so `Scalar` yields the provider `ScalarTypeProvider` and the component name `ScalarTypeProviderComponent`. Note that the default is keyed off the *associated type* name, not the trait name. You can override it by passing a provider name, as with `#[cgp_component]`:

```rust
#[cgp_type(ProvideScalar)]
pub trait HasScalarType {
    type Scalar;
}
```

A bound on the associated type is preserved everywhere the type appears in the expansion. For example `type Scalar: Copy;` carries the `Copy` bound onto the generated provider trait and into the `where` clauses of the generated provider impls.

## Syntax Grammar

The attribute argument of `#[cgp_type]` is the same grammar as [`#[cgp_component]`](cgp_component.md)'s `CgpComponentArgs` — a bare provider name or the keyed `name`/`provider`/`context` form:

```ebnf
CgpTypeArgs -> CgpComponentArgs    // see #[cgp_component]
```

The only difference from `#[cgp_component]` is the default applied when `provider` is omitted: instead of failing, the macro derives the provider name from the *associated type's* name with a `TypeProvider` suffix (so `type Scalar;` yields `ScalarTypeProvider`). All other keys and their defaults behave exactly as documented for `#[cgp_component]`.

## Expansion

`#[cgp_type]` expands to the full `#[cgp_component]` output for the trait, followed by two abstract-type provider impls. The component part is exactly what `#[cgp_component(ScalarTypeProvider)]` would produce for an associated-type trait — the consumer trait, the provider trait, the consumer and provider blanket impls, the `ScalarTypeProviderComponent` marker, and the standard `UseContext` and `RedirectLookup` provider impls. The difference from a behavioral component is that every blanket impl forwards the *associated type* rather than a method; see [`#[cgp_component]`](cgp_component.md) for that core shape.

The first extra construct is the [`UseType`](../providers/use_type.md) blanket impl, which is the heart of `#[cgp_type]`. It implements the provider trait for `UseType<Scalar>` by setting the abstract associated type to the generic parameter `Scalar`. Starting from:

```rust
#[cgp_type]
pub trait HasScalarType {
    type Scalar;
}
```

the macro generates:

```rust
impl<Scalar, __Context__> ScalarTypeProvider<__Context__> for UseType<Scalar> {
    type Scalar = Scalar;
}
```

This says that `UseType<T>` is a provider that supplies `T` as the abstract type. Wiring a context's `ScalarTypeProviderComponent` to `UseType<f64>` therefore implements `HasScalarType` for that context with `Scalar = f64`, with no bespoke provider needed. If the associated type carries a bound, that bound is copied into the impl's `where` clause so the concrete type must satisfy it.

The second extra construct is a `WithProvider` impl, which adapts the foundational [`HasType`/`TypeProvider`](../components/has_type.md) machinery into this component. It implements the provider trait for `WithProvider<__Provider__>` whenever `__Provider__` is a `TypeProvider` for the component:

```rust
impl<__Provider__, Scalar, __Context__> ScalarTypeProvider<__Context__>
    for WithProvider<__Provider__>
where
    __Provider__: TypeProvider<__Context__, ScalarTypeProviderComponent, Type = Scalar>,
{
    type Scalar = Scalar;
}
```

The `HasType`/`TypeProvider` relationship this builds on is CGP's single built-in abstract-type component: `HasType<Tag>` is the consumer trait, `TypeProvider` is its provider trait, and `UseType` is itself a `TypeProvider` (`impl<Context, Tag, Type> TypeProvider<Context, Tag> for UseType<Type> { type Type = Type; }`). The `WithProvider` impl lets a `#[cgp_type]` component be backed by a generic `TypeProvider`, so the same `UseType<T>` value satisfies both the built-in `HasType` and any user-defined `#[cgp_type]` component.

As with the other macros, each generated provider impl is paired with a matching `IsProviderFor` impl carrying the same bounds, and the desugarings above are the exact shape the macro emits today.

## Examples

A complete use defines the abstract type, wires a concrete type through `UseType`, and consumes `Self::Scalar` in generic code:

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

fn zero<Context>() -> Context::Scalar
where
    Context: HasScalarType,
    Context::Scalar: Default,
{
    Default::default()
}
```

`App` wires `ScalarTypeProviderComponent` to `UseType<f64>`, so the generated `UseType` blanket impl makes `App` implement `HasScalarType` with `Scalar = f64`. The `Copy` bound on the associated type is enforced on `f64` at the wiring site.

The abstract type can equally be implemented directly on a concrete context, bypassing both `delegate_components!` and `UseType`:

```rust
impl HasScalarType for App {
    type Scalar = f64;
}
```

This direct form is only marginally longer than the wired form and is the most approachable for readers new to CGP, since it makes plain that an abstract-type component is an ordinary trait with an associated type.

## Related constructs

`#[cgp_type]` is the abstract-type specialization of [`#[cgp_component]`](cgp_component.md), inheriting its full expansion and provider-name override syntax while keying the default name off the associated type. Its central generated construct is the [`UseType`](../providers/use_type.md) provider, the type-level analogue of the [`UseField`](../providers/use_field.md) provider that [`#[cgp_getter]`](cgp_getter.md) generates. It builds on CGP's foundational [`HasType`/`TypeProvider`](../components/has_type.md) component via the generated `WithProvider` impl. Abstract-type components are wired with [`delegate_components!`](delegate_components.md) and checked with [`check_components!`](check_components.md), and they are imported into other definitions with [`#[use_type]`](../attributes/use_type.md). When an abstract type's only role is to be a getter's return type, [`#[cgp_auto_getter]`](cgp_auto_getter.md) can declare it inline instead.

## Source

The macro entry point is `cgp_type` in [crates/macros/cgp-macro-lib/src/cgp_type.rs](../../../crates/macros/cgp-macro-lib/src/cgp_type.rs), which extracts the single associated type, derives the default `{Type}TypeProvider` provider name from the associated type's identifier, runs the `#[cgp_component]` `preprocess → eval` pipeline, and converts the result into `ItemCgpType`. The logic lives in [crates/macros/cgp-macro-core/src/types/cgp_type/item.rs](../../../crates/macros/cgp-macro-core/src/types/cgp_type/item.rs), which validates the trait shape (`extract_item_type_from_trait`) and builds the `UseType` and `WithProvider` provider impls. The runtime `HasType`, `TypeProvider`, and `UseType` definitions are in [crates/core/cgp-type/src/](../../../crates/core/cgp-type/src/) (`traits/has_type.rs` and `impls/use_type.rs`). Behavioral and expansion-snapshot tests are in [crates/tests/cgp-tests/tests/abstract_types/](../../../crates/tests/cgp-tests/tests/abstract_types/) (notably `cgp_type_macro.rs`).
