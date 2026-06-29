# `UseDelegatedType`

`UseDelegatedType<Components>` is a zero-sized type provider that resolves an abstract CGP type by looking the type tag up in a delegation table, rather than fixing it to a single concrete type.

## Purpose

`UseDelegatedType` exists for the case where the concrete type an abstract type resolves to should itself be decided by a type-level table. The plain [`UseType<T>`](use_type.md) provider binds an abstract type to one fixed `T`. But sometimes a single provider must answer several abstract-type components at once, or route each type tag to a different concrete type chosen elsewhere — for instance when a preset or a higher-order provider supplies a coherent bundle of types. Hand-writing one `UseType` wiring per tag would scatter that decision; `UseDelegatedType` concentrates it into one `Components` table that the provider consults.

The mechanism is the same indirection that [`UseDelegate`](use_delegate.md) provides for behavioral components, lifted to the type level. Where `UseDelegate<Components>` dispatches a *method call* to whichever provider `Components` maps the active tag to, `UseDelegatedType<Components>` dispatches a *type resolution* to whichever concrete type `Components` maps the active tag to. It is the type-level analogue of `UseDelegate`: both read an entry out of a [`DelegateComponent`](../traits/delegate_component.md) table keyed by the tag, but one yields behavior and the other yields a type.

Like every CGP provider, `UseDelegatedType` carries no runtime value — it is a `PhantomData`-only marker named in wiring, never constructed.

## Definition

`UseDelegatedType` is a phantom-typed struct parameterized by the lookup table it consults, defined in `cgp-type`:

```rust
pub struct UseDelegatedType<Components>(pub PhantomData<Components>);

pub type WithDelegatedType<Components> = WithProvider<UseDelegatedType<Components>>;
```

The `Components` parameter is a type that implements [`DelegateComponent`](../traits/delegate_component.md) for each type tag the provider must answer — the same kind of type-level key-value map that `delegate_components!` builds. The `WithDelegatedType<Components>` alias wraps the provider in [`WithProvider`](with_provider.md), so a user-defined [`#[cgp_type]`](../macros/cgp_type.md) component (whose generated `WithProvider` impl forwards to any `TypeProvider`) can be backed by a delegated lookup as well as the built-in [`HasType`](../components/has_type.md) component. Neither `UseDelegatedType` nor `WithDelegatedType` is re-exported through `cgp::prelude`; reach them through `cgp::core::types`.

## Behavior

`UseDelegatedType<Components>` implements [`TypeProvider`](../components/has_type.md) by looking the type tag `Tag` up in `Components` and reporting the delegate it finds as the abstract type:

```rust
#[cgp_provider(TypeProviderComponent)]
impl<Context, Tag, Components, Type> TypeProvider<Context, Tag> for UseDelegatedType<Components>
where
    Components: DelegateComponent<Tag, Delegate = Type>,
{
    type Type = Type;
}
```

The `where` clause is the whole of the behavior: `Components: DelegateComponent<Tag, Delegate = Type>` reads the entry stored at key `Tag` in the `Components` table, and the impl sets the abstract `Type` to that delegate. Because the lookup is keyed by `Tag`, one `UseDelegatedType<Components>` provider answers as many distinct type tags as `Components` has entries, each resolving to its own concrete type. If `Components` has no entry for a given tag, the `DelegateComponent` bound is unsatisfied and the context simply does not implement `HasType` for that tag — the missing-entry diagnostic from `DelegateComponent` surfaces the gap.

Contrast this with `UseType<T>`, whose `TypeProvider` impl is unconditional and always reports the single type `T`. `UseDelegatedType` adds exactly one level of indirection — the `DelegateComponent` lookup — so the concrete type comes from the table instead of from the provider's own parameter.

## Examples

A typical use defines a lookup table mapping type tags to concrete types and wires a context's type component to `UseDelegatedType` over that table. The table is an ordinary type carrying `DelegateComponent` entries:

```rust
use cgp::prelude::*;
use cgp::core::types::UseDelegatedType; // not re-exported through the prelude

#[cgp_type]
pub trait HasScalarType {
    type Scalar;
}

#[cgp_type]
pub trait HasIndexType {
    type Index;
}

pub struct App;
pub struct AppTypes;

delegate_components! {
    AppTypes {
        ScalarTypeProviderComponent: f64,
        IndexTypeProviderComponent: usize,
    }
}

delegate_components! {
    App {
        [
            ScalarTypeProviderComponent,
            IndexTypeProviderComponent,
        ]: UseDelegatedType<AppTypes>,
    }
}
```

`App` routes both its scalar and index type components through `UseDelegatedType<AppTypes>`. When the wiring asks for `App`'s `Scalar`, the provider looks `ScalarTypeProviderComponent` up in `AppTypes` and finds `f64`; for `Index` it finds `usize`. A single provider entry on `App` thus answers two abstract types, with the concrete choices held in one place in `AppTypes`.

This is what makes `UseDelegatedType` valuable for bundling: the set of concrete types lives in the `AppTypes` table and can be reused, swapped, or supplied by a preset, while each context only points its type components at the table.

## Related constructs

`UseDelegatedType` is the type-level counterpart of [`UseDelegate`](use_delegate.md), which performs the same `DelegateComponent` lookup for behavioral (method) components. It resolves through the [`DelegateComponent`](../traits/delegate_component.md) trait, the type-level key-value map that `delegate_components!` populates, and implements the [`HasType` / `TypeProvider`](../components/has_type.md) component it answers for. Its sibling [`UseType`](use_type.md) is the simpler provider that fixes an abstract type to one concrete type without a lookup. Its `WithDelegatedType` alias is one of the named wrappers around [`WithProvider`](with_provider.md), used to back a [`#[cgp_type]`](../macros/cgp_type.md) component with a delegated lookup.

## Source

The `UseDelegatedType` struct, its `WithDelegatedType` alias, and the `TypeProvider` impl are in [crates/core/cgp-type/src/impls/use_delegated_type.rs](../../../crates/core/cgp-type/src/impls/use_delegated_type.rs). The `HasType` consumer trait and `TypeProvider` provider trait are in [crates/core/cgp-type/src/traits/has_type.rs](../../../crates/core/cgp-type/src/traits/has_type.rs), and `DelegateComponent` is in [crates/core/cgp-component/src/traits/delegate_component.rs](../../../crates/core/cgp-component/src/traits/delegate_component.rs).
