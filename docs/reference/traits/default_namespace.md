# `DefaultNamespace`, `DefaultImpls1`, `DefaultImpls2`

`DefaultNamespace<Components>`, `DefaultImpls1<T, Components>`, and `DefaultImpls2<T1, T2, Components>` are the hierarchical lookup traits that back namespaces and presets, each mapping a key to a `Delegate` type so that a context can inherit a whole group of default wirings and resolve them by component name, by one type parameter, or by two.

## Purpose

This family exists to give namespaces and presets a uniform, per-key lookup surface. A namespace is a reusable table of default wirings that a context can opt into and then selectively override; resolving such a default means asking, "for this key, what does the namespace delegate to?" The three traits are the answer-bearers, differing only in how many type parameters take part in the key. `DefaultNamespace` keys a default purely on the component name. `DefaultImpls1` keys it on the component name *and* one further type — the typical shape for a per-type default, where the same component resolves differently for `String` than for `u64`. `DefaultImpls2` does the same for two further types, for components parameterized by a pair.

The reason to have all three rather than one variadic trait is that each fixes the arity of the key at the type level, which lets the projection `<Key as Trait<…, Delegate = Provider>>` resolve cleanly. A context that joins a namespace forwards its lookups into one of these traits, and a `for … in` loop that pulls per-type defaults reads them by projecting the `Delegate`. The whole mechanism is type-level and inheritance-with-override in spirit: a directly-wired entry on a context wins over a namespace fallback, exactly as a preset is meant to be customizable.

These traits are the plumbing beneath the [`#[cgp_namespace]`](../macros/cgp_namespace.md) macro and the `namespace` / `for … in` syntax of [`delegate_components!`](../macros/delegate_components.md). A user writing namespaces names them only in the namespace header and in the `for … in` loop target; the macros generate the impls and the forwarding.

## Definition

All three are key-value lookup traits carrying a single `Delegate` associated type, differing only in how many lookup-type parameters precede the `Components` table parameter:

```rust
pub trait DefaultNamespace<Components> {
    type Delegate;
}

pub trait DefaultImpls1<T, Components> {
    type Delegate;
}

pub trait DefaultImpls2<T1, T2, Components> {
    type Delegate;
}
```

`Self` is the key being looked up — a component-name type for `DefaultNamespace`, and the component-name type for the `DefaultImpls` variants too, with the per-instance types carried as the leading parameters. `Components` is the table the lookup is performed against, threaded through so that the same key can resolve differently depending on which context's table is consulted. The leading `T`, or `T1`/`T2`, are the instance types a per-type or per-pair default is keyed on. `Delegate` is the resolved value: the provider (or further redirect) the key maps to. As with [`DelegateComponent`](delegate_component.md), there is no method and no data; resolution is the projection of `Delegate` from the matching impl.

## Behavior

Each trait is implemented once per default entry, and resolving a default is reading `Delegate` from the matching impl. `DefaultNamespace<Components>` is implemented for a component-name key when a namespace supplies a default for that component regardless of any type parameter; the [`#[prefix(...)]`](../macros/cgp_namespace.md) attribute that attaches a component to a namespace emits exactly such an impl, with `Delegate` a [`RedirectLookup`](../providers/redirect_lookup.md) that re-routes the lookup along a path. `DefaultImpls1<T, Components>` is implemented for a component-name key carrying one instance type `T`, so the same component resolves per type; the [`#[default_impl(T in DefaultImpls1<Component>)]`](../macros/cgp_namespace.md) attribute on a provider impl registers the provider as the default for that `T`, emitting `impl<Components> DefaultImpls1<Component, Components> for T { type Delegate = Provider; }`. `DefaultImpls2` does the same with two instance types for a pair-parameterized component.

The hierarchical part is how a context consumes these defaults, which the [`delegate_components!`](../macros/delegate_components.md) `namespace` header and `for … in` syntax generate. A `namespace N;` header emits a blanket [`DelegateComponent`](delegate_component.md) impl on the context that forwards every key through `N`: `impl<Key, Value> DelegateComponent<Key> for App where Key: N<App, Delegate = Value> { type Delegate = Value; }`, paired with the matching [`IsProviderFor`](is_provider_for.md) forwarding so dependencies stay diagnosable. A `for <T, Provider> in DefaultImpls1<Component> { … }` loop emits a `DelegateComponent` impl keyed on a path whose `where` clause projects the default: `where T: DefaultImpls1<Component, App, Delegate = Provider>`. Reading the loop: for each type `T` that has a `DefaultImpls1` default, wire that path to the projected `Provider`. The same loop works against a `DefaultNamespace`-style table or any namespace trait by changing the `in` target.

Inheritance and override compose on top. A namespace that inherits from a parent (`new Child: DefaultNamespace { … }`) emits a blanket impl forwarding any key the parent resolves to the child, so the child resolves everything the parent does plus its own entries. A context's directly-wired entry resolves before the namespace fallback, so it shadows the inherited default for that key without disturbing the rest — the inheritance-with-override pattern presets rely on, expressed entirely through these projections with no runtime cost.

## Examples

A per-type default registered with `#[default_impl]` and then pulled into a context shows the chain. A provider declares itself the default for one type:

```rust
use cgp::prelude::*;
use core::fmt::Display;

#[cgp_component(ShowImpl)]
#[prefix(@test in DefaultNamespace)]
pub trait Show<T> {
    fn show(&self, value: &T) -> String;
}

#[cgp_impl(new ShowString)]
#[default_impl(String in DefaultImpls1<ShowImplComponent>)]
impl ShowImpl<String> {
    fn show(&self, value: &String) -> String {
        value.clone()
    }
}
```

The `#[default_impl]` attribute emits `impl<Components> DefaultImpls1<ShowImplComponent, Components> for String { type Delegate = ShowString; }`, registering `ShowString` as the per-type default for `String`. A context then joins the namespace and pulls those defaults in with a `for … in` loop, optionally overriding one entry:

```rust
pub struct App;

delegate_components! {
    App {
        namespace DefaultNamespace;

        for <T, Provider> in DefaultImpls1<ShowImplComponent> {
            @test.ShowImplComponent.T: Provider,
        }

        @test.ShowImplComponent.u64:
            ShowWithDisplay, // overrides the inherited default for u64
    }
}
```

The `namespace DefaultNamespace;` line forwards `App`'s lookups through `DefaultNamespace<App>`, and the loop wires each `T` by projecting `T: DefaultImpls1<ShowImplComponent, App, Delegate = Provider>`. The direct `u64` line shadows whatever the namespace would otherwise supply for that type. A namespace can also be defined wholesale and used as the loop target:

```rust
cgp_namespace! {
    new DefaultShowComponents {
        [String, u64]: ShowWithDisplay,
    }
}
```

Pointing a `for <T, Provider> in DefaultShowComponents { … }` loop at this namespace wires the listed types to `ShowWithDisplay` through the same projection mechanism.

## Related constructs

`DefaultNamespace`, `DefaultImpls1`, and `DefaultImpls2` are the lookup traits the [`#[cgp_namespace]`](../macros/cgp_namespace.md) macro builds on, and they are consumed by the `namespace` header and `for … in` loop of [`delegate_components!`](../macros/delegate_components.md). Their `Delegate` entries are commonly a [`RedirectLookup`](../providers/redirect_lookup.md), which re-routes a lookup along a type-level path rather than naming a provider outright. A context's `namespace` header forwards through these traits into a blanket [`DelegateComponent`](delegate_component.md) impl, with the matching [`IsProviderFor`](is_provider_for.md) forwarding so dependency errors stay readable. For the broader picture of how namespaces and presets fit together, see [namespaces](../../concepts/namespaces.md).

## Source

The three traits are defined in [crates/core/cgp-component/src/namespaces.rs](../../../crates/core/cgp-component/src/namespaces.rs), with `DefaultNamespace` re-exported through [crates/core/cgp-component/src/macro_prelude.rs](../../../crates/core/cgp-component/src/macro_prelude.rs). The namespace macro that builds the namespace trait and its inheritance impl lives in [crates/macros/cgp-macro-core/src/types/namespace/](../../../crates/macros/cgp-macro-core/src/types/namespace/); the `#[default_impl(... in DefaultImpls1<...>)]` attribute that registers a per-type default is parsed and lowered in [crates/macros/cgp-macro-core/src/types/attributes/default_impl/](../../../crates/macros/cgp-macro-core/src/types/attributes/default_impl/). The `namespace` header and `for … in` loop are handled by the `delegate_components!` codegen in [crates/macros/cgp-macro-core/src/types/delegate_component/](../../../crates/macros/cgp-macro-core/src/types/delegate_component/). Expansion snapshots covering `DefaultNamespace`, `DefaultImpls1`, the `for … in` loop, and namespace inheritance are in [crates/tests/cgp-tests/tests/namespaces/](../../../crates/tests/cgp-tests/tests/namespaces/).
