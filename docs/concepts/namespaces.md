# Namespaces

A namespace is a reusable, named lookup table of component wirings that a context can inherit wholesale and then selectively override, giving CGP its preset-style configuration without any separate preset construct.

## The idea

A namespace lifts a delegation table out of any one context, gives it a name, and lets other contexts inherit it. With [`delegate_components!`](../reference/macros/delegate_components.md) alone, every context spells out its own wiring entry by entry, and two contexts that should share the same set of providers must repeat it. A namespace captures "this exact set of wirings" as a thing contexts can refer to: a context says "use everything in this namespace" and gets the whole group at once. It is the answer to the question "how do I reuse a block of wiring across many contexts," and it is defined with [`cgp_namespace!`](../reference/macros/cgp_namespace.md).

The defining feature is inheritance with override. A context that joins a namespace inherits its entries as defaults, then adds its own entries that win over the inherited ones — because a directly-wired entry on the context resolves before the namespace fallback is consulted. So a namespace behaves like a base configuration that contexts specialize: most of the wiring comes for free, and each context tweaks the handful of entries it cares about. Namespaces can also inherit from one another, so a base namespace can be extended into a richer one that every context downstream picks up.

Crucially, a namespace is *not* a context. It is a trait — named after the namespace — that carries a `Delegate` associated type and is implemented once per key. A context opts in and forwards its lookups through that trait, so the namespace supplies defaults without ever being instantiated or holding any wiring of its own at the context level.

## Path-based redirection

What lets one namespace inherit from another, and lets a context shadow a single inherited entry without disturbing the rest, is that the forwarding is keyed by a *path* rather than a bare component name. A path is a type-level list of symbols and component names — written with the `@` sigil as a dotted sequence like `@MyFooComponent`, `@app.ErrorRaiserComponent`, or `@cgp.core.error` — and each namespace entry redirects a key along such a path instead of naming a provider outright. The redirection is carried by the [`RedirectLookup`](../reference/providers/redirect_lookup.md) provider, which resolves a key by walking the given path inside whatever table it is handed:

```rust
cgp_namespace! {
    new MyNamespace {
        FooProviderComponent =>
            @MyFooComponent,
    }
}
```

This says that when `MyNamespace` is asked for `FooProviderComponent`, it should look up the path `MyFooComponent` rather than resolve to a fixed provider — the actual provider is decided wherever the path eventually lands. Because lookups are paths rather than flat keys, a parent namespace's entire subtree can be rerouted at once (`@cgp.core.error => @app` redirects everything under that prefix), and a child context can introduce a more specific path that takes precedence over an inherited one. Under the hood every `@` path desugars into a [`PathCons`](../reference/types/path_cons.md) type-level list built by the [`Path!`](../reference/macros/path.md) macro, with lowercase dotted segments becoming `Symbol` string literals and capitalized segments becoming named types; the `=>` entries become `RedirectLookup` impls while plain `:` entries map a key straight to a provider as in `delegate_components!`.

## Per-component dispatch with `open`

The most common place a path appears is the `open` statement, which uses path-based redirection to dispatch a single component on its generic parameter — inline, in a context's own table. Writing `open { ValueSerializerComponent };` at the head of a [`delegate_components!`](../reference/macros/delegate_components.md) block redirects that component's lookup along a path rooted at the component name into the context's own table; the per-value entries that follow are then ordinary `@`-path keys pointing into that route:

```rust
delegate_components! {
    AppA {
        open {ValueSerializerComponent};

        @ValueSerializerComponent.Vec<u8>:
            SerializeHex,
        @ValueSerializerComponent.DateTime<Utc>:
            SerializeRfc3339Date,
    }
}
```

Each `@ValueSerializerComponent.Vec<u8>: SerializeHex` entry maps one value of the component's dispatch parameter — here the `Vec<u8>` serialized type — to the provider that handles it, and the [`RedirectLookup`](../reference/providers/redirect_lookup.md) impl every component carries appends that parameter onto the redirect path to find the entry. The effect is the per-type dispatch that the legacy `UseDelegate` nested table also provides, but with the entries living on the context rather than in a separate table type, so no `UseDelegate` wrapper or `#[derive_delegate]` attribute is involved. This is the form the [modular serialization](../examples/modular-serialization.md) example uses throughout.

`open` is a lightweight form of the full namespace machinery, suited to small applications and self-contained examples where a single context wires its own components directly. It roots a component's route at the bare component name and adds the per-value entries to that context's own table, without joining any shared namespace. The full namespace feature is what scales to a large code base: a library registers each component into a shared namespace under a path prefix with the `#[prefix(...)]` attribute, and many contexts join that namespace with a `namespace …;` header to inherit the whole bundle of wirings at once — the inherit-and-override pattern the rest of this page describes.

The two forms do not combine for the same component. When a context joins a namespace in which a component carries a prefix, that component's lookups are already routed under the prefix path, so `open`-ing it — which would root the route at the bare component name — no longer reaches those entries. The per-value entries must instead be written with the full prefixed path, `@prefix.SomeComponent.Key: Provider`, exactly as the override entries shown later do. So `open` is the convenience for a component wired directly on a context; once a component lives behind a namespace prefix, the full path is what reaches it.

## Attaching components and joining namespaces

A namespace is consumed from two sides: components register themselves into it, and contexts join it. A component attaches to a namespace through the `#[prefix(...)]` attribute on its trait, which emits one extra impl registering the component into the named namespace under a path prefix. CGP's own [`HasErrorType`](../reference/components/has_error_type.md), for instance, carries `#[prefix(@cgp.core.error in DefaultNamespace)]`, placing it into the built-in `DefaultNamespace` under the `cgp.core.error` prefix so any context joining that namespace inherits the standard error wiring. A context joins a namespace inside [`delegate_components!`](../reference/macros/delegate_components.md) with a `namespace` header line, after which every lookup it cannot resolve directly forwards through the namespace:

```rust
delegate_components! {
    AppA {
        namespace DefaultNamespace;

        @test.ShowImplComponent.u64:
            ShowWithDisplay,   // a direct entry overriding the namespace default
    }
}
```

The `namespace DefaultNamespace;` line makes `AppA` fall back to `DefaultNamespace`'s entries, while the direct line on the same context shadows just the `u64` entry — the override-by-precedence rule in action. Joining through [`delegate_and_check_components!`](../reference/macros/delegate_and_check_components.md) instead does the same while verifying the merged wiring.

## Preset-style configuration

Namespaces are how CGP expresses presets, and there is no separate `cgp_preset!` macro — a preset *is* a namespace. The pattern a preset library would offer elsewhere, "a curated bundle of defaults you adopt and then customize," is exactly the inherit-wholesale-and-override behavior namespaces provide. A library publishes a namespace of sensible default wirings, possibly extended from a base namespace; an application joins it, inherits the bundle, and overrides the few entries specific to its needs:

```rust
cgp_namespace! {
    new ExtendedNamespace: DefaultNamespace {
        @cgp.core.error =>
            @app,
    }
}
```

`ExtendedNamespace` inherits everything `DefaultNamespace` resolves and additionally reroutes the `@cgp.core.error` subtree to `@app`, so any context joining `ExtendedNamespace` gets the merged result. Because this is all expressed through trait resolution and type-level paths, the configuration has no runtime cost: the inheritance, the overrides, and the redirections are resolved entirely at compile time, and a preset is just one more namespace in the chain.

## Related constructs

Namespaces are defined with [`cgp_namespace!`](../reference/macros/cgp_namespace.md), whose `#[prefix(...)]` attribute (on a [`#[cgp_component]`](../reference/macros/cgp_component.md) trait) registers a component into a namespace and whose entries are resolved through the [`RedirectLookup`](../reference/providers/redirect_lookup.md) provider. Inheritance and per-type default lookups go through the [`DefaultNamespace` / `DefaultImpls` traits](../reference/traits/default_namespace.md) in `cgp-component`. Every `@` path desugars into a [`PathCons`](../reference/types/path_cons.md) type-level list built by the [`Path!`](../reference/macros/path.md) macro. A context joins a namespace inside [`delegate_components!`](../reference/macros/delegate_components.md) via its `namespace` header, or [`delegate_and_check_components!`](../reference/macros/delegate_and_check_components.md) to join and verify at once; the underlying per-key table that `RedirectLookup` walks is [`DelegateComponent`](../reference/traits/delegate_component.md). There is no `cgp_preset!` macro — presets are expressed entirely through namespaces.

## Source

The macro is in [crates/macros/cgp-macro-core/src/types/namespace/](../../crates/macros/cgp-macro-core/src/types/namespace/), with the `#[prefix(...)]` attribute in [attributes/prefix.rs](../../crates/macros/cgp-macro-core/src/types/attributes/prefix.rs). The runtime `DefaultNamespace`/`DefaultImpls1`/`DefaultImpls2` traits are in [crates/core/cgp-component/src/namespaces.rs](../../crates/core/cgp-component/src/namespaces.rs) and `RedirectLookup` in [crates/core/cgp-component/src/providers/redirect_lookup.rs](../../crates/core/cgp-component/src/providers/redirect_lookup.rs); expansion snapshots are in [crates/tests/cgp-tests/tests/namespaces/](../../crates/tests/cgp-tests/tests/namespaces/).
