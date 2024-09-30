# Changelog

## Pre-Release

-  Introduce `cgp-type` crate with various refactoring [#23](https://github.com/contextgeneric/cgp/pull/23)
    - Introduce `cgp-type` crate, with the `HasType` component.
    - Introduce `FieldGetter` as a manual provider trait for `HasField`.
    - Introduce `HasFieldMut` trait to `cgp-field`, and auto derive it in `#[derive(HasField)]`.
    - Introduce `DelegateTo` in `cgp-component` as a generalized delegation component.
    - Introduce `WithProvider` in `cgp-component` as a generalized provider transformation component.
    - Introduce `UseContext` in `cgp-component` for generalized implementation of provider via context.
    - Replace `DelegateErrorComponents` in `cgp-error` and replace it with `DelegateTo`.
    - Use `core::error::Error` instead of `std::error::Error` in `cgp-error-std`.