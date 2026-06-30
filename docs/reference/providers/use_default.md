# `UseDefault`

`UseDefault` is a zero-sized marker provider that a component is wired to when its implementation should come entirely from the consumer trait's default method bodies.

## Purpose

`UseDefault` names the "use the trait's own defaults" choice in CGP wiring. A consumer trait may define default bodies for its methods, the same way any Rust trait can. When every method of a component has a usable default, there is no behavior left for a provider to supply — the provider only needs to exist so the component can be wired. `UseDefault` is that empty provider: wiring a component to `UseDefault` selects an implementation whose method bodies are inherited from the trait's defaults rather than supplied by a dedicated provider.

This keeps a default-only component consistent with the rest of CGP wiring. Without it, a component whose methods are all defaulted would still need some provider type and some `delegate_components!` entry to participate in the delegation table; `UseDefault` is the shared name for that role, so authors do not invent a one-off marker each time. A context that wants the defaults wires the component to `UseDefault` and writes no method bodies of its own.

`UseDefault` is a bare marker that CGP defines but does not implement for any trait. Unlike [`UseContext`](use_context.md), [`UseFields`](use_fields.md), or [`UseField`](use_field.md), no macro generates a provider impl for it; the provider impl is written by the author, typically with [`#[cgp_impl]`](../macros/cgp_impl.md) and an empty body so the trait's defaults take effect. This is what distinguishes `UseDefault` from the providers that carry generated behavior: it is purely a conventional name for an author-supplied, default-bodied implementation.

Like every CGP provider, `UseDefault` carries no runtime value. It is a unit struct used purely as a type-level marker, and its provider impls never read the `self` position.

## Definition

`UseDefault` is a unit struct with no fields, defined in `cgp-component`:

```rust
pub struct UseDefault;
```

It is exported from `cgp-component` but is not part of the `cgp::prelude`, so reaching it requires naming it through `cgp_component::UseDefault`. The struct carries no behavior of its own; meaning is given to it by the provider impl an author writes against it.

## Behavior

A component is wired to `UseDefault` by implementing the provider trait for it with empty method bodies, which causes the consumer trait's default bodies to be used. The implementing author leaves each method body out, so the default defined on the consumer trait fills in. Consider a getter and a greeter whose methods both have defaults:

```rust
#[cgp_getter]
pub trait HasName {
    fn name(&self) -> &str {
        "John"
    }
}

#[cgp_component(Greeter)]
pub trait CanGreet: HasName {
    fn greet(&self) -> String {
        format!("Hello, {}!", self.name())
    }
}
```

Provider impls against `UseDefault` are written with empty bodies, so the defaults take over:

```rust
#[cgp_impl(UseDefault)]
impl<Context> NameGetter for Context {}

#[cgp_impl(UseDefault)]
impl<Context: HasName> Greeter for Context {}
```

The first impl makes `UseDefault` a `NameGetter` provider whose `name` is the trait default `"John"`; the second makes it a `Greeter` provider whose `greet` is the trait default that formats around `self.name()`. The `Greeter` impl restates the consumer-side dependency `HasName` in its bound, because the default body of `greet` calls `name`. Each `#[cgp_impl]` also generates the matching [`IsProviderFor`](../traits/is_provider_for.md) impl, so the dependency propagates to the [check traits](../../concepts/check-traits.md) exactly as for any other provider.

Because `UseDefault` has no generated impls, the author controls precisely which components it serves. A type that has not had a provider impl written for a given component is simply not a provider for it; there is no automatic fallback, and the empty-body `#[cgp_impl]` is the explicit opt-in.

## Examples

The use of `UseDefault` is to wire several default-bodied components to one shared marker, then delegate them all to it in the context's table. Building on the traits above:

```rust
use cgp::prelude::*;
use cgp_component::UseDefault;

#[cgp_getter]
pub trait HasName {
    fn name(&self) -> &str {
        "John"
    }
}

#[cgp_component(Greeter)]
pub trait CanGreet: HasName {
    fn greet(&self) -> String {
        format!("Hello, {}!", self.name())
    }
}

#[cgp_impl(UseDefault)]
impl<Context> NameGetter for Context {}

#[cgp_impl(UseDefault)]
impl<Context: HasName> Greeter for Context {}

pub struct App;

delegate_components! {
    App {
        [
            NameGetterComponent,
            GreeterComponent,
        ]:
            UseDefault,
    }
}
```

`App` delegates both `NameGetterComponent` and `GreeterComponent` to `UseDefault`, so `App.greet()` produces `"Hello, John!"` entirely from the two default bodies — no method is implemented on `App` or on a dedicated provider. The array syntax routes both components to the single `UseDefault` marker in one entry.

## Related constructs

`UseDefault` is wired with [`delegate_components!`](../macros/delegate_components.md) and its provider impls are written with [`#[cgp_impl]`](../macros/cgp_impl.md), distinguishing it from [`UseContext`](use_context.md) and the getter providers [`UseFields`](use_fields.md) and [`UseField`](use_field.md), which carry macro-generated behavior rather than relying on author-written empty bodies. Its dependency tracking flows through [`IsProviderFor`](../traits/is_provider_for.md) and is checked with [`check_components!`](../macros/check_components.md), the same as any other provider.

## Source

The struct is defined in [crates/core/cgp-component/src/providers/use_default.rs](../../../crates/core/cgp-component/src/providers/use_default.rs) and re-exported in [crates/core/cgp-component/src/providers/mod.rs](../../../crates/core/cgp-component/src/providers/mod.rs); the file contains only the bare struct, with no macro-generated impls. The empty-body `#[cgp_impl]` pattern that wires components to a default-only marker is exercised in [crates/tests/cgp-tests/tests/component_tests/cgp_component/default_impl.rs](../../../crates/tests/cgp-tests/tests/component_tests/cgp_component/default_impl.rs).
