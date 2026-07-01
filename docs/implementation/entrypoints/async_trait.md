# `#[async_trait]` — implementation

`#[async_trait]` rewrites every `async fn` declared in a trait into a plain method returning `-> impl Future`, so a trait can advertise async methods without tripping the `async_fn_in_trait` lint. This document covers how the macro is built; for the accepted syntax and the full expansion, read the reference document [reference/macros/async_trait.md](../../reference/macros/async_trait.md).

## Entry point

The macro lives entirely in the [cgp-async-macro](../../../crates/macros/cgp-async-macro/) crate — it does not use `cgp-macro-core` at all. The `#[proc_macro_attribute]` shim `async_trait` in [cgp-async-macro/src/lib.rs](../../../crates/macros/cgp-async-macro/src/lib.rs) discards its attribute arguments unconditionally and forwards the annotated item to `impl_async` in [cgp-async-macro/src/impl_async.rs](../../../crates/macros/cgp-async-macro/src/impl_async.rs). Because the attribute tokens are ignored, the macro is always written bare.

## Pipeline

There is no staged pipeline; `impl_async` is a single `syn::parse2::<ItemTrait>` followed by an in-place mutation pass. It attempts to parse the item as a trait, and on success walks the trait's methods, rewriting each `async` signature and returning the modified trait; on any parse failure it returns the original tokens unchanged. This parse-or-passthrough structure is what makes the macro composable — applying it to an `impl` block (which does not parse as an `ItemTrait`) is a no-op, so a provider's `async fn` bodies are left intact while the matching trait declaration carries the rewritten signature.

## Generated items

The macro emits the same trait it was given, with each `async` method signature rewritten. For a method whose signature is `async`, it clears the `async` keyword and replaces the return type `T` with `impl ::core::future::Future<Output = T>`; a method with no return arrow is treated as returning `()`:

```rust
// async fn fetch(&self, id: &str) -> Result<Vec<u8>, Error>;
fn fetch(&self, id: &str) -> impl ::core::future::Future<Output = Result<Vec<u8>, Error>>;

// async fn run(&self);
fn run(&self) -> impl ::core::future::Future<Output = ()>;
```

Only method signatures are touched. Non-async methods, associated types, and associated constants pass through unchanged, and the method body — for the rare method that has one — is never rewritten (see Known issues).

## Behavior and corner cases

The **only structural transform is to the signature**: the `async` token is removed and the return type is wrapped in `impl Future`. The macro introduces no boxing, no allocation, and no `Send` bound — it is a mechanical spelling of return-position `impl Trait` in a trait, and the returned future is exactly the one the method body produces.

The **passthrough on non-trait items is deliberate**, not an error path in the usual sense: any item that fails to parse as an `ItemTrait` is returned verbatim, which is what lets `#[async_trait]` sit harmlessly on an `impl` block that a host macro like [`#[cgp_fn]`](cgp_fn.md) copies the attribute onto.

## Known issues

The macro **mishandles an async method that carries a default body**. It strips `async` and rewrites the return type to `impl Future`, but leaves the body verbatim rather than wrapping it in an `async { … }` block. The result is a non-async method whose body still returns a plain value (and may use `.await`), which fails to compile. In practice this is rarely hit — async trait methods are almost always bodyless declarations, with behavior supplied by a provider impl — but a default-bodied `async fn` inside a `#[async_trait]` trait is not supported. The correct behavior would be to wrap a rewritten method's body in `async move { … }` so the signature and body agree. The user-visible consequence, together with the absence of a `Send` bound on the generated future, is documented under Known issues in [reference/macros/async_trait.md](../../reference/macros/async_trait.md).

## Tests

The macro is exercised through its interaction with the constructs it stacks onto rather than on its own:

- [async_and_send/cgp_fn_async.rs](../../../crates/tests/cgp-tests/tests/async_and_send/cgp_fn_async.rs) — a `snapshot_cgp_fn!` over an async `#[cgp_fn]` that also carries `#[async_trait]`, pinning the rewritten trait declaration; this snapshot is owned by the `#[cgp_fn]` feature, not by `#[async_trait]`.
- [async_and_send/spawn.rs](../../../crates/tests/cgp-tests/tests/async_and_send/spawn.rs) — async components declared with `#[async_trait]` whose futures are handed to a `Send + 'static`-demanding executor.
- [dispatching/auto_dispatch_async_self_ref_only.rs](../../../crates/tests/cgp-tests/tests/dispatching/auto_dispatch_async_self_ref_only.rs) and the other `auto_dispatch_async_*` files — `#[async_trait]` stacked with [`#[cgp_auto_dispatch]`](cgp_auto_dispatch.md) on async dispatch traits.

There is no dedicated `snapshot_async_trait!` macro; the rewrite is only pinned incidentally through the `#[cgp_fn]` snapshot above.

## Source

- Entry point: `async_trait` in [cgp-async-macro/src/lib.rs](../../../crates/macros/cgp-async-macro/src/lib.rs).
- The rewrite: `impl_async` in [cgp-async-macro/src/impl_async.rs](../../../crates/macros/cgp-async-macro/src/impl_async.rs).
- Re-exported into the prelude from [crates/main/cgp-core/src/prelude.rs](../../../crates/main/cgp-core/src/prelude.rs).
- It is most often stacked with [`#[cgp_component]`](cgp_component.md) and [`#[cgp_fn]`](cgp_fn.md).
