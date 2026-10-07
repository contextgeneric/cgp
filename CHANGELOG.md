# Changelog

## v0.9.0 (2026-09-29)

New features:

- In `delegate_components!`, write a type or getter component as the value alone when the provider is `UseType` or `UseField` - [#188](https://github.com/contextgeneric/cgp/issues/188)
- Introduce `cgp_preset!`, so one preset entry expands to many `DelegateComponent` impls and two presets can be combined - [#26](https://github.com/contextgeneric/cgp/issues/26)
- Introduce `#[derive_provider(WithProvider)]`, deriving the `WithProvider` impl instead of writing it by hand - [#30](https://github.com/contextgeneric/cgp/issues/30)
- Introduce `#[cgp_auto_impl]`, turning a trait method body into a blanket provider - [#181](https://github.com/contextgeneric/cgp/issues/181)
- Introduce `#[cgp_auto_error]`, generating `HasErrorType` / `CanRaiseError` / `CanWrapError` providers from one error definition - [#182](https://github.com/contextgeneric/cgp/issues/182)
- Allow a `#[cgp_getter]` / `#[cgp_auto_getter]` trait to consist of an associated type, reusing the `#[cgp_type]` path - [#183](https://github.com/contextgeneric/cgp/issues/183)
- Allow `#[helper]` methods inside `#[cgp_impl]` that are not part of the provider trait - [#184](https://github.com/contextgeneric/cgp/issues/184)
- Introduce `CanLog` and `#[cgp_auto_log]`, a logger component with a blanket impl parallel to `#[cgp_auto_getter]` - [#185](https://github.com/contextgeneric/cgp/issues/185)
- Read `#[implicit]` and `#[field]` arguments from the context in `#[cgp_fn]` and `#[cgp_computer]`, and add `#[derive_promote]` to wrap a `Computer` as a component provider - [#193](https://github.com/contextgeneric/cgp/issues/193)
- Introduce `Struct!` and `Enum!`, writing a struct or enum shape as its declaration instead of a `Product!` or `Sum!` of `Field` entries - [#276](https://github.com/contextgeneric/cgp/pull/276)
- Accept variants with no fields in `#[derive(CgpVariant)]`, `#[derive(CgpData)]`, `#[derive(ExtractField)]`, and `#[derive(FromVariant)]`, with payload `Nil` - [#278](https://github.com/contextgeneric/cgp/pull/278)
