# Changelog

## v0.6.1 (2026-02-01)

- Support implicit `Context` type in `#[cgp_impl]` - [#189](https://github.com/contextgeneric/cgp/pull/189)
- Support provider checks in `check_components!` - [#190](https://github.com/contextgeneric/cgp/pull/190)
- Support associated type in getter trait - [#191](https://github.com/contextgeneric/cgp/pull/191)

## v0.6.0 (2025-10-26)

- Introduce `#[cgp_impl]` to simplify provider trait implementation - [#174](https://github.com/contextgeneric/cgp/pull/174)
- Allow context types to delegate components directly - [#175](https://github.com/contextgeneric/cgp/pull/175)
- Introduce `#[cgp_inherit]` for inheriting from presets - [#176](https://github.com/contextgeneric/cgp/pull/176)
- Fix use of self inside default trait method implementation body - [#179](https://github.com/contextgeneric/cgp/pull/179)

## v0.5.0 (2025-10-11)

- Monadic computation support
    - Use Monadic composition to simplify the implementation of `DispatchMatchers` - [#127](https://github.com/contextgeneric/cgp/pull/127)
    - Add full monad transformer support for monadic composition of CGP handlers - [#129](https://github.com/contextgeneric/cgp/pull/129)

- Extensible data types improvements
    - Add generalized field transformation on partial records to support building with default values. - [#128](https://github.com/contextgeneric/cgp/pull/128)
    - Add `CanFinalizeWithDefault` method for partial records - [#130](https://github.com/contextgeneric/cgp/pull/130)
    - Extensible Visitor Improvements - [#132](https://github.com/contextgeneric/cgp/pull/132)
    - Add support for passing additional arguments to extensible visitors - [#134](https://github.com/contextgeneric/cgp/pull/134)
    - Add extensible visitor support mutable partial variants - [#135](https://github.com/contextgeneric/cgp/pull/135)
    - Introduce `UpdateField` trait to generalize field transformation in partial data - [#148](https://github.com/contextgeneric/cgp/pull/148)
    - Add optional field builder constructs for extensible records - [#152](https://github.com/contextgeneric/cgp/pull/152)
    - Introduce `#[derive(CgpData)]` and `#[derive(CgpRecord)]` Macros - [#154](https://github.com/contextgeneric/cgp/pull/154)
    - Add `__` prefix to Partial data types generated - [#156](https://github.com/contextgeneric/cgp/pull/156)
    - Implement ref variants of matcher providers - [#161](https://github.com/contextgeneric/cgp/pull/161)

- Handler improvements
    - Introduce `AsyncComputer` Component - [#136](https://github.com/contextgeneric/cgp/pull/136)
    - Derive `AsyncComputer` when using `#[cgp_computer]` with async functions - [137](https://github.com/contextgeneric/cgp/pull/137)
    - Add new `cgp-extra-macro` crate - [#138](https://github.com/contextgeneric/cgp/pull/138)

- Dispatch macro
    - Implement `#[cgp_dispatch]` macro - [#139](https://github.com/contextgeneric/cgp/pull/139)
    - `#[cgp_dispatch]` macro improvements - [142](https://github.com/contextgeneric/cgp/pull/142)
    - Rename `#[cgp_dispatch]` to `#[cgp_auto_dispatch]` - [#155](https://github.com/contextgeneric/cgp/pull/155)

- Component system improvements
    - Allow only provider name to be specified inside `#[cgp_getter]` - [#143](https://github.com/contextgeneric/cgp/pull/143)
    - Support use of lifetime parameters inside CGP traits - [#145](https://github.com/contextgeneric/cgp/pull/145)
    - Add `?Sized` bound to `DelegateComponent`'s `Name` parameter - [#147](https://github.com/contextgeneric/cgp/pull/147)
    - Allow shortcut definition of provider name in `#[cgp_type]` - [#163](https://github.com/contextgeneric/cgp/pull/163)
    - Fix use of `new` in `delegate_components!` when keys array are used - [#164](https://github.com/contextgeneric/cgp/pull/164)
    - Add `Life` type to represent lifetimes as types - [#166](https://github.com/contextgeneric/cgp/pull/166)

- Field improvements
    - Desugar field constructs into their symbolic forms - [#151](https://github.com/contextgeneric/cgp/pull/151)
    - Allow first argument in `#[cgp_getter]` methods to be non-self - [#169](https://github.com/contextgeneric/cgp/pull/169)

- Symbol improvements
    - Implement `MatchStr` for static string matching with symbols - [#153](https://github.com/contextgeneric/cgp/pull/153)
    - Implement `StaticString` trait to produce `&'static str` from symbol types - [#159](https://github.com/contextgeneric/cgp/pull/159)

- Breaking changes toward stabilization
    - Remove `Async` trait and `Send` bounds in async functions - [#149](https://github.com/contextgeneric/cgp/pull/149)
    - Reorganize exports in `cgp-field` - [#150](https://github.com/contextgeneric/cgp/pull/150)
    - Rename `Char` to `Chars` - [#165](https://github.com/contextgeneric/cgp/pull/165)
    - Remove `HasInner` trait - [#170](https://github.com/contextgeneric/cgp/pull/170)
    - Remove lowercase `symbol!` macro - [#173](https://github.com/contextgeneric/cgp/pull/173)

## v0.4.2 (2025-07-07)

- Datatype-generic Programming Support
    - Data-generic programming support in CGP - [#112](https://github.com/contextgeneric/cgp/pull/112)
    - Implement `DispatchHandlersRef` for dispatching based on enum references - [#116](https://github.com/contextgeneric/cgp/pull/116)
    - Builder dispatcher refactoring - [#117](https://github.com/contextgeneric/cgp/pull/117)
    - Introduce ref version of handler traits - [#118](https://github.com/contextgeneric/cgp/pull/118)
    - Dispatcher Refactoring - [#119](https://github.com/contextgeneric/cgp/pull/119)
    - Remove Input param from `MatchWithFieldHandlers` - [#120](https://github.com/contextgeneric/cgp/pull/120)
    - Support use of ref handlers in handler macros - [#121](https://github.com/contextgeneric/cgp/pull/121)

- Preset Improvements
    - Add new direct delegation mode and use it in preset inheritance - [#111](https://github.com/contextgeneric/cgp/pull/111)

- Bug Fixes
  - Derive `HasField<Index<{i}>>` for structs with unnamed fields - [#115](https://github.com/contextgeneric/cgp/pull/115)
  - Allow unsized generic arguments in `IsProviderFor` trait - [#114](https://github.com/contextgeneric/cgp/pull/114)

## v0.4.1 (2025-05-24)

- New `cgp-handler` Crate
    - Introduce new `cgp-handler` crate - [#105](https://github.com/contextgeneric/cgp/pull/105)

- Macro Improvements
    - Support wrapping of `Preset::Provider` inside `cgp_preset!` macro - [#103](https://github.com/contextgeneric/cgp/pull/103)
    - Support derivation of `UseDelegate` inside `#[cgp_component]` - [#106](https://github.com/contextgeneric/cgp/pull/106)

- CGP Field Improvements
    - Format `Char` statically without `self` - [#104](https://github.com/contextgeneric/cgp/pull/104)

- Miscellaneous
    - Minor improvements - [#107](https://github.com/contextgeneric/cgp/pull/107)

- Documentation
    - Add inline Rust documentation for common CGP constructs - [#109](https://github.com/contextgeneric/cgp/pull/109)

## v0.4.0 (2025-05-09)

- Debugging Improvements
    - Implement `delegate_and_check_components!` - [#98](https://github.com/contextgeneric/cgp/pull/98)
    - Implement `check_components!` macro to check if a context implements a component - [#78](https://github.com/contextgeneric/cgp/pull/78)
    - Permanently enable provider supertrait - [#73](https://github.com/contextgeneric/cgp/pull/73)
    - Introduce `#[new_cgp_provider]` to also generate provider struct definition - [#67](https://github.com/contextgeneric/cgp/pull/67)
    - Preserve original provider bounds inside `#[cgp_provider]` - [#65](https://github.com/contextgeneric/cgp/pull/65)
    - Introduce `IsProviderFor` trait to help improve compile errors - [#63](https://github.com/contextgeneric/cgp/pull/63)

- CGP Presets Implementation
    - Fix nested inheritance and multi inheritance for presets - [#92](https://github.com/contextgeneric/cgp/pull/92)
    - Preset Macro Improvements - [#91](https://github.com/contextgeneric/cgp/pull/91)
    - Expand `cgp_preset!` into a preset module - [#72](https://github.com/contextgeneric/cgp/pull/72)
    - Re-export imports inside a `re_export` submodule - [#71](https://github.com/contextgeneric/cgp/pull/71)
    - Introduce `#[cgp::re_export_imports]` macro - [#70](https://github.com/contextgeneric/cgp/pull/70)

- CGP Getter Improvements
    - CGP Getter Improvements - [#94](https://github.com/contextgeneric/cgp/pull/94)
    - Getter Macro Improvements - [#87](https://github.com/contextgeneric/cgp/pull/87)
    - Refactoring and Improvements on `#[cgp_getter]` - [#81](https://github.com/contextgeneric/cgp/pull/81)
    - Fix component generics inside derivation of `WithProvider` for `#[cgp_getter]` - [#80](https://github.com/contextgeneric/cgp/pull/80)
    - Fix `#[cgp_getter]` macro when the getter trait contains generic parameters - [#76](https://github.com/contextgeneric/cgp/pull/76)
    - Allow generic arguments inside `#[cgp_auto_getter]` traits - [#64](https://github.com/contextgeneric/cgp/pull/64)

- Macro Improvements
    - Simplify attribute arguments for `#[cgp_component]`, `#[cgp_provider]` and `#[cgp_context]` - [#96](https://github.com/contextgeneric/cgp/pull/96)
    - Allow definition of new structs inside `delegate_components!` - [#93](https://github.com/contextgeneric/cgp/pull/93)
    - Migrate macro tests to `cgp-tests` crate, part 1 - [#90](https://github.com/contextgeneric/cgp/pull/90)
    - Automatically derive `UseContext` implementation inside `#[cgp_component]` - [#88](https://github.com/contextgeneric/cgp/pull/88)
    - Macro Crates Reorganization - [#83](https://github.com/contextgeneric/cgp/pull/83)
    - Refactoring and rename `#[trait_alias]` to `#[blanket_trait]` - [#82](https://github.com/contextgeneric/cgp/pull/82)
    - Introduce `#[trait_alias]` macro to simplify definition of alias traits - [#79](https://github.com/contextgeneric/cgp/pull/79)
    - CGP Macro Refactoring - [#77](https://github.com/contextgeneric/cgp/pull/77)
    - Rename `#[new_cgp_provider]` to `#[cgp_new_provider]` - [#75](https://github.com/contextgeneric/cgp/pull/74)
    - Introduce `#[cgp_context]` attribute macro for CGP contexts - [#66](https://github.com/contextgeneric/cgp/pull/66)

- Component System Improvements
    - Rename `HasProvider` trait to `HasCgpProvider` - [#97](https://github.com/contextgeneric/cgp/pull/97)
    - Add support for `const` item in component traits - [#95](https://github.com/contextgeneric/cgp/pull/95)
    - Remove `'static` bound from `Async` trait alias - [#89](https://github.com/contextgeneric/cgp/pull/89)
    - Rename `HasComponents` trait to `HasProvider` - [#69](https://github.com/contextgeneric/cgp/pull/69)
    - Redesign `cgp_type` to work as attribute macro - [#68](https://github.com/contextgeneric/cgp/pull/68)

- CGP Field Improvements
    - Add `Display` instance for symbol types - [#86](https://github.com/contextgeneric/cgp/pull/85)
    - Use Unicode Greek alphabets for visual representation of CGP field types - [#85](https://github.com/contextgeneric/cgp/pull/85)
    - Implement traits and derive macros for `HasFields`, `FromFields`, and `ToFields` - [#84](https://github.com/contextgeneric/cgp/pull/84)
    - Add `#[diagnostic::do_not_recommend]` to blanket impl of `HasField` - [#74](https://github.com/contextgeneric/cgp/pull/74)

## v0.3.1 (2025-01-16)

- Update Rust MSRV to v1.84 - [#58](https://github.com/contextgeneric/cgp/pull/58)

- Add `HasAsyncErrorType` to prelude - [#59](https://github.com/contextgeneric/cgp/pull/59)

- Add `CanRaiseAsyncError` and `CanWrapAsyncError` to `cgp-error` and prelude - [#60](https://github.com/contextgeneric/cgp/pull/60)


## v0.3.0 (2025-01-08)

- Introduce Accessor Component Macros - [#56](https://github.com/contextgeneric/cgp/pull/55)
    - Introduce `#[cgp_getter]` attribute macro that extends `#[cgp_component]` and implement
      `UseFields` and `UseField` for accessor traits.
    - Introduce `#[cgp_auto_getter]` attribute macro for deriving accessor traits with
      blanket implementations that use `HasField` directly.

- Introduce `cgp_type!` macro for defining simple abstract CGP types - [#55](https://github.com/contextgeneric/cgp/pull/55)
    - Use `cgp_type!` to derive `HasErrorType` and `HasRuntimeType`.

- Implement `ErrorWrapper` on generic `ErrorRaiser` providers - [#54](https://github.com/contextgeneric/cgp/pull/54)
    - Implement `ErrorWrapper` for the following providers: `DebugError`, `DisplayError`,
      `DebugAnyhowError`, `DisplayAnyhowError`, `RaiseAnyhowError`,
      `DebugEyreError`, `DisplayEyreError`, `RaiseEyreError`,
      `DebugBoxedStdError`, `DisplayBoxedStdError`.

- Reorganize crate exports - [#53](https://github.com/contextgeneric/cgp/pull/53)
    - Move generic error providers to the `cgp-error-extra` crate.
    - Add an `alloc` feature to `cgp-error-extra` to enable use of `alloc` in providers.
    - Make private the sub-modules inside CGP crates.
    - Explicitly export module items instead of using `*`.

- Move `cgp-inner` to `cgp-extra` - [#51](https://github.com/contextgeneric/cgp/pull/51)
    - Remove re-export of `cgp-inner` from `cgp-core`.
    - Re-export `cgp-inner` and `cgp-runtime` from `cgp-extra`.

- Introduce `cgp-runtime` crate - [#50](https://github.com/contextgeneric/cgp/pull/50)
    - Introduce the `HasRuntimeType` and `HasRuntime` traits.
    - Introduce `HasAsyncRuntimeType` trait used for adding `Async` constraint to `HasRuntimeType::Error`.

- Error crates refactoring - [#48](https://github.com/contextgeneric/cgp/pull/48)
    - Remove `Async` trait bound from `HasErrorType::Error`.
    - Introduce `HasAsyncErrorType` trait used for adding `Async` constraint to `HasErrorType::Error`.
    - Introduce `CanWrapError` trait.
    - Introduce generic `ErrorRaiser` providers in `cgp-error`.
    - Rename and reoganize constructs in `cgp-error-eyre` and `cgp-error-std`.
    - Introduce `cgp-error-anyhow` crate.

- Decouple component and field macro crates from the library crates - [#47](https://github.com/contextgeneric/cgp/pull/47)
    - Remove `cgp-component-macro` crate from being a dependency of `cgp-component`.
    - Remove `cgp-field-macro` crate from being a dependency of `cgp-field`.

## v0.2.0 (2025-12-08)

- Rename `define_components!` to `cgp_preset!` with slight improvement - [#41](https://github.com/contextgeneric/cgp/pull/41)
    - Introduce `replace_with!` macro that allows replacement of an identifier with a list of component types in the body.
    - Introduce `for_each_replace!` macro that allows repeated replacement of an identifier with each element of components in the list in the body.
    - Rename `define_components!` to `cgp_preset!`.
    - Use `replace_with!` inside the generated `with_preset!` macro.
    - Re-introduce the `IsPreset` trait to allow bulk delegation of components.


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
