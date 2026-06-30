# Modularity hierarchy

CGP and vanilla Rust together form a ladder of modularity: each rung allows strictly more independent implementations of one interface than the rung below, by loosening a coherence constraint at a matching cost in syntax or coupling, so the right rung is the lowest one that still expresses what a use case needs.

## A ladder, not a switch

Adopting CGP is not an all-or-nothing jump from ordinary Rust traits to fully context-generic code. The same capability — say, serializing a value — can be expressed at several levels of modularity, and the levels form a gradient from "exactly one implementation, no wiring" to "any number of implementations, wired per type per provider." Each step up admits more overlapping or orphan implementations that vanilla Rust's [coherence](coherence.md) rules would reject, and each step costs something: more boilerplate, a changed interface, or tighter coupling between providers. Reading the rungs in order shows what each technique buys and what it asks for, so a use case can settle at the lowest rung that still works rather than reaching for the most powerful tool by default.

The ladder is illustrated throughout with one running capability — serializing a value, mirroring the [modular serialization](../examples/modular-serialization.md) example — so the only thing that changes between rungs is the modularity technique, not the problem. All snippets assume `use cgp::prelude::*;`.

## Rung 1 — one implementation per interface

The least modular rung is a generic function or a blanket trait impl, which allows exactly one implementation of the interface it defines. A blanket impl over a generic type captures a single piece of logic that applies everywhere the bound holds, and there can be only one such impl:

```rust
pub trait CanSerializeBytes {
    fn serialize_bytes<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error>;
}

impl<Value: AsRef<[u8]>> CanSerializeBytes for Value {
    fn serialize_bytes<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_bytes(self.as_ref())
    }
}
```

A blanket trait is preferred over a bare generic function because it hides the `AsRef<[u8]>` bound behind a clean interface rather than leaking it to every transitive caller — the [impl-side dependency](impl-side-dependencies.md) idea in its simplest form. The limitation is absolute, though: this is the *only* way `CanSerializeBytes` is ever implemented. There is no room for a second strategy, so this rung fits a capability that genuinely has one implementation for all types, and nothing more.

## Rung 2 — one implementation per type

Vanilla Rust traits climb one rung by allowing a different implementation for each type, while coherence still permits at most one implementation per type. This is the everyday Rust trait, where `Vec<u8>` and `&[u8]` can each implement `Serialize` their own way:

```rust
impl Serialize for Vec<u8> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_bytes(self.as_ref())
    }
}

impl<'a> Serialize for &'a [u8] {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_bytes(self)
    }
}
```

The gain over rung 1 is per-type variation; the cost is that each type needs its own explicit impl even when several share logic, and the [overlap rule](coherence.md) forbids any blanket impl that would collide. Reusable building blocks can still be factored out — both bodies above could call the rung-1 `serialize_bytes` — but the trait itself admits no alternatives: once `Serialize for Vec<u8>` is chosen, that choice is global and final. This is where Rust's coherence guarantee delivers its value and also where it starts to bind: a type gets exactly one implementation of a trait, no matter what a particular application would prefer.

## Rung 3 — many implementations, one wiring per type

The first CGP rung keeps the type in the `Self` position but splits the trait into a consumer/provider pair, so many overlapping implementations can coexist as named providers while each type still commits to one of them globally. Applying [`#[cgp_component]`](../reference/macros/cgp_component.md) to the trait and writing providers with [`#[cgp_impl]`](../reference/macros/cgp_impl.md) lets `SerializeBytes` and a `Serialize`-deferring `UseSerde` both exist, overlapping freely on any type that is both `AsRef<[u8]>` and `Serialize`:

```rust
#[cgp_component(ValueSerializer)]
pub trait CanSerialize {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error>;
}

#[cgp_impl(new SerializeBytes)]
impl ValueSerializer
where
    Self: AsRef<[u8]>,
{
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_bytes(self.as_ref())
    }
}
```

A type then picks one provider with a [`delegate_components!`](../reference/macros/delegate_components.md) entry — `Vec<u8>` becomes its own context, wiring its serializer component to `SerializeBytes`:

```rust
delegate_components! {
    Vec<u8> {
        ValueSerializerComponent: SerializeBytes,
    }
}
```

This rung's advantage is backward compatibility: the original trait is extended without changing its interface, a type can still implement it directly, and many reusable providers replace the hand-copied logic of rung 2. Its limitation is that coherence is only partly lifted. The wiring still keys on the type in the `Self` position, so `Vec<u8>` commits to one provider globally — there can be no separate wiring for a generic `Vec<T>` that would overlap it, and the [orphan rule](coherence.md) still applies, since `delegate_components!` for `Vec<u8>` must live in a crate that owns either the trait or `Vec`. This rung suits retrofitting modular providers onto an existing trait when one global choice per type is acceptable.

## Rung 4 — many implementations, one wiring per type per context

The decisive CGP rung moves the type being implemented out of `Self` and into an explicit parameter, so the `Self` position names a context that owns the wiring — which lifts the orphan rule and lets each context choose providers per type independently. The trait gains a `Value` parameter, leaving `Self` free to be any application context:

```rust
#[cgp_component(ValueSerializer)]
pub trait CanSerializeValue<Value: ?Sized> {
    fn serialize<S>(&self, value: &Value, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer;
}
```

Now two application contexts can serialize the *same* type differently, each coherent within itself, by opening the component and keying on the value type — `AppA` encoding `Vec<u8>` as hexadecimal where `AppB` uses base64:

```rust
delegate_components! {
    AppA {
        open {ValueSerializerComponent};
        @ValueSerializerComponent.Vec<u8>: SerializeHex,
    }
}

delegate_components! {
    AppB {
        open {ValueSerializerComponent};
        @ValueSerializerComponent.Vec<u8>: SerializeBase64,
    }
}
```

This rung nearly eliminates the coherence restrictions. Because the wiring keys on the context rather than on `Vec<u8>`, a crate that owns neither the trait nor `Vec` can still wire a serializer for `Vec<u8>` as long as it owns the context, so the orphan rule no longer bites and overlapping providers coexist without any global commitment. The cost is that the trait must be designed with the extra context parameter from the start — it cannot be retrofitted onto an existing trait like `serde::Serialize` without a breaking change — and wiring must be spelled out for every value type a context uses. This is the rung most idiomatic CGP code lives on, and the one the [modular serialization](../examples/modular-serialization.md) and [money-transfer API](../examples/money-transfer-api.md) examples build on; the per-type dispatch it relies on is the subject of [dispatching](dispatching.md), wired through the [`open` statement](../reference/macros/delegate_components.md) over [namespaces](namespaces.md).

## Rung 5 — many implementations, wiring per type per provider

The top rung lets one provider override the wiring of a nested type locally, without routing that choice back through the context, by taking the inner provider as a parameter — a [higher-order provider](higher-order-providers.md). A recursive serializer for a collection ordinarily asks the context how to serialize each element; a higher-order variant instead accepts an explicit element serializer, defaulting to the context only when none is given:

```rust
pub struct SerializeIteratorWith<Provider = UseContext>(pub PhantomData<Provider>);

#[cgp_impl(SerializeIteratorWith<Provider>)]
impl<Value, Provider> ValueSerializer<Value>
where
    for<'a> &'a Value: IntoIterator,
    Provider: for<'a> ValueSerializer<Self, <&'a Value as IntoIterator>::Item>,
{
    fn serialize<S>(&self, value: &Value, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    { /* serialize each item through `Provider` */ }
}
```

With this in hand a context can fix the element encoding for one collection while leaving others to the context's general wiring — serializing a `Vec<Vec<u8>>` whose inner byte vectors are hexadecimal even though plain `Vec<u8>` elsewhere is encoded as raw bytes:

```rust
delegate_components! {
    AppA {
        open {ValueSerializerComponent};
        @ValueSerializerComponent.Vec<u8>: SerializeBytes,
        @ValueSerializerComponent.Vec<Vec<u8>>: SerializeIteratorWith<SerializeHex>,
        @ValueSerializerComponent.Vec<u64>: SerializeIteratorWith,
    }
}
```

This rung adds fine-grained, per-provider control on top of rung 4's per-context control: the `Vec<u64>` entry omits the parameter and so still routes its elements through the context, while the `Vec<Vec<u8>>` entry pins its inner encoding to `SerializeHex` regardless of how the context serializes `Vec<u8>` on its own. The `UseContext` default is what makes both forms read the same; the [`UseContext` provider](../reference/providers/use_context.md) routes back to the context's wiring when no override is supplied. The cost is the extra coupling and the higher-order machinery, which is why this rung is reserved for the cases where local override genuinely matters rather than used by default.

## Choosing a rung

The guiding rule is to settle at the lowest rung that expresses the use case, because each step up trades simplicity for modularity that may not be needed. A capability with one universal implementation belongs on rung 1; one that varies by type but never by application belongs on rung 2; a trait that should gain alternative providers without changing its interface belongs on rung 3; a capability where different applications must encode the same type differently belongs on rung 4, the home of most CGP code; and only a provider that must override a nested type's wiring locally needs rung 5. Climbing higher than necessary adds context parameters, wiring, and coupling that buy nothing, while stopping too low forces the hand-written impls and global commitments the higher rungs exist to avoid.

## Related constructs

The mechanism that makes rungs 3 through 5 possible — splitting a trait into a [consumer and provider trait](consumer-and-provider-traits.md) so overlapping and orphan implementations become legal — is the subject of [bypassing coherence](coherence.md), and the dependency threading every rung relies on is [impl-side dependencies](impl-side-dependencies.md). The constructs the rungs introduce are [`#[cgp_component]`](../reference/macros/cgp_component.md) and [`#[cgp_impl]`](../reference/macros/cgp_impl.md) for the trait split, [`delegate_components!`](../reference/macros/delegate_components.md) for the wiring, the [`open` statement](../reference/macros/delegate_components.md) over [namespaces](namespaces.md) and the [dispatching](dispatching.md) idea for per-type selection, and [higher-order providers](higher-order-providers.md) with the [`UseContext` provider](../reference/providers/use_context.md) for the top rung. The whole progression is worked through on a real capability in the [modular serialization](../examples/modular-serialization.md) example.
