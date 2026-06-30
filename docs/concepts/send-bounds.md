# Recovering `Send` bounds

Recovering a `Send` bound is the pattern of restoring the "the returned future is `Send`" guarantee that an async trait method drops, by re-declaring the method on a concrete context where the compiler can prove the bound itself — a hand-written stand-in for the Return Type Notation that stable Rust does not yet offer.

## Why async trait methods lose their `Send` bound

An async CGP method advertises a future whose auto-traits the caller cannot name. The standard way to declare an asynchronous component is to write an `async fn` in the consumer trait and let [`#[async_trait]`](../reference/macros/async_trait.md) rewrite it into a return-position `impl Future`:

```rust
#[cgp_component(ApiHandler)]
#[async_trait]
#[derive_delegate(UseDelegate<Api>)]
pub trait CanHandleApi<Api>: HasErrorType {
    type Request;
    type Response;

    async fn handle_api(
        &self,
        _api: PhantomData<Api>,
        request: Self::Request,
    ) -> Result<Self::Response, Self::Error>;
}
```

That rewrite is faithful and zero-cost, but it drops every auto-trait bound. `handle_api` becomes `fn handle_api(..) -> impl Future<Output = Result<Self::Response, Self::Error>>`, with no boxing and no implicit bounds — and no `Send`. The future is `Send` only when the concrete future the body produces happens to be, and a caller working through the trait has no way to *say* "I need it to be." The opacity that makes return-position `impl Trait` zero-cost is exactly what hides the future's auto-traits behind the trait boundary.

## Why spawning needs `Send`, and why you cannot ask for it

The bound becomes load-bearing the moment the future is spawned onto a multi-threaded executor. A work-stealing runtime — the default Tokio runtime that an Axum server runs on, for instance — may migrate a task between threads while it is suspended, so every future it drives must be `Send`. When a generic handler routes a request through `handle_api`, the surrounding task captures and awaits that future, and the task is `Send` only if the future is. For a *concrete* context the compiler can check this directly; for a context left generic it cannot, because the future's `Send`-ness is precisely the fact the trait refuses to expose.

The bound you want to write does not exist on stable Rust. Conceptually the requirement is "for whatever arguments `handle_api` is called with, its future is `Send`", and Rust has a notation for exactly that — Return Type Notation (RTN), which lets a bound name a method's return type:

```rust
// Not available on stable Rust:
fn spawn_handler<App, Api>(app: App)
where
    App: CanHandleApi<Api, handle_api(..): Send> + Send + 'static,
{
    tokio::spawn(async move { /* ... app.handle_api(...).await ... */ });
}
```

The `handle_api(..): Send` clause says the future returned by the method is `Send` no matter the arguments, which is what a work-stealing spawn demands. RTN is not stabilized, however, so this bound cannot be written in production code today, and a generic caller is left unable to express the one requirement the executor imposes.

## Recovering the bound with a concrete `Send` trait

The workaround is to declare a second, ordinary trait whose method states the `Send` bound directly in its return type, sidestepping RTN entirely:

```rust
pub trait CanHandleApiSend<Api>:
    CanHandleApi<Api, Request: Send, Response: Send> + Send + Sync
{
    fn handle_api_send(
        &self,
        _api: PhantomData<Api>,
        request: Self::Request,
    ) -> impl Future<Output = Result<Self::Response, Self::Error>> + Send;
}
```

`CanHandleApiSend` is a plain trait, not a [component](../reference/macros/cgp_component.md) — it adds nothing to the wiring and exists only to carry stronger bounds. It inherits the full capability from `CanHandleApi` as a supertrait, additionally requiring the request and response to be `Send` and the context itself to be `Send + Sync`, and it spells out `+ Send` on the future explicitly rather than through a clause on `handle_api`. A caller that holds `App: CanHandleApiSend<Api>` therefore knows the future is `Send` from the signature alone, with no RTN in sight. This is the bound a spawning handler can finally name:

```rust
where
    App: CanHandleApiSend<Api>,
```

## Why the implementation must be concrete

The recovered trait cannot be implemented once, generically — that would require the very bound RTN is missing. The natural impl to reach for is a blanket one over every context that already handles the API:

```rust
// Does not compile on stable Rust:
impl<App, Api> CanHandleApiSend<Api> for App
where
    App: CanHandleApi<Api, Request: Send, Response: Send> + Send + Sync,
{
    async fn handle_api_send(
        &self,
        api: PhantomData<Api>,
        request: Self::Request,
    ) -> Result<Self::Response, Self::Error> {
        self.handle_api(api, request).await
    }
}
```

The body wraps `self.handle_api(..)` in an `async` block, and that block is `Send` only if the future it awaits is `Send`. For a generic `App` and `Api` the awaited future is an opaque `impl Future` whose auto-traits are unknown — the same gap restated — so the impl fails to prove its own `+ Send` return type. A generic blanket impl is just RTN wearing a disguise, and it is blocked for the same reason.

Dropping to a concrete context and a concrete API closes the gap. When `Self` is a fixed type and `Api` is a fixed marker, `self.handle_api(api, request)` resolves through the wiring to a concrete provider stack producing a concrete future, and the compiler computes that future's auto-traits and finds it `Send` — no annotation required, because `Send` is inferred structurally for a known type. The impl is therefore written per concrete `(context, API)` pair:

```rust
impl CanHandleApiSend<QueryBalanceApi> for MockApp {
    async fn handle_api_send(
        &self,
        api: PhantomData<QueryBalanceApi>,
        request: Self::Request,
    ) -> Result<Self::Response, Self::Error> {
        self.handle_api(api, request).await
    }
}

impl CanHandleApiSend<TransferApi> for MockApp {
    async fn handle_api_send(
        &self,
        api: PhantomData<TransferApi>,
        request: Self::Request,
    ) -> Result<Self::Response, Self::Error> {
        self.handle_api(api, request).await
    }
}
```

Each impl is mechanical — it forwards to `handle_api` and awaits — yet each is also a proof, accepted only because at this concrete instantiation the future really is `Send`. The repetition is the cost of the missing notation: one concrete impl per API per context replaces the single generic impl RTN would have allowed. The [money-transfer API example](../examples/money-transfer-api.md) shows the pattern in its place, with `MockApp` recovering the bound so its handlers can be served by Axum.

## Related constructs

The dropped bound originates with [`#[async_trait]`](../reference/macros/async_trait.md), whose Known issues section records the same `Send`-less future from the macro's side. The recovered trait sits atop a [component](../reference/macros/cgp_component.md) defined the usual way and consumed through the [consumer/provider duality](consumer-and-provider-traits.md); the [`Handler`](../reference/components/handler.md) family is the built-in async component most likely to need this treatment when its futures are spawned. The concrete impls forward through whatever provider stack the context wires with [`delegate_components!`](../reference/macros/delegate_components.md), and the [higher-order providers](higher-order-providers.md) in that stack are part of what makes the resolved future a concrete, checkable type.

## Source

The behavior this pattern compensates for lives in the `#[async_trait]` rewrite at [crates/macros/cgp-async-macro/src/impl_async.rs](../../crates/macros/cgp-async-macro/src/impl_async.rs), which produces a bare `-> impl Future` with no auto-trait bounds. The recovery itself is application-level rather than a CGP construct: it is an ordinary trait whose method declares `+ Send` on its return type and whose impls are written for concrete contexts, so there is no macro or core trait that implements it — only the language feature, Return Type Notation, that would make it unnecessary.
