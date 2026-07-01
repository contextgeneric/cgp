# `#[cgp_producer]`

`#[cgp_producer]` is an attribute macro that turns a no-argument function into a [`Producer`](../components/producer.md) provider, generating the provider struct, the provider impl, and the wiring that makes the produced value flow out of every handler shape.

## Purpose

`#[cgp_producer]` exists for the degenerate handler that takes no input at all: a computation that yields a value from nothing, such as a constant or a value drawn from the context. It is the input-less sibling of [`#[cgp_computer]`](cgp_computer.md), and like that macro it lets the author write a plain function and have the macro synthesize the provider struct and impl, sparing them the handler plumbing. Where `#[cgp_computer]` defines a [`Computer`](../components/computer.md), `#[cgp_producer]` defines a [`Producer`](../components/producer.md) — the handler-family member whose method takes only the context and `Code`, with no input value.

Because a producer can stand in for any handler — a handler that ignores its input is just a producer with an unused parameter — the macro wires the generated producer into every member of the handler family. A single `#[cgp_producer]` function therefore answers `produce`, `compute`, `try_compute`, `compute_async`, `handle`, and all the `…Ref` variants, every one of them yielding the same produced value.

## Syntax

`#[cgp_producer]` is applied to a free function and accepts an optional provider name as its argument:

```rust
#[cgp_producer]
fn magic_number() -> u64 {
    42
}

#[cgp_producer(TheAnswer)]
fn magic_number() -> u64 {
    42
}
```

When the argument is omitted the provider struct takes the function name in PascalCase — `magic_number` becomes `MagicNumber` — and when given it is used verbatim. The function's return type becomes the producer's output. The macro constrains the function tightly to match what a producer can be: it must have no parameters (a producer takes no input and no `self` receiver), it must not be `async` (the producer trait is synchronous), and it must have no generic parameters. Violating any of these is a compile error pointing at the offending part of the signature.

## Syntax Grammar

The attribute argument of `#[cgp_producer]` is a single optional provider name:

```ebnf
CgpProducerArgs -> ProviderName?

ProviderName    -> IDENTIFIER
```

When the argument is omitted, the generated provider struct takes the function name converted to PascalCase; a given `IDENTIFIER` is used verbatim. The annotated function is plain Rust, but the macro constrains it to a producer's shape — no parameters, no `async`, and no generic parameters — as described in Syntax above.

## Expansion

The macro emits three items: the original function unchanged, a `#[cgp_new_provider]` impl of the [`Producer`](../components/producer.md) trait that calls the function, and a `delegate_components!` block wiring the whole handler family to the [`PromoteProducer`](../providers/handler_combinators.md) bundle. Given

```rust
#[cgp_producer]
pub fn magic_number() -> u64 {
    42
}
```

the macro expands to the function plus:

```rust
#[cgp_new_provider]
impl<__Context__, __Code__> Producer<__Context__, __Code__> for MagicNumber {
    type Output = u64;

    fn produce(_context: &__Context__, _code: PhantomData<__Code__>) -> Self::Output {
        magic_number()
    }
}

delegate_components! {
    MagicNumber {
        [
            ComputerComponent,
            ComputerRefComponent,
            TryComputerComponent,
            TryComputerRefComponent,
            AsyncComputerComponent,
            AsyncComputerRefComponent,
            HandlerComponent,
            HandlerRefComponent,
        ]:
            PromoteProducer<Self>,
    }
}
```

The `#[cgp_new_provider]` attribute defines the `MagicNumber` struct and its `IsProviderFor` impl alongside the `Producer` impl. The context and code generic parameters are introduced under the reserved names `__Context__` and `__Code__`; the `produce` method ignores both and simply calls the function. The `delegate_components!` block then routes all eight handler components to [`PromoteProducer<Self>`](../providers/handler_combinators.md), the promotion bundle built for a producer base. That bundle wires `ComputerComponent` to `Promote<Self>` — which discards the computer's input and calls `produce` — and derives the remaining members through `PromoteComputer<Self>`, so every handler shape emits the produced value regardless of any input it is handed.

Unlike `#[cgp_computer]`, the expansion has no variation. Because the function cannot be async, cannot have generics, and is not analyzed for a `Result` return, there is a single base trait and a single promotion bundle for every `#[cgp_producer]` function — the producer's output type is taken exactly as written, whether or not it happens to be a `Result`.

## Examples

A self-contained use defines a producer and wires a minimal context, then reads the same value out through every handler shape:

```rust
use cgp::prelude::*;
use cgp::extra::handler::{Producer, Computer, TryComputer, Handler};

#[cgp_producer]
pub fn magic_number() -> u64 {
    42
}

pub struct App;

delegate_components! {
    App {
        ErrorTypeProviderComponent: UseType<String>,
    }
}

// The single `magic_number` definition answers every shape, all yielding 42:
// MagicNumber::produce(&App, PhantomData::<()>)               == 42
// MagicNumber::compute(&App, PhantomData::<()>, &())          == 42
// MagicNumber::try_compute(&App, PhantomData::<()>, &())      == Ok(42)
// MagicNumber::handle(&App, PhantomData::<()>, &())           resolves to Ok(42)
```

The computer and handler forms accept an input argument and ignore it, since the underlying producer takes none. The error type wired into `App` is what lets the fallible shapes (`try_compute`, `handle`) form their `Result`; the produced value is always returned as `Ok`.

## Related constructs

`#[cgp_producer]` defines a provider for the [`Producer`](../components/producer.md) component, part of the handler family described in [handlers](../../concepts/handlers.md). It is the input-less counterpart of [`#[cgp_computer]`](cgp_computer.md), which defines a [`Computer`](../components/computer.md) from a function with parameters, and both are handler-world analogues of [`#[cgp_fn]`](cgp_fn.md). The generated impl is emitted through [`#[cgp_new_provider]`](cgp_new_provider.md), and the producer is lifted into the full handler family by the [`PromoteProducer`](../providers/handler_combinators.md) bundle wired through [`delegate_components!`](delegate_components.md).

## Source

The macro entrypoint is [crates/macros/cgp-extra-macro/src/lib.rs](../../../crates/macros/cgp-extra-macro/src/lib.rs), forwarding to the implementation in [crates/macros/cgp-extra-macro-lib/src/entrypoints/cgp_producer.rs](../../../crates/macros/cgp-extra-macro-lib/src/entrypoints/cgp_producer.rs). The `Producer` trait is defined in [crates/extra/cgp-handler/src/components/produce.rs](../../../crates/extra/cgp-handler/src/components/produce.rs), and the `PromoteProducer` bundle in [crates/extra/cgp-handler/src/providers/promote_all.rs](../../../crates/extra/cgp-handler/src/providers/promote_all.rs). Behavioral and snapshot tests live in [crates/tests/cgp-tests/tests/handlers/producer_macro.rs](../../../crates/tests/cgp-tests/tests/handlers/producer_macro.rs).
