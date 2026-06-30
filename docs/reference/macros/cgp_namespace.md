# `#[cgp_namespace]`

`cgp_namespace!` defines a *namespace* — a reusable, named lookup table that maps component keys to providers — so that a concrete context can inherit a whole group of wirings at once and still override individual entries.

## Purpose

`cgp_namespace!` exists to make groups of component wirings reusable across contexts. With [`delegate_components!`](delegate_components.md) alone, every context spells out its own table entry by entry; two contexts that should share the same wiring must repeat it. A namespace lifts that table out of any single context and gives it a name, turning "this exact set of providers" into a thing other contexts can refer to and build on.

The mechanism that makes this work is a layer of indirection between a context's delegation table and the actual providers. A namespace is not itself a context; it is a trait (named after the namespace) carrying a `Delegate` associated type, implemented per key. A context that opts into a namespace forwards every lookup through that trait, so the namespace's entries become the context's defaults. The forwarding is keyed by a *path* — a type-level list of symbols and component names — rather than by a bare component name, which is what lets one namespace inherit from another and lets a context shadow a single inherited entry without disturbing the rest.

The payoff is preset-style configuration with selective override. A context can say "use everything in this namespace" and then add a handful of its own entries that win over the inherited ones, because a directly-wired entry on the context resolves before the namespace fallback is consulted. This is the same inheritance-with-override pattern presets rely on, expressed entirely through the trait system with no runtime cost.

## Syntax

`cgp_namespace!` is a function-like macro whose body resembles a `delegate_components!` table with an optional namespace header. The simplest form defines a fresh namespace with `new` and lists entries that map component keys to redirect paths:

```rust
cgp_namespace! {
    new MyNamespace {
        FooProviderComponent =>
            @MyFooComponent,
    }
}
```

The `new` keyword tells the macro to also emit the namespace's marker struct and its lookup trait; omit it only when those are already declared elsewhere. `MyNamespace` is the namespace name, which becomes both a trait and (with `new`) a backing struct. The entries inside the braces are the namespace's wiring.

Two distinct entry forms appear in the body, and they generate different table contents. A `=>` entry redirects a key to a path: `FooProviderComponent => @MyFooComponent` says "when this namespace is asked for `FooProviderComponent`, look up the path `@MyFooComponent` instead." A `:` entry maps a key directly to a provider, as in `delegate_components!`: `[String, u64]: ShowWithDisplay` makes the namespace resolve those keys straight to the `ShowWithDisplay` provider. Paths written with the `@` sigil — `@MyFooComponent`, `@app.ErrorRaiserComponent`, `@cgp.core.error` — are dotted sequences of symbols and type names that desugar into type-level path lists.

A namespace can inherit from a parent namespace by naming it after a colon in the header:

```rust
cgp_namespace! {
    new ExtendedNamespace: DefaultNamespace {
        @cgp.core.error =>
            @app,
    }
}
```

Here `ExtendedNamespace` inherits every entry of `DefaultNamespace` and additionally rewrites the `@cgp.core.error` path prefix to `@app`. The parent may itself be parameterized (it is parsed as a path with type arguments), and the entries in the child body layer on top of the inherited ones.

Defining a namespace is only half of the pattern; a context joins a namespace through `delegate_components!` using a `namespace` header line, and individual components attach to a namespace through the [`#[prefix(...)]`](cgp_component.md) attribute on their trait. Those two constructs are where namespaces are consumed, and they are described under Examples and Related constructs.

## Syntax Grammar

The body of `cgp_namespace!` is an optional generic list and `new` keyword, a namespace name, an optional parent namespace, and a brace-delimited table:

```ebnf
CgpNamespace    -> Generics? `new`? NamespaceName ( `:` ParentNamespace )? `{` NamespaceBody `}`

NamespaceName   -> IDENTIFIER GenericArgs?
ParentNamespace -> TypePath GenericArgs?

NamespaceBody   -> Statement* ( Mapping ( `,` Mapping )* `,`? )?
```

The mappings in `NamespaceBody` are the same `Mapping` production as [`delegate_components!`](delegate_components.md) — most often the `` `=>` `` redirect to an `@`-`Path` or a `` `:` `` direct provider. The `` `:` `` between `NamespaceName` and `ParentNamespace` is the inheritance colon, distinct from a mapping's `:`. `NamespaceName` is an identifier with optional generic arguments (it becomes both a trait and, with `new`, a struct); `ParentNamespace` is a type path that may itself be parameterized.

This macro also owns the two namespace statement forms that a context's [`delegate_components!`](delegate_components.md) table uses to join a namespace:

```ebnf
Statement     -> NamespaceStmt | ForStmt

NamespaceStmt -> `namespace` IDENTIFIER `;`

ForStmt       -> `for` `<` IDENTIFIER `,` IDENTIFIER `>` `in` TypePath WhereClause?
                 `{` ( NormalMapping ( `,` NormalMapping )* `,`? )? `}`

NormalMapping -> Key `:` ProviderValue
```

A `NamespaceStmt` forwards every lookup on the table through the named namespace. A `ForStmt` binds a key variable and a provider variable, reads each entry of the table named after `in`, and emits one mapping per entry — its body holds only `` `:` `` mappings (`NormalMapping`), whose `Key` and `ProviderValue` are the shared productions from [`delegate_components!`](delegate_components.md). `TypePath` and `WhereClause` are Rust grammar productions.

## Expansion

`cgp_namespace!` emits, in order, an optional marker struct, an optional lookup trait, and one `impl` of that trait per entry (plus one inheritance `impl` when a parent is named). Take the `new` namespace with a single redirect entry:

```rust
cgp_namespace! {
    new MyNamespace {
        FooProviderComponent =>
            @MyFooComponent,
    }
}
```

Because `new` is present, the macro first emits a backing struct whose name is the namespace name wrapped in `__…Components`, then the lookup trait. The trait carries the table's generic `__Table__` parameter and a single `Delegate` associated type:

```rust
pub struct __MyNamespaceComponents;

pub trait MyNamespace<__Table__> {
    type Delegate;
}
```

Each `=>` entry becomes an `impl` of that trait for the entry's key, whose `Delegate` is a [`RedirectLookup`](../providers/redirect_lookup.md) pointing the table at the entry's path. The `@MyFooComponent` path desugars into a `PathCons<…, Nil>` type-level list:

```rust
impl<__Table__> MyNamespace<__Table__> for FooProviderComponent {
    type Delegate = RedirectLookup<__Table__, PathCons<MyFooComponent, Nil>>;
}
```

Reading this back: `MyNamespace<__Table__>::Delegate` for the key `FooProviderComponent` is "look up the path `MyFooComponent` inside whatever table `__Table__` is." `RedirectLookup<Components, Path>` is a CGP-defined provider that resolves by delegating `Components` along `Path`; the namespace never names a concrete provider for this key, it only re-routes the lookup, so the actual provider is decided wherever the path eventually lands.

A `:` entry instead maps the key directly to the named provider, with no `RedirectLookup` indirection. From the array form `[String, u64]: ShowWithDisplay`, the macro emits one `impl` per key:

```rust
impl<__Table__> DefaultShowComponents<__Table__> for String {
    type Delegate = ShowWithDisplay;
}
impl<__Table__> DefaultShowComponents<__Table__> for u64 {
    type Delegate = ShowWithDisplay;
}
```

When a parent namespace is named, the macro prepends one extra blanket `impl` that forwards unmatched keys to the parent. For `new ExtendedNamespace: DefaultNamespace { … }`, the inherited entries arrive through this impl:

```rust
impl<__Table__, __Key__, __Value__> ExtendedNamespace<__Table__> for __Key__
where
    __Key__: DefaultNamespace<__ExtendedNamespaceComponents>,
    __Key__: DefaultNamespace<__Table__, Delegate = __Value__>,
{
    type Delegate = __Value__;
}
```

This says: for any `__Key__` the parent `DefaultNamespace` resolves, `ExtendedNamespace` resolves it to the same `__Value__`. The body entries of the child are emitted after this blanket impl and take precedence where their keys are more specific. The path-rewriting entry `@cgp.core.error => @app` becomes an impl keyed on the `cgp.core.error` path prefix whose `Delegate` is a `RedirectLookup` onto the `@app` prefix — rerouting an entire subtree of the parent's namespace rather than a single component.

The other half of the pattern is what attaches a component to a namespace, via [`#[prefix(...)]`](cgp_component.md) on the component's trait. Given:

```rust
#[cgp_component(BarProvider)]
#[prefix(@MyBarComponent in MyNamespace)]
pub trait Bar {
    fn bar(&self);
}
```

`#[cgp_component]` emits its usual items, and `#[prefix]` adds one extra impl that registers `BarProviderComponent` into `MyNamespace` under the prefix path `@MyBarComponent`:

```rust
impl<__Components__> MyNamespace<__Components__> for BarProviderComponent {
    type Delegate = RedirectLookup<
        __Components__,
        PathCons<MyBarComponent, PathCons<BarProviderComponent, Nil>>,
    >;
}
```

So `MyNamespace`, asked for `BarProviderComponent`, redirects the lookup to the path `MyBarComponent → BarProviderComponent`. A component may carry several `#[prefix]` attributes to register itself into several namespaces at once.

Two details of the expansion are worth holding onto. The table parameter is literally named `__Table__` and the inheritance blanket impl uses `__Key__`/`__Value__`; the examples keep those names because they appear verbatim in compiler errors. And every path under `@` becomes a `PathCons`/`Symbol`/`Chars` type-level list — `@my_app.MyFooComponent` expands to `PathCons<Symbol<6, Chars<'m', …>>, PathCons<MyFooComponent, Nil>>`, with dotted lowercase segments becoming `Symbol` string literals and capitalized segments becoming the named type.

## Examples

A namespace becomes useful once a context joins it and overrides part of it. Start with a namespace that supplies default per-type providers, defined with `new`:

```rust
use cgp::prelude::*;

cgp_namespace! {
    new DefaultShowComponents {
        [String, u64]: ShowWithDisplay,
    }
}
```

A context then opts into a namespace inside `delegate_components!` with a `namespace` header line, and may add its own entries that win over the namespace defaults. Joining `DefaultNamespace` and pulling defaults in through a `for` loop over `DefaultShowComponents`:

```rust
pub struct AppB;

delegate_components! {
    AppB {
        namespace DefaultNamespace;

        for <T, Provider> in DefaultShowComponents {
            @test.ShowImplComponent.T: Provider,
        }
    }
}
```

The `namespace DefaultNamespace;` line makes `AppB` forward every component lookup through `DefaultNamespace<AppB>`, and the `for … in DefaultShowComponents` block wires `AppB`'s `ShowImplComponent` entries by reading `DefaultShowComponents`'s `Delegate` for each type `T`. To override a single entry, a later direct line on the same context simply names a different provider for that key; because the context's own entry resolves before the namespace fallback, it shadows the inherited one without touching the others:

```rust
delegate_components! {
    AppA {
        namespace DefaultNamespace;

        for <T, Provider> in DefaultImpls1<ShowImplComponent> {
            @test.ShowImplComponent.T: Provider,
        }

        @test.ShowImplComponent.u64:
            ShowWithDisplay,   // overrides the inherited entry for u64
    }
}
```

Inheritance composes the same way at the namespace level: `ExtendedNamespace: DefaultNamespace` produces a namespace that resolves everything `DefaultNamespace` does, plus the child's own entries, and any context joining `ExtendedNamespace` gets the merged result.

## Related constructs

`cgp_namespace!` sits between component definitions and context wiring, so it relates to constructs on both sides. [`#[cgp_component]`](cgp_component.md) defines the components whose keys a namespace maps, and its [`#[prefix(...)]`](cgp_component.md) attribute is what registers a component into a namespace under a path. [`delegate_components!`](delegate_components.md) is where a context joins a namespace (via its `namespace` header) and where individual overrides are written; [`delegate_and_check_components!`](delegate_and_check_components.md) does the same while also verifying the resulting wiring. The namespace's `Delegate` entries are resolved through [`RedirectLookup`](../providers/redirect_lookup.md), and per-type defaults are commonly expressed through [`use_delegate`](../providers/use_delegate.md)-style dispatch and the `DefaultNamespace` / `DefaultImpls1` traits in `cgp-component`. The underlying per-key table machinery is [`DelegateComponent`](../traits/delegate_component.md), which `RedirectLookup` walks at resolution time.

## Source

The macro entry point is `cgp_namespace` in [crates/macros/cgp-macro-lib/src/cgp_namespace.rs](../../../crates/macros/cgp-macro-lib/src/cgp_namespace.rs), which parses a `NamespaceTable` and calls `.eval()`. The logic lives in [crates/macros/cgp-macro-core/src/types/namespace/](../../../crates/macros/cgp-macro-core/src/types/namespace/): `table.rs` parses the header (`new`, namespace name, optional `: parent`) and builds the trait, struct, per-entry impls, and the parent-inheritance impl; `inherit.rs` builds the path-rewriting inheritance entry; `eval.rs` holds the emitted `EvaluatedNamespaceTable`. The `#[prefix(...)]` attribute that attaches a component to a namespace is parsed in [crates/macros/cgp-macro-core/src/types/attributes/prefix.rs](../../../crates/macros/cgp-macro-core/src/types/attributes/prefix.rs), and the matching `RedirectLookup` provider impl is emitted by [crates/macros/cgp-macro-core/src/types/cgp_component/evaluated/to_redirect_lookup_impl.rs](../../../crates/macros/cgp-macro-core/src/types/cgp_component/evaluated/to_redirect_lookup_impl.rs). The runtime traits `DefaultNamespace`/`DefaultImpls1`/`DefaultImpls2` are in [crates/core/cgp-component/src/namespaces.rs](../../../crates/core/cgp-component/src/namespaces.rs) and `RedirectLookup` in [crates/core/cgp-component/src/providers/redirect_lookup.rs](../../../crates/core/cgp-component/src/providers/redirect_lookup.rs). Expansion snapshots covering the basic, symbol-path, multi-namespace, extended, and default-impls cases are in [crates/tests/cgp-tests/tests/namespace_tests/namespace_macro/](../../../crates/tests/cgp-tests/tests/namespace_tests/namespace_macro/) and [crates/tests/cgp-tests/src/namespaces/](../../../crates/tests/cgp-tests/src/namespaces/).
