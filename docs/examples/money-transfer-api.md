# Money-transfer API

This example builds the backend for a small money-transfer web service — querying a user's balance and moving funds between accounts — as a set of composable API handlers that an HTTP server drives. It progresses from a single dispatched handler component, through reusable handler wrappers that add request decoding and authentication, to an in-memory context wired to all of it and finally served over HTTP. It is a template for any request/response service whose endpoints share cross-cutting concerns and whose backend should be swappable behind abstract types.

The concepts each step demonstrates are documented in full in the reference; this example only notes which one is in play and links to it:

- abstract domain types — [`#[cgp_type]`](../reference/macros/cgp_type.md) and the [abstract-types concept](../concepts/abstract-types.md)
- an async, per-endpoint-dispatched component — [`#[cgp_component]`](../reference/macros/cgp_component.md), [`#[async_trait]`](../reference/macros/async_trait.md), [`#[derive_delegate]`](../reference/attributes/derive_delegate.md)
- handlers that wrap other handlers — [higher-order providers](../concepts/higher-order-providers.md) written with [`#[cgp_impl]`](../reference/macros/cgp_impl.md)
- backend providers reading context fields — [implicit field access](../concepts/implicit-arguments.md) via [`#[cgp_auto_getter]`](../reference/macros/cgp_auto_getter.md)
- raising status-coded errors through an application-specific error component — [modular error handling](../concepts/modular-error-handling.md) over [`HasErrorType`](../reference/components/has_error_type.md)
- per-endpoint wiring — [`delegate_components!`](../reference/macros/delegate_components.md) with [`UseDelegate`](../reference/providers/use_delegate.md) and a [`check_components!`](../reference/macros/check_components.md) assertion
- restoring a `Send` bound for the HTTP server — the [recovering `Send` bounds concept](../concepts/send-bounds.md)

All snippets assume `use cgp::prelude::*;`. The service speaks in terms of a handful of domain types that are kept abstract so the same handlers work whatever concrete types a deployment chooses.

## Abstract domain types

The service never names a concrete user id, currency, or amount; it names abstract types a context supplies. Each is a one-line [abstract-type component](../concepts/abstract-types.md) defined with [`#[cgp_type]`](../reference/macros/cgp_type.md), carrying only the bound the rest of the code needs — here, that every domain value can be displayed in an error message:

```rust
#[cgp_type]
pub trait HasUserIdType {
    type UserId: Display;
}

#[cgp_type]
pub trait HasQuantityType {
    type Quantity: Display;
}

#[cgp_type]
pub trait HasCurrencyType {
    type Currency: Display;
}
```

Keeping these abstract is what lets one balance-query handler serve a context whose currency is a rich enum and another whose currency is a bare string, without rewriting the handler. A context binds each type once during wiring, shown later.

## The dispatched handler component

Every endpoint is one case of a single component that dispatches on a marker type naming the API. The consumer trait `CanHandleApi<Api>` takes the endpoint marker as a generic parameter and, for that endpoint, fixes a `Request` and `Response` type and an async method that turns one into the other:

```rust
#[cgp_component(ApiHandler)]
#[async_trait]
#[derive_delegate(UseDelegate<Api>)]
#[use_type(HasErrorType::Error)]
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

The three attributes each pull their weight. [`#[cgp_component]`](../reference/macros/cgp_component.md) makes `ApiHandler` a wireable component so each endpoint can bind a different provider; [`#[async_trait]`](../reference/macros/async_trait.md) keeps the async method's declaration lint-clean; and [`#[derive_delegate(UseDelegate<Api>)]`](../reference/attributes/derive_delegate.md) generates a [`UseDelegate`](../reference/providers/use_delegate.md) provider that routes each call to a per-`Api` entry in an inner table. The markers `TransferApi` and `QueryBalanceApi` are empty structs used only as keys; `PhantomData<Api>` carries the choice at the type level so the right provider is selected with no runtime branch.

## Endpoint handlers

Each endpoint is a provider for `ApiHandler` that depends on business capabilities rather than on any concrete backend. The transfer endpoint, for instance, reads the logged-in sender and the transfer details from its request, then calls the `CanTransferMoney` capability — itself an abstract async component the context implements however it likes:

```rust
#[cgp_component(MoneyTransferrer)]
#[async_trait]
#[use_type(HasUserIdType::UserId, HasCurrencyType::Currency, HasQuantityType::Quantity, HasErrorType::Error)]
pub trait CanTransferMoney {
    async fn transfer_money(
        &self,
        sender: &UserId,
        recipient: &UserId,
        currency: &Currency,
        quantity: &Quantity,
    ) -> Result<(), Error>;
}

#[cgp_impl(new HandleTransfer<Request>)]
#[uses(CanTransferMoney, CanRaiseHttpError<ErrUnauthorized, String>)]
#[use_type(HasErrorType::Error)]
impl<Api, Request> ApiHandler<Api>
where
    Request: HasLoggedInUser<Self> + HasTransferMoneyFields<Self>,
{
    type Request = Request;
    type Response = ();

    async fn handle_api(
        &self,
        _api: PhantomData<Api>,
        request: Request,
    ) -> Result<(), Error> {
        let sender = request.logged_in_user().as_ref().ok_or_else(|| {
            Self::raise_http_error(ErrUnauthorized, "you must first login".into())
        })?;

        self.transfer_money(
            sender,
            request.recipient(),
            request.currency(),
            request.quantity(),
        )
        .await?;

        Ok(())
    }
}
```

The endpoint is generic over its request shape. `HandleTransfer<Request>` works for any `Request` type that exposes a logged-in user and the transfer fields through the [getter traits](../reference/macros/cgp_auto_getter.md) named in its `where` clause, so the same handler logic serves whatever request struct a deployment decodes from the wire. The `Self: ...` bounds are [impl-side dependencies](../concepts/impl-side-dependencies.md): they hold the context to providing money-transfer and HTTP-error capabilities without those leaking into the consumer trait.

## Reusable handler wrappers

Cross-cutting concerns are handlers that wrap another handler, which makes them [higher-order providers](../concepts/higher-order-providers.md). Each takes an inner handler as a type parameter, implements `ApiHandler` itself, and threads the call through — transforming the request or response on the way. Three small wrappers cover decoding, authentication, and JSON encoding.

`HandleFromRequest` adapts the request type, letting an endpoint that wants a clean domain request sit behind a handler whose request is the raw type the HTTP layer produces:

```rust
#[cgp_impl(new HandleFromRequest<Request, InHandler>)]
#[use_type(HasErrorType::Error)]
impl<Api, Request, InHandler> ApiHandler<Api>
where
    InHandler: ApiHandler<Self, Api>,
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

`UseBasicAuth` performs authentication before delegating, resolving a basic-auth header into a logged-in user and mutating the request in place:

```rust
#[cgp_impl(new UseBasicAuth<InHandler>)]
#[uses(CanQueryUserHashedPassword, CanCheckPassword)]
#[use_type(HasUserIdType::UserId, HasErrorType::Error)]
impl<Api, InHandler> ApiHandler<Api>
where
    InHandler: ApiHandler<Self, Api>,
    InHandler::Request: HasLoggedInUserMut<Self> + HasBasicAuthHeader<Self>,
    UserId: Clone,
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
            if let Some(hashed) = self.query_user_hashed_password(user_id).await?
                && Self::check_password(password, &hashed)
            {
                *request.logged_in_user() = Some(user_id.clone());
            }
        }

        InHandler::handle_api(self, api, request).await
    }
}
```

`ResponseToJson` adapts in the other direction, wrapping whatever the inner handler returns in an Axum `Json` envelope:

```rust
#[cgp_impl(ResponseToJson<InHandler>)]
#[use_type(HasErrorType::Error)]
impl<Api, InHandler> ApiHandler<Api>
where
    InHandler: ApiHandler<Self, Api>,
{
    type Request = InHandler::Request;
    type Response = Json<InHandler::Response>;

    async fn handle_api(
        &self,
        api: PhantomData<Api>,
        request: Self::Request,
    ) -> Result<Self::Response, Error> {
        let response = InHandler::handle_api(self, api, request).await?;
        Ok(Json(response))
    }
}
```

Because each wrapper is itself an `ApiHandler`, they nest into a pipeline. Writing `HandleFromRequest<Raw, ResponseToJson<UseBasicAuth<HandleQueryBalance<Clean>>>>` reads outside-in as the stages a request passes through — decode the raw request, authenticate, run the endpoint, JSON-encode the response — with each layer adding exactly one concern and the endpoint at the center oblivious to all of them.

## The backend behind the capabilities

The business capabilities are satisfied by a provider that reads its data from context fields. `UseMockedApp` is an in-memory backend that implements `MoneyTransferrer` and the other capabilities by reaching into maps stored on the context, retrieved through [`#[cgp_auto_getter]`](../reference/macros/cgp_auto_getter.md) traits:

```rust
#[cgp_auto_getter]
#[use_type(HasUserIdType::UserId, HasCurrencyType::Currency, HasQuantityType::Quantity)]
pub trait HasMockedUserBalances {
    fn user_balances(
        &self,
    ) -> &Arc<Mutex<BTreeMap<(UserId, Currency), Quantity>>>;
}

#[cgp_impl(UseMockedApp)]
#[uses(
    HasMockedUserBalances,
    CanRaiseHttpError<ErrNotFound, String>,
    CanRaiseHttpError<ErrBadRequest, String>,
)]
#[use_type(HasUserIdType::UserId, HasCurrencyType::Currency, HasQuantityType::Quantity, HasErrorType::Error)]
impl MoneyTransferrer
where
    Quantity: CheckedAdd + CheckedSub,
    UserId: Ord + Clone,
    Currency: Ord + Clone,
{
    async fn transfer_money(
        &self,
        sender: &UserId,
        recipient: &UserId,
        currency: &Currency,
        quantity: &Quantity,
    ) -> Result<(), Error> {
        let mut balances = self.user_balances().lock().await;
        /* debit the sender, credit the recipient, raising on overflow or missing accounts */
        Ok(())
    }
}
```

A real deployment would swap this one provider for a database-backed one. Since `UseMockedApp` is selected per context in the wiring, replacing it with a `UsePostgres` provider that implements the same capabilities changes which backend runs without touching a single endpoint or wrapper.

## Wiring a context

A concrete context becomes the running application by binding every abstract type and component in one place. `MockApp` holds the in-memory state and wires it all together: the domain types resolve to concrete types via [`UseType`](../reference/providers/use_type.md), the business capabilities resolve to `UseMockedApp`, and each endpoint of `ApiHandler` resolves to its own wrapper pipeline through an inner [`UseDelegate`](../reference/providers/use_delegate.md) table keyed by the API marker:

```rust
#[derive(HasField, Default)]
pub struct MockApp {
    pub user_balances: Arc<Mutex<BTreeMap<(String, DemoCurrency), u64>>>,
    pub user_passwords: BTreeMap<String, String>,
}

delegate_components! {
    MockApp {
        ErrorTypeProviderComponent:
            UseType<AppError>,
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
            MoneyTransferrerComponent,
        ]:
            UseMockedApp,
        ApiHandlerComponent:
            UseDelegate<new MockAppApiHandlers {
                QueryBalanceApi:
                    HandleFromRequest<AxumQueryBalanceRequest,
                        ResponseToJson<UseBasicAuth<
                            HandleQueryBalance<QueryBalanceRequest>>>>,
                TransferApi:
                    HandleFromRequest<AxumTransferRequest,
                        UseBasicAuth<HandleTransfer<TransferRequest>>>,
            }>,
    }
}
```

The two endpoints assemble different pipelines from the same parts. Both decode an Axum request and authenticate, but the balance query also JSON-encodes its response while the transfer returns nothing, so only the query wraps in `ResponseToJson`. The nested `UseDelegate<new MockAppApiHandlers { ... }>` builds the per-`Api` lookup table inline, so a call to `handle_api` with the `TransferApi` marker resolves to the transfer pipeline and one with `QueryBalanceApi` to the balance pipeline.

Because CGP wiring is [checked lazily](../concepts/check-traits.md), a companion [`check_components!`](../reference/macros/check_components.md) block proves at compile time that every endpoint is fully satisfied, listing the API markers to verify for the generic `ApiHandler` component:

```rust
check_components! {
    MockApp {
        MoneyTransferrerComponent,
        UserBalanceQuerierComponent,
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

Each impl just forwards to `handle_api`, but at a concrete context and API the awaited future is a concrete type whose `Send`-ness the compiler can confirm — which is exactly why the impls cannot be folded into one generic blanket impl. The full reasoning, and why this is a stand-in for the Return Type Notation stable Rust lacks, is in [recovering `Send` bounds](../concepts/send-bounds.md). With `CanHandleApiSend` in hand, an Axum route handler can bound `App: CanHandleApiSend<Api>` and spawn the handler safely, completing the path from a request on the wire to one of the wired endpoint pipelines.
