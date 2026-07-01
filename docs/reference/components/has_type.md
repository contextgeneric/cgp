# `HasType`

`HasType<Tag>` is CGP's single built-in abstract-type component: a consumer trait that gives a context an abstract type named by a tag, with `TypeProvider` as its provider trait and `TypeOf<Context, Tag>` as the alias for the resolved type.

## Purpose

`HasType` exists to let generic code refer to a type that is chosen per context without committing to a concrete one. An abstract type in CGP is a trait with a single associated type, and `HasType<Tag>` is the foundational, tag-indexed instance of that pattern: a context can carry many distinct abstract types — one per `Tag` — and resolve each to a concrete type through wiring. Generic code names `Self::Type` (or `TypeOf<Context, Tag>`), the concrete type stays hidden behind the tag, and any context that wires the tag to a type satisfies the bound. See [abstract types](../../concepts/abstract-types.md) for how this generalizes across a codebase.

What makes `HasType` special is that it is the *only* abstract-type component built into CGP, and it is the machinery that [`#[cgp_type]`](../macros/cgp_type.md) defers to. Every named abstract-type component a user defines with `#[cgp_type]` is wired on top of this one through a generated `WithProvider` impl, so the same [`UseType<T>`](../providers/use_type.md) marker that resolves `HasType` also resolves any `#[cgp_type]` component. `HasType` is the common substrate; `#[cgp_type]` is the ergonomic layer that gives each abstract type its own named trait.

## Definition

`HasType<Tag>` is declared with `#[cgp_component]`, which makes it a full component — a consumer trait paired with a generated provider trait — rather than a plain trait. Its source is the entire definition:

```rust
#[cgp_component(TypeProvider)]
#[derive_delegate(UseDelegate<Tag>)]
pub trait HasType<Tag> {
    type Type;
}

pub type TypeOf<Context, Tag> = <Context as HasType<Tag>>::Type;
```

The `Tag` parameter is the type-level name that distinguishes one abstract type from another within the same context, and `Type` is the associated type it resolves to. The `#[cgp_component(TypeProvider)]` attribute names the provider trait `TypeProvider`, so the provider-side mirror of `HasType<Tag>` is `TypeProvider<Context, Tag>` with a `type Type`. The [`#[derive_delegate(UseDelegate<Tag>)]`](../attributes/derive_delegate.md) attribute wires `UseDelegate` so a type lookup can be dispatched per tag through a delegation table. The `TypeOf<Context, Tag>` alias is the convenient spelling of the resolved type, used wherever writing `<Context as HasType<Tag>>::Type` in full would be noise.

## Behavior

Because `HasType` is a `#[cgp_component]`, it carries the standard component machinery: a consumer blanket impl that forwards `HasType<Tag>` to whatever provider the context wires for `TypeProviderComponent`, the generated `TypeProvider` provider trait, and the usual `UseContext` and `RedirectLookup` provider impls. A context obtains an abstract type either by implementing `HasType<Tag>` directly or, more commonly, by wiring `TypeProviderComponent` to a provider in `delegate_components!`.

The provider that makes wiring ergonomic is [`UseType`](../providers/use_type.md), a zero-sized marker `UseType<Type>(PhantomData<Type>)` that carries no runtime value. It implements `TypeProvider` for any context and tag by setting the abstract type to its own parameter:

```rust
impl<Context, Tag, Type> TypeProvider<Context, Tag> for UseType<Type> {
    type Type = Type;
}
```

This single impl is what lets a context name a concrete type in its wiring rather than write a bespoke provider. Wiring `TypeProviderComponent: UseType<String>` makes the context resolve `HasType<Tag>` to `Type = String` for that tag. The same `UseType<T>` is reused by every `#[cgp_type]` component: `#[cgp_type]` generates a `WithProvider` impl whose `where` clause requires the wired provider to be a `TypeProvider`, so `UseType<T>` — being a `TypeProvider` — satisfies both the built-in `HasType` and any user-defined abstract-type component at once. The [`#[use_type]`](../attributes/use_type.md) attribute is the related but distinct construct that rewrites bare type names and adds `HasType` bounds inside `#[cgp_fn]`/`#[cgp_impl]` definitions; it is an attribute, not this provider.

## Examples

A direct use defines no new component and resolves an abstract type by tag through `UseType`:

```rust
use cgp::prelude::*;

pub struct ScalarTag;

pub struct App;

delegate_components! {
    App {
        TypeProviderComponent: UseType<f64>,
    }
}

fn zero<Context>() -> TypeOf<Context, ScalarTag>
where
    Context: HasType<ScalarTag>,
    TypeOf<Context, ScalarTag>: Default,
{
    Default::default()
}
```

`App` wires `TypeProviderComponent` to `UseType<f64>`, so the `UseType` impl makes `App` implement `HasType<ScalarTag>` with `Type = f64`. In most code you would not use `HasType<Tag>` directly; you would define a named abstract type with [`#[cgp_type]`](../macros/cgp_type.md) — `#[cgp_type] trait HasScalarType { type Scalar; }` — which gives a readable `Self::Scalar` and its own provider, all resolving down to this `HasType` substrate.

## Related constructs

`HasType` is the foundation that [`#[cgp_type]`](../macros/cgp_type.md) builds every named abstract-type component on, via a generated `WithProvider` impl that adapts a `TypeProvider`. Its ergonomic provider is [`UseType`](../providers/use_type.md), the zero-sized marker that supplies a concrete type to the abstract one — not to be confused with the [`#[use_type]`](../attributes/use_type.md) attribute, which rewrites type names in definitions. The general idea of context-chosen types is covered in [abstract types](../../concepts/abstract-types.md). [`HasErrorType`](has_error_type.md) is a concrete abstract-type component defined with `#[cgp_type]` on top of this machinery.

## Source

- The trait, the `TypeProvider` provider trait, and the `TypeOf` alias are defined in [crates/core/cgp-type/src/traits/has_type.rs](../../../crates/core/cgp-type/src/traits/has_type.rs).
- The `UseType` provider and its `TypeProvider` impl are in [crates/core/cgp-type/src/impls/use_type.rs](../../../crates/core/cgp-type/src/impls/use_type.rs).
- The `#[cgp_type]` macro that builds named components on this substrate lives in [crates/macros/cgp-macro-core/src/types/cgp_type/](../../../crates/macros/cgp-macro-core/src/types/cgp_type/).
- For how it is generated and the index of tests, see the implementation document [implementation/entrypoints/cgp_type](../../implementation/entrypoints/cgp_type.md).
