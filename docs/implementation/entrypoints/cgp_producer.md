# `#[cgp_producer]` — implementation

`#[cgp_producer]` turns a no-argument function into a [`Producer`](../../reference/components/producer.md) provider by emitting a `#[cgp_new_provider]` impl that calls the function and a `delegate_components!` block that promotes the whole handler family from it. This document covers how the macro is built; for the accepted syntax and the full expansion, read the reference document [reference/macros/cgp_producer.md](../../reference/macros/cgp_producer.md).

## Entry point

The macro is the `cgp_producer` function in [cgp-extra-macro-lib/src/entrypoints/cgp_producer.rs](../../../crates/macros/cgp-extra-macro-lib/src/entrypoints/cgp_producer.rs). Like [`#[cgp_computer]`](cgp_computer.md), it is a self-contained procedural function rather than a driver over a `cgp-macro-core` AST stack: it parses the body into a `syn::ItemFn`, resolves the provider name (the attribute tokens if present, otherwise the function name in PascalCase via `to_camel_case_str`), validates the signature, and assembles the output with `quote!`.

## Pipeline

There is no staged AST pipeline and, unlike `#[cgp_computer]`, no branching — a producer has exactly one shape. The function validates the signature against the producer's constraints and then emits a fixed set of three items. The three constraints are each checked before any code is generated:

- **no parameters** — a producer takes no input and no `self` receiver.
- **not `async`** — the `Producer` trait is synchronous.
- **no generic parameters** — the producer impl introduces only the reserved context and code parameters.

Each violation returns a spanned `syn::Error` pointing at the offending part of the signature.

## Generated items

The macro emits three items: the original function unchanged, a `#[cgp_new_provider]` impl of the [`Producer`](../../reference/components/producer.md) trait, and a `delegate_components!` block. The impl introduces the reserved `__Context__` and `__Code__` parameters, ignores both in its `produce` body, and simply calls the function:

```rust
// #[cgp_producer] fn magic_number() -> u64 { 42 }
#[cgp_new_provider]
impl<__Context__, __Code__> Producer<__Context__, __Code__> for MagicNumber {
    type Output = u64;

    fn produce(_context: &__Context__, _code: PhantomData<__Code__>) -> Self::Output {
        magic_number()
    }
}
```

The `delegate_components!` block then routes all eight handler components — including `ComputerComponent`, which `#[cgp_computer]` never delegates because the computer *is* its own base — to the single `PromoteProducer<Self>` bundle. Because a producer ignores its input, that bundle lets every handler shape yield the produced value regardless of any input it is handed. There is no `Result` analysis: the `Output` associated type is the return type verbatim, whether or not it is a `Result`.

## Behavior and corner cases

An **omitted return type** defaults to `()`, so a producer written with no return arrow yields `Output = ()`.

The three signature checks are the macro's only corner cases, and all reject rather than reinterpret: a parameter, an `async` keyword, or a generic parameter each aborts expansion with a spanned error rather than being silently accepted. This is stricter than `#[cgp_computer]`, which accepts parameters, `async`, and generics.

## Tests

The behavioral test exercises the generated provider across the handler family:

- [handlers/producer_macro.rs](../../../crates/tests/cgp-tests/tests/handlers/producer_macro.rs) — an input-free function called as `produce`, `compute`, `try_compute`, `compute_async`, and `handle` plus their `…Ref` variants, all yielding the same value.

There is no dedicated `snapshot_cgp_producer!` macro; the macro's expansion is not pinned by a snapshot and is exercised only behaviorally.

## Source

- Entry point: `cgp_producer` in [cgp-extra-macro-lib/src/entrypoints/cgp_producer.rs](../../../crates/macros/cgp-extra-macro-lib/src/entrypoints/cgp_producer.rs), forwarded from the proc-macro shim in [cgp-extra-macro/src/lib.rs](../../../crates/macros/cgp-extra-macro/src/lib.rs).
- The emitted items lean on [`#[cgp_new_provider]`](cgp_new_provider.md) for the base impl and [`delegate_components!`](delegate_components.md) for the wiring.
- The input-carrying sibling macro is [`#[cgp_computer]`](cgp_computer.md).
