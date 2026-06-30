# `HasField`

`HasField<Tag>` is the consumer trait for reading a single named field out of a context by a type-level tag, with `HasFieldMut<Tag>` adding mutable access, the provider-side mirrors `FieldGetter` and `MutFieldGetter` supplying the same capability through CGP wiring, and the lifetime helpers `MapField`/`FieldMapper` letting chained field accesses borrow correctly.

## Purpose

`HasField` exists so that a provider can demand a specific value from its context without naming the context's concrete type. The recurring problem in CGP is that a provider is generic over the context but still needs a `name`, a `port`, or some other field out of it; it cannot reach into a struct it does not know. `HasField<Tag>` solves this by keying each field with a *tag type* that stands in for the field's name, so a provider can write `Context: HasField<Symbol!("name"), Value = String>` in its `where` clause and receive the field through the trait system. This makes field access an impl-side dependency rather than part of any public interface — any context that supplies a matching field satisfies the bound automatically. See [impl-side dependencies](../../concepts/impl-side-dependencies.md) for why this constraint-based style is the heart of CGP.

The trait is deliberately tiny because it is the foundation that the value-injection macros stand on. [`#[derive(HasField)]`](../derives/derive_has_field.md) generates the impls from a struct's fields, and higher-level constructs — `#[cgp_auto_getter]`, `#[cgp_getter]` through the [`UseField`](../providers/use_field.md) provider, and `#[implicit]` arguments — all desugar into `HasField` bounds and `get_field` calls. Understanding `HasField` is understanding how every one of those reaches a field.

## Definition

`HasField<Tag>` carries the field's type as an associated `Value` and returns a reference to it, taking a `PhantomData<Tag>` argument that exists only to disambiguate which field is meant when several `HasField` impls are in scope:

```rust
pub trait HasField<Tag> {
    type Value;

    fn get_field(&self, _tag: PhantomData<Tag>) -> &Self::Value;
}
```

The `Tag` parameter is a type-level name. A named struct field is keyed by [`Symbol!("field_name")`](../macros/symbol.md), the type-level string of its identifier; a tuple field is keyed by [`Index<N>`](../types/index.md), the type-level natural number of its position. Because the tag is a type and not a value, `get_field` receives `PhantomData<Tag>` purely to tell the compiler which impl to select.

`HasFieldMut<Tag>` is the mutable extension. It supertraits `HasField<Tag>` and adds a method returning `&mut Self::Value`:

```rust
pub trait HasFieldMut<Tag>: HasField<Tag> {
    fn get_field_mut(&mut self, tag: PhantomData<Tag>) -> &mut Self::Value;
}
```

Both traits carry `#[diagnostic::on_unimplemented]` notes that point a reader at `#[derive(HasField)]` when the bound is unsatisfied, so a missing field surfaces as a readable error rather than an opaque trait failure.

The consumer side has a provider-side mirror so field access can be wired like any other component rather than only implemented directly on the context. `FieldGetter<Context, Tag>` is the provider trait corresponding to `HasField`: instead of `&self`, it takes the context as an explicit argument, which is the shape CGP providers use:

```rust
pub trait FieldGetter<Context, Tag> {
    type Value;

    fn get_field(context: &Context, _tag: PhantomData<Tag>) -> &Self::Value;
}

pub trait MutFieldGetter<Context, Tag>: FieldGetter<Context, Tag> {
    fn get_field_mut(context: &mut Context, tag: PhantomData<Tag>) -> &mut Self::Value;
}
```

Alongside these, `MapField` and `FieldMapper` add a borrow-through-a-closure variant of the same access. They exist to organize lifetime inference: chaining `context.get_field().get_field()` would otherwise force `Self::Value` to be `'static`, so `map_field` takes a `for<'a> FnOnce(&'a Self::Value) -> &'a T` closure and applies it to the borrowed field, letting the compiler infer the correct lifetime:

```rust
pub trait MapField<Tag>: HasField<Tag> {
    fn map_field<T>(
        &self,
        _tag: PhantomData<Tag>,
        mapper: impl for<'a> FnOnce(&'a Self::Value) -> &'a T,
    ) -> &T;
}

pub trait FieldMapper<Context, Tag>: FieldGetter<Context, Tag> {
    fn map_field<T>(
        context: &Context,
        _tag: PhantomData<Tag>,
        mapper: impl for<'a> FnOnce(&'a Self::Value) -> &'a T,
    ) -> &T;
}
```

## Behavior

The consumer impls of `HasField` come almost entirely from `#[derive(HasField)]`; the trait file itself provides only the blanket impls that make the access compose. The first is a `Deref` forwarding impl: when a context dereferences to a target that has the field, the context inherits it, so a `HasField` bound passes transparently through smart-pointer wrappers. This impl is marked `#[diagnostic::do_not_recommend]` so the compiler does not suggest the blanket path in error messages. `HasFieldMut` carries the analogous `DerefMut` forwarding impl, with the target additionally bounded `'static`.

The provider side is what connects field access to wiring. `UseContext` implements `FieldGetter<Context, Tag>` for any context that itself has the field, delegating straight to `context.get_field(...)`:

```rust
impl<Context, Tag, Field> FieldGetter<Context, Tag> for UseContext
where
    Context: HasField<Tag, Value = Field>,
{
    type Value = Field;
    fn get_field(context: &Context, _tag: PhantomData<Tag>) -> &Self::Value {
        context.get_field(PhantomData)
    }
}
```

`FieldMapper` is implemented as a blanket impl for any `FieldGetter` (with the getter and tag `'static`), so every provider-side getter automatically gains the lifetime-friendly `map_field`. Likewise `MapField` is a blanket impl for every `HasField` whose tag is `'static`. The split between the two sides is the standard CGP consumer/provider duality: `HasField`/`HasFieldMut` are what generic code bounds against, while `FieldGetter`/`MutFieldGetter` are what gets wired — the [`UseField`](../providers/use_field.md) provider is the wiring-side implementation that `#[cgp_getter]` targets.

## Examples

A provider that needs a value from its context expresses the need as a `HasField` bound and reads the field with `get_field`:

```rust
use cgp::prelude::*;

#[cgp_component(Greeter)]
pub trait CanGreet {
    fn greet(&self);
}

#[cgp_impl(new GreetHello)]
impl Greeter
where
    Self: HasField<Symbol!("name"), Value = String>,
{
    fn greet(&self) {
        println!("Hello, {}!", self.get_field(PhantomData));
    }
}

#[derive(HasField)]
pub struct Person {
    pub name: String,
}

delegate_components! {
    Person {
        GreeterComponent: GreetHello,
    }
}
```

Because `Person` derives `HasField`, it implements `HasField<Symbol!("name"), Value = String>`, which is exactly the bound `GreetHello` requires; the wiring type-checks and `person.greet()` prints the name. In practice the explicit bound is rarely hand-written — `#[cgp_auto_getter]`, `#[cgp_getter]`, and `#[implicit]` arguments all generate it for you on top of these impls.

## Related constructs

`HasField` is generated by [`#[derive(HasField)]`](../derives/derive_has_field.md), which emits one `HasField` and one `HasFieldMut` impl per struct field. Its tags come from [`Symbol!`](../macros/symbol.md) for named fields and [`Index<N>`](../types/index.md) for tuple fields. The provider-side [`UseField`](../providers/use_field.md) provider is the wiring-side implementation of `FieldGetter` that `#[cgp_getter]` targets, and field access in general is the canonical example of [impl-side dependencies](../../concepts/impl-side-dependencies.md). For the whole-struct structural view rather than single-field access, see [`HasFields`](has_fields.md).

## Source

The consumer traits and their blanket impls are in [crates/core/cgp-field/src/traits/has_field.rs](../../../crates/core/cgp-field/src/traits/has_field.rs) (`HasField`, `FieldGetter`, the `Deref` forwarding impl, and the `UseContext` provider impl) and [crates/core/cgp-field/src/traits/has_field_mut.rs](../../../crates/core/cgp-field/src/traits/has_field_mut.rs) (`HasFieldMut`, `MutFieldGetter`, the `DerefMut` forwarding impl). The `MapField`/`FieldMapper` lifetime helpers are in [crates/core/cgp-field/src/traits/map_field.rs](../../../crates/core/cgp-field/src/traits/map_field.rs). The `UseField` provider lives in [crates/core/cgp-field/src/impls/use_field.rs](../../../crates/core/cgp-field/src/impls/use_field.rs). Behavioral tests are in [crates/tests/cgp-tests](../../../crates/tests/cgp-tests).
