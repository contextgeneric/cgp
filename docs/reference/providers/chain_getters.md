# `ChainGetters`

`ChainGetters<Getters>` is a zero-sized provider that composes a list of field getters into one, navigating from an outer context through a sequence of intermediate values to reach a deeply nested field.

## Purpose

`ChainGetters` exists to reach a field that does not live directly on the context but several hops inside it. A single [`UseField`](use_field.md) reads one field of one context. But CGP contexts often nest — a context holds a config, the config holds a connection, the connection holds a timeout — and a getter may need the innermost value. Writing one provider that walks the whole path by hand is tedious and couples the getter to the nesting. `ChainGetters<Getters>` takes a list of getters, applies them in order, and threads the reference from each step into the next, so the chain reads like the path it traverses: outer getter, then the next, then the next, ending at the target field.

The list is a type-level [`Cons`](../types/cons.md) spine — `Cons<GetterA, Cons<GetterB, Nil>>` — where each element is itself a field getter for the value the previous step produced. `ChainGetters` recurses down this list, so it is naturally written with the [`Product!`](../macros/product.md) macro that builds such spines. Like every CGP provider, `ChainGetters<Getters>` carries no runtime value; it is a `PhantomData`-only marker named in wiring.

## Definition

`ChainGetters` is a phantom-typed struct parameterized by the list of getters it chains, defined in `cgp-field`:

```rust
pub struct ChainGetters<Getters>(pub PhantomData<Getters>);
```

`Getters` is a [`Cons`](../types/cons.md)/`Nil` list whose elements are field getters. The provider has no `With...` alias; it is used directly in wiring as the provider for a getter component, or composed inside other getter wiring. `ChainGetters` is not re-exported through `cgp::prelude`; reach it through `cgp::core::field::impls`.

## Implementations

`ChainGetters` implements the provider-side getter [`FieldGetter`](../traits/has_field.md) with two impls that together perform the recursion over the list — one for a non-empty `Cons` and one for the empty `Nil`. The `Cons` impl applies the head getter, then delegates the remainder of the path to `ChainGetters` over the tail:

```rust
impl<Context, Tag, Getter, RestGetters, ValueA, ValueB> FieldGetter<Context, Tag>
    for ChainGetters<Cons<Getter, RestGetters>>
where
    Getter: FieldMapper<Context, Tag, Value = ValueA>,
    ChainGetters<RestGetters>: FieldGetter<ValueA, Tag, Value = ValueB>,
{
    type Value = ValueB;

    fn get_field(context: &Context, tag: PhantomData<Tag>) -> &ValueB {
        Getter::map_field(context, tag, |value| {
            <ChainGetters<RestGetters>>::get_field(value, tag)
        })
    }
}
```

The head `Getter` reads `ValueA` from the `Context`, and the rest of the chain — `ChainGetters<RestGetters>` — reads `ValueB` from that `ValueA`, so the whole chain's `Value` is `ValueB`, the value at the end of the path. The head is applied through [`FieldMapper`](../traits/has_field.md) rather than `FieldGetter` directly: `map_field` hands the intermediate `ValueA` reference to a closure that runs the rest of the chain on it, which is the construct that keeps the borrowed lifetimes inferring correctly across each hop. Note that every step is asked under the same `Tag` — the chain navigates *contexts*, not different field names per step; each getter in the list decides for itself which field of its input it reads.

The recursion bottoms out at the empty list, where `ChainGetters<Nil>` is the identity getter — it returns the context it was given:

```rust
impl<Context, Tag> FieldGetter<Context, Tag> for ChainGetters<Nil> {
    type Value = Context;

    fn get_field(context: &Context, _tag: PhantomData<Tag>) -> &Context {
        context
    }
}
```

So a chain of one getter resolves to that getter applied to the context (its tail being `Nil` returns the value unchanged), a chain of two applies the first then the second, and so on down the `Cons` spine.

## Examples

A typical use reaches a field on a nested inner context by chaining the getter that produces the inner context with the getter that reads the field. Here an `App` holds a `Config`, and the target is the `Config`'s field:

```rust
use cgp::prelude::*;
use cgp::core::field::impls::ChainGetters; // not re-exported through the prelude

#[cgp_getter]
pub trait HasPort {
    fn port(&self) -> &u16;
}

#[derive(HasField)]
pub struct Config {
    pub port: u16,
}

#[derive(HasField)]
pub struct App {
    pub config: Config,
}

delegate_components! {
    App {
        PortGetterComponent: ChainGetters<
            Product![
                UseField<Symbol!("config")>,
                UseField<Symbol!("port")>,
            ],
        >,
    }
}
```

`App` wires `PortGetterComponent` to `ChainGetters` over a two-element list. The first getter, `UseField<Symbol!("config")>`, reads `App`'s `config` field to produce a `&Config`; the second, `UseField<Symbol!("port")>`, reads that `Config`'s `port` field to produce the `&u16`. `ChainGetters` threads the reference from the first step into the second, so `app.port()` returns the port nested two levels in, without any hand-written walking code.

## Related constructs

`ChainGetters` composes the same provider-side [`FieldGetter`](../traits/has_field.md) that [`UseField`](use_field.md) and [`UseFieldRef`](use_field_ref.md) implement, using each as a step in the path; it applies each step through [`FieldMapper`](../traits/has_field.md) to keep borrowed lifetimes inferring across hops. Its list of getters is a type-level [`Cons`](../types/cons.md)/`Nil` spine, most conveniently written with the [`Product!`](../macros/product.md) macro, and its steps read fields keyed by [`Symbol!`](../macros/symbol.md) on contexts deriving [`#[derive(HasField)]`](../derives/derive_has_field.md). It is wired as the provider for a getter defined with [`#[cgp_getter]`](../macros/cgp_getter.md).

## Source

The `ChainGetters` struct and its two `FieldGetter` impls (the `Cons` recursion step and the `Nil` base case) are in [crates/core/cgp-field/src/impls/chain.rs](../../../crates/core/cgp-field/src/impls/chain.rs). The `FieldGetter` and `FieldMapper` traits it builds on are in [crates/core/cgp-field/src/traits/has_field.rs](../../../crates/core/cgp-field/src/traits/has_field.rs) and [crates/core/cgp-field/src/traits/map_field.rs](../../../crates/core/cgp-field/src/traits/map_field.rs), and the `Cons`/`Nil` spine it recurses over is defined in [crates/core/cgp-base-types/src/types/](../../../crates/core/cgp-base-types/src/types/) (`cons.rs` and `nil.rs`).
