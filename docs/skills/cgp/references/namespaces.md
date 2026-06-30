# Namespaces

A namespace is a reusable, named lookup table of component wirings that a context inherits wholesale and then selectively overrides — CGP's preset mechanism, expressed entirely at the type level with no runtime cost.

As the component count of an application grows, the [wiring](wiring.md) on each context grows with it: every context spells out its own `delegate_components!` table entry by entry, and two contexts that should share the same set of providers must repeat the whole block. A namespace lifts that block out of any single context, gives it a name, and lets other contexts say "use everything in this namespace" to pull in the entire group at once. This is exactly the preset pattern — a curated bundle of defaults you adopt and then customize — and CGP has no separate `cgp_preset!` construct because a preset *is* a namespace. This file covers `#[cgp_namespace]` (defining and inheriting a namespace), the `RedirectLookup` provider that makes the indirection work, the `Path!` macro that addresses entries, and the `DefaultNamespace` family of default-resolution traits.

## What a namespace is

A namespace is not a context — it is a trait, named after the namespace, that carries a single `Delegate` associated type and is implemented once per key. A context that joins a namespace forwards its lookups through that trait, so the namespace supplies defaults without ever being instantiated or holding any wiring at the context level. The defining behavior is inheritance with override: a context inherits the namespace's entries as defaults, and any entry it wires directly on itself wins, because a context's own [`DelegateComponent`](wiring.md) entry resolves before the namespace fallback is consulted.

The forwarding is keyed by a *path* rather than a bare component name, and that is what makes inheritance and selective override possible. A path is a type-level list of symbols and component names; keying on it lets one namespace inherit from another, lets a parent's whole subtree be rerouted at once, and lets a child context shadow a single inherited entry without disturbing the rest.

## Defining a namespace with `#[cgp_namespace]`

`cgp_namespace!` defines a namespace from a body that resembles a `delegate_components!` table. The `new` keyword tells the macro to emit the namespace's marker struct and its lookup trait; the entries inside map keys to redirect paths or to providers:

```rust
cgp_namespace! {
    new DefaultShowComponents {
        [String, u64]: ShowWithDisplay,
    }
}
```

Two entry forms appear in the body and generate different table contents. A `:` entry maps a key straight to a provider, exactly as in `delegate_components!` — the `[String, u64]: ShowWithDisplay` line above resolves both keys to the `ShowWithDisplay` provider. A `=>` entry instead redirects a key along a path: `FooProviderComponent => @MyFooComponent` says "when this namespace is asked for `FooProviderComponent`, look up the path `@MyFooComponent` instead of naming a provider outright," leaving the actual provider to be decided wherever the path lands.

A namespace inherits from a parent by naming it after a colon in the header. The child then resolves everything the parent does, plus its own entries:

```rust
cgp_namespace! {
    new ExtendedNamespace: DefaultNamespace {
        @cgp.core.error =>
            @app,
    }
}
```

`ExtendedNamespace` inherits every entry `DefaultNamespace` resolves and additionally reroutes the entire `@cgp.core.error` subtree to `@app` — a single path-rewriting entry redirects a whole prefix of the parent namespace at once, not just one component.

### What the macro generates

With `new` present, the macro emits a backing struct (named with an `__…Components` wrapper) and the lookup trait carrying the table's `__Table__` generic parameter and a `Delegate` associated type:

```rust
pub struct __MyNamespaceComponents;

pub trait MyNamespace<__Table__> {
    type Delegate;
}
```

Each `=>` entry becomes an `impl` of that trait for the entry's key, whose `Delegate` is a `RedirectLookup` pointing the table at the entry's path; each `:` entry becomes an `impl` whose `Delegate` is the named provider directly. For `FooProviderComponent => @MyFooComponent` the macro emits:

```rust
impl<__Table__> MyNamespace<__Table__> for FooProviderComponent {
    type Delegate = RedirectLookup<__Table__, PathCons<MyFooComponent, Nil>>;
}
```

When a parent is named, the macro prepends a blanket impl that forwards every key the parent resolves down to the child, so the child inherits the parent's full table; the child's own entries are emitted after it and take precedence where their keys are more specific.

## Attaching components and joining namespaces

A namespace is consumed from two sides: components register into it, and contexts join it. A [component](components.md) attaches to a namespace through the `#[prefix(@path in Namespace)]` attribute on its [`#[cgp_component]`](components.md) trait, which emits one extra impl registering the component into the named namespace under a path prefix. CGP's own [`HasErrorType`](components.md), for example, carries `#[prefix(@cgp.core.error in DefaultNamespace)]`, placing the standard error wiring into the built-in `DefaultNamespace` so any context joining that namespace inherits it. The generated impl routes the component's lookup under the prefix path:

```rust
impl<__Components__> MyNamespace<__Components__> for BarProviderComponent {
    type Delegate = RedirectLookup<
        __Components__,
        PathCons<MyBarComponent, PathCons<BarProviderComponent, Nil>>,
    >;
}
```

A context joins a namespace inside `delegate_components!` with a `namespace` header line, after which every lookup it cannot resolve directly forwards through the namespace. A direct entry on the same context shadows just that key, leaving the rest of the inherited wiring intact:

```rust
delegate_components! {
    AppA {
        namespace DefaultNamespace;

        @test.ShowImplComponent.u64:
            ShowWithDisplay,   // overrides only the u64 entry
    }
}
```

The `namespace DefaultNamespace;` line emits a blanket `DelegateComponent` impl on `AppA` that forwards every key through `DefaultNamespace<AppA>`, paired with the matching `IsProviderFor` forwarding so dependency errors stay diagnosable through [checking](checking.md). The direct `@test.ShowImplComponent.u64` line resolves first, so it wins for `u64` only — the inherit-and-override pattern in action. Joining through `delegate_and_check_components!` instead does the same while verifying the merged wiring.

## Paths with `Path!`

A path is the type-level address that namespace entries redirect along, and `Path!` is the macro that builds one from a readable dotted, `@`-prefixed form. Each segment narrows the lookup one step — through a namespace, through a prefix, down to a component key — and the leading `@` is the sigil marking the body as a path rather than a plain type:

```rust
type ErrorRoute = Path!(@app.error.ErrorRaiserComponent);
// PathCons<Symbol!("app"),
//     PathCons<Symbol!("error"),
//         PathCons<ErrorRaiserComponent, Nil>>>
```

The encoding of each segment is decided by its first character: a single lowercase identifier that is not a primitive type name (like `app` or `error`) becomes a [`Symbol`](abstract-types.md) type-level string, while every capitalized segment (like `ErrorRaiserComponent`) is kept as the named type — typically a component key or namespace marker. The macro folds the segments right-to-left onto `Nil`, wrapping each in a `PathCons`. You rarely call `Path!` directly; the same `@`-path syntax is embedded inside `#[cgp_namespace]` entries and `#[prefix(...)]` attributes, which is where paths are most often written. These [type-level primitives](type-level-primitives.md) carry no runtime value.

## The `RedirectLookup` provider

`RedirectLookup<Components, Path>` is the [provider](components.md) — a zero-sized marker struct, no runtime value — that turns a path-addressed entry back into a concrete provider, and it is the mechanism every namespace runs on under the hood. The ordinary provider blanket impl looks a component up in the context's own table keyed by the component-name marker; `RedirectLookup` instead consults a table keyed by an arbitrary type-level path, then delegates to whatever provider that entry holds. This decouples *which key* a component is looked up under from *which table* answers it — the basis for organizing wiring into namespaces.

`RedirectLookup` is never written by hand; it is emitted by the macros. Every `#[cgp_component]` generates a `RedirectLookup` impl of its provider trait, and the namespace `=>` entries and `#[prefix]` attributes generate entries whose `Delegate` is a `RedirectLookup`. The generated impl performs one `DelegateComponent` lookup keyed on the path rather than on the component name:

```rust
impl<__Context__, __Components__, __Path__> Greeter<__Context__>
    for RedirectLookup<__Components__, __Path__>
where
    __Components__: DelegateComponent<__Path__>,
    <__Components__ as DelegateComponent<__Path__>>::Delegate: Greeter<__Context__>,
{
    fn greet(__context__: &__Context__) -> String {
        <__Components__ as DelegateComponent<__Path__>>::Delegate::greet(__context__)
    }
}
```

So `RedirectLookup` implements the provider trait whenever the table maps the path to a delegate that itself implements that trait, and forwards the call to it. When the provider trait carries a generic type parameter, the impl additionally appends that parameter onto the path before the lookup, so the redirected key can encode the generic argument — this is how per-type dispatch through a namespace works.

## Default resolution: the `DefaultNamespace` family

The traits that back namespaces come in three arities, differing only in how many type parameters take part in the key. `DefaultNamespace<Components>` keys a default purely on the component name; `DefaultImpls1<T, Components>` keys it on the component name *and* one further type — the shape for a per-type default where the same component resolves differently for `String` than for `u64` — and `DefaultImpls2` does the same for a pair. Each carries a single `Delegate` associated type and no method or data; resolution is the projection of `Delegate` from the matching impl:

```rust
pub trait DefaultNamespace<Components> {
    type Delegate;
}

pub trait DefaultImpls1<T, Components> {
    type Delegate;
}
```

`DefaultNamespace` is the built-in namespace that `#[prefix(... in DefaultNamespace)]` registers components into and that a context joins with `namespace DefaultNamespace;`. A per-type default is registered with the `#[default_impl(T in DefaultImpls1<Component>)]` attribute on a provider impl, which emits `impl<Components> DefaultImpls1<Component, Components> for T { type Delegate = Provider; }`. A context then pulls those defaults in with a `for … in` loop that projects the `Delegate` for each type:

```rust
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

The `for <T, Provider> in DefaultImpls1<ShowImplComponent>` loop wires each type `T` by reading `T: DefaultImpls1<ShowImplComponent, App, Delegate = Provider>`, and the direct `u64` line shadows whatever the namespace would otherwise supply for that type. The loop target can equally be a whole namespace defined with `cgp_namespace!`, such as the `DefaultShowComponents` namespace shown earlier — `for <T, Provider> in DefaultShowComponents { … }` wires its listed types through the same projection.

## Defining a preset once, reusing it across contexts

The payoff is that a bundle of wiring is defined once and reused everywhere. A library publishes a namespace of sensible defaults — possibly extended from a base namespace as `ExtendedNamespace: DefaultNamespace` does — and any number of applications join it, inherit the whole bundle, and override only the entries specific to their needs. Each context's table shrinks to a `namespace` header plus a short list of overrides, no matter how many components the namespace bundles. Because the inheritance, the overrides, and the redirections are all resolved through trait projection and type-level paths, the entire arrangement is resolved at compile time with no runtime cost — a preset is simply one more namespace in the chain.

## Related constructs

Namespaces build directly on [wiring](wiring.md): a namespace is a `DelegateComponent` table addressed by paths, and a context joins one inside `delegate_components!`. The keys a namespace maps are the `…Component` markers of [components](components.md), attached via the `#[prefix(...)]` attribute on a `#[cgp_component]` trait, and the `IsProviderFor` forwarding a `namespace` header emits feeds the completeness guarantees in [checking](checking.md). The `@`-paths and `Symbol` segments are [type-level primitives](type-level-primitives.md). The per-type dispatch `RedirectLookup` enables — appending a generic parameter onto the lookup path — is the same mechanism the type-parameter dispatch forms in [higher-order providers](higher-order-providers.md) use.

## Further reference

Online docs: [concepts/namespaces.md](https://github.com/contextgeneric/cgp/blob/main/docs/concepts/namespaces.md), [reference/macros/cgp_namespace.md](https://github.com/contextgeneric/cgp/blob/main/docs/reference/macros/cgp_namespace.md), [reference/providers/redirect_lookup.md](https://github.com/contextgeneric/cgp/blob/main/docs/reference/providers/redirect_lookup.md), [reference/macros/path.md](https://github.com/contextgeneric/cgp/blob/main/docs/reference/macros/path.md).
