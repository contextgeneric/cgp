# Money-transfer API

This example builds the backend for a small money-transfer web service — querying a user's balance and moving funds between accounts — as a set of composable API handlers that an HTTP server drives. It progresses from abstract domain types and a status-coded error component, through a per-endpoint-dispatched handler and the reusable wrappers that add decoding, authentication, and encoding, to an in-memory context whose whole wiring is organized into a namespace and served over HTTP. It is a template for any request/response service whose endpoints share cross-cutting concerns and whose backend should be swappable behind abstract types.

The concepts each step demonstrates are documented in full elsewhere; this example only notes which one is in play and links to it:

- abstract domain types — [`#[cgp_type]`](../reference/macros/cgp_type.md) and the [abstract-types concept](../concepts/abstract-types.md)
- status-coded errors through an application-specific error component — [modular error handling](../concepts/modular-error-handling.md) over [`HasErrorType`](../reference/components/has_error_type.md)
- an async, per-endpoint-dispatched component — [`#[cgp_component]`](../reference/macros/cgp_component.md) with [`#[async_trait]`](../reference/macros/async_trait.md)
- handlers, and a business capability, that wrap another provider — [higher-order providers](../concepts/higher-order-providers.md) written with [`#[cgp_impl]`](../reference/macros/cgp_impl.md) and [`#[use_provider]`](../reference/attributes/use_provider.md)
- a backend reading context fields — [implicit field access](../concepts/implicit-arguments.md) via [`#[implicit]`](../reference/attributes/implicit.md) arguments, with [`#[cgp_auto_getter]`](../reference/macros/cgp_auto_getter.md) reserved for the request fields a handler reads through a `where` bound
- organizing the wiring — path prefixes and a namespace, per the [namespaces-and-prefixes guide](../guides/namespaces-and-prefixes.md), backed by [`#[prefix]`](../reference/macros/cgp_namespace.md), [`#[default_impl]`](../reference/traits/default_namespace.md), and [`delegate_components!`](../reference/macros/delegate_components.md) with a [`check_components!`](../reference/macros/check_components.md) assertion
- restoring a `Send` bound for the HTTP server — the [recovering `Send` bounds concept](../concepts/send-bounds.md)

All snippets assume `use cgp::prelude::*;`. The service speaks in terms of a handful of domain types kept abstract so the same handlers work whatever concrete types a deployment chooses.

## Abstract domain types

The service never names a concrete user id, currency, or amount; it names abstract types a context supplies. Each is a one-line [abstract-type component](../concepts/abstract-types.md) defined with [`#[cgp_type]`](../reference/macros/cgp_type.md), carrying only the bound the rest of the code needs — here, that every domain value can be displayed in an error message:

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

#[cgp_type]
#[prefix(@app.finance.types in DefaultNamespace)]
pub trait HasCurrencyType {
    type Currency: Display;
}
```

Keeping these abstract is what lets one balance-query handler serve a context whose currency is a rich enum and another whose currency is a bare string, without rewriting the handler. The authentication types `HasPasswordType` and `HasHashedPasswordType` are defined the same way under `@app.auth.types`. The [`#[prefix(@path in DefaultNamespace)]`](../reference/macros/cgp_namespace.md) attribute on each files the component under a path — `@app.auth.types`, `@app.finance.types` — that the wiring section uses to organize the table; a first-time reader can ignore it until then, or read the [namespaces-and-prefixes guide](../guides/namespaces-and-prefixes.md) for why the types sit on a `types` sub-path apart from the logic.

## Status-coded errors

Endpoints fail with an HTTP status code, but the handlers never construct one directly — they raise through an application-specific error component so the mapping from a domain failure to a status code lives in one place. `CanRaiseHttpError<Code, Detail>` is a [component](../concepts/modular-error-handling.md) that turns a marker code and a detail value into the context's abstract [error type](../reference/components/has_error_type.md):

```rust
#[cgp_component(HttpErrorRaiser)]
#[prefix(@app.error in DefaultNamespace)]
#[use_type(HasErrorType.Error)]
pub trait CanRaiseHttpError<Code, Detail> {
    fn raise_http_error(_code: Code, detail: Detail) -> Error;
}

pub struct ErrUnauthorized;
pub struct ErrBadRequest;
pub struct ErrNotFound;
```

The code markers double as a table from marker to status code through an ordinary trait, so a provider can turn `ErrUnauthorized` into `401` without a match:

```rust
pub trait IsStatusCode {
    fn status_code() -> StatusCode;
}

impl IsStatusCode for ErrUnauthorized {
    fn status_code() -> StatusCode {
        StatusCode::UNAUTHORIZED
    }
}
// ErrBadRequest -> BAD_REQUEST, ErrNotFound -> NOT_FOUND, …

#[cgp_impl(new DisplayHttpError)]
#[use_type(HasErrorType.{Error = AppError})]
impl<Code, Detail> HttpErrorRaiser<Code, Detail>
where
    Code: IsStatusCode,
    Detail: Display,
{
    fn raise_http_error(_code: Code, detail: Detail) -> AppError {
        AppError {
            status_code: Code::status_code(),
            detail: anyhow!("{detail}"),
        }
    }
}
```

`DisplayHttpError` is generic over both the code and the detail, so one provider serves every `raise_http_error` call whose detail is `Display`. It pins the abstract error to the concrete `AppError` with the [`#[use_type]` equality form](../guides/importing-abstract-types.md) — `HasErrorType.{Error = AppError}` — which is why the method can build an `AppError` directly. A sibling `HandleHttpErrorWithAnyhow` handles details that are already an `anyhow::Error`; the wiring picks between them per detail type.

## The dispatched API-handler component

Every endpoint is one case of a single component that dispatches on a marker type naming the API. The consumer trait `CanHandleApi<Api>` takes the endpoint marker as a generic parameter and, for that endpoint, fixes a `Request` and `Response` type and an async method that turns one into the other:

```rust
#[cgp_component(ApiHandler)]
#[prefix(@app.api in DefaultNamespace)]
#[async_trait]
#[use_type(HasErrorType.Error)]
pub trait CanHandleApi<Api> {
    type Request;
    type Response;

    async fn handle_api(
        &self,
        _api: PhantomData<Api>,
        request: Self::Request,
    ) -> Result<Self::Response, Error>;
}

pub struct TransferApi;
pub struct QueryBalanceApi;
```

[`#[cgp_component]`](../reference/macros/cgp_component.md) makes `ApiHandler` a wireable component so each endpoint can bind a different provider, and [`#[async_trait]`](../reference/macros/async_trait.md) keeps the async method's declaration lint-clean. Because the component is generic over `Api`, a context dispatches it per marker — `TransferApi` to one provider, `QueryBalanceApi` to another — through the [namespace path machinery](../guides/dispatching-per-type.md) shown in the wiring section, with no runtime branch: `PhantomData<Api>` carries the choice at the type level.

## Endpoint handlers

Each endpoint is a provider for `ApiHandler` that depends on business capabilities rather than on any concrete backend. The transfer endpoint reads the logged-in sender and the transfer details from its request, then calls the `CanTransferMoney` capability — itself an abstract async component the context implements however it likes:

```rust
#[cgp_impl(new HandleTransfer<Request>)]
#[uses(CanTransferMoney, CanRaiseHttpError<ErrUnauthorized, String>)]
#[use_type(HasErrorType.Error)]
impl<Api, Request> ApiHandler<Api>
where
    Request: HasLoggedInUser<Self> + HasTransferMoneyFields<Self>,
{
    type Request = Request;
    type Response = ();

    async fn handle_api(&self, _api: PhantomData<Api>, request: Request) -> Result<(), Error> {
        let sender = request.logged_in_user().as_ref().ok_or_else(|| {
            Self::raise_http_error(ErrUnauthorized, "you must first login".into())
        })?;

        self.transfer_money(sender, request.recipient(), request.currency(), request.quantity())
            .await?;

        Ok(())
    }
}
```

The endpoint is generic over its request shape: `HandleTransfer<Request>` works for any `Request` that exposes a logged-in user and the transfer fields through the [getter traits](../reference/macros/cgp_auto_getter.md) named in its `where` clause, so the same logic serves whatever request struct a deployment decodes. The `Self: ...` capability bounds are [impl-side dependencies](../concepts/impl-side-dependencies.md) declared with [`#[uses]`](../guides/declaring-dependencies.md), holding the context to providing money-transfer and error-raising without those leaking into the consumer trait.

The balance query has the same shape but returns a value, so it fixes a response type. Its `QueryBalanceResponse<App>` stays generic over the context's abstract `Quantity`, and `#[derive(Serialize)]` lets the JSON wrapper encode it:

```rust
#[derive(Serialize)]
pub struct QueryBalanceResponse<App>
where
    App: HasQuantityType,
{
    pub balance: App::Quantity,
}

#[cgp_impl(new HandleQueryBalance<Request>)]
#[uses(CanQueryUserBalance, CanRaiseHttpError<ErrUnauthorized, String>)]
#[use_type(HasErrorType.Error)]
impl<Api, Request> ApiHandler<Api>
where
    Request: HasLoggedInUser<Self> + HasQueryBalanceFields<Self>,
{
    type Request = Request;
    type Response = QueryBalanceResponse<Self>;

    async fn handle_api(
        &self,
        _api: PhantomData<Api>,
        request: Request,
    ) -> Result<QueryBalanceResponse<Self>, Error> {
        let user = request.logged_in_user().as_ref().ok_or_else(|| {
            Self::raise_http_error(ErrUnauthorized, "you must first login".into())
        })?;

        let balance = self.query_user_balance(user, request.currency()).await?;

        Ok(QueryBalanceResponse { balance })
    }
}
```

## Reusable handler wrappers

Cross-cutting concerns are handlers that wrap another handler, which makes them [higher-order providers](../concepts/higher-order-providers.md): each takes an inner handler as a type parameter, declared with [`#[use_provider]`](../reference/attributes/use_provider.md), implements `ApiHandler` itself, and threads the call through — transforming the request or response on the way. Three small wrappers cover decoding, authentication, and JSON encoding.

`HandleFromRequest` adapts the request type, letting an endpoint that wants a clean domain request sit behind a handler whose request is the raw type the HTTP layer produces:

```rust
#[cgp_impl(new HandleFromRequest<Request, InHandler>)]
#[use_type(HasErrorType.Error)]
#[use_provider(InHandler: ApiHandler<Api>)]
impl<Api, Request, InHandler> ApiHandler<Api>
where
    Request: Into<InHandler::Request>,
{
    type Request = Request;
    type Response = InHandler::Response;

    async fn handle_api(
        &self,
        api: PhantomData<Api>,
        request: Self::Request,
    ) -> Result<Self::Response, Error> {
        InHandler::handle_api(self, api, request.into()).await
    }
}
```

`UseBasicAuth` authenticates before delegating, resolving a basic-auth header into a logged-in user and mutating the request in place; it depends on the `CanQueryUserHashedPassword` and `CanCheckPassword` capabilities:

```rust
#[cgp_impl(new UseBasicAuth<InHandler>)]
#[uses(CanQueryUserHashedPassword, CanCheckPassword)]
#[use_type(HasErrorType.Error)]
#[use_provider(InHandler: ApiHandler<Api>)]
impl<Api, InHandler> ApiHandler<Api>
where
    InHandler::Request: HasLoggedInUserMut<Self> + HasBasicAuthHeader<Self>,
    Self::UserId: Clone,
{
    type Request = InHandler::Request;
    type Response = InHandler::Response;

    async fn handle_api(
        &self,
        api: PhantomData<Api>,
        mut request: Self::Request,
    ) -> Result<Self::Response, Error> {
        if request.logged_in_user().is_none()
            && let Some((user_id, password)) = request.basic_auth_header()
        {
            let m_hashed_password = self.query_user_hashed_password(user_id).await?;

            if let Some(hashed_password) = m_hashed_password
                && Self::check_password(password, &hashed_password)
            {
                *request.logged_in_user() = Some(user_id.clone());
            }
        }

        InHandler::handle_api(self, api, request).await
    }
}
```

`ResponseToJson` adapts in the other direction, wrapping whatever the inner handler returns in an Axum `Json` envelope. Because each wrapper is itself an `ApiHandler`, they nest into a pipeline: `HandleFromRequest<Raw, ResponseToJson<UseBasicAuth<HandleQueryBalance<Clean>>>>` reads outside-in as the stages a request passes through — decode the raw request, JSON-encode the response, authenticate, run the endpoint — with each layer adding exactly one concern and the endpoint at the center oblivious to all of them.

## The backend behind the capabilities

The business capabilities are satisfied by a provider that reads its data from context fields. `UseMockedApp` is an in-memory backend that implements `UserBalanceQuerier`, `MoneyTransferrer`, and the auth capabilities by reaching into maps stored on the context, pulled in as [`#[implicit]`](../reference/attributes/implicit.md) arguments — the balances map is read by reference (`&Arc<Mutex<…>>`) with no clone and no getter trait to declare:

```rust
#[cgp_impl(UseMockedApp)]
#[default_impl(@app.finance.UserBalanceQuerierComponent in MockNamespace)]
#[uses(CanRaiseHttpError<ErrNotFound, String>)]
#[use_type(HasUserIdType.UserId, HasCurrencyType.Currency, HasQuantityType.Quantity, HasErrorType.Error)]
impl UserBalanceQuerier
where
    UserId: Ord + Clone,
    Currency: Ord + Clone,
    Quantity: Clone,
{
    async fn query_user_balance(
        &self,
        user: &UserId,
        currency: &Currency,
        #[implicit] user_balances: &Arc<Mutex<BTreeMap<(UserId, Currency), Quantity>>>,
    ) -> Result<Quantity, Error> {
        let balances = user_balances.lock().await;
        balances
            .get(&(user.clone(), currency.clone()))
            .cloned()
            .ok_or_else(|| Self::raise_http_error(ErrNotFound, format!("user not found: {user}")))
    }
}
```

The `#[implicit]` argument reads the same `user_balances` field a getter would, but as a `&Arc<Mutex<…>>` bound at the top of the method — the [preferred form](../guides/reading-context-fields.md) for a field a provider reads from its own context. The request-field getters `HasLoggedInUser` and `HasBasicAuthHeader`, by contrast, stay [`#[cgp_auto_getter]`](../reference/macros/cgp_auto_getter.md) traits, because they read from the *request* type and are required as `where` bounds on it (`Request: HasBasicAuthHeader<Self>`) — a case an implicit argument, which reads only from `self`, cannot cover. The [`#[default_impl]`](../reference/traits/default_namespace.md) attribute registers this provider into the application's namespace; that is a wiring concern, explained next.

A business capability can be wrapped the same way an API handler can. `NoTransferToSelf` is a [higher-order provider](../concepts/higher-order-providers.md) for `MoneyTransferrer` that rejects a self-transfer and otherwise delegates to an inner transfer provider:

```rust
#[cgp_impl(new NoTransferToSelf<InHandler>)]
#[use_type(HasUserIdType.UserId, HasCurrencyType.Currency, HasQuantityType.Quantity, HasErrorType.Error)]
#[uses(CanRaiseHttpError<ErrBadRequest, String>)]
#[use_provider(InHandler: MoneyTransferrer)]
impl<InHandler> MoneyTransferrer
where
    UserId: Eq,
{
    async fn transfer_money(
        &self,
        sender: &UserId,
        recipient: &UserId,
        currency: &Currency,
        quantity: &Quantity,
    ) -> Result<(), Error> {
        if sender != recipient {
            InHandler::transfer_money(self, sender, recipient, currency, quantity).await
        } else {
            Err(Self::raise_http_error(
                ErrBadRequest,
                format!("cannot transfer with the same sender and recipient: {sender}"),
            ))
        }
    }
}
```

A real deployment would swap `UseMockedApp` for a database-backed provider. Since the backend is selected per context in the wiring, replacing it with a `UsePostgres` provider that implements the same capabilities changes which backend runs without touching a single endpoint or wrapper.

## Organizing the wiring with a namespace

A concrete context becomes the running application by resolving every abstract type and component. Rather than spell all of that out on the context, the application lifts it into a reusable **namespace** — the [namespaces-and-prefixes guide](../guides/namespaces-and-prefixes.md) develops this technique in full; the essentials are shown here. `MockNamespace` inherits the built-in `DefaultNamespace` and wires the pieces that have no `#[cgp_impl]` block of their own — the concrete error type, the per-detail HTTP-error dispatch, and the abstract type choices — as [namespace body entries](../reference/macros/cgp_namespace.md) keyed by the paths the `#[prefix]` attributes established:

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

The business-logic providers, which *do* have `#[cgp_impl]` blocks, register themselves into the same namespace from their own definition with the [`#[default_impl(@path in MockNamespace)]`](../reference/traits/default_namespace.md) attribute seen on `UseMockedApp`'s `UserBalanceQuerier` above — so `MockNamespace` resolves `@app.finance.UserBalanceQuerierComponent` to `UseMockedApp` without a line in its body. Registering the wiring next to the implementation it wires is what keeps the namespace body down to the handful of entries that have nowhere else to live.

The application's API surface is a separate reusable table keyed by the API marker, so any backend can pull it in:

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

With the backend in a namespace and the API surface in a table, the context's own wiring shrinks to three statements: join the namespace, pull the API handlers onto the `ApiHandler` dispatch path with a `for` loop, and override the one component it wants to treat specially — wrapping money transfers in `NoTransferToSelf`:

```rust
#[derive(HasField, Default)]
pub struct MockApp {
    pub user_balances: Arc<Mutex<BTreeMap<(String, DemoCurrency), u64>>>,
    pub user_passwords: BTreeMap<String, String>,
}

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

Each remaining line states a decision rather than a mechanical fact: `MockApp` uses the mock backend, serves the default API surface, and guards transfers. The override works because `MockNamespace` deliberately does *not* register `@app.finance.MoneyTransferrerComponent` — the base `MoneyTransferrer` provider is used only as the inner handler of `NoTransferToSelf<UseMockedApp>` — so the context is free to wire that path directly; had the namespace claimed it, the two entries would conflict. The two endpoints assemble different pipelines from the same parts: both decode and authenticate, but only the balance query wraps its response in `ResponseToJson`, since the transfer returns nothing.

Because CGP wiring is [checked lazily](../concepts/check-traits.md), a companion [`check_components!`](../reference/macros/check_components.md) block proves at compile time that every endpoint is fully satisfied, listing the API markers to verify for the generic `ApiHandler` component:

```rust
check_components! {
    MockApp {
        QuantityTypeProviderComponent,
        UserBalanceQuerierComponent,
        MoneyTransferrerComponent,
        ApiHandlerComponent: [
            QueryBalanceApi,
            TransferApi,
        ],
    }
}
```

## Serving over HTTP

Handing the handlers to an HTTP server needs one bound the component cannot provide: that each handler's future is `Send`. Axum runs on a multi-threaded, work-stealing runtime that may move a task between threads while it is suspended, so the futures it drives must be `Send` — but the `async fn` in `CanHandleApi` desugars to a bare `impl Future` with no such bound, and stable Rust has no way to require it generically. The fix is a plain trait whose method declares `+ Send` directly and which is implemented for the concrete context, where the compiler can verify the bound itself:

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

impl CanHandleApiSend<QueryBalanceApi> for MockApp {
    async fn handle_api_send(
        &self,
        api: PhantomData<QueryBalanceApi>,
        request: Self::Request,
    ) -> Result<Self::Response, Self::Error> {
        self.handle_api(api, request).await
    }
}

// … and the same one-line forwarding impl for TransferApi.
```

Each impl just forwards to `handle_api`, but at a concrete context and API the awaited future is a concrete type whose `Send`-ness the compiler can confirm — which is why the impls cannot be folded into one generic blanket impl. The full reasoning, and why this is a stand-in for the Return Type Notation stable Rust lacks, is in [recovering `Send` bounds](../concepts/send-bounds.md).

With `CanHandleApiSend` in hand, the routing layer bounds `App: CanHandleApiSend<Api>` and mounts each endpoint. An `add_route` on an Axum `Router` reads the request out of the HTTP layer, calls `handle_api_send`, and maps a raised `AppError` to its status code, so a single `add_main_api_routes` assembles the whole service:

```rust
impl<App> CanAddMainApiRoutes<App> for Router<Arc<App>>
where
    Self: CanAddRoute<App, QueryBalanceApi, GetMethod> + CanAddRoute<App, TransferApi, PostMethod>,
{
    fn add_main_api_routes(self) -> Self {
        self.add_route(PhantomData::<(QueryBalanceApi, GetMethod)>, "/balance")
            .add_route(PhantomData::<(TransferApi, PostMethod)>, "/transfer")
    }
}
```

The `main` function then constructs a `MockApp`, builds the router with `add_main_api_routes`, and serves it — completing the path from a request on the wire, through the decode-authenticate-handle-encode pipeline the namespace wired, to a JSON response or a status-coded error.
