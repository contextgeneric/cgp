# `#[cgp_computer]`

`#[cgp_computer]` is an attribute macro that turns a plain function into a [`Computer`](../components/computer.md) provider, generating the provider struct, the provider impl, and the wiring that fills in the rest of the handler family by promotion.

## Purpose

`#[cgp_computer]` exists to make the simplest handler — a pure function from input to output — definable as an ordinary Rust function, the way [`#[cgp_fn]`](cgp_fn.md) makes a blanket-impl trait definable as a function. A handler in CGP is normally a provider struct with one or more impls across the [`Computer`](../components/computer.md), [`TryComputer`](../components/try_computer.md), `AsyncComputer`, and [`Handler`](../components/handler.md) traits, each carrying the `context`/`code`/`input` plumbing. Writing that by hand for a computation as small as "add two numbers" is disproportionate boilerplate. `#[cgp_computer]` lets the author write just the computation as a function and have the macro synthesize the provider and wire it into every member of the handler family.

The macro encodes a single decision: whether the function is synchronous or asynchronous, and whether it returns a plain value or a `Result`. From that it picks the right base trait to implement and the right [promotion bundle](../providers/handler_combinators.md) to derive the rest of the family from. The result is that one function definition yields a provider usable wherever any handler shape — `compute`, `try_compute`, `compute_async`, `handle`, and their `…Ref` variants — is expected.

## Syntax

`#[cgp_computer]` is applied to a free function and accepts an optional provider name as its argument:

```rust
#[cgp_computer]
fn add(a: u64, b: u64) -> u64 {
    a + b
}

#[cgp_computer(MyAdder)]
fn add(a: u64, b: u64) -> u64 {
    a + b
}
```

When the argument is omitted, the generated provider struct takes the function name converted to PascalCase — `add` becomes `Add`. When an argument is given, it is used verbatim as the provider name. The function's parameters become the handler's input, its return type becomes the handler's output, and its generic parameters and `where` clause carry over to the generated impl. The function may not have a `self` receiver, since a handler provider has no receiver — the context is supplied separately by the handler machinery. The function may be `async`, which selects the asynchronous base trait described below.

## Syntax Grammar

The attribute argument of `#[cgp_computer]` is a single optional provider name:

```ebnf
CgpComputerArgs -> ProviderName?

ProviderName    -> IDENTIFIER
```

When the argument is omitted, the generated provider struct takes the function name converted to PascalCase; a given `IDENTIFIER` is used verbatim. The shape of the annotated function — its parameters, return type, `async`-ness, generics, and `where` clause — is plain Rust and is read by the macro to choose the base trait and promotion bundle, as described in Expansion.

## Expansion

The macro emits three items: the original function unchanged, a `#[cgp_new_provider]` impl of a base handler trait that calls the function, and a `delegate_components!` block that wires the remaining handler components to a promotion bundle. The base trait and the bundle are chosen from two independent axes — sync versus async, and value-returning versus `Result`-returning.

For a synchronous function returning a plain value, the base trait is [`Computer`](../components/computer.md). The function's parameters are collected into a tuple that becomes the single `Input` type, and the body destructures that tuple back into the original arguments before calling the function. Given

```rust
#[cgp_computer]
fn add(a: u64, b: u64) -> u64 {
    a + b
}
```

the macro expands to the function plus:

```rust
#[cgp_new_provider]
impl<__Context__, __Code__> Computer<__Context__, __Code__, (u64, u64)> for Add {
    type Output = u64;

    fn compute(
        _context: &__Context__,
        _code: PhantomData<__Code__>,
        (arg_0, arg_1): (u64, u64),
    ) -> Self::Output {
        add(arg_0, arg_1)
    }
}

delegate_components! {
    Add {
        [
            ComputerRefComponent,
            TryComputerComponent,
            TryComputerRefComponent,
            AsyncComputerComponent,
            AsyncComputerRefComponent,
            HandlerComponent,
            HandlerRefComponent,
        ] ->
            PromoteComputer<Self>,
    }
}
```

The `#[cgp_new_provider]` attribute defines the `Add` struct and the `IsProviderFor` impl alongside the `Computer` impl, exactly as it does for any provider. The context and code generic parameters are introduced under the reserved names `__Context__` and `__Code__`. The `delegate_components!` block routes every other handler component to [`PromoteComputer<Self>`](../providers/handler_combinators.md), the promotion bundle that derives `TryComputer`, `AsyncComputer`, `Handler`, and all the `…Ref` variants from a `Computer` base. The result is that `Add` answers `compute`, `try_compute`, `compute_async`, and `handle`, all computing `a + b`.

When the function returns a `Result`, the base trait stays `Computer` — its `Output` is the `Result` type as written — but the promotion bundle changes to [`PromoteTryComputer<Self>`](../providers/handler_combinators.md), which interprets the `Result`-valued computer as a genuinely fallible handler. So

```rust
#[cgp_computer]
fn add_with_error(a: u64, b: u64) -> Result<u64, String> {
    a.checked_add(b).ok_or_else(|| "Overflow".to_string())
}
```

produces a `Computer` impl whose `Output` is `Result<u64, String>`, and a `delegate_components!` block routing the rest of the family to `PromoteTryComputer<Self>`. Through that bundle, `try_compute` and `handle` surface the `Ok`/`Err` outcome as the handler's success or failure rather than as a plain value.

When the function is `async`, the base trait is `AsyncComputer` instead of `Computer`, the generated method is `compute_async` and `.await`s the function call, and the promotion bundle is the async-base counterpart. A plain-value async function wires the remaining components to [`PromoteAsyncComputer<Self>`](../providers/handler_combinators.md); an async function returning a `Result` wires them to [`PromoteHandler<Self>`](../providers/handler_combinators.md). In both async cases the macro delegates a smaller set of components — `AsyncComputerRefComponent`, `HandlerComponent`, and `HandlerRefComponent` — since the synchronous members of the family are not derived from an async base.

Generic parameters and bounds on the function flow into the impl. A function such as

```rust
#[cgp_computer]
pub fn add_generic<T: core::ops::Add<Output = T>>(a: T, b: T) -> T {
    a + b
}
```

carries its `T` and the `T: Add<Output = T>` bound onto the generated `Computer` impl (appended ahead of the introduced `__Context__` and `__Code__` parameters), so the provider is itself generic over `T`. A reference parameter is likewise preserved: `fn to_string_ref<Value: Display>(value: &Value) -> String` becomes a `Computer` whose input tuple is `(&Value)`, and the `PromoteComputer` bundle's `PromoteRef` entries make it serve the `…Ref` components as well.

## Examples

A self-contained use defines two computers and wires a context that supplies the error type, then exercises every handler shape from the single function definitions:

```rust
use cgp::prelude::*;
use cgp::extra::handler::{Computer, TryComputer, AsyncComputer, Handler};

#[cgp_computer]
fn add(a: u64, b: u64) -> u64 {
    a + b
}

pub struct App;

delegate_components! {
    App {
        ErrorTypeProviderComponent: UseType<String>,
    }
}

// All four shapes are answered by the single `add` definition:
// Add::compute(&App, PhantomData::<()>, (1, 2))        == 3
// Add::try_compute(&App, PhantomData::<()>, (1, 2))    == Ok(3)
// Add::compute_async(&App, PhantomData::<()>, (1, 2))  resolves to 3
// Add::handle(&App, PhantomData::<()>, (1, 2))         resolves to Ok(3)
```

Because the function returns a plain `u64`, the `try_compute` and `handle` forms always succeed with `Ok`. Switching the function to return `Result<u64, String>` instead — as `add_with_error` above — makes those same forms propagate the `Err` when the computation fails, with no change to the call sites beyond the now-fallible outcome.

## Related constructs

`#[cgp_computer]` defines a provider for the [`Computer`](../components/computer.md) component (or `AsyncComputer` for async functions), part of the handler family described in [handlers](../../concepts/handlers.md). It is the handler-world analogue of [`#[cgp_fn]`](cgp_fn.md), which defines a blanket-impl trait from a function; and the input-less counterpart [`#[cgp_producer]`](cgp_producer.md) defines a [`Producer`](../components/producer.md) the same way. The generated impl is emitted through [`#[cgp_new_provider]`](cgp_new_provider.md), and the rest of the handler family is filled in by the [promotion bundles](../providers/handler_combinators.md) (`PromoteComputer`, `PromoteTryComputer`, `PromoteAsyncComputer`, `PromoteHandler`) wired through [`delegate_components!`](delegate_components.md).

## Source

- Entrypoint: [crates/macros/cgp-extra-macro/src/lib.rs](../../../crates/macros/cgp-extra-macro/src/lib.rs), forwarding to the implementation in [crates/macros/cgp-extra-macro-lib/src/entrypoints/cgp_computer.rs](../../../crates/macros/cgp-extra-macro-lib/src/entrypoints/cgp_computer.rs).
- `Result`-versus-value detection: the `MaybeResultType` parser in [crates/macros/cgp-extra-macro-lib/src/parse/maybe_result.rs](../../../crates/macros/cgp-extra-macro-lib/src/parse/maybe_result.rs).
- Base `Computer`/`AsyncComputer` traits: [crates/extra/cgp-handler/src/components/](../../../crates/extra/cgp-handler/src/components/); the promotion bundles in [crates/extra/cgp-handler/src/providers/promote_all.rs](../../../crates/extra/cgp-handler/src/providers/promote_all.rs).
- Internal walkthrough (the sync/async and value/`Result` branching, the generated items, and the index of behavioral tests): [implementation/entrypoints/cgp_computer.md](../../implementation/entrypoints/cgp_computer.md).
