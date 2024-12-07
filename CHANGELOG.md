# Changelog

## Pre-Release

- Redesign `derive_component` to `cgp_component` with improved syntax - [#38](https://github.com/contextgeneric/cgp/pull/38)
    - Rename the attribute `#[derive_component]` to `#[cgp_component]`
    - The macro syntax has been changed as follows:
    - Old: `#[derive_component(NameGetterComponent, NameGetter<MyContext>)]`
    - New: `#[cgp_component { name: NameGetterComponent, context: MyContext, provider: NameGetter }]`
    - For migration, the following regex can be used in a global search and replace:
    - Search: `#\[derive_component\(([\w<>, ]+), (\w+)<(\w+)>\)\]`
    - Replace: `#[cgp_component {\n  name: $1,\n  provider: $2,\n  context: $3,\n}]`

- Support async-generic feature flags in cgp-async - [#37](https://github.com/contextgeneric/cgp/pull/37)
    - Introduce the following feature flags to `cgp-async`:
    - `async`
    - `send`
    - `sync`
    - `static`
    - `full` - default feature with all enabled
    - Introduce the following traits in `cgp-async`:
    - `MaybeSend` - alias to `Send` when the `send` feature is enabled, otherwise nothing.
    - `MaybeSync` - alias to `Sync` when the `sync` feature is enabled, otherwise nothing.
    - `MaybeStatic` - alias to `'static` when the `static` feature is enabled, otherwise nothing.
    - Update the `Async` trait from `Sized + Send + Sync + 'static` to `MaybeSend + MaybeSync + MaybeStatic`.
    - The `Sized` constraint is removed from `Async` to allow use inside `dyn` traits.
    - Update the `#[async_trait]` macro to desugar async functions to return `impl Future<Output: MaybeSend>`.
    - Use of `#[async_trait]` now requires import of `cgp::prelude::*` to allow `MaybeSend` to be auto imported.
    - `cgp-async` now re-exports `cgp_sync::strip_async` if the `async` feature is not enabled.
    - i.e. async functions are desugared into sync functions if the `async` feature is disabled.
    - Crates such as `cgp` and `cgp-core` offers the `full` feature, which can be disabled to disable the indirect default features in `cgp-async`.

- Introduce new cgp-field constructs - [#36](https://github.com/contextgeneric/cgp/pull/36)
    - Introduce the product type constructs `Cons` and `Nil`.
    - Introduce the sum type constructs `Either` and `Void`.
    - Introduce the `Field` type for tagged field value.
    - Introduce the `Product!` macro for building product types.
    - Introduce the `product!` macro for building product expressions.
    - Introduce the `Sum!` macro for building sum types.
    - Change the `symbol!` macro to generate product type of `Char` using `Cons` and `Nil`.

- Rename `HasField::Field` to `HasField::Value` - [#35](https://github.com/contextgeneric/cgp/pull/35)

- Remove `Sized` constraint from `Async` trait - [#34](https://github.com/contextgeneric/cgp/pull/34)

- Component pattern improvements - [#24](https://github.com/contextgeneric/cgp/pull/24)
    - Rename `DelegateTo` to `UseDelegate`.
    - Implement `FieldGetter` for `UseContext`.
    - Introduce `UseDelegatedType`.

- Introduce `cgp-type` crate with various refactoring - [#23](https://github.com/contextgeneric/cgp/pull/23)
    - Introduce `cgp-type` crate, with the `HasType` component.
    - Introduce `FieldGetter` as a manual provider trait for `HasField`.
    - Introduce `HasFieldMut` trait to `cgp-field`, and auto derive it in `#[derive(HasField)]`.
    - Introduce `DelegateTo` in `cgp-component` as a generalized delegation component.
    - Introduce `WithProvider` in `cgp-component` as a generalized provider transformation component.
    - Introduce `UseContext` in `cgp-component` for generalized implementation of provider via context.
    - Replace `DelegateErrorComponents` in `cgp-error` and replace it with `DelegateTo`.
    - Use `core::error::Error` instead of `std::error::Error` in `cgp-error-std`.