# Handler combinators

The handler combinators are the provider structs of `cgp-handler` that build, sequence, and adapt handlers — composing two providers end to end, threading a whole list through a pipeline, returning the input unchanged, and lifting a provider written for one handler shape into another.

## Purpose

The handler combinators exist because the handler family is not one trait but several closely related ones — [`Computer`](../components/computer.md), [`TryComputer`](../components/try_computer.md), `AsyncComputer`, [`Handler`](../components/handler.md), and [`Producer`](../components/producer.md), each with a `…Ref` variant — and code is rarely written against all of them at once. A provider author writes a plain synchronous `Computer`, or a fallible `TryComputer`, or an async `Handler`, depending on what is natural for the computation. The combinators let those single-shape providers be wired wherever a different shape is expected, and let several providers be glued into a larger one, without forcing the author to re-implement each provider across every trait in the family.

They divide into three jobs. The *composition* combinators (`ComposeHandlers`, `PipeHandlers`) sequence handlers so that the output of one becomes the input of the next. The *identity* combinator (`ReturnInput`) is the neutral element of that composition, passing its input straight through. The *promotion* combinators (`Promote` and friends) lift a provider from one handler trait to another — from a `Computer` up to a `TryComputer`, from a synchronous computer to an async one, from a value handler to a reference handler — so that one written implementation satisfies many traits. Because every combinator is itself a CGP provider, all of this happens at the type level through delegation, and the combinators nest freely inside one another.

Like all CGP providers, these combinators are zero-sized: their type parameters are inner providers carried in `PhantomData`, never runtime values. The combinator names the providers to compose or promote, and the method bodies forward to those providers' associated functions.

## The handler family

Every combinator below is defined in terms of the handler component traits, so a brief orientation helps. The family shares a common method signature: a context reference, a `PhantomData<Code>` tag selecting the operation, and an input, producing an associated `Output`. The members differ in fallibility and asynchrony. [`Computer`](../components/computer.md) is the plain synchronous, infallible form, with `compute(context, code, input) -> Output`. [`TryComputer`](../components/try_computer.md) is synchronous but fallible, returning `Result<Output, Context::Error>` and requiring `Context: HasErrorType`. `AsyncComputer` is asynchronous and infallible. [`Handler`](../components/handler.md) is asynchronous and fallible, the most general member. Each of these has a `…Ref` companion (`ComputerRef`, `TryComputerRef`, `AsyncComputerRef`, `HandlerRef`) whose method takes the input by reference (`&Input`) instead of by value. [`Producer`](../components/producer.md) is the degenerate case that takes no input at all, producing an `Output` from the context and `Code` alone.

The promotion combinators trade on the natural orderings among these. A `Computer` is also a valid `TryComputer` (wrap the output in `Ok`) and a valid `AsyncComputer` (the future is ready immediately); a `TryComputer` is a valid `Handler`; a value handler can serve a reference handler by dereferencing. The combinators encode exactly these one-directional lifts.

## Composing handlers in sequence

`ComposeHandlers<ProviderA, ProviderB>` runs two handlers back to back, feeding the output of the first as the input of the second, and is the fundamental sequencing combinator. It implements every member of the handler family by threading the intermediary value through both providers under the same context and `Code`:

```rust
pub struct ComposeHandlers<ProviderA, ProviderB>(pub PhantomData<(ProviderA, ProviderB)>);
```

For the plain `Computer` shape, the impl requires `ProviderA: Computer<Context, Code, Input>` and `ProviderB: Computer<Context, Code, ProviderA::Output>` — the second provider's input type is pinned to the first's output type — and the composite `Output` is `ProviderB::Output`:

```rust
impl<Context, Code, Input, ProviderA, ProviderB> Computer<Context, Code, Input>
    for ComposeHandlers<ProviderA, ProviderB>
where
    ProviderA: Computer<Context, Code, Input>,
    ProviderB: Computer<Context, Code, ProviderA::Output>,
{
    type Output = ProviderB::Output;

    fn compute(context: &Context, code: PhantomData<Code>, input: Input) -> Self::Output {
        let intermediary = ProviderA::compute(context, code, input);
        ProviderB::compute(context, code, intermediary)
    }
}
```

The same shape is implemented for `TryComputer`, `AsyncComputer`, and `Handler`. The fallible and async variants differ only in how the intermediary is obtained: `TryComputer` and `Handler` use `?` to short-circuit on the first provider's error (and so require `Context: HasErrorType`), and `AsyncComputer` and `Handler` `.await` each step. In every case the two providers share one context and one `Code`; only the value flowing between them changes type.

## Composing a list of handlers

`PipeHandlers<Providers>` generalizes `ComposeHandlers` from two providers to a type-level list of them, composing the whole pipeline right to left into a single nested `ComposeHandlers`. It is parameterized by a [`Product!`](../macros/product.md) list of providers:

```rust
pub struct PipeHandlers<Providers>(pub PhantomData<Providers>);
```

`PipeHandlers` carries no handler impls of its own. Instead it delegates every component to whatever single provider the list folds down to, computed by an internal `ComposeProviders` trait that walks the `Cons`/`Nil` list. A list of one provider folds to that provider unchanged; a list `Cons<ProviderA, rest>` folds to `ComposeHandlers<ProviderA, fold(rest)>`. The delegation entry then routes any handler component on `PipeHandlers<Providers>` to that folded provider:

```rust
delegate_components! {
    <Component, Provider, Providers: ComposeProviders<Provider = Provider>>
    PipeHandlers<Providers> {
        Component: Provider,
    }
}
```

The practical effect is that `PipeHandlers<Product![A, B, C]>` behaves exactly as `ComposeHandlers<A, ComposeHandlers<B, C>>`, threading the input through `A`, then `B`, then `C`, with each stage's output type feeding the next stage's input type. Because the delegation is generic over the `Component` key, the same pipeline simultaneously serves as a `Computer`, `TryComputer`, `AsyncComputer`, or `Handler` — whichever the wiring asks for — provided every stage supports that shape. This is the combinator to reach for when wiring a multi-stage transformation: list the stages in order and let `PipeHandlers` build the composition.

## Returning the input unchanged

`ReturnInput` is the identity handler: it ignores the context and `Code` and returns its input as its output. It is a plain unit struct with no type parameters:

```rust
pub struct ReturnInput;
```

It implements `Computer`, `TryComputer`, `AsyncComputer`, and `Handler`, in each case setting `Output = Input` and returning the input directly (wrapped in `Ok` for the fallible variants, which therefore require `Context: HasErrorType`). `ReturnInput` is the neutral element of handler composition: composing it before or after any other handler leaves that handler's behavior unchanged. It is useful as a placeholder stage, as the base case of a conditionally-built pipeline, or wherever a handler slot must be filled but no transformation is wanted.

## Promoting a provider to another handler shape

The promotion combinators each take a single inner `Provider` and re-expose it under a different member of the handler family, so that an implementation written once satisfies several traits. Each is a one-parameter struct carrying the inner provider in `PhantomData`, and each implements the *target* traits in terms of the inner provider's *source* trait. The lifts they perform are summarized here and detailed below.

`Promote<Provider>` lifts upward along the infallible-to-fallible and sync-to-async axes, treating a less capable provider as a more capable one without adding error or async behavior of its own. It gives three impls:

```rust
pub struct Promote<Provider>(pub PhantomData<Provider>);
```

As a `Computer`, `Promote<Provider>` requires the inner `Provider: Producer<Context, Code>` and ignores its own input, calling `Provider::produce` — this is how a producer (which takes no input) is adapted to fill a computer slot (which is handed an input it does not need). As a `TryComputer`, it requires `Provider: Computer` and wraps the infallible result in `Ok`. As a `Handler`, it requires `Provider: AsyncComputer` and wraps the awaited result in `Ok`. In each case the promotion adds the missing capability — discarding an input, introducing an always-`Ok` result — without changing what the inner provider computes.

`PromoteAsync<Provider>` lifts a synchronous provider into an asynchronous one:

```rust
pub struct PromoteAsync<Provider>(pub PhantomData<Provider>);
```

As an `AsyncComputer`, it requires `Provider: Computer` and runs it synchronously inside the async method (the returned future is immediately ready). As a `Handler`, it requires `Provider: TryComputer` and returns that fallible synchronous result, so a synchronous fallible computer becomes an async fallible handler.

`PromoteRef<Provider>` bridges between value handlers and reference handlers by dereferencing, and is the most thoroughly implemented promotion — it covers all four families in both directions:

```rust
pub struct PromoteRef<Provider>(pub PhantomData<Provider>);
```

For each of `Computer`/`ComputerRef`, `TryComputer`/`TryComputerRef`, `AsyncComputer`/`AsyncComputerRef`, and `Handler`/`HandlerRef`, `PromoteRef` provides two impls. One direction implements the by-value trait given an inner by-reference provider plus `Input: Deref<Target = Target>`, calling the inner provider on `input.deref()`. The other direction implements the by-reference trait given an inner by-value provider that works `for<'a>` over `&'a Input`, calling the inner provider on the borrowed input. This lets a provider written to take `&T` serve a slot that hands it a smart pointer to `T`, and vice versa, without manual deref boilerplate.

`TryPromote<Provider>` lifts in both directions across the boundary between a `Result`-valued output and a fallible trait, unifying the two ways of expressing fallibility:

```rust
pub struct TryPromote<Provider>(pub PhantomData<Provider>);
```

As a `TryComputer`, it requires the inner `Provider: Computer` whose `Output` is itself a `Result<Output, Context::Error>`, and unwraps that into the `TryComputer` result — turning a computer that *returns* a `Result` into a genuine fallible computer. As a `Computer`, it goes the other way: given `Provider: TryComputer`, its output type is `Result<Output, Context::Error>` and it surfaces the fallible result as an ordinary value. The analogous pair lifts between `Handler` (from an `AsyncComputer` returning a `Result`) and `AsyncComputer` (from a `Handler`). All four impls require `Context: HasErrorType`.

## Promotion bundles

Several promotion adapters are not handler impls themselves but delegation tables that wire a whole cluster of handler components to the right single-trait promotion at once. They exist so that a provider author can implement just one trait — say `Computer` — and have the bundle fill in every other member of the family by promotion. Each is defined with [`delegate_components!`](../macros/delegate_components.md) over a generic inner `Provider`, and is what the [`#[cgp_computer]`](../macros/cgp_computer.md) and [`#[cgp_producer]`](../macros/cgp_producer.md) macros wire their generated providers into.

`PromoteComputer<Provider>` starts from a provider that implements `Computer` (the by-value, synchronous, infallible base) and fills in every other family member. It routes `TryComputerComponent` to `Promote<Provider>` (wrap in `Ok`), `AsyncComputerComponent` and `HandlerComponent` to `PromoteAsync<Provider>` (run synchronously in an async method), and all the `…Ref` components to `PromoteRef<Provider>` (dereference, then defer to the base):

```rust
delegate_components! {
    <Provider>
    new PromoteComputer<Provider> {
        ComputerRefComponent: PromoteRef<Provider>,
        TryComputerComponent: Promote<Provider>,
        TryComputerRefComponent: PromoteRef<Provider>,
        AsyncComputerComponent: PromoteAsync<Provider>,
        AsyncComputerRefComponent: PromoteRef<Provider>,
        HandlerComponent: PromoteAsync<Provider>,
        HandlerRefComponent: PromoteRef<Provider>,
    }
}
```

`PromoteTryComputer<Provider>` starts from a provider that implements `TryComputer`. It routes `TryComputerComponent` to `TryPromote<Provider>` and defers all the remaining components to `PromoteComputer<Provider>`, so the fallible base is first turned into a plain computer and the rest of the family is derived from there.

`PromoteProducer<Provider>` starts from a `Producer` — a provider that takes no input. It routes `ComputerComponent` to `Promote<Provider>` (which discards the computer's input and calls `produce`) and defers the rest to `PromoteComputer<Provider>`, so a single produced value flows out of every handler shape regardless of input.

`PromoteAsyncComputer<Provider>` starts from a provider that implements `AsyncComputer`. It wires `HandlerComponent` to `Promote<Provider>` (wrap the awaited value in `Ok`) and the `AsyncComputerRefComponent` and `HandlerRefComponent` to `PromoteRef<Provider>`. It is the async-base counterpart to `PromoteComputer`.

`PromoteHandler<Provider>` starts from the most general base, a provider that implements `Handler`. It routes `HandlerComponent` to `TryPromote<Provider>` and defers the async-ref components to `PromoteAsyncComputer<Provider>`.

## Dispatching on the input type with `UseInputDelegate`

`UseInputDelegate<Components>` is a delegate-style dispatcher analogous to [`UseDelegate`](use_delegate.md), but it keys its lookup table on the handler's `Input` type rather than on the `Code` type. It is defined as a one-parameter struct holding the lookup table:

```rust
pub struct UseInputDelegate<Components>(pub PhantomData<Components>);
```

Whereas `UseDelegate` dispatches on the first generic parameter of a provider trait — `Code` for the handler family — `UseInputDelegate` dispatches on the `Input` parameter, so that the provider handling a value is chosen by the type of that value. The handler component traits enable both dispatchers at once: each is declared with two `#[derive_delegate]` directives, `UseDelegate<Code>` and `UseInputDelegate<Input>`, as on the `Computer` component:

```rust
#[cgp_component(Computer)]
#[derive_delegate(UseDelegate<Code>)]
#[derive_delegate(UseInputDelegate<Input>)]
pub trait CanCompute<Code, Input> {
    type Output;

    fn compute(&self, _code: PhantomData<Code>, input: Input) -> Self::Output;
}
```

For `UseInputDelegate<Components>`, the second directive makes `#[cgp_component]` generate a provider impl that looks up `Components` keyed on the `Input` type and forwards to the matching delegate:

```rust
impl<Context, Code, Input, Components, Delegate> Computer<Context, Code, Input>
    for UseInputDelegate<Components>
where
    Components: DelegateComponent<Input, Delegate = Delegate>,
    Delegate: Computer<Context, Code, Input>,
{
    type Output = Delegate::Output;

    fn compute(context: &Context, code: PhantomData<Code>, input: Input) -> Self::Output {
        Delegate::compute(context, code, input)
    }
}
```

The lookup key is the dispatched parameter — here `Input` — while `Code` and `Context` pass through unchanged, exactly as with `UseDelegate`. (The `#[derive_delegate]` machinery groups the dispatched parameters into a tuple, which for a single parameter collapses to the parameter type itself, so the table is keyed directly on the concrete input type.) `UseInputDelegate` is wired through a nested table inside `delegate_components!` in the same way: the outer entry routes a handler component to `UseInputDelegate<SomeTable>`, and that inner table maps each concrete input type to the provider responsible for it. The same impl shape is generated for every member of the handler family, since each declares the `UseInputDelegate<Input>` directive.

## Examples

A pipeline of computers shows the composition combinators with `PipeHandlers`. Suppose `Multiply<Field>` and `Add<Field>` are `Computer` providers over `u64` that read a factor or addend from a context field, and a context carries `foo`, `bar`, and `baz`:

```rust
use cgp::prelude::*;
use cgp::extra::handler::{CanCompute, PipeHandlers};

#[derive(HasField)]
pub struct MyContext {
    pub foo: u64,
    pub bar: u64,
    pub baz: u64,
}

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
```

Wiring `ComputerComponent` to `PipeHandlers` over the three-stage list composes them into `ComposeHandlers<Multiply<…>, ComposeHandlers<Add<…>, Multiply<…>>>`. Computing over an input of `5` on a context with `foo = 2`, `bar = 3`, `baz = 4` first multiplies by `foo`, then adds `bar`, then multiplies by `baz`, yielding `((5 * 2) + 3) * 4`. The same list could be wired to `HandlerComponent` instead, provided every stage supports the handler shape — and stages of mismatched shapes can be reconciled inline, as in `PromoteAsync<Promote<Add<Symbol!("bar")>>>`, which lifts a plain `Computer` stage up to the async `Handler` shape the pipeline expects.

The promotion bundles appear most often indirectly. A provider author writing a single `Computer` impl and wiring it with [`#[cgp_computer]`](../macros/cgp_computer.md) gets `PromoteComputer<Self>` wired across the rest of the family automatically, so the one implementation answers `try_compute`, `compute_async`, and `handle` as well. Reaching for `PromoteComputer<MyProvider>` by hand achieves the same effect when wiring the components explicitly.

## Related constructs

The combinators are providers of the handler component traits: [`Computer`](../components/computer.md), [`TryComputer`](../components/try_computer.md), [`Handler`](../components/handler.md), and [`Producer`](../components/producer.md), with the conceptual overview in [handlers](../../concepts/handlers.md). The composition and promotion combinators are wired automatically by the [`#[cgp_computer]`](../macros/cgp_computer.md) and [`#[cgp_producer]`](../macros/cgp_producer.md) macros, which generate a single-trait provider and delegate the rest of the family through the promotion bundles. `UseInputDelegate` is the `Input`-keyed sibling of [`UseDelegate`](use_delegate.md), both generated by the `#[derive_delegate]` directive of [`#[cgp_component]`](../macros/cgp_component.md) and wired through nested tables with [`delegate_components!`](../macros/delegate_components.md). The promotion bundles are themselves delegation tables, so they rely on [`DelegateComponent`](../traits/delegate_component.md) and propagate dependencies through [`IsProviderFor`](../traits/is_provider_for.md) to the [check traits](../../concepts/check-traits.md).

## Source

The combinators are defined in `cgp-handler` under [crates/extra/cgp-handler/src/providers/](../../../crates/extra/cgp-handler/src/providers/): `compose.rs` (`ComposeHandlers`), `pipe.rs` (`PipeHandlers` and the internal `ComposeProviders` fold), `return_input.rs` (`ReturnInput`), `promote.rs` (`Promote`), `promote_async.rs` (`PromoteAsync`), `promote_ref.rs` (`PromoteRef`), `try_promote.rs` (`TryPromote`), and `promote_all.rs` (the `PromoteComputer`, `PromoteTryComputer`, `PromoteProducer`, `PromoteAsyncComputer`, and `PromoteHandler` bundles). `UseInputDelegate` is defined in [crates/extra/cgp-handler/src/types.rs](../../../crates/extra/cgp-handler/src/types.rs), and its provider impls are generated from the `#[derive_delegate(UseInputDelegate<Input>)]` directive on the handler component traits in [crates/extra/cgp-handler/src/components/](../../../crates/extra/cgp-handler/src/components/). Behavioral tests exercising the combinators live in [crates/tests/cgp-tests/tests/handler_tests/](../../../crates/tests/cgp-tests/tests/handler_tests/), notably `pipe.rs`.
