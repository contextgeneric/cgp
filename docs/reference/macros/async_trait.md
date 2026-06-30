# `#[async_trait]`

`#[async_trait]` rewrites each `async fn` declared in a trait into an ordinary method that returns `impl Future`, so a trait can advertise async methods without tripping the `async_fn_in_trait` lint.

## Purpose

`#[async_trait]` exists to make async methods in traits ergonomic to declare. Writing a bare `async fn` directly inside a trait definition compiles, but the compiler emits the `async_fn_in_trait` lint, because a bare `async fn` in a trait leaves the returned future's auto-traits unnameable by callers and is easy to misuse. The hand-written alternative — declaring the method as `fn name(&self) -> impl Future<Output = T>` — silences the lint but is verbose and obscures the intent. `#[async_trait]` lets the author write the natural `async fn` form and performs that rewrite mechanically, so the trait reads as async code while the generated declaration is the lint-clean `impl Future` form.

The rewrite is a zero-cost desugaring to return-position `impl Trait` in traits: no boxing, no allocation, and no implicit `Send` bound are introduced. The returned future is exactly the one the method body produces. This is why the macro is used pervasively alongside [`#[cgp_component]`](cgp_component.md) and [`#[cgp_fn]`](cgp_fn.md) whenever a CGP capability is asynchronous — it is the standard way to spell an async method in a CGP trait.

## Syntax

`#[async_trait]` is an attribute macro applied to a trait definition, taking no arguments. Any tokens placed in the attribute's argument position are ignored, so it is always written bare:

```rust
#[async_trait]
pub trait CanFetchStorageObject {
    async fn fetch_storage_object(&self, object_id: &str) -> anyhow::Result<Vec<u8>>;
}
```

Only the methods declared `async` are affected; non-async methods, associated types, and associated constants in the same trait pass through untouched. When `#[async_trait]` is stacked with a host macro, the ordering follows what that macro needs. With [`#[cgp_component]`](cgp_component.md), place `#[async_trait]` outermost (first) so it rewrites the trait before the component macro reads it:

```rust
#[async_trait]
#[cgp_component(StorageObjectFetcher)]
pub trait CanFetchStorageObject {
    async fn fetch_storage_object(&self, object_id: &str) -> anyhow::Result<Vec<u8>>;
}
```

With [`#[cgp_fn]`](cgp_fn.md), which generates the trait from a function, `#[async_trait]` is written *below* `#[cgp_fn]` on the `async fn`; `#[cgp_fn]` copies it onto the trait and impl it generates:

```rust
#[cgp_fn]
#[async_trait]
pub async fn fetch_storage_object(
    &self,
    #[implicit] storage_client: &Client,
    object_id: &str,
) -> anyhow::Result<Vec<u8>> {
    /* ... */
}
```

## Expansion

For each method in the trait whose signature is `async`, `#[async_trait]` removes the `async` keyword and replaces the return type `T` with `impl ::core::future::Future<Output = T>`. A method written without an explicit return type is treated as returning `()`. The trait above expands to:

```rust
pub trait CanFetchStorageObject {
    fn fetch_storage_object(
        &self,
        object_id: &str,
    ) -> impl ::core::future::Future<Output = anyhow::Result<Vec<u8>>>;
}
```

A method with no return arrow, such as `async fn run(&self);`, expands with `()` as the future's output:

```rust
// async fn run(&self);
fn run(&self) -> impl ::core::future::Future<Output = ()>;
```

The macro rewrites only trait definitions. When it is applied to any other item — most importantly an `impl` block — it leaves the tokens completely unchanged. This passthrough is what makes the macro compose so cleanly: an `async fn` is already legal in an inherent or trait `impl` on stable Rust, and only a trait *declaration* triggers the `async_fn_in_trait` lint. So a provider implementation can keep the natural `async fn` body while the matching trait declaration carries the rewritten `-> impl Future` signature, and the two still agree because an `async fn` desugars to exactly such a future-returning method.

This composition is visible in how [`#[cgp_fn]`](cgp_fn.md) uses it. Given an async `#[cgp_fn]` carrying `#[async_trait]`, `#[cgp_fn]` first produces a trait and a blanket impl, attaching `#[async_trait]` to both:

```rust
#[async_trait]
pub trait FetchStorageObject {
    async fn fetch_storage_object(&self, object_id: &str) -> anyhow::Result<Vec<u8>>;
}

#[async_trait]
impl<__Context__> FetchStorageObject for __Context__
where
    Self: HasField<Symbol!("storage_client"), Value = Client>,
{
    async fn fetch_storage_object(&self, object_id: &str) -> anyhow::Result<Vec<u8>> {
        let storage_client: &Client =
            self.get_field(PhantomData::<Symbol!("storage_client")>);
        /* ... */
    }
}
```

`#[async_trait]` then runs on each. On the trait it rewrites the declaration to `fn fetch_storage_object(&self, object_id: &str) -> impl ::core::future::Future<Output = anyhow::Result<Vec<u8>>>`. On the impl it is a no-op, so the `async fn` body remains as written — and that `async fn` satisfies the trait's `impl Future` method.

## Examples

The most common use is declaring an asynchronous CGP component. The consumer trait carries `#[async_trait]` so its method is a clean `impl Future` declaration, and each provider implements it with a natural `async fn` body:

```rust
use cgp::prelude::*;

#[async_trait]
#[cgp_component(StorageObjectFetcher)]
pub trait CanFetchStorageObject {
    async fn fetch_storage_object(&self, object_id: &str) -> anyhow::Result<Vec<u8>>;
}

#[cgp_impl(new FetchS3Object)]
impl StorageObjectFetcher {
    async fn fetch_storage_object(
        &self,
        #[implicit] storage_client: &Client,
        #[implicit] bucket_id: &str,
        object_id: &str,
    ) -> anyhow::Result<Vec<u8>> {
        let output = storage_client
            .get_object()
            .bucket(bucket_id)
            .key(object_id)
            .send()
            .await?;

        Ok(output.body.collect().await?.into_bytes().to_vec())
    }
}
```

Defining the same capability as a single implementation with [`#[cgp_fn]`](cgp_fn.md) needs `#[async_trait]` directly below it, as shown in the Syntax section. In both forms the author writes only `async fn`, and the lint-clean `impl Future` declaration is generated.

## Related constructs

`#[async_trait]` is most often stacked with [`#[cgp_component]`](cgp_component.md), which builds the consumer and provider traits for an async capability, and with [`#[cgp_fn]`](cgp_fn.md), which generates an async trait and blanket impl from a function. Providers for an async component are written with [`#[cgp_impl]`](cgp_impl.md) using ordinary `async fn` bodies, since the macro's passthrough on impl blocks leaves those untouched. It is independent of CGP's wiring layer — [`delegate_components!`](delegate_components.md) and [`check_components!`](check_components.md) treat an async component exactly like a synchronous one.

## Known issues

`#[async_trait]` rewrites only the method signature, never the method body, so an async trait method that carries a *default body* is mishandled. The macro strips `async` from such a method's signature and changes its return type to `impl Future`, but the body is left verbatim — it is not wrapped in an `async { ... }` block. The result is a non-async method whose body still returns a plain value and may use `.await`, which fails to compile. In practice this is rarely hit, because async methods in a trait are almost always declarations without bodies, and provided async behavior is supplied by a provider impl instead; but a default-bodied `async fn` inside a `#[async_trait]` trait is not supported.

The generated future carries no `Send` bound. Because the rewrite produces a bare `impl Future<Output = T>`, the returned future is `Send` only when the concrete future happens to be, and the trait does not require it. Code that must spawn the future onto a work-stealing, multi-threaded executor — which demands `Send` futures — cannot express that requirement through this macro and must add the bound by other means, as described in [recovering `Send` bounds](../../concepts/send-bounds.md).

## Source

The macro entry point is `async_trait` in [crates/macros/cgp-async-macro/src/lib.rs](../../../crates/macros/cgp-async-macro/src/lib.rs), a `#[proc_macro_attribute]` that discards its attribute arguments and forwards the annotated item to `impl_async`. The rewrite lives in [crates/macros/cgp-async-macro/src/impl_async.rs](../../../crates/macros/cgp-async-macro/src/impl_async.rs): it parses the item as a `syn::ItemTrait`, and on success walks the trait's methods, replacing each `async` signature's output with `-> impl ::core::future::Future<Output = ...>` and clearing the `async` keyword; if the item does not parse as a trait, it is returned unchanged. The macro is re-exported into the prelude from [crates/main/cgp-core/src/prelude.rs](../../../crates/main/cgp-core/src/prelude.rs), so `use cgp::prelude::*;` brings it into scope. Its interaction with `#[cgp_fn]` is exercised by the expansion snapshot in [crates/tests/cgp-tests/tests/cgp_fn_tests/async.rs](../../../crates/tests/cgp-tests/tests/cgp_fn_tests/async.rs), and async providers spawned onto a runtime appear in [crates/tests/cgp-tests/src/tests/async/spawn.rs](../../../crates/tests/cgp-tests/src/tests/async/spawn.rs).
</content>
