# Bypassing coherence

CGP works around Rust's coherence restrictions by splitting every trait into a pair, so that implementations are written incoherently against a unique provider type and coherence is then restored locally, one context at a time, by an explicit wiring step.

## What coherence guarantees

Coherence is the property that every trait lookup resolves to one globally unique implementation, no matter where in the program it is performed. Rust depends on this because its trait system doubles as a dependency-injection mechanism: a generic `impl` can require `where T: Display` without the caller ever naming that bound, and the compiler resolves the dependency — and every transitive dependency beneath it — by global lookup. For that lookup to be sound, it must always find the same implementation, so the answer cannot depend on which crate is asking or what else happens to be in scope.

Two rules enforce this uniqueness. The **overlap rule** forbids two implementations that could both apply to the same type, since the compiler would have no principled way to choose between them. The **orphan rule** forbids implementing a trait for a type unless the current crate owns either the trait or the type, since otherwise two unrelated crates could each define their own implementation and a program depending on both would face an irreconcilable conflict. Together they are what let dependency injection work transitively without ambiguity.

## The cost

These rules are necessary for soundness but they forbid a great deal of code that would be perfectly useful. The canonical casualty is the blanket implementation: one cannot implement `serde::Serialize` for *every* type that implements `Display`, or for every type that implements `AsRef<[u8]>`, because a type might implement several of those bounds at once and the implementations would overlap — and Rust permits at most one such blanket impl, with no way to say which should win.

Spelled out in code, the second of two overlapping blanket impls is rejected by the compiler, because some type — `String`, say, is both `Display` and `AsRef<[u8]>` — could match both:

```rust
use serde::{Serialize, Serializer};

// Legal on its own: serialize anything printable as a string.
impl<T: core::fmt::Display> Serialize for T {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(&self.to_string())
    }
}

// error[E0119]: conflicting implementation — overlaps the impl above
// on every `T: Display + AsRef<[u8]>`.
impl<T: AsRef<[u8]>> Serialize for T {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_bytes(self.as_ref())
    }
}
```

The orphan rule bites just as often, and independently of any overlap: a crate that wants to serialize a `Person` type it did not define, using a `Serialize` trait it also did not define, simply cannot, because it owns neither side:

```rust
use serde::{Serialize, Serializer};
use other_crate::Person; // defined in a crate we do not own

// error[E0117]: only traits defined in the current crate can be
// implemented for types defined outside of the crate.
impl Serialize for Person {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        /* ... */
    }
}
```

The programmer is left writing the same boilerplate by hand for each type, or pressuring upstream crates to add derives they may not want.

## Moving `Self` to a parameter

CGP's first move is to take the type that coherence ranges over — the `Self` of the implementation — and turn it into something the implementing crate always owns. A CGP component is declared once but compiled into two traits: a consumer trait that callers invoke with the ordinary `Self` receiver, and a **provider trait** whose `Self` is a dedicated, zero-sized provider struct while the original receiver becomes an explicit `Context` type parameter. Because the provider implements the provider trait *for its own struct* — `ValueSerializer<Context, Value>` for `SerializeBytes`, never `Serialize` for some foreign type — neither coherence rule applies: the `Self` type is local and unique, so the implementation is neither an orphan nor an overlap, however general its `Context` and other parameters are. The full mechanics of this split, and the blanket impls that make a context's `context.serialize(...)` resolve to a chosen provider, are the subject of [consumer and provider traits](consumer-and-provider-traits.md).

This is what makes incoherent implementations expressible. A crate can define `UseSerde` for any `Value: Serialize`, `SerializeBytes` for any `Value: AsRef<[u8]>`, and `SerializeWithDisplay` for any `Value: Display`, all at once — three blanket implementations of the same capability that overlap freely, since a type like `String` satisfies all three bounds at once, each a distinct provider struct the crate owns. The three impls read almost exactly like the rejected `Serialize` impls above, differing only in the provider name given to [`#[cgp_impl]`](../reference/macros/cgp_impl.md), which becomes the unique `Self` type:

```rust
#[cgp_impl(UseSerde)]
impl<Value> ValueSerializer<Value>
where
    Value: serde::Serialize,
{
    fn serialize<S>(&self, value: &Value, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer { value.serialize(serializer) }
}

#[cgp_impl(SerializeBytes)]
impl<Value> ValueSerializer<Value>
where
    Value: AsRef<[u8]>,
{
    fn serialize<S>(&self, value: &Value, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer { serializer.serialize_bytes(value.as_ref()) }
}

#[cgp_impl(new SerializeWithDisplay)]
#[uses(CanSerializeValue<String>)]
impl<Value> ValueSerializer<Value>
where
    Value: core::fmt::Display,
{
    fn serialize<S>(&self, value: &Value, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer { self.serialize(&value.to_string(), serializer) }
}
```

Because each impl targets its own `Self` — `UseSerde`, `SerializeBytes`, `SerializeWithDisplay` — all three compile together and overlap on `String` without complaint, where any two of the vanilla `Serialize` impls were rejected. A downstream crate can add yet more for types it does not own, since the orphan rule never enters: the `Self` type is always a struct that crate defines.

## Restoring coherence locally

Removing coherence from implementations would be useless if it also removed coherence from *use* — a caller still needs `context.serialize(value, s)` to mean one definite thing. CGP's second move is therefore to reintroduce coherence at a smaller scale: instead of one global choice of implementation per trait, each context makes its own coherent choice, valid within that context alone. The insight is that incoherence is wanted only when writing implementations; when using them, what is wanted is many independent local scopes, each internally consistent.

The wiring step is what creates those scopes. A concrete context selects exactly one provider per component by becoming a type-level lookup table — written with [`delegate_components!`](../reference/macros/delegate_components.md) — whose entry for a component names the provider to use. Within that one context the choice is unambiguous, so `context.serialize(...)` resolves coherently; a different context can wire the same component to a different provider and resolve differently, with no conflict between them because the resolution is keyed on the context type.

Two applications can thus serialize `Vec<u8>` as hexadecimal and as base64 respectively, each coherent in its own scope, where global coherence would have forced a single answer on both. The two scopes are simply two wiring tables, each opening the serialization component and choosing a provider for `Vec<u8>`:

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

`AppA` serializes a `Vec<u8>` as hexadecimal and `AppB` as base64; the overlapping `SerializeHex` and `SerializeBase64` providers coexist globally, while each context's choice stays unambiguous within itself. The `open` statement and its `@`-path entries are the [namespace](namespaces.md)-based wiring form that keys this dispatch on the value type. Because the table is consulted only at the leaves of dependency injection and never appears in a generic bound, the soundness problem that motivated the overlap rule — a blanket impl silently chosen inside generic code and later contradicted by a specialized one — cannot arise: there is no global blanket impl to be silently chosen, only a per-context entry.

## Composing and selecting incoherent implementations

Two further patterns let a context draw on many incoherent implementations at once rather than picking a single one. When the choice of provider should depend on a type argument — serialize *this* value type one way and *that* one another — the [`open` statement](../reference/macros/delegate_components.md) wires a provider per value of that argument directly into the context's table, so one component fans out to a type-specific provider per case; this is the [dispatching](dispatching.md) idea applied to a generic parameter, resolved through the [`RedirectLookup`](../reference/providers/redirect_lookup.md) impl each component carries. (The legacy [`UseDelegate`](../reference/providers/use_delegate.md) provider performs the same second lookup through a separate nested table.) When one implementation should be expressed in terms of another — wrap, scale, or recurse over a base case — a [higher-order provider](higher-order-providers.md) takes the inner provider as a parameter and composes with it. Both compose entirely in types, so a context can wire a deep tree of overlapping providers and pay nothing at runtime.

Dependency injection is what threads the chosen implementations through without explicit plumbing. A provider states the capabilities it needs as bounds in its `where` clause — that the context can serialize a `String`, that it can fetch an arena allocator — and the wiring satisfies them by resolving each through the same context, exactly the [impl-side dependency](impl-side-dependencies.md) mechanism ordinary CGP code uses. The context therefore acts as the implicit carrier of every implementation a computation needs, which is how CGP emulates capability-passing — supplying values and behaviors to deeply nested code through the context rather than as explicit arguments — in stable Rust.

## Related constructs

The trait split that makes incoherent implementations expressible is documented in full in [consumer and provider traits](consumer-and-provider-traits.md), generated by [`#[cgp_component]`](../reference/macros/cgp_component.md) and implemented through [`#[cgp_impl]`](../reference/macros/cgp_impl.md). The local-coherence wiring step is [`delegate_components!`](../reference/macros/delegate_components.md), which builds the per-context [`DelegateComponent`](../reference/traits/delegate_component.md) table; [`check_components!`](../reference/macros/check_components.md) verifies that a context's chosen providers actually satisfy their dependencies, the subject of [check traits](check-traits.md). The two composition patterns are per-value dispatch through the [`open` statement](../reference/macros/delegate_components.md) and its [`RedirectLookup`](../reference/providers/redirect_lookup.md) resolution (see [namespaces](namespaces.md) and [dispatching](dispatching.md); the legacy [`UseDelegate`](../reference/providers/use_delegate.md) is the older equivalent) and [higher-order providers](higher-order-providers.md), and the dependency-threading that ties them to a context is [impl-side dependencies](impl-side-dependencies.md). The [modularity hierarchy](modularity-hierarchy.md) concept places this strategy on a ladder, showing how the trait split and per-context wiring described here are the upper rungs above vanilla Rust's one-implementation-per-type coherence. The [modular serialization](../examples/modular-serialization.md) example works the whole strategy through end to end on Serde's `Serialize` and `Deserialize`.

## Source

The blanket impls that restore coherent use from the provider traits, and the [`DelegateComponent`](../reference/traits/delegate_component.md)/[`IsProviderFor`](../reference/traits/is_provider_for.md)/[`CanUseComponent`](../reference/traits/can_use_component.md) machinery they rely on, live in [crates/core/cgp-component/src/](../../crates/core/cgp-component/src/); the macros that generate them are in [crates/macros/cgp-macro-core/src/types/cgp_component/](../../crates/macros/cgp-macro-core/src/types/cgp_component/).
