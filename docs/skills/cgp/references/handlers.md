# Handlers

The handler family is a spectrum of CGP components that all transform an `Input` into an `Output` under a phantom `Code` tag, varying along three axes — synchronous vs. async, infallible vs. fallible, and input vs. no-input — together with the macros that build handlers from functions, the combinators that compose and lift them, the dispatchers that route over extensible data, and the monads that chain them.

Assume `use cgp::prelude::*;` throughout; the CGP version is v0.7.0. Every handler is an ordinary [component](components.md) — a consumer trait the context calls, a provider trait a zero-sized provider implements, and a `…Component` marker that [wiring](wiring.md) maps to a provider — so nothing here needs machinery beyond what you already know.

## The shared shape and the three axes

Every handler maps `(context, Code, Input) -> Output`, where `Code` is a phantom tag carried as `PhantomData<Code>` and `Output` is an *associated type* the provider chooses, not a parameter the caller fixes. The `Code` tag holds no data; it exists so one context can host many handlers keyed by distinct tags, and so wiring can dispatch on it. The family is large because real computations differ along three independent axes, and each combination gets its own component so a provider declares exactly the capabilities it has.

The first axis is **synchronous vs. async**: an async component returns a future and its method is `async`, marked by `Async` in the name (`AsyncComputer` is the async `Computer`). The second is **infallible vs. fallible**: a fallible component returns `Result<Output, Error>` against the context's abstract error and so supertraits [`HasErrorType`](abstract-types.md), marked by the `Try` prefix. The third is **owned vs. by-reference input**: every base component has a `*Ref` sibling that borrows `&Input` instead of consuming `Input`. The general principle is that you write the *weakest* variant that fits and let combinators promote it upward, because an infallible computation is trivially fallible, a sync one trivially async, and an owned-input one can serve a borrow — but never the reverse.

The components, by corner, are these. `Computer`/`CanCompute` is the pure-computation base — synchronous and infallible, with `AsyncComputer` and the `*Ref` variants alongside it. `TryComputer`/`CanTryCompute` is fallible but synchronous. `Handler`/`CanHandle` is the fully general async-and-fallible computation, the bound a generic consumer targets because every simpler provider promotes up to it. `Producer`/`CanProduce` sits apart as the no-input case, producing a value from the context and `Code` alone. A related pair, `CanRun`/`CanSendRun`, runs a named task rather than transforming a value.

## The computation components

`CanCompute` is the simplest member and the one to reach for first. Its method takes the `Code` tag and the input by value and returns the chosen `Output` with no failure path:

```rust
#[cgp_component(Computer)]
#[derive_delegate(UseDelegate<Code>)]
#[derive_delegate(UseInputDelegate<Input>)]
pub trait CanCompute<Code, Input> {
    type Output;
    fn compute(&self, _code: PhantomData<Code>, input: Input) -> Self::Output;
}
```

The provider trait `Computer<Context, Code, Input>` moves the context into an explicit first parameter, as for any component; the marker is `ComputerComponent`. The two `#[derive_delegate]` directives let a context route to different providers by `Code` (via `UseDelegate`) or by `Input` type (via the family's `UseInputDelegate`), the basis of [dispatching](#dispatching-over-extensible-data). `AsyncComputer`/`CanComputeAsync` is identical but declares `async fn compute_async` under `#[async_trait]`, and `ComputerRef`/`AsyncComputerRef` borrow the input as `&Input`. None of the four supertrait `HasErrorType`, because none can fail.

`CanTryCompute` adds the failure path. It supertraits `HasErrorType` so its `Result` can name the context's abstract error, which is what keeps a provider generic over the error backend:

```rust
#[cgp_component(TryComputer)]
#[derive_delegate(UseDelegate<Code>)]
#[derive_delegate(UseInputDelegate<Input>)]
pub trait CanTryCompute<Code, Input>: HasErrorType {
    type Output;
    fn try_compute(&self, _code: PhantomData<Code>, input: Input)
        -> Result<Self::Output, Self::Error>;
}
```

A `TryComputer` provider carries a `Context: HasErrorType` bound and typically converts a concrete source error into the abstract one with [`CanRaiseError`](error-handling.md). `TryComputerRef` is the by-reference sibling.

`CanHandle` is the general corner — async *and* fallible — and is the bound generic pipeline code targets, since `Computer`, `AsyncComputer`, and `TryComputer` all promote up to it:

```rust
#[async_trait]
#[cgp_component(Handler)]
#[derive_delegate(UseDelegate<Code>)]
#[derive_delegate(UseInputDelegate<Input>)]
pub trait CanHandle<Code, Input>: HasErrorType {
    type Output;
    async fn handle(&self, _tag: PhantomData<Code>, input: Input)
        -> Result<Self::Output, Self::Error>;
}
```

A function bounded by `Context: CanHandle<Code, Input>` accepts any wired computation regardless of which capabilities the underlying provider actually uses. `HandlerRef` borrows the input.

`CanProduce` is the no-input case: it threads no `Input`, only the context and a `Code` tag, and so carries a single `#[derive_delegate(UseDelegate<Code>)]` with no `UseInputDelegate` counterpart and does not supertrait `HasErrorType`:

```rust
#[cgp_component(Producer)]
#[derive_delegate(UseDelegate<Code>)]
pub trait CanProduce<Code> {
    type Output;
    fn produce(&self, _code: PhantomData<Code>) -> Self::Output;
}
```

A producer is the natural source of values that flow into a pipeline — a constant or a context-derived default — and promotes into any input-taking handler by ignoring the supplied input.

A handler provider is an ordinary zero-sized provider implementing the provider trait for a generic context. The built-in `UseField<Tag>` is a `Computer` that forwards the computation to the value held in the context's `Tag` field; beyond it, the combinators below supply the rest.

## Task runners: `CanRun` and `CanSendRun`

The runner pair executes a unit of work selected by a `Code` tag rather than transforming a value, completing to `Result<(), Error>`. `CanRun<Code>` is the base async form; `CanSendRun<Code>` exists to recover a `Send` future for spawning:

```rust
#[cgp_component(Runner)]
#[async_trait]
#[derive_delegate(UseDelegate<Code>)]
pub trait CanRun<Code>: HasErrorType {
    async fn run(&self, _code: PhantomData<Code>) -> Result<(), Self::Error>;
}

#[cgp_component(SendRunner)]
#[async_trait]
#[derive_delegate(UseDelegate<Code>)]
pub trait CanSendRun<Code>: HasErrorType {
    fn send_run(&self, _code: PhantomData<Code>)
        -> impl Future<Output = Result<(), Self::Error>> + Send;
}
```

A context hosts many tasks by wiring `RunnerComponent` to a `UseDelegate` table keyed on `Code`, so `app.run(PhantomData::<ActionA>)` and `app.run(PhantomData::<ActionB>)` reach distinct providers. A runner provider reaches the runtime through `HasRuntime` to spawn or await work; the `CanSendRun` variant is the mechanism for the `Send`-recovery pattern described under [recovering `Send` bounds](#recovering-send-bounds).

## Defining handlers from functions

`#[cgp_computer]` turns a plain function into a `Computer` provider and wires the rest of the family by promotion, so a computation as small as "add two numbers" needs no hand-written plumbing:

```rust
#[cgp_computer]
fn add(a: u64, b: u64) -> u64 {
    a + b
}
```

The function name becomes the provider name in PascalCase (`Add`), unless an explicit name is given as `#[cgp_computer(MyAdder)]`. The parameters are collected into a tuple that becomes the single `Input` type and destructured back inside the generated method; the return type becomes `Output`. The macro emits the function unchanged, a `#[cgp_new_provider]` impl of the base trait, and a `delegate_components!` block routing every other handler component to a promotion bundle:

```rust
#[cgp_new_provider]
impl<__Context__, __Code__> Computer<__Context__, __Code__, (u64, u64)> for Add {
    type Output = u64;
    fn compute(_context: &__Context__, _code: PhantomData<__Code__>,
        (arg_0, arg_1): (u64, u64)) -> Self::Output {
        add(arg_0, arg_1)
    }
}
// delegate_components! routes the rest of the family to PromoteComputer<Self>
```

The base trait and bundle are chosen from two axes the macro reads off the signature. A synchronous plain-value function uses `Computer` + `PromoteComputer`; a synchronous `Result`-returning one keeps `Computer` as the base (its `Output` is the `Result`) but uses `PromoteTryComputer`, which surfaces the `Ok`/`Err` as genuine success/failure; an `async` plain-value function uses `AsyncComputer` + `PromoteAsyncComputer`; an `async` `Result` function uses `AsyncComputer` + `PromoteHandler`. Generic parameters and `where` clauses carry onto the impl; a `&Value` parameter makes the input tuple borrow and the bundle's `PromoteRef` entries serve the `*Ref` components. The result is that one `add` answers `compute`, `try_compute`, `compute_async`, and `handle`.

`#[cgp_producer]` is the input-less sibling, turning a zero-argument function into a `Producer` and wiring `PromoteProducer` across the whole family:

```rust
#[cgp_producer]
fn magic_number() -> u64 {
    42
}
```

The function must take no parameters, must not be `async`, and must have no generics — a producer's shape is fixed, so there is no variation in the expansion. The generated `MagicNumber` answers `produce` and, because `PromoteProducer` discards the input that each computer slot supplies, also `compute`, `try_compute`, `handle`, and the `*Ref` forms, every one yielding `42`.

## Composing and promoting with handler combinators

The handler combinators are zero-sized providers of `cgp-handler` that build, sequence, and lift handlers, carrying their inner providers in `PhantomData`. They divide into composition, the identity element, and promotion.

`ComposeHandlers<ProviderA, ProviderB>` runs two handlers back to back, pinning the second's input to the first's output and exposing the pair as a member of every handler family; the fallible variants `?`-short-circuit and the async variants `.await` each step. `PipeHandlers<Providers>` generalizes it to a `Product![...]` list, folding right to left so `PipeHandlers<Product![A, B, C]>` behaves as `ComposeHandlers<A, ComposeHandlers<B, C>>` and serves whichever handler shape the wiring asks for, provided every stage supports it:

```rust
delegate_components! {
    MyContext {
        ComputerComponent:
            PipeHandlers<Product![
                Multiply<Symbol!("foo")>,
                Add<Symbol!("bar")>,
                Multiply<Symbol!("baz")>,
            ]>,
    }
}
// input 5 over foo=2, bar=3, baz=4 -> ((5 * 2) + 3) * 4
```

`ReturnInput` is the identity handler — it ignores the context and `Code` and returns its input unchanged (wrapped in `Ok` for the fallible variants) — and is the neutral element of composition, useful as a placeholder stage.

The promotion combinators each take one inner provider and re-expose it under a different, more capable family member, encoding the one-directional lifts the axes permit. `Promote<Provider>` lifts upward without adding behavior: a `Producer` into a `Computer` (ignoring the input), a `Computer` into a `TryComputer` (wrapping in `Ok`), an `AsyncComputer` into a `Handler`. `PromoteAsync<Provider>` lifts a sync provider into an async one whose future is immediately ready. `PromoteRef<Provider>` bridges value and reference handlers by dereferencing or re-borrowing. `TryPromote<Provider>` bridges a `Result`-valued `Computer` and a genuine `TryComputer` in both directions. You rarely name these one at a time; instead the *promotion bundles* — `PromoteComputer`, `PromoteTryComputer`, `PromoteProducer`, `PromoteAsyncComputer`, `PromoteHandler` — are `delegate_components!` tables that wire a whole cluster of components to the right single-step promoter from a given base. These are exactly what `#[cgp_computer]` and `#[cgp_producer]` wire for you, and reaching for `PromoteComputer<MyProvider>` by hand achieves the same when wiring explicitly.

## Dispatching over extensible data

Dispatching routes an [extensible-data](extensible-data.md) value to per-variant or per-field handlers: for an enum, match the current variant and run its handler; for a record, run a handler per field to build it. It keeps the per-variant/per-field structure of a concrete `match` or struct literal but lets the shape and handlers be chosen by type, so the same matcher serves many enums.

`#[cgp_auto_dispatch]` is the highest-level entry point. Written above a trait that already has one impl per payload type, it generates a blanket impl of that trait for any extensible enum of those types, dispatching each variant to that payload's impl:

```rust
#[cgp_auto_dispatch]
pub trait HasArea {
    fn area(&self) -> f64;
}
// with impls for Circle and Rectangle, and a #[derive(CgpData)] enum Shape,
// HasArea is now implemented for Shape too:
let shape = Shape::Rectangle(Rectangle { width: 2.0, height: 2.0 });
assert_eq!(shape.area(), 4.0);
```

For each method it emits a per-variant computer via `#[cgp_computer]` (named `Compute` plus the method name) and an enum-level impl that runs the appropriate value-handler matcher — `MatchWithValueHandlers` for `&self`, its `Mut` form for `&mut self`, and the `MatchFirstWith…` family when the method takes extra arguments, with the `Async` form for `async` methods. A method may not have non-lifetime generic parameters, since the generated impl would need a quantified bound Rust lacks; such a method must use the combinators directly.

Underneath, the dispatch combinators of `cgp-dispatch` express both directions as handler providers. On the matching side, `MatchWithHandlers<Handlers>` (with `Ref`/`Mut` and `MatchFirstWith…` forms) converts the input to its extractor and runs a list of per-variant adapters — `ExtractFieldAndHandle<Tag, Provider>` tries one variant and forwards its payload, `HandleFieldValue` strips the `Field` wrapper to hand the bare value to a computer, and `DowncastAndHandle` matches a group of variants to a sub-matcher. The list runs as a monadic pipeline that short-circuits on the first match and proves exhaustiveness without a wildcard, because each miss rules out one variant until the final remainder is uninhabited. `MatchWithValueHandlers<Provider>` builds that list automatically from the enum's own field list. Dispatch on the *input* type uses the `Computer` component's `UseInputDelegate` directive, wired as a nested table with one entry per input type:

```rust
delegate_components! {
    App {
        ComputerComponent:
            UseInputDelegate<new AreaComputers {
                [Circle, Rectangle]: ComputeArea,
                [Shape]: MatchWithValueHandlers,
            }>,
    }
}
```

Each entry routes one input type (the array form sharing a provider across several), so a `Circle` or `Rectangle` input reaches `ComputeArea` while a whole `Shape` enum reaches the matcher. This input-type dispatch keys on the `Input` parameter through the component's `UseInputDelegate<Input>` directive, which is why it is written as a nested table rather than through the `open` statement — `open` rides the default `RedirectLookup` that keys on the primary `Code` parameter (see [wiring](wiring.md) for the `open` form, which is the modern way to do `Code`-keyed dispatch). The nested-table form is the established way to dispatch on the input type, and this concerns only the *wiring* — the dispatch combinators above are unaffected.

On the building side, `BuildWithHandlers<Output, Handlers>` starts from an empty builder, pipes it through per-field adapters — `BuildAndSetField<Tag, Provider>` computes and sets one field, `BuildAndMerge<Provider>` merges a whole record's shared fields — and finalizes. Because finalization is available only when every field is present, omitting a handler for a field is a compile error rather than a runtime half-built value.

## Composing through a monad

Plain composition feeds each output straight into the next step, which is wrong the moment a step can produce a value meaning "stop here." Monadic handlers solve this: the monad decides which branch of a step's output threads forward and which short-circuits out as the final result. `PipeMonadic<M, Providers>` is the entry point — a monad marker plus a `Product![...]` list — and the pipeline it builds is itself a `Computer`-family provider:

```rust
PipeMonadic::<ErrMonadic, Product![Increment, Increment, Increment]>::compute(&context, code, 253)
// 253 -> Ok(254) -> Ok(255) -> Err("overflow"); the third overflow becomes the output
```

CGP ships three monad markers. `IdentMonadic` threads every value forward and never short-circuits, recovering plain `PipeHandlers` composition. `ErrMonadic` short-circuits on `Err` and continues on `Ok` — the `?`-style early return where the first error wins. `OkMonadic` is the mirror, stopping at the first `Ok`, which suits retry-until-success and is what the dispatch matcher loop runs under. The two `Result`-branching markers have transformer forms, `OkMonadicTrans<M>` and `ErrMonadicTrans<M>`, that stack their behavior on a base monad so a pipeline over a nested `Result<Result<T, E>, F>` can short-circuit on the outer error while threading the inner result.

The per-step providers `BindOk<M, Cont>` and `BindErr<M, Cont>` implement a single bind and are what `PipeMonadic` composes internally; they can also be dropped into a `PipeHandlers` list directly for step-by-step control. `BindErr` runs `Cont` on an `Ok` payload and short-circuits an `Err`; `BindOk` is its mirror. `PipeMonadic` also implements the fallible and async-fallible components by demoting each provider through `TryPromote`, applying `ErrMonadic` as a transformer over `M`, and re-wrapping — so a monadic pipeline reached through `try_compute` or `handle` short-circuits on the context's error type as well.

## Recovering `Send` bounds

An async trait method advertises a future whose auto-traits the caller cannot name. The `#[async_trait]` rewrite turns `async fn handle(..)` into `fn handle(..) -> impl Future<..>` with no boxing and no `Send`, which is faithful and zero-cost but drops the `Send` guarantee. The bound becomes load-bearing when the future is spawned onto a work-stealing executor (the default Tokio runtime an Axum server uses), which may migrate a suspended task between threads. The clause you want — "the future of `handle` is `Send` for any arguments" — is Return Type Notation (`handle(..): Send`), which stable Rust does not yet offer.

The workaround is a second, ordinary trait whose method states `+ Send` directly in its return type, sidestepping RTN:

```rust
pub trait CanHandleApiSend<Api>:
    CanHandleApi<Api, Request: Send, Response: Send> + Send + Sync
{
    fn handle_api_send(&self, _api: PhantomData<Api>, request: Self::Request)
        -> impl Future<Output = Result<Self::Response, Self::Error>> + Send;
}
```

This is a plain trait, not a component — it adds nothing to the wiring and exists only to carry the stronger bound. It cannot be implemented with a single generic blanket impl, because the body wraps `self.handle_api(..)` in an `async` block whose `Send`-ness depends on the opaque future it awaits — the same gap restated, just RTN in disguise. The impl must therefore be written per *concrete* `(context, API)` pair, where `self.handle_api(api, request)` resolves through the wiring to a concrete future whose auto-traits the compiler computes structurally and finds `Send`:

```rust
impl CanHandleApiSend<TransferApi> for MockApp {
    async fn handle_api_send(&self, api: PhantomData<TransferApi>, request: Self::Request)
        -> Result<Self::Response, Self::Error> {
        self.handle_api(api, request).await
    }
}
```

Each impl is mechanical forwarding, yet each is also a proof accepted only because the future really is `Send` at that instantiation. One concrete impl per API per context replaces the single generic impl RTN would have allowed — the cost of the missing notation. The built-in `CanSendRun` runner applies the same pattern as a `SendRunner` proxy on the concrete context, letting a spawning runner provider clone the context into a `Send` future without `Send` bounds leaking into any abstract interface.

## Further reference

Online docs: [concepts/handlers.md](https://github.com/contextgeneric/cgp/blob/main/docs/concepts/handlers.md), [concepts/dispatching.md](https://github.com/contextgeneric/cgp/blob/main/docs/concepts/dispatching.md), [concepts/monadic-handlers.md](https://github.com/contextgeneric/cgp/blob/main/docs/concepts/monadic-handlers.md), [concepts/send-bounds.md](https://github.com/contextgeneric/cgp/blob/main/docs/concepts/send-bounds.md); reference docs [components/computer.md](https://github.com/contextgeneric/cgp/blob/main/docs/reference/components/computer.md), [components/try_computer.md](https://github.com/contextgeneric/cgp/blob/main/docs/reference/components/try_computer.md), [components/handler.md](https://github.com/contextgeneric/cgp/blob/main/docs/reference/components/handler.md), [components/producer.md](https://github.com/contextgeneric/cgp/blob/main/docs/reference/components/producer.md), [components/runner.md](https://github.com/contextgeneric/cgp/blob/main/docs/reference/components/runner.md), [macros/cgp_computer.md](https://github.com/contextgeneric/cgp/blob/main/docs/reference/macros/cgp_computer.md), [macros/cgp_producer.md](https://github.com/contextgeneric/cgp/blob/main/docs/reference/macros/cgp_producer.md), [macros/cgp_auto_dispatch.md](https://github.com/contextgeneric/cgp/blob/main/docs/reference/macros/cgp_auto_dispatch.md), [providers/handler_combinators.md](https://github.com/contextgeneric/cgp/blob/main/docs/reference/providers/handler_combinators.md), [providers/dispatch_combinators.md](https://github.com/contextgeneric/cgp/blob/main/docs/reference/providers/dispatch_combinators.md), [providers/monad_providers.md](https://github.com/contextgeneric/cgp/blob/main/docs/reference/providers/monad_providers.md).
