# `WithProvider<Provider>`

`WithProvider<Provider>` is a zero-sized adapter provider that turns a foundational provider — one implementing `TypeProvider` or `FieldGetter` — into a provider of a specific CGP component.

## Purpose

`WithProvider` bridges the gap between CGP's two layers of provider trait. Foundational traits like [`TypeProvider`](../components/has_type.md) and [`FieldGetter`](../traits/has_field.md) are generic, component-agnostic mechanisms: a `TypeProvider` supplies *some* abstract type for *some* tag, and a `FieldGetter` reads *some* field for *some* output tag, without either knowing which named component it serves. A CGP component, by contrast, has a specific provider trait — `NameTypeProvider`, `NameGetter` — that a context wires to. `WithProvider<Provider>` is the adapter that lets a foundational provider stand in as the provider for one of these named components: it implements the component's provider trait by forwarding to the foundational provider's method.

This adapter is what lets the foundational layer be wired without each foundational provider having to implement every component trait by hand. A field getter implemented once as a `FieldGetter` can serve any number of getter components through `WithProvider`; an abstract-type implementation written once as a `TypeProvider` can serve any type component the same way. The component-specific glue is generated, and `WithProvider` is the type that carries it.

`WithProvider` is rarely written in full by users, because its common uses are packaged as aliases. The family `WithContext`, `WithType`, `WithField`, `WithFieldRef`, and `WithDelegatedType` are all `WithProvider<...>` specialized to a particular inner provider, and these aliases are what appears in everyday wiring. Understanding `WithProvider` is what explains why those aliases work.

Like every CGP provider, `WithProvider` carries no runtime value. The `Provider` type parameter is held in `PhantomData`, and the struct exists only as a type-level marker naming the foundational provider to adapt.

## Definition

`WithProvider` is a struct parameterized by the inner provider, defined in `cgp-component`:

```rust
pub struct WithProvider<Provider>(pub PhantomData<Provider>);
```

The single type parameter `Provider` is the foundational provider being adapted — typically a [`TypeProvider`](../components/has_type.md) or [`FieldGetter`](../traits/has_field.md). The `PhantomData` makes `Provider` a parameter of a valueless struct; nothing of `Provider` is ever constructed.

## Behavior

`#[cgp_type]` and `#[cgp_getter]` generate a `WithProvider` impl that forwards a component's provider-trait method to the inner provider's foundational method. For a type component such as

```rust
#[cgp_type]
pub trait HasNameType {
    type Name;
}
```

`#[cgp_type]` emits a `WithProvider` impl that defines the component's associated type from the inner `TypeProvider` (shown with the macro's real placeholder identifiers):

```rust
impl<__Provider__, Name, __Context__> NameTypeProvider<__Context__> for WithProvider<__Provider__>
where
    __Provider__: TypeProvider<__Context__, NameTypeProviderComponent, Type = Name>,
{
    type Name = Name;
}
```

For a single-method getter, `#[cgp_getter]` emits an analogous `WithProvider` impl that reads the value through the inner `FieldGetter`:

```rust
impl<__Context__, __Provider__> NameGetter<__Context__> for WithProvider<__Provider__>
where
    __Provider__: FieldGetter<__Context__, NameGetterComponent, Value = String>,
{
    fn name(__context__: &__Context__) -> &str {
        __Provider__::get_field(__context__, PhantomData::<NameGetterComponent>).as_str()
    }
}
```

In both cases the bound names the foundational trait — `TypeProvider` or `FieldGetter` — keyed by the component-name struct, and the method or associated type forwards to it. `#[cgp_getter]` generates the `WithProvider` impl only when the getter trait has exactly one method, since a single foundational getter cannot serve several methods at once. Each impl is paired with a matching `IsProviderFor` impl so dependencies reach the [check traits](../../concepts/check-traits.md).

The aliases specialize `WithProvider` to a fixed inner provider so the common cases need no `WithProvider<...>` spelled out. `WithContext = WithProvider<UseContext>` adapts the context's own consumer-trait implementation; `WithType<Type> = WithProvider<UseType<Type>>` and `WithField<Tag> = WithProvider<UseField<Tag>>` adapt the foundational type and field providers; `WithFieldRef<Tag, Value> = WithProvider<UseFieldRef<Tag, Value>>` adapts a getter that returns a reference borrowed through `AsRef`; and `WithDelegatedType<Components> = WithProvider<UseDelegatedType<Components>>` adapts a type provider that looks its type up in a delegation table. `WithContext` lives in `cgp-component` beside `WithProvider`, while the `WithType`/`WithDelegatedType` pair is defined in `cgp-type` and the `WithField`/`WithFieldRef` pair in `cgp-field`, each next to the inner provider it wraps.

## Examples

The everyday way to use `WithProvider` is through one of its aliases, which read as a single wiring choice. Adapting the context's own field getter into a getter component uses `WithField`:

```rust
use cgp::prelude::*;

#[cgp_getter]
pub trait HasName {
    fn name(&self) -> &str;
}

#[derive(HasField)]
pub struct Person {
    pub first_name: String,
}

delegate_components! {
    Person {
        NameGetterComponent: WithField<Symbol!("first_name")>,
    }
}
```

`WithField<Symbol!("first_name")>` expands to `WithProvider<UseField<Symbol!("first_name")>>`, so `Person`'s `NameGetter` provider is the adapter wrapping the foundational `UseField` getter for the `first_name` field. The generated `WithProvider` impl forwards `name()` to `UseField`'s `FieldGetter::get_field`, which reads `first_name`. The same shape recurs for abstract types: wiring a type component to `WithType<String>` adapts `UseType<String>`, and to `WithDelegatedType<SomeTable>` adapts a `UseDelegatedType` that resolves the type through a table.

## Related constructs

`WithProvider`'s impls are generated by [`#[cgp_type]`](../macros/cgp_type.md) and [`#[cgp_getter]`](../macros/cgp_getter.md), adapting the foundational [`TypeProvider`](../components/has_type.md) and [`FieldGetter`](../traits/has_field.md) traits into component providers. Its alias family wraps the foundational providers documented separately: [`UseContext`](use_context.md) via `WithContext`, [`UseType`](use_type.md) via `WithType`, [`UseField`](use_field.md) via `WithField`, [`UseFieldRef`](use_field_ref.md) via `WithFieldRef`, and [`UseDelegatedType`](use_delegated_type.md) via `WithDelegatedType`. Aliases are wired with [`delegate_components!`](../macros/delegate_components.md), and the dependency propagation that makes them checkable flows through [`IsProviderFor`](../traits/is_provider_for.md).

## Source

The struct is defined in [crates/core/cgp-component/src/providers/with_provider.rs](../../../crates/core/cgp-component/src/providers/with_provider.rs), and the `WithContext` alias in [crates/core/cgp-component/src/providers/use_context.rs](../../../crates/core/cgp-component/src/providers/use_context.rs). The remaining aliases are defined beside their inner providers: `WithType` and `WithDelegatedType` in [crates/core/cgp-type/src/impls/use_type.rs](../../../crates/core/cgp-type/src/impls/use_type.rs) and [use_delegated_type.rs](../../../crates/core/cgp-type/src/impls/use_delegated_type.rs), and `WithField` and `WithFieldRef` in [crates/core/cgp-field/src/impls/use_field.rs](../../../crates/core/cgp-field/src/impls/use_field.rs) and [use_ref.rs](../../../crates/core/cgp-field/src/impls/use_ref.rs). The component `WithProvider` impls are generated by `#[cgp_type]` in [crates/macros/cgp-macro-core/src/types/cgp_type/item.rs](../../../crates/macros/cgp-macro-core/src/types/cgp_type/item.rs) and by `#[cgp_getter]` in [crates/macros/cgp-macro-core/src/types/cgp_getter/with_provider.rs](../../../crates/macros/cgp-macro-core/src/types/cgp_getter/with_provider.rs). The generated getter `WithProvider` impl is shown in the expansion snapshot in [crates/tests/cgp-tests/tests/getter_tests/string.rs](../../../crates/tests/cgp-tests/tests/getter_tests/string.rs).
