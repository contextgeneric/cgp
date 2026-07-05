# Organizing wiring with namespaces and prefixes

A context's [`delegate_components!`](../reference/macros/delegate_components.md) table grows one entry at a time until it is the hardest thing in the codebase to read; this guide shows how to shrink it back down with path prefixes, namespaces, and per-type defaults, worked as a refactoring of a real application.

## The problem: a wiring table that outgrows its reader

The first thing a newcomer sees when they open an application's context module is its wiring table, and by the time the application does anything interesting that table is long enough to overwhelm them. Each component the application uses adds a line, each abstract type adds a line, and each per-type dispatch adds several. The table is mechanically correct and every entry is doing real work, but there is no structure to hold onto — it reads as one flat list of thirty unrelated facts, and a reader cannot tell at a glance which entries belong together or which are the ones they came to change.

Consider a small money-transfer web service. It has an authentication layer, a finance layer, an HTTP error-mapping layer, and an API layer, wired onto a single `MockApp` context whose backend is an in-memory mock. Written out entry by entry, its table looks like this:

```rust
delegate_components! {
    MockApp {
        open {
            HttpErrorRaiserComponent,
            ApiHandlerComponent,
        };

        ErrorTypeProviderComponent: UseType<AppError>,

        @HttpErrorRaiserComponent.<Code> Code.String:
            DisplayHttpError,
        @HttpErrorRaiserComponent.<Code> Code.anyhow::Error:
            HandleHttpErrorWithAnyhow,

        [
            UserIdTypeProviderComponent,
            PasswordTypeProviderComponent,
            HashedPasswordTypeProviderComponent,
        ]:
            UseType<String>,
        QuantityTypeProviderComponent:
            UseType<u64>,
        CurrencyTypeProviderComponent:
            UseType<DemoCurrency>,

        [
            PasswordCheckerComponent,
            UserHashedPasswordQuerierComponent,
            UserBalanceQuerierComponent,
        ]:
            UseMockedApp,
        MoneyTransferrerComponent:
            NoTransferToSelf<UseMockedApp>,

        @ApiHandlerComponent.QueryBalanceApi:
            HandleFromRequest<
                AxumQueryBalanceRequest,
                ResponseToJson<UseBasicAuth<HandleQueryBalance<QueryBalanceRequest>>>,
            >,
        @ApiHandlerComponent.TransferApi:
            HandleFromRequest<
                AxumTransferRequest,
                UseBasicAuth<HandleTransfer<TransferRequest>>,
            >,
    }
}
```

Every line here is necessary, and nothing about it is wrong. The problem is purely one of presentation: the table mixes error handling, abstract types, business logic, and API routing with no visible seam between them, and a second context that shared most of this wiring would have to copy the whole block. The rest of this guide refactors this exact table down to a handful of lines, introducing one technique at a time. The three techniques compose, and each is useful on its own, so you can stop at whichever level of organization your application needs.

## Technique 1: group components under path prefixes with `#[prefix]`

The first move is to give each component a **path** — a dotted address like `@app.auth` or `@app.finance` — so that related components sort together instead of scattering through the table. The [`#[prefix(@path in Namespace)]`](../reference/macros/cgp_namespace.md) attribute on a component's [`#[cgp_component]`](../reference/macros/cgp_component.md) (or [`#[cgp_type]`](../reference/macros/cgp_type.md)) trait registers that component into a namespace under a path prefix, so that from then on the component is addressed by its path rather than by its bare marker name. The standard namespace to register into is the built-in [`DefaultNamespace`](../reference/traits/default_namespace.md), which every context can join.

The abstract types divide cleanly by the layer that owns them, so they take a `types` sub-path under each layer:

```rust
#[cgp_type]
#[prefix(@app.auth.types in DefaultNamespace)]
pub trait HasUserIdType {
    type UserId: Display;
}

#[cgp_type]
#[prefix(@app.finance.types in DefaultNamespace)]
pub trait HasQuantityType {
    type Quantity: Display;
}
```

The capability components take the layer path directly, without the `types` segment, so the authentication logic sits under `@app.auth` and the finance logic under `@app.finance`:

```rust
#[cgp_component(PasswordChecker)]
#[prefix(@app.auth in DefaultNamespace)]
#[use_type(HasPasswordType.Password, HasHashedPasswordType.HashedPassword)]
pub trait CanCheckPassword {
    fn check_password(password: &Password, hashed_password: &HashedPassword) -> bool;
}

#[cgp_component(UserBalanceQuerier)]
#[prefix(@app.finance in DefaultNamespace)]
#[async_trait]
#[use_type(HasUserIdType.UserId, HasCurrencyType.Currency, HasQuantityType.Quantity, HasErrorType.Error)]
pub trait CanQueryUserBalance {
    async fn query_user_balance(&self, user: &UserId, currency: &Currency) -> Result<Quantity, Error>;
}
```

The HTTP error raiser and the API handler each get their own layer path, `@app.error` and `@app.api`. With every component prefixed, the application's whole namespace looks like a directory tree — `@app.auth.types`, `@app.auth`, `@app.finance.types`, `@app.finance`, `@app.error`, `@app.api` — and a reader can find the auth wiring without reading the finance wiring.

### Choosing prefixes for implementations you have not written yet

The prefix you choose depends less on how *this* application wires a component and more on whether a *different* implementation would ever wire it separately. The natural instinct is to group by the provider you happen to use: since one `UseType<String>` serves all three auth types in this application, you might put them wherever is convenient. Resist collapsing distinctions that a future implementation would need. A component author picks prefixes for every implementation the component might ever have, not just the one in front of them, because the prefix is part of the component's public surface and is expensive to change once downstream code depends on it.

The rule that follows is to **give components a separate sub-path whenever they are likely to need separate providers**, even when the current implementation happens to wire them the same way. The abstract types sit under `@app.auth.types` rather than sharing `@app.auth` with the logic, because a real backend would supply the auth *logic* (password checking, hashed-password lookup) very differently from how it supplies the auth *types* — the types are almost always plain `UseType<Concrete>` while the logic talks to a database. Keeping them on separate sub-paths means a production context can point `@app.auth` at a database provider while leaving `@app.auth.types` on the same concrete types, without either wiring disturbing the other. Grouping them together would have read fine today and forced them apart tomorrow.

## Technique 2: bind providers to a namespace so the context just joins it

Prefixes organize the table but do not shorten it — the context still names every path. The second technique lifts the wiring off the context entirely and into a reusable **namespace**, so that most contexts join the namespace with a single line and wire nothing directly. A namespace defined with [`cgp_namespace!`](../reference/macros/cgp_namespace.md) is a preset: a named table of default wirings a context inherits wholesale and then selectively overrides. (The [namespaces concept](../concepts/namespaces.md) explains the mechanism; this section is about how to *use* it to organize an application.)

Define one namespace for the application's mock backend, inheriting `DefaultNamespace` so a context that joins it also inherits every standard default:

```rust
cgp_namespace! {
    new MockNamespace: DefaultNamespace {
        @cgp.core.error.ErrorTypeProviderComponent:
            UseType<AppError>,

        @app.error.HttpErrorRaiserComponent.<Code> Code.String:
            DisplayHttpError,
        @app.error.HttpErrorRaiserComponent.<Code> Code.anyhow::Error:
            HandleHttpErrorWithAnyhow,

        @app.auth.types.{
            UserIdTypeProviderComponent,
            PasswordTypeProviderComponent,
            HashedPasswordTypeProviderComponent,
        }:
            UseType<String>,
        @app.finance.types.QuantityTypeProviderComponent:
            UseType<u64>,
        @app.finance.types.CurrencyTypeProviderComponent:
            UseType<DemoCurrency>,
    }
}
```

These entries are the wirings that have no [`#[cgp_impl]`](../reference/macros/cgp_impl.md) block of their own to attach an attribute to: the concrete error type, the HTTP error dispatch, and the abstract-type choices are all built from library providers like [`UseType`](../reference/providers/use_type.md), so they are written directly in the namespace **body**, keyed by their full paths. The body of a namespace accepts exactly the same key and value forms as `delegate_components!`, including grouped keys (`@app.auth.types.{A, B, C}`) and generic-parameter dispatch keys (`@app.error.HttpErrorRaiserComponent.<Code> Code.String`).

### Registering a provider with `#[default_impl]`

The application's business logic *does* have `#[cgp_impl]` blocks — the mock backend implements password checking, hashed-password lookup, and balance querying — so those providers register themselves into the namespace from their own definition, with the [`#[default_impl(@path in Namespace)]`](../reference/traits/default_namespace.md) attribute:

```rust
#[cgp_impl(UseMockedApp)]
#[default_impl(@app.auth.UserHashedPasswordQuerierComponent in MockNamespace)]
#[use_type(HasUserIdType.UserId, HasHashedPasswordType.HashedPassword, HasErrorType.Error)]
impl UserHashedPasswordQuerier
where
    UserId: Ord,
    HashedPassword: Clone,
{
    async fn query_user_hashed_password(
        &self,
        user_id: &UserId,
        #[implicit] user_passwords: &BTreeMap<UserId, HashedPassword>,
    ) -> Result<Option<HashedPassword>, Error> {
        Ok(user_passwords.get(user_id).cloned())
    }
}
```

`#[default_impl]` emits one extra impl that maps the path `@app.auth.UserHashedPasswordQuerierComponent` to `UseMockedApp` inside `MockNamespace`, without changing the provider itself. The effect is that the wiring lives next to the implementation it wires, which is where a reader looks for it, rather than in a distant table. A context that joins `MockNamespace` now resolves `UserHashedPasswordQuerier` to `UseMockedApp` automatically.

With the namespace carrying the whole backend, a context that wants the mock backend joins it with one statement:

```rust
delegate_components! {
    MockApp {
        namespace MockNamespace;
    }
}
```

Everything the earlier table spelled out by hand — the error type, the error dispatch, the abstract types, the auth and finance logic — now arrives through the namespace, and a second mock context would need only the same single line.

### Keep `#[prefix]` and `#[default_impl]` in different namespaces

A subtle but important rule governs which namespace each attribute names: **register a component's `#[prefix]` into a base namespace and its `#[default_impl]` into a namespace that inherits the base, never the same one.** In the example, every `#[prefix]` names `DefaultNamespace` while every `#[default_impl]` names `MockNamespace`, and `MockNamespace: DefaultNamespace` inherits the prefixes. This separation is not stylistic. A namespace's entry for a key, once defined, cannot be overridden — so if the prefixes and the mock defaults shared one namespace, a *second* backend (a production one, say) could not reuse the prefixes without also inheriting the mock's providers, and would have to redeclare every prefix from scratch. Splitting them lets the prefix layer be reused by any number of backend namespaces, each supplying its own `#[default_impl]` bindings on top of the shared prefixes. Put the prefixes in the base namespace that describes the application's *structure*, and put each backend's provider choices in an inheriting namespace that describes one *configuration*.

## Technique 3: merge several namespaces in one table with a `for` loop

The API handlers are the one part of the wiring that is neither a plain library provider nor a `#[cgp_impl]` block — each is a hand-assembled pipeline of combinators — and they describe the application's public API surface rather than one backend's choices. That makes them a good fit for a *separate* namespace that any backend can pull in. Define the API surface as its own table keyed by the API marker:

```rust
cgp_namespace! {
    new DefaultApiHandlers {
        QueryBalanceApi:
            HandleFromRequest<
                AxumQueryBalanceRequest,
                ResponseToJson<UseBasicAuth<HandleQueryBalance<QueryBalanceRequest>>>,
            >,
        TransferApi:
            HandleFromRequest<
                AxumTransferRequest,
                UseBasicAuth<HandleTransfer<TransferRequest>>,
            >,
    }
}
```

A context then pulls this table onto its own `ApiHandler` dispatch path with a `for` loop, which reads each entry of the named table and emits one mapping per entry:

```rust
delegate_components! {
    MockApp {
        namespace MockNamespace;

        for <Key, Value> in DefaultApiHandlers {
            @app.api.ApiHandlerComponent.Key: Value,
        }
    }
}
```

The `for <Key, Value> in DefaultApiHandlers` loop binds each `QueryBalanceApi`/`TransferApi` entry as `Key` and its pipeline as `Value`, wiring `MockApp`'s `ApiHandler` for that API to that pipeline. This is how a single table draws on **more than one** namespace at once: `MockApp` joins `MockNamespace` for its backend *and* loops over `DefaultApiHandlers` for its API surface, keeping the two concerns in separate reusable tables while merging them onto one context.

One constraint shapes how the loop key is written: **the loop's bound key must appear inside a path, never as the whole key.** Writing `Key: Value` on its own would collide with the general `DelegateComponent<Key>` impl that the `namespace` statement already generates for every key; embedding it in a path as `@app.api.ApiHandlerComponent.Key` keeps it distinct. This is also why the loop is the natural tool for a component with a generic parameter — the API marker *is* the dispatch parameter of `ApiHandlerComponent`, so each looped entry lands on that component's per-type dispatch path.

## The payoff, and overriding through a namespace

The three techniques together turn the thirty-line opening table into four lines, and the difference is not just length — it is that each remaining line now states a *decision* rather than a mechanical fact:

```rust
delegate_components! {
    MockApp {
        namespace MockNamespace;

        for <Key, Value> in DefaultApiHandlers {
            @app.api.ApiHandlerComponent.Key: Value,
        }

        @app.finance.MoneyTransferrerComponent:
            NoTransferToSelf<UseMockedApp>,
    }
}
```

`MockApp` uses the mock backend, serves the default API surface, and — the one place it departs from the defaults — wraps money transfers in a `NoTransferToSelf` guard that rejects a transfer whose sender and recipient are the same account. That last entry is an **override**, and overriding through a namespace follows a rule worth stating outright: **a context can only wire a path that the namespace it joins does not itself register.** `MockNamespace` deliberately does *not* register `@app.finance.MoneyTransferrerComponent` — the base `MoneyTransferrer` provider is used only as the inner handler of the `NoTransferToSelf` wrapper, never registered on its own — so the context is free to wire that path directly. Had the namespace registered a provider at that exact path, the context's entry and the namespace's blanket forwarding would both implement `DelegateComponent` for that key, and the compiler would reject the overlap. To leave a path open for a context to override, route the component through the namespace but terminate the redirect on the context, not in the namespace.

## Limitations: why `#[default_impl]` is for the basic case

`#[default_impl]` is the most convenient of these tools and also the most constrained, and knowing where it stops keeps you from designing around it and then hitting a wall. Its central limitation is that **a `#[default_impl]` must live in the same crate as the namespace it registers into, whenever the component carries a prefix.** The attribute expands to `impl Namespace<_> for PathCons<..>`, and for a prefixed component that path is built entirely from the `cgp`-owned `PathCons`/`Symbol` types and the component's marker; Rust's orphan rule then accepts the impl only if the crate owns the `Namespace` trait. A downstream crate cannot register a default for an upstream prefixed component into an upstream namespace — the whole impl is foreign to it.

The restriction relaxes only when no prefix is involved. For a component without a prefix, `#[default_impl(Component in Namespace)]` expands to `impl Namespace<_> for Component`, whose key is the component's own marker; a downstream crate that owns that marker can register it into a foreign namespace, because the marker is a local type. So a crate may register a default when it owns *either* the namespace or the un-prefixed component key — but a prefixed component's key is a foreign path, leaving only the "owns the namespace" option.

Two consequences follow for how you reach for the tool. First, `#[default_impl]` couples an implementation to the namespace's crate, so splitting an application into finer crates eventually forces the provider, the component, and the namespace together in ways that reduce modularity — the opposite of what CGP's crate split is for. Second, because a namespace entry cannot be overridden once set, a `#[default_impl]` bakes in a choice that inheriting namespaces cannot revise. For these reasons **`#[default_impl]` is best seen as the tool for the basic case** — an application still written in a single crate, as a newcomer naturally starts — where it earns its keep by letting them add CGP wiring gradually, keeping their impls looking almost exactly like ordinary trait impls, and deferring the full wiring tables until later. When an application outgrows a single crate, move the affected wirings from `#[default_impl]` attributes into namespace **body** entries, which have no such crate restriction because the namespace's own crate writes them.

## Advanced: flattening multi-provider dispatch into one table

The deepest payoff of namespace paths appears when a single generic-parameter component is served by *several* providers depending on its parameter, and the providers live in different crates. The traditional way to dispatch such a component is a nested [`UseDelegate`](../reference/providers/use_delegate.md) table per provider, which forces the wiring into a tree of inner tables. Namespace paths let you flatten that tree into one table, because every entry is addressed by a full path and so entries for the same component but different providers can sit side by side.

A handler component dispatched on a code parameter shows the shape. Rather than one `UseDelegate` table wrapping all handlers, each group of codes is mapped to its provider by a path with the code inline, and the groups for different providers coexist in one namespace:

```rust
cgp_namespace! {
    new AppNamespace: DefaultNamespace {
        @cgp.extra.handler.HandlerComponent.[
            BytesToString,
            <T> ConvertTo<T>,
            <Handlers> Pipe<Handlers>,
        ]:
            BaseHandlerProvider,

        @cgp.extra.handler.HandlerComponent.[
            <Path, Args> ReadFile<Path, Args>,
            StreamToBytes,
        ]:
            FileHandlerProvider,

        @cgp.extra.handler.HandlerComponent.[
            <Method, Url> HttpRequest<Method, Url>,
        ]:
            HttpHandlerProvider,
    }
}
```

Each `@cgp.extra.handler.HandlerComponent.[..]` block maps a set of code types — some carrying their own generic parameters, bound with the inline `<T>` form — to one provider, and the three blocks for three providers read as one flat list of "which provider handles which codes." The equivalent `UseDelegate` wiring would nest three inner tables inside an outer dispatch, and merging a fourth provider's codes would mean editing the tree; here it is one more block. Flattening this way is the pattern to reach for once a component is dispatched across providers that live in separate crates, since each crate can contribute its block to a shared namespace by owning the block's provider, without any crate needing to own the others' entries.

### Collapsing a whole subtree with nested groups

When one provider serves *many* components spread across several sub-paths, the path grouping nests, so a single entry can map an entire subtree to one provider. A `.{ … }` group after a path segment holds a comma-separated list of continuations, each of which is itself a path that may carry its own `.[ … ]` code list or another `.{ … }` group. Mapping every component that a single reqwest-based provider handles — some under a `core` sub-path, some under a `reqwest` sub-path, several dispatched on their own code parameters — collapses to one entry:

```rust
@app.{
    core.{
        HttpMethodTypeProviderComponent,
        UrlTypeProviderComponent,
        MethodArgExtractorComponent.[GetMethod, PostMethod],
    },
    reqwest.RequestBuilderUpdaterComponent.[
        <Args> WithHeaders<Args>,
        <Key, Value> Header<Key, Value>,
    ],
}:
    ReqwestProvider,
```

The `@app.{ core.{ … }, reqwest.… }` shape reads as a directory listing: everything under `core` and everything under `reqwest` named here resolves to `ReqwestProvider`, with the leaf `.[ … ]` lists dispatching the generic-parameter components on their codes. Written out flat, this is seven separate `@app.core.…` and `@app.reqwest.…` entries; nested, it is one. Reach for nesting when a provider owns a cohesive slice of the namespace — it keeps the "one provider, these components" relationship on a single entry instead of scattering it down the table.

## Choosing how far to go

The three techniques form a ladder, and most applications should climb only as far as they need. Reach for `#[prefix]` as soon as a table has enough components that grouping helps a reader — it costs nothing and pays off immediately in navigability, and it is the one technique with no downstream restriction. Add a namespace with `#[default_impl]` once two contexts would share most of their wiring, or once a newcomer wants their impls to carry their own wiring instead of a central table, accepting that this ties the wiring to one crate. Split wiring across several namespaces with `for` loops, and flatten multi-provider dispatch into paths, when the application is large enough that different concerns — a backend, an API surface, a set of handlers — genuinely deserve their own reusable tables. Stop at the rung that makes the wiring clear; the goal is a table a reader can hold in their head, not the maximum use of the machinery.

## Related documentation

- [`#[cgp_namespace]`](../reference/macros/cgp_namespace.md) — the full syntax of defining a namespace, the `namespace`/`for … in` statements, and the `#[prefix]` attribute.
- [`DefaultNamespace`, `DefaultImpls1`, `DefaultImpls2`](../reference/traits/default_namespace.md) — the lookup traits behind namespaces and the `#[default_impl]` attribute.
- [`delegate_components!`](../reference/macros/delegate_components.md) — the wiring table these techniques restructure, including the `open` statement for the self-contained case.
- [Namespaces](../concepts/namespaces.md) — the mechanism (inheritance, `RedirectLookup`, and paths) that makes the preset pattern work.
- [Guides summary](README.md#summary) — the condensed cheat-sheet for the provider, dependency, and abstract-type idioms the code above uses.
