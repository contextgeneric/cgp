# `UseField`

`UseField<Tag>` is a zero-sized provider that implements a getter component by reading a field named by `Tag` from the context through [`HasField`](../traits/has_field.md), letting the field name differ from the getter method name.

## Purpose

`UseField` exists to decouple a getter's method name from the field it reads. A getter component defined with [`#[cgp_getter]`](../macros/cgp_getter.md) describes a value the context can supply — `fn name(&self) -> &str` — but the context may store that value under a different field, say `first_name`, or different contexts may store it under different names. `UseField<Tag>` carries the field name as its type parameter, so wiring a context's getter component to `UseField<Symbol!("first_name")>` makes the getter read `first_name` even though the method is `name`. The field name lives in the wiring, not in the trait.

This is the provider that [`#[cgp_getter]`](../macros/cgp_getter.md) targets. When a getter trait has a single method, `#[cgp_getter]` generates a `UseField` impl for the getter's provider trait with the field tag left as a free parameter, so a context picks the field by writing `UseField<Symbol!("...")>` in its delegation table. `UseField` itself is the general-purpose provider underneath that pattern: it works for any tag the context's [`HasField`](../traits/has_field.md) impl supports.

The `Tag` is usually a type-level string built with [`Symbol!`](../macros/symbol.md), such as `Symbol!("name")`, or a type-level integer wrapped in `Index<N>` for tuple fields — these are exactly the tags that [`#[derive(HasField)]`](../derives/derive_has_field.md) generates `HasField` impls for. Any other type works as a tag too, but then the context must supply the matching `HasField` impl itself. As with every CGP provider, `UseField<Tag>` carries no runtime value; it is a `PhantomData`-only marker named in wiring.

## Definition

`UseField` is a phantom-typed struct parameterized by the field tag, defined in `cgp-field`:

```rust
pub struct UseField<Tag>(pub PhantomData<Tag>);

pub type WithField<Tag> = WithProvider<UseField<Tag>>;
```

The `WithField<Tag>` alias wraps `UseField<Tag>` in [`WithProvider`](with_provider.md), the adapter that lets a generic field getter back a specific getter component. The `#[cgp_getter]` macro generates a `WithProvider` impl for its component, so a getter can be wired with either `UseField<Symbol!("...")>` (through the macro's own generated `UseField` impl) or `WithField<Symbol!("...")>` (through `WithProvider`).

## Implementations

`UseField<Tag>` implements three provider traits, each forwarding to the context's [`HasField`](../traits/has_field.md) impl for `Tag`. The central one is the provider-side getter, [`FieldGetter`](../traits/has_field.md), which reads the field by reference:

```rust
impl<Context, OutTag, Tag, Value> FieldGetter<Context, OutTag> for UseField<Tag>
where
    Context: HasField<Tag, Value = Value>,
{
    type Value = Value;

    fn get_field(context: &Context, _tag: PhantomData<OutTag>) -> &Value {
        context.get_field(PhantomData)
    }
}
```

Two tags appear here for a reason. `OutTag` is the tag the *component* asks under (the getter component's name), while `Tag` is the *field* tag the provider was parameterized with. The impl ignores `OutTag` entirely and reads `Tag` from the context, which is precisely the decoupling: the component's identity and the field name are independent. The associated `Value` is taken from the context's `HasField<Tag>` impl, so the returned reference is to the real field.

`UseField<Tag>` also implements the mutable getter [`MutFieldGetter`](../traits/has_field.md) the same way, requiring `Context: HasFieldMut<Tag>` and returning `&mut Value` via `get_field_mut`. And it implements [`TypeProvider`](../components/has_type.md), reporting the field's `Value` type as an abstract type — so the *type* of a field can itself be wired as a context's abstract type. Each impl is paired with an `IsProviderFor` impl carrying the same `HasField` bound, so delegation propagates the dependency and check traits report a missing field precisely.

## Examples

The defining use wires a getter to a field whose name differs from the method, which is the case the simpler [`#[cgp_auto_getter]`](../macros/cgp_auto_getter.md) cannot express. The trait method is `name`, but the context stores the value in `first_name`:

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
        NameGetterComponent: UseField<Symbol!("first_name")>,
    }
}

fn greet(person: &Person) {
    println!("Hello, {}!", person.name()); // reads the first_name field
}
```

`Person` wires `NameGetterComponent` to `UseField<Symbol!("first_name")>`. The generated getter resolves through the `FieldGetter` impl above, whose `Tag` is `Symbol!("first_name")`, so `person.name()` reads `Person`'s `first_name` field — the method name and the field name diverge, with the field name supplied entirely by the wiring.

The same binding can be written with the `WithField` alias, which routes through `WithProvider`:

```rust
delegate_components! {
    Person {
        NameGetterComponent: WithField<Symbol!("first_name")>,
    }
}
```

Both forms read `first_name`; `UseField` is the idiomatic choice for binding a getter to a named field without hand-writing a provider.

## Related constructs

`UseField` is the provider that [`#[cgp_getter]`](../macros/cgp_getter.md) generates an impl for, the mechanism that lets a getter's field name differ from its method name. It implements the provider-side [`FieldGetter` / `MutFieldGetter`](../traits/has_field.md) traits by reading the consumer-side [`HasField`](../traits/has_field.md) impl that [`#[derive(HasField)]`](../derives/derive_has_field.md) produces, keyed by tags built with [`Symbol!`](../macros/symbol.md) or `Index<N>`. Its `WithField` alias is one of the named wrappers around [`WithProvider`](with_provider.md). It is the field-level analogue of the [`UseType`](use_type.md) provider that [`#[cgp_type]`](../macros/cgp_type.md) generates for abstract types. For a getter whose value is reached through `AsRef`/`AsMut` rather than read directly, see [`UseFieldRef`](use_field_ref.md); for chaining getters across nested contexts, see [`ChainGetters`](chain_getters.md).

## Source

The `UseField` struct, its `WithField` alias, and the `FieldGetter`, `MutFieldGetter`, and `TypeProvider` impls are in [crates/core/cgp-field/src/impls/use_field.rs](../../../crates/core/cgp-field/src/impls/use_field.rs). The `HasField`, `FieldGetter`, and (in `has_field_mut.rs`) `HasFieldMut`, `MutFieldGetter` traits are in [crates/core/cgp-field/src/traits/](../../../crates/core/cgp-field/src/traits/). The `#[cgp_getter]`-generated `UseField` impl is built in [crates/macros/cgp-macro-core/src/types/cgp_getter/use_field.rs](../../../crates/macros/cgp-macro-core/src/types/cgp_getter/use_field.rs). Behavioral and expansion tests are in [crates/tests/cgp-tests/tests/getter.rs](../../../crates/tests/cgp-tests/tests/getter.rs) and the `getter_tests/` modules beside it.
