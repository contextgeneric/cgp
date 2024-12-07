# Changelog

## Pre-Release

- Introduce new cgp-field constructs [#36](https://github.com/contextgeneric/cgp/pull/36)
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