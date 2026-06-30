# Modular serialization

This example rebuilds Serde's `Serialize` and `Deserialize` as CGP components, so that how each value type is encoded becomes a per-context wiring choice rather than a single fixed implementation baked into the type. It progresses from the two serialization components, through a family of overlapping providers that vanilla Rust would reject, to two application contexts that serialize the same nested data into different JSON formats by changing only a handful of wiring lines, and finally to a context-dependent deserializer that allocates into an arena. It is the template for any capability where the same type needs several interchangeable implementations selected per application, and where the orphan rule would otherwise force a library to derive the trait on every data type itself.

The concepts each step demonstrates are documented in full in the reference; this example notes which one is in play and links to it:

- splitting a trait so overlapping and orphan implementations are legal — [consumer and provider traits](../concepts/consumer-and-provider-traits.md) and the [coherence](../concepts/coherence.md) strategy behind it
- defining the components — [`#[cgp_component]`](../reference/macros/cgp_component.md)
- writing the providers — [`#[cgp_impl]`](../reference/macros/cgp_impl.md), with [`#[uses]`](../reference/attributes/uses.md) for the capabilities they look up through the context
- serializing a struct with no serialization-specific derive — [extensible records](../concepts/extensible-records.md) via [`#[derive(CgpData)]`](../reference/derives/derive_cgp_data.md)
- selecting a provider per value type, inline in the context's own table — the `open` statement of [`delegate_components!`](../reference/macros/delegate_components.md) with [`@`-path keys](../concepts/namespaces.md)
- verifying a context's wiring — [`check_components!`](../reference/macros/check_components.md)
- pulling a capability from the context during deserialization — [`#[cgp_auto_getter]`](../reference/macros/cgp_auto_getter.md) over [`HasField`](../reference/traits/has_field.md), with the [`HasErrorType`](../reference/components/has_error_type.md) and [`CanRaiseError`](../reference/components/can_raise_error.md) error components wired through [modular error handling](../concepts/modular-error-handling.md)

All snippets assume `use cgp::prelude::*;` and use Serde's `Serializer`/`Deserializer` traits directly. The providers shown here serialize and deserialize, but the example builds up the serialization side first and then mirrors it for deserialization.

## The two serialization components

The starting point is a context-generic restatement of Serde's two traits. Each moves the type being serialized out of the `Self` position — where Serde keeps it — and into an explicit `Value` parameter, leaving `Self` to name a **context** that carries the wiring:

```rust
#[cgp_component(ValueSerializer)]
pub trait CanSerializeValue<Value: ?Sized> {
    fn serialize<S>(&self, value: &Value, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer;
}

#[cgp_component(ValueDeserializer)]
pub trait CanDeserializeValue<'de, Value> {
    fn deserialize<D>(&self, deserializer: D) -> Result<Value, D::Error>
    where
        D: serde::Deserializer<'de>;
}
```

`CanSerializeValue` and `CanDeserializeValue` are the [consumer traits](../concepts/consumer-and-provider-traits.md) callers use as `context.serialize(value, s)`; `ValueSerializer` and `ValueDeserializer` are the provider traits implementations are written against. The extra `&self` is the whole point — it gives every implementation access to the context, both to look up how to serialize nested values and, for deserialization, to pull runtime dependencies out of it. Because `Value` is a generic parameter rather than the `Self` type, a context can later wire a different provider for each concrete value type, which is the per-type dispatch set up when the contexts are wired below.

## Overlapping providers

With the type moved off `Self`, several implementations of the same component can coexist even though they overlap — each is written for its own zero-sized provider struct, which the defining crate owns, so the [coherence](../concepts/coherence.md) rules never apply. The simplest provider stays compatible with the existing Serde ecosystem by deferring to Serde's own `Serialize`:

```rust
pub struct UseSerde;

#[cgp_impl(UseSerde)]
impl<Value> ValueSerializer<Value>
where
    Value: serde::Serialize,
{
    fn serialize<S>(&self, value: &Value, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        value.serialize(serializer)
    }
}
```

`#[cgp_impl(UseSerde)]` reads like a blanket impl of the consumer trait, but the provider name in the attribute is what becomes the actual `Self` type, so this works for *any* `Context` and *any* `Value: Serialize` without overlapping anything. A second provider serializes any byte container directly as bytes, overlapping `UseSerde` on every type that is both `Serialize` and `AsRef<[u8]>` — `Vec<u8>` among them:

```rust
pub struct SerializeBytes;

#[cgp_impl(SerializeBytes)]
impl<Value> ValueSerializer<Value>
where
    Value: AsRef<[u8]>,
{
    fn serialize<S>(&self, value: &Value, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_bytes(value.as_ref())
    }
}
```

In vanilla Rust these two blanket implementations could not both exist; as named providers they are simply two entries a context may choose between. Which one a given context uses for `Vec<u8>` is decided entirely by its wiring, shown further below.

## Looking serialization up through the context

A provider needs more than its own logic when it serializes by delegating to another encoding — and it gets that by asking the context. `SerializeWithDisplay` formats any `Display` value to a string and then serializes *that string through the context*, so the eventual byte-level representation of the string is itself a wiring choice rather than fixed here. The capability it depends on is declared with [`#[uses]`](../reference/attributes/uses.md), which adds the bound `Self: CanSerializeValue<String>` as an [impl-side dependency](../concepts/impl-side-dependencies.md):

```rust
#[cgp_impl(new SerializeWithDisplay)]
#[uses(CanSerializeValue<String>)]
impl<Value> ValueSerializer<Value>
where
    Value: core::fmt::Display,
{
    fn serialize<S>(&self, value: &Value, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let str_value = value.to_string();
        self.serialize(&str_value, serializer)
    }
}
```

The `new` keyword defines the `SerializeWithDisplay` struct in passing. The same shape encodes a byte container as a hexadecimal string — converting to a `String` and handing it back to the context — which is one of the two formats the demo needs:

```rust
pub struct SerializeHex;

#[cgp_impl(SerializeHex)]
#[uses(CanSerializeValue<String>)]
impl<Value> ValueSerializer<Value>
where
    Value: hex::ToHex,
{
    fn serialize<S>(&self, value: &Value, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let str_value = value.encode_hex::<String>();
        self.serialize(&str_value, serializer)
    }
}
```

A base64 provider is identical but for the encoding call, and a date provider serializes a `DateTime<Utc>` by formatting it to an RFC 3339 string and serializing that through the context, while an alternative serializes the same `DateTime<Utc>` as a Unix timestamp `i64`. Each is a separate provider overlapping the others on its value type, and the context picks one.

## Serializing collections and structs

Two recursive providers handle composite values by serializing each part through the context, so customization reaches arbitrarily deep without any provider knowing the concrete shape. `SerializeIterator` serializes any iterable as a sequence, asking the context how to serialize each item:

```rust
#[cgp_impl(new SerializeIterator)]
impl<Value> ValueSerializer<Value>
where
    for<'a> &'a Value: IntoIterator,
    Self: for<'a> CanSerializeValue<<&'a Value as IntoIterator>::Item>,
{
    fn serialize<S>(&self, value: &Value, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut seq = serializer.serialize_seq(None)?;
        for item in value.into_iter() {
            seq.serialize_element(&SerializeWithContext::new(self, &item))?;
        }
        seq.end()
    }
}
```

The item bound is a higher-ranked `Self: for<'a> CanSerializeValue<...>` rather than a `#[uses]` line, because a higher-ranked bound is beyond what the simplified `#[uses]` syntax expresses. `SerializeWithContext` is the adapter that pairs a value with a context and implements Serde's own `Serialize`, which is how a nested value re-enters the context's wiring — it appears again at the top level below. The companion `SerializeFields` serializes any struct as a map by walking its fields, available because the struct derives [`CgpData`](../reference/derives/derive_cgp_data.md) and so exposes its fields through [`HasFields`](../reference/traits/has_fields.md):

```rust
#[cgp_impl(new SerializeFields)]
impl<Value> ValueSerializer<Value>
where
    Value: HasFields,
    Value::Fields: FieldsSerializer<Self, Value>,
{
    fn serialize<S>(&self, value: &Value, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let map = serializer.serialize_map(None)?;
        Value::Fields::serialize_fields(self, value, map)
    }
}
```

This is the payoff for the orphan rule: a data type needs no serialization-specific derive and no dependency on this crate at all. Deriving the general-purpose `CgpData` is enough for `SerializeFields` to serialize it generically — see [extensible records](../concepts/extensible-records.md) for what that derive exposes — so a library never has to implement a serialization trait on types it owns just because a downstream application wants to encode them.

## The data types

The demo serializes a small tree of encrypted-messaging types, each carrying a byte field or a `DateTime` whose encoding the applications will want to control. Their only derive is `CgpData`:

```rust
use chrono::{DateTime, Utc};

#[derive(CgpData)]
pub struct EncryptedMessage {
    pub message_id: u64,
    pub author_id: u64,
    pub date: DateTime<Utc>,
    pub encrypted_data: Vec<u8>,
}

#[derive(CgpData)]
pub struct MessagesByTopic {
    pub encrypted_topic: Vec<u8>,
    pub messages: Vec<EncryptedMessage>,
}

#[derive(CgpData)]
pub struct MessagesArchive {
    pub decryption_key: Vec<u8>,
    pub messages_by_topics: Vec<MessagesByTopic>,
}
```

## Wiring an application context

A context turns this pile of overlapping providers into one coherent scheme by choosing, per value type, which provider runs. The `open` statement in [`delegate_components!`](../reference/macros/delegate_components.md) opens the serialization component for per-type wiring directly in the context's own table; after it, an `@ValueSerializerComponent.<Type>: <Provider>` entry assigns a provider to each value type the archive touches, the type written as a [`@`-path key](../concepts/namespaces.md). `open` is the lightweight wiring form that suits a self-contained application like this one; a large code base with many components instead shares wiring through named [namespaces](../concepts/namespaces.md) that contexts join and selectively override:

```rust
pub struct AppA;

delegate_components! {
    AppA {
        open {ValueSerializerComponent};

        @ValueSerializerComponent.<'a, T> &'a T:
            SerializeDeref,
        @ValueSerializerComponent.[
            u64,
            String,
        ]:
            UseSerde,
        @ValueSerializerComponent.Vec<u8>:
            SerializeHex,
        @ValueSerializerComponent.DateTime<Utc>:
            SerializeRfc3339Date,
        @ValueSerializerComponent.[
            Vec<EncryptedMessage>,
            Vec<MessagesByTopic>,
        ]:
            SerializeIterator,
        @ValueSerializerComponent.[
            MessagesArchive,
            MessagesByTopic,
            EncryptedMessage,
        ]:
            SerializeFields,
    }
}
```

Reading the table top to bottom: a borrowed value routes to `SerializeDeref` (which forwards to the value behind the reference, encountered while serializing nested items), the scalar types fall back to plain Serde, `Vec<u8>` is encoded as hexadecimal, `DateTime<Utc>` as an RFC 3339 string, the collections as sequences, and the structs as maps. Because the byte and date entries are the only ones that fix a *format*, a second application differs in only a few lines — base64 instead of hex, Unix timestamps instead of RFC 3339, plus an `i64` entry for the timestamps:

```rust
pub struct AppB;

delegate_components! {
    AppB {
        open {ValueSerializerComponent};

        @ValueSerializerComponent.<'a, T> &'a T:
            SerializeDeref,
        @ValueSerializerComponent.[
            i64,
            u64,
            String,
        ]:
            UseSerde,
        @ValueSerializerComponent.Vec<u8>:
            SerializeBase64,
        @ValueSerializerComponent.DateTime<Utc>:
            SerializeTimestamp,
        @ValueSerializerComponent.[
            Vec<EncryptedMessage>,
            Vec<MessagesByTopic>,
        ]:
            SerializeIterator,
        @ValueSerializerComponent.[
            MessagesArchive,
            MessagesByTopic,
            EncryptedMessage,
        ]:
            SerializeFields,
    }
}
```

The two contexts resolve `Vec<u8>` to overlapping providers — `SerializeHex` and `SerializeBase64` — with no conflict, because each choice is coherent only within its own context. CGP wiring is [checked lazily](../concepts/check-traits.md), so a [`check_components!`](../reference/macros/check_components.md) block asserts at compile time that each context can actually serialize every value type, listing them as the `Value` parameters of `ValueSerializerComponent`:

```rust
check_components! {
    AppA {
        ValueSerializerComponent: [
            u64,
            String,
            Vec<u8>,
            DateTime<Utc>,
            EncryptedMessage,
            MessagesByTopic,
            MessagesArchive,
        ],
    }
}
```

## Producing JSON

Because the providers ultimately defer to a real `serde::Serializer`, the existing JSON ecosystem still does the writing. The bridge is `SerializeWithContext`, which wraps a context and a value into a type that implements Serde's `Serialize` by calling the context's `CanSerializeValue`:

```rust
let archive = MessagesArchive { /* ... */ };

let json_a = serde_json::to_string_pretty(&SerializeWithContext::new(&AppA, &archive)).unwrap();
let json_b = serde_json::to_string_pretty(&SerializeWithContext::new(&AppB, &archive)).unwrap();
```

`json_a` encodes every byte field as hexadecimal and every date as an RFC 3339 string; `json_b` encodes the same archive with base64 and Unix timestamps. Nothing in the data types or the providers changed between the two — only which context wraps the value.

## Deserializing with a context-supplied capability

Deserialization mirrors serialization, and the extra `&self` becomes load-bearing in a way Serde cannot match: the context can supply runtime *capabilities* a provider pulls in by dependency injection. The motivating case is an [arena allocator](https://en.wikipedia.org/wiki/Region-based_memory_management) — deserializing many borrowed `&'a T` values into one arena instead of heap-allocating each. The context exposes the arena through a getter trait, written with [`#[cgp_auto_getter]`](../reference/macros/cgp_auto_getter.md) so any context with a matching field implements it automatically:

```rust
use typed_arena::Arena;

#[cgp_auto_getter]
pub trait HasArena<'a, T: 'a> {
    fn arena(&self) -> &&'a Arena<T>;
}
```

The provider for the borrowed type then deserializes the owned value through the context and moves it into the arena fetched from the context. Both capabilities it needs — the arena and the ability to deserialize the owned `Value` — are declared with [`#[uses]`](../reference/attributes/uses.md):

```rust
#[cgp_impl(new DeserializeAndAllocate)]
#[uses(HasArena<'a, Value>, CanDeserializeValue<'de, Value>)]
impl<'de, 'a, Value> ValueDeserializer<'de, &'a Value> {
    fn deserialize<D>(&self, deserializer: D) -> Result<&'a Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = self.deserialize(deserializer)?;
        let value = self.arena().alloc(value);
        Ok(value)
    }
}
```

The data and the context complete the picture. The structs derive `CgpData` for generic field-by-field deserialization, and the context carries the arena as an ordinary field, deriving [`HasField`](../reference/traits/has_field.md) so `HasArena` is satisfied:

```rust
#[derive(CgpData)]
pub struct Coord {
    pub x: u64,
    pub y: u64,
    pub z: u64,
}

#[derive(CgpData)]
pub struct Cluster<'a> {
    pub id: u64,
    pub coords: Vec<&'a Coord>,
}

#[derive(HasField)]
pub struct App<'a> {
    pub arena: &'a Arena<Coord>,
}
```

The wiring opens the deserialization component and keys on the value type as before, routing the bare `Coord` and `Cluster` to a record-field deserializer, the borrowed `&'a Coord` to the arena allocator, and the `Vec<&'a Coord>` to a sequence deserializer. Deserialization can fail, so the context also wires the [`HasErrorType`](../reference/components/has_error_type.md) and [`CanRaiseError`](../reference/components/can_raise_error.md) error components to an `anyhow`-backed backend — those are ordinary component entries, written without `open` because they are not dispatched per value type:

```rust
use cgp::core::error::{ErrorRaiserComponent, ErrorTypeProviderComponent};
use cgp_error_anyhow::{RaiseAnyhowError, UseAnyhowError};

delegate_components! {
    <'s> App<'s> {
        open {ValueDeserializerComponent};

        @ValueDeserializerComponent.u64:
            UseSerde,
        @ValueDeserializerComponent.[
            Coord,
            <'a> Cluster<'a>,
        ]:
            DeserializeRecordFields,
        @ValueDeserializerComponent.<'a> &'a Coord:
            DeserializeAndAllocate,
        @ValueDeserializerComponent.<'a> Vec<&'a Coord>:
            DeserializeExtend,

        ErrorTypeProviderComponent:
            UseAnyhowError,
        ErrorRaiserComponent:
            RaiseAnyhowError,
    }
}
```

With the context built around an arena, deserializing a JSON cluster allocates its coordinates into that arena, and the returned `Cluster` borrows from it:

```rust
let serialized = r#"
    {
        "id": 8,
        "coords": [
            { "x": 1, "y": 2, "z": 3 },
            { "x": 4, "y": 5, "z": 6 }
        ]
    }
"#;

let arena = Arena::new();
let app = App { arena: &arena };

let cluster: Cluster<'_> = app.deserialize_json_string(serialized).unwrap();
```

The arena was never an argument to a deserialize function — Serde's `from_str` has no slot for one. It reached `DeserializeAndAllocate` through the context, which is how CGP supplies a capability to code nested arbitrarily deep without threading it explicitly, the [dependency-injection](../concepts/impl-side-dependencies.md) idea applied to deserialization.
