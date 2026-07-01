# `UseFieldRef`

`UseFieldRef<Tag, Value>` is a zero-sized provider that implements a getter component by reading a field named by `Tag` and then dereferencing it through `AsRef`/`AsMut`, so the getter exposes `&Value` rather than the field's own type.

## Purpose

`UseFieldRef` exists for getters whose return type is reached *through* a field rather than being the field itself. The plain [`UseField`](use_field.md) provider returns a reference to a field exactly as stored: a `String` field yields `&String`. But a getter often wants to expose a borrowed view — `&str` from a `String`, `&[T]` from a `Vec<T>`, `&Path` from a `PathBuf` — where the stored type implements `AsRef<Value>` for the desired `Value`. `UseFieldRef<Tag, Value>` carries both the field tag and the target `Value`, reads the field, and calls `as_ref()` to produce the view, so the getter's signature can name the borrowed type while the context stores the owning type.

This complements `UseField` rather than replacing it. `UseField<Tag>` is the right provider when the getter returns the field's type directly; `UseFieldRef<Tag, Value>` is the right one when the getter returns a reference obtained by `AsRef` from the field. Both decouple the field name from the method name through `Tag`; `UseFieldRef` additionally decouples the *exposed type* from the *stored type* through `Value`. As with every CGP provider, `UseFieldRef<Tag, Value>` carries no runtime value — it is a `PhantomData`-only marker named in wiring.

## Definition

`UseFieldRef` is a phantom-typed struct parameterized by the field tag and the borrowed value type, defined in `cgp-field`:

```rust
pub struct UseFieldRef<Tag, Value>(pub PhantomData<(Tag, Value)>);

pub type WithFieldRef<Tag, Value> = WithProvider<UseFieldRef<Tag, Value>>;
```

`Tag` names the field, as in [`UseField`](use_field.md), and `Value` is the type the getter exposes — the type the stored field can be borrowed as via `AsRef<Value>`. The `WithFieldRef<Tag, Value>` alias wraps the provider in [`WithProvider`](with_provider.md), so a getter component can be backed by a ref-style field accessor through the macro-generated `WithProvider` impl. Unlike the more common [`UseField`](use_field.md), neither `UseFieldRef` nor `WithFieldRef` is re-exported through `cgp::prelude`; reach them through `cgp::core::field::impls`.

## Implementations

`UseFieldRef<Tag, Value>` implements the provider-side getter [`FieldGetter`](../traits/has_field.md) by reading the field at `Tag` and dereferencing it to `&Value`:

```rust
impl<Context, OutTag, Tag, Value> FieldGetter<Context, OutTag> for UseFieldRef<Tag, Value>
where
    Context: HasField<Tag, Value: AsRef<Value> + 'static>,
{
    type Value = Value;

    fn get_field(context: &Context, _tag: PhantomData<OutTag>) -> &Value {
        context.get_field(PhantomData).as_ref()
    }
}
```

The `where` clause carries the defining constraint: the context's field at `Tag` must implement `AsRef<Value>`, so the stored type can be borrowed as the exposed type. As in `UseField`, `OutTag` is the tag the component asks under and is ignored; the field is read at `Tag`. The body reads the field and calls `as_ref()`, so a `String` field exposed with `Value = str` yields `&str`. The `'static` bound on the field type lets Rust infer the borrow's lifetime through the `AsRef` call.

`UseFieldRef` also implements the mutable getter [`MutFieldGetter`](../traits/has_field.md), requiring the field type to implement both `AsRef<Value>` and `AsMut<Value>` and returning `&mut Value` via `as_mut()`. Unlike `UseField`, `UseFieldRef` does not implement `TypeProvider`, because its purpose is borrowed field access rather than abstract-type resolution.

## Examples

A typical use exposes a `&str` getter over a context that stores the name as a `String`, with the getter's signature naming the borrowed type. The component returns `&str`, while `Person` stores a `String`:

```rust
use cgp::prelude::*;
use cgp::core::field::impls::UseFieldRef; // not re-exported through the prelude

#[cgp_getter]
pub trait HasName {
    fn name(&self) -> &str;
}

#[derive(HasField)]
pub struct Person {
    pub name: String,
}

delegate_components! {
    Person {
        NameGetterComponent: UseFieldRef<Symbol!("name"), str>,
    }
}
```

`Person` wires `NameGetterComponent` to `UseFieldRef<Symbol!("name"), str>`. The `FieldGetter` impl reads the `name` field — a `String` — and, because `String: AsRef<str>`, returns `&str` from `as_ref()`. The getter exposes the borrowed `str` view while the context owns the `String`.

The same binding can be written with the `WithFieldRef` alias, which routes through `WithProvider`:

```rust
delegate_components! {
    Person {
        NameGetterComponent: WithFieldRef<Symbol!("name"), str>,
    }
}
```

## Related constructs

`UseFieldRef` is the borrow-through-`AsRef` variant of [`UseField`](use_field.md): both read a field named by `Tag` and implement the provider-side [`FieldGetter` / `MutFieldGetter`](../traits/has_field.md) traits over [`HasField`](../traits/has_field.md), but `UseFieldRef` adds a `Value` parameter and exposes `&Value` via `AsRef`/`AsMut` instead of the field's own type. It reads fields produced by [`#[derive(HasField)]`](../derives/derive_has_field.md), keyed by tags from [`Symbol!`](../macros/symbol.md) or `Index<N>`, and is wired to a getter defined with [`#[cgp_getter]`](../macros/cgp_getter.md). Its `WithFieldRef` alias is one of the named wrappers around [`WithProvider`](with_provider.md). For chaining getters across nested contexts, see [`ChainGetters`](chain_getters.md).

## Source

- The `UseFieldRef` struct, its `WithFieldRef` alias, and the `FieldGetter` and `MutFieldGetter` impls are in [crates/core/cgp-field/src/impls/use_ref.rs](../../../crates/core/cgp-field/src/impls/use_ref.rs).
- The `HasField` and `FieldGetter` traits are in [crates/core/cgp-field/src/traits/has_field.rs](../../../crates/core/cgp-field/src/traits/has_field.rs), and `HasFieldMut`/`MutFieldGetter` are in [crates/core/cgp-field/src/traits/has_field_mut.rs](../../../crates/core/cgp-field/src/traits/has_field_mut.rs).
