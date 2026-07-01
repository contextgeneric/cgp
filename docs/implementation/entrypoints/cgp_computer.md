# `#[cgp_computer]` — implementation

`#[cgp_computer]` turns a plain function into a [`Computer`](../../reference/components/computer.md) (or `AsyncComputer`) provider by emitting a `#[cgp_new_provider]` impl that calls the function and a `delegate_components!` block that promotes the rest of the handler family from that base. This document covers how the macro is built; for the accepted syntax and the full expansion a user sees, read the reference document [reference/macros/cgp_computer.md](../../reference/macros/cgp_computer.md).

## Entry point

The macro is the `cgp_computer` function in [cgp-extra-macro-lib/src/entrypoints/cgp_computer.rs](../../../crates/macros/cgp-extra-macro-lib/src/entrypoints/cgp_computer.rs). Unlike the `cgp-macro-lib` macros, it is a self-contained procedural function rather than a driver over a `cgp-macro-core` AST stack: it parses the body into a `syn::ItemFn`, resolves the provider name, inspects the signature, and assembles the output with `quote!` directly. The only piece it borrows from the core crate is `to_camel_case_str`, used to derive the default provider name.

The provider name comes from the attribute: when the attribute is empty the function name is converted to PascalCase (`add` becomes `Add`), otherwise the attribute tokens are parsed as the provider identifier. A `self` receiver on the function is rejected with a spanned error, since a handler provider has no receiver.

## Pipeline

There is no staged AST pipeline; the function branches on two independent axes read off the signature and emits the three items in one pass.

- **Sync vs. async** — `fn_sig.asyncness` selects between a [`Computer`](../../reference/components/computer.md) base (the `compute` method) and an `AsyncComputer` base (an `async fn compute_async` that `.await`s the call).
- **Value vs. `Result`** — the function's return type is re-parsed as a [`MaybeResultType`](../../../crates/macros/cgp-extra-macro-lib/src/parse/maybe_result.rs), a small speculative parser that reports whether the output is written as `Result<_, E>`. This choice does not change the base impl (the `Output` associated type is the return type verbatim); it only selects which promotion bundle the `delegate_components!` block wires the rest of the family to.

## Generated items

The macro emits three items in order: the original function unchanged, a `#[cgp_new_provider]` impl of the base trait, and a `delegate_components!` block. The base impl collects the function's parameters into a single input tuple and destructures it back into positional `arg_0, arg_1, …` bindings inside the method body before calling the function:

```rust
// #[cgp_computer] fn add(a: u64, b: u64) -> u64 { a + b }
#[cgp_new_provider]
impl<__Context__, __Code__> Computer<__Context__, __Code__, (u64, u64)> for Add {
    type Output = u64;

    fn compute(_context: &__Context__, _code: PhantomData<__Code__>, (arg_0, arg_1): (u64, u64)) -> Self::Output {
        add(arg_0, arg_1)
    }
}
```

The function's own generics and `where` clause are cloned onto the impl, and the reserved `__Context__` and `__Code__` type parameters are appended after them, so a generic function stays generic in its provider. The `delegate_components!` block then routes every remaining handler component to a promotion bundle chosen by the two axes:

- **sync, value** → `PromoteComputer<Self>`, over the seven non-`Computer` components.
- **sync, `Result`** → `PromoteTryComputer<Self>`, over the same seven components.
- **async, value** → `PromoteAsyncComputer<Self>`, over the smaller async set (`AsyncComputerRefComponent`, `HandlerComponent`, `HandlerRefComponent`).
- **async, `Result`** → `PromoteHandler<Self>`, over the same async set.

The async branches delegate fewer components because the synchronous members of the family are not derivable from an async base.

## Behavior and corner cases

A **`self` receiver** is rejected with a spanned `syn::Error` ("Computer functions cannot have a receiver"); every other parameter is treated as part of the input tuple. Parameter *patterns* are discarded — each input is rebound positionally as `arg_i`, so a destructuring pattern in the source signature is replaced by a plain binding.

A **reference parameter** is kept verbatim in the input-tuple type, so `fn f(value: &Value)` yields an input tuple `(&Value)`; the `PromoteComputer` bundle's `…Ref` entries then make the provider serve the `…Ref` components as well.

The **`Result` detection is purely syntactic**. `MaybeResultType` forks the token stream and checks whether the return type's leading identifier is literally `Result`; a type-aliased result or a `Result` under a different name reads as the value case and wires `PromoteComputer` rather than `PromoteTryComputer`.

An **omitted return type** defaults to `()`, so a unit-returning function produces a value-case `Computer` with `Output = ()`.

## Tests

The behavioral tests exercise the generated provider across the handler family:

- [handlers/computer_macro.rs](../../../crates/tests/cgp-tests/tests/handlers/computer_macro.rs) — a synchronous infallible function, a `Result`-returning function, and a generic function, each called as `compute`, `try_compute`, `compute_async`, and `handle` plus their `…Ref` variants.
- [handlers/handler_macro.rs](../../../crates/tests/cgp-tests/tests/handlers/handler_macro.rs) — an `async` function (value and `Result` bodies) reached through the async promotion into `Handler`.
- [handlers/pipe_computers.rs](../../../crates/tests/cgp-tests/tests/handlers/pipe_computers.rs), [handlers/pipe_handlers.rs](../../../crates/tests/cgp-tests/tests/handlers/pipe_handlers.rs) — `#[cgp_computer]` providers composed through `PipeHandlers`.
- [dispatching/compose.rs](../../../crates/tests/cgp-tests/tests/dispatching/compose.rs) — `#[cgp_computer]` field-reader providers composed into a higher-order provider.
- [monadic_handlers/ok_monadic.rs](../../../crates/tests/cgp-tests/tests/monadic_handlers/ok_monadic.rs), [monadic_handlers/err_monadic.rs](../../../crates/tests/cgp-tests/tests/monadic_handlers/err_monadic.rs), [monadic_handlers/ok_err_monadic_trans.rs](../../../crates/tests/cgp-tests/tests/monadic_handlers/ok_err_monadic_trans.rs) — `#[cgp_computer]` providers chained through the monadic combinators.

There is no dedicated `snapshot_cgp_computer!` macro; the macro's expansion is not pinned by a snapshot and is exercised only behaviorally.

## Source

- Entry point: `cgp_computer` in [cgp-extra-macro-lib/src/entrypoints/cgp_computer.rs](../../../crates/macros/cgp-extra-macro-lib/src/entrypoints/cgp_computer.rs), forwarded from the proc-macro shim in [cgp-extra-macro/src/lib.rs](../../../crates/macros/cgp-extra-macro/src/lib.rs).
- The `Result`-versus-value split: [`MaybeResultType`](../../../crates/macros/cgp-extra-macro-lib/src/parse/maybe_result.rs).
- The emitted items lean on the base impl from [`#[cgp_new_provider]`](cgp_new_provider.md) and the wiring from [`delegate_components!`](delegate_components.md).
- The input-less sibling macro is [`#[cgp_producer]`](cgp_producer.md).
