# Modularity hierarchy

A spectrum of how decoupled an implementation can be from the type it serves, from plain generic functions up to per-provider wiring, so you can pick how much CGP machinery a problem actually needs.

CGP is not all-or-nothing. The same capability — here, serializing a value with Serde — can be expressed at several levels of modularity, each more decoupled than the last and each carrying more machinery in exchange. This page walks the spectrum on one running example so a reader can stop at the first level that solves the problem rather than reaching for the heaviest tool by reflex. Assume `use cgp::prelude::*;` throughout; the CGP version is v0.7.0.

## The coherence problem the hierarchy escapes

What forces this hierarchy to exist is Rust's coherence rules, which guarantee that every trait lookup resolves to one globally unique implementation. Two rules enforce that uniqueness. The **overlap rule** forbids two implementations that could both apply to the same type — you cannot blanket-implement `Serialize` for every `T: Display` *and* for every `T: AsRef<[u8]>`, because a `String` satisfies both and the compiler has no principled way to choose. The **orphan rule** forbids implementing a trait for a type unless your crate owns either the trait or the type — you cannot implement someone else's `Serialize` for someone else's `Vec<u8>`. Each level below loosens one more of these constraints. CGP's escape route is to move the type that coherence ranges over — the `Self` of the implementation — into a position the implementing crate always owns, then restore a single unambiguous answer locally, one [context](components.md) at a time, through [wiring](wiring.md). See [coherence](https://github.com/contextgeneric/cgp/blob/main/docs/concepts/coherence.md) for the full framing.

## Level 1 — one implementation per interface

The least machinery is a generic function or a blanket trait impl, which both define exactly one implementation behind an interface. A generic function captures the logic and its bounds in one place:

```rust
pub fn serialize_bytes<Value: AsRef<[u8]>, S: Serializer>(
    value: &Value,
    serializer: S,
) -> Result<S::Ok, S::Error> { ... }
```

A blanket trait carries the same one-implementation limitation but reads more ergonomically at the call site, since the bound hides behind the trait impl and the caller writes a method:

```rust
pub trait CanSerializeBytes {
    fn serialize_bytes<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error>;
}

impl<Value: AsRef<[u8]>> CanSerializeBytes for Value {
    fn serialize_bytes<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> { ... }
}
```

The gain is reuse with zero ceremony. The limitation is absolute: there can be exactly one blanket impl, so you cannot offer two ways to serialize bytes and let a caller pick between them.

## Level 2 — one unique implementation per type per interface

A vanilla Rust trait lifts the one-implementation limit slightly: many types may share the interface, but coherence still permits at most one implementation per type. Each type that wants the behavior writes its own impl:

```rust
pub trait Serialize {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error>;
}

impl Serialize for Vec<u8> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.serialize_bytes(serializer)
    }
}

impl<'a> Serialize for &'a [u8] {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.serialize_bytes(serializer)
    }
}
```

The gain is that different types can be serialized differently. The cost is duplication: `Vec<u8>` and `&[u8]` each need an explicit impl even though the logic is identical. The body can still call out to a Level-1 building block such as `CanSerializeBytes` to share the actual work, so the duplication is confined to the boilerplate of forwarding. The remaining limitation is the one-impl-per-type ceiling — there is still no way to give `Vec<u8>` two serialization strategies and choose between them.

## Level 3 — multiple implementations per type, globally unique wiring

Applying basic CGP to a vanilla trait removes the duplication of Level 2 by turning the shared logic into a reusable [provider](components.md) and letting each type [wire](wiring.md) to it. The trait keeps its original shape; `#[cgp_component]` generates the [consumer trait](components.md) and [provider trait](components.md) pair, `#[cgp_impl(new ...)]` defines a named provider once, and `delegate_components!` points each type at it:

```rust
#[cgp_component(ValueSerializer)]
pub trait Serialize {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error>;
}

#[cgp_impl(new SerializeBytes)]
impl<Value: AsRef<[u8]>> ValueSerializer for Value {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> { ... }
}

delegate_components! {
    Vec<u8> {
        ValueSerializerComponent: SerializeBytes,
    }
}

delegate_components! {
    <'a> &'a [u8] {
        ValueSerializerComponent: SerializeBytes,
    }
}
```

The gain is real reuse without modifying the interface: `Serialize` is unchanged, so a type can still implement it directly without opting into CGP at all, and existing users of the trait are unaffected. The `ValueSerializer` provider trait removes the need for ad-hoc interfaces like `CanSerializeBytes`, and `delegate_components!` removes the manual forwarding of Level 2. The limitation is that coherence still binds the wiring itself: each type carries one global wiring, so a `Vec<u8>` entry conflicts with any overlapping `Vec<T>` entry, the choice cannot be overridden per context, and the orphan rule still means you can only wire `Vec<u8>` from a crate that owns either `Serialize` or `Vec`.

## Level 4 — unique wiring per type, per context

Adding an explicit context parameter fully decouples the implementation from the type, so each context wires its own choices and the orphan rule lifts entirely. The trait changes shape: the original `Self` becomes an explicit `Value` parameter, so the component now dispatches on which concrete value type it serializes. Each context then folds its per-type choices straight into its own table with the `open` statement of `delegate_components!`:

```rust
#[cgp_component(ValueSerializer)]
pub trait CanSerializeValue<Value: ?Sized> {
    fn serialize<S>(&self, value: &Value, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer;
}

delegate_components! {
    new MyAppA {
        open { ValueSerializerComponent };

        @ValueSerializerComponent.Vec<u8>: SerializeBytes,
        @ValueSerializerComponent.Vec<u64>: SerializeIterator,
    }
}

delegate_components! {
    new MyAppB {
        open { ValueSerializerComponent };

        @ValueSerializerComponent.Vec<u8>: SerializeHex,
        @ValueSerializerComponent.Vec<u64>: SerializeIterator,
    }
}
```

The `open { ValueSerializerComponent };` header opens the component for per-value wiring, and each `@ValueSerializerComponent.Value: Provider` entry assigns a provider for one concrete value type. The gain is that `MyAppA` and `MyAppB` resolve `Vec<u8>` to different providers — bytes versus hex — with no conflict, because each choice is coherent only within its own context. The orphan rule no longer applies: a context can wire `Vec<u8>` even when its crate owns neither `CanSerializeValue` nor `Vec`, as long as it owns the context type, so you never commit to a global serialization for `Vec` up front. The costs are that the trait must be modified to add the context parameter, and that every value type a context touches must be wired explicitly, which grows tedious for a large type set.

The `open` form rides the dispatch machinery that every `#[cgp_component]` already generates, so the trait needs no extra option. A legacy alternative writes the same dispatch with `derive_delegate: UseDelegate<Value>` on the trait and a `UseDelegate<new ValueSerializerComponents { Vec<u8>: SerializeBytes, ... }>` nested table in each context's wiring; it is retained for compatibility but `open` is preferred for new code, and the two forms appear side by side in [wiring](wiring.md).

## Level 5 — explicit wiring per type, per provider

The finest grain overrides wiring *inside* a provider rather than at the context, using a [higher-order provider](higher-order-providers.md) whose inner provider defaults to `UseContext`. The default routes nested lookups back through the context as usual, while an explicit inner provider overrides one branch locally without touching the context's table:

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
    { ... }
}

delegate_components! {
    new MyAppA {
        open { ValueSerializerComponent };

        @ValueSerializerComponent.Vec<u8>: SerializeBytes,
        @ValueSerializerComponent.Vec<Vec<u8>>: SerializeIteratorWith<SerializeHex>,
        @ValueSerializerComponent.Vec<u64>: SerializeIteratorWith,
        @ValueSerializerComponent.[u8, u64]: UseSerde,
    }
}
```

Here `Vec<Vec<u8>>` serializes its inner `Vec<u8>` as hex strings, while a bare `Vec<u8>` elsewhere in the same context still serializes as bytes — the inner provider is overridden for that one branch only. Where `SerializeIteratorWith` is left without an argument, as for `Vec<u64>`, the `UseContext` default takes over and the item lookup goes back through the context, so the `u64` items resolve to `UseSerde` from the table. The gain is per-provider control: a wiring decision can be pinned at the point of use instead of globally at the context level. The cost is the higher-order plumbing itself — the extra provider parameter, the explicit context argument in the inner bound, and the discipline of choosing when to override versus when to defer to the context.

## Choosing a level

Read the spectrum as a ladder and stop at the first rung that fits. Levels 1 and 2 are plain Rust and need no CGP at all — reach for them when one implementation, or one per type, is genuinely all you need. Level 3 buys reuse and swappable providers while leaving the trait and its existing users untouched, the right entry point for retrofitting CGP onto an established trait. Level 4 is the canonical CGP shape, paying a modified interface for full per-context freedom and escape from the orphan rule. Level 5 is a local refinement layered on top of Level 4, used only where a single nested branch must diverge from the context's global choice. Each step up trades ceremony for decoupling, so the discipline is to climb only as far as the problem demands.

Further reference: [coherence](https://github.com/contextgeneric/cgp/blob/main/docs/concepts/coherence.md) for the rules this hierarchy escapes, and [modular serialization](https://github.com/contextgeneric/cgp/blob/main/docs/examples/modular-serialization.md) for the full worked example.
