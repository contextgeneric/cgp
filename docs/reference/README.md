# CGP Construct Reference

This directory documents every CGP construct — one self-contained document per construct, each explaining its purpose, syntax or definition, expansion or behavior, examples, related constructs, and source. The documents are written for agents who need precise per-construct semantics. The high-level conceptual framing that connects the constructs lives in the sibling [concepts/](../concepts/README.md) directory; the `/cgp` skill remains a complementary teaching aid. The authoring rules, document template, and the requirement to keep these documents in sync with the code live in [../CLAUDE.md](../CLAUDE.md).

## Directory layout

The documents are grouped into subdirectories by the *kind* of construct, so a reader looking for "the macro I invoke", "the trait the macro generates", or "the provider I wire" each has an obvious place to start. A new document goes in the subdirectory that matches what the construct is; when you add one, place it accordingly and register it in the matching section below. The high-level conceptual overviews that tie multiple constructs together — the consumer/provider duality, dependency injection, namespaces, handlers, and so on — live in the sibling [concepts/](../concepts/README.md) directory rather than here, each pointing into these per-construct documents for the mechanics.

The [macros/](macros/) directory holds the procedural macros a programmer invokes directly: the attribute macros that define components and providers, the function-like macros that wire and check them, and the type-level construction macros (`Symbol!`, `Product!`, `Sum!`, `Path!`). The [derives/](derives/) directory holds the `#[derive(...)]` macros, a distinct family large enough to warrant its own space. The [attributes/](attributes/) directory holds the modifier attributes that refine what the definition macros generate — they are not standalone macros but options consumed by a host macro such as `#[cgp_fn]` or `#[cgp_impl]`.

The remaining directories hold the runtime library constructs the macros expand into. The [components/](components/) directory documents the built-in CGP components CGP ships with — full consumer/provider trait pairs such as `HasType`, `HasErrorType`, and the handler family — that an application consumes and wires like any component it defines itself. The [providers/](providers/) directory documents the zero-sized provider structs that appear in wiring — `UseField`, `UseType`, `UseDelegate`, `UseContext`, and the rest — the values a context delegates a component to. The [traits/](traits/) directory documents the capability and mechanism traits that are *not* themselves components: the wiring traits (`DelegateComponent`, `IsProviderFor`, `CanUseComponent`), the field and type capabilities (`HasField`, `HasFields`), the extensible-data builder and extractor families, and the type-level operations. The [types/](types/) directory documents the type-level building-block types the rest of CGP is constructed from (`Field`, `Index`, the `Cons`/`Nil` product spine, the `Either`/`Void` sum spine, and the `Chars`/`PathCons` lists).

The distinction between [components/](components/) and [traits/](traits/) is whether the trait is a CGP component: a document belongs in `components/` when its trait is defined with `#[cgp_component]`, `#[cgp_type]`, or `#[cgp_getter]` and therefore has a generated provider trait and `…Component` marker that contexts wire; it belongs in `traits/` when it is an ordinary capability or mechanism trait that the machinery uses but no one delegates.

This index is the catalog of constructs. When you add, remove, or rename a construct, update both its document and this index in the same change. Because documents live in different subdirectories, a cross-link between two of them is a relative path — a sibling in the same directory is `name.md`, and a document in another directory is `../that-dir/name.md`.

## Component definition macros — [macros/](macros/)

These macros define CGP components and the providers that implement them — the core act of writing CGP code.

- [`#[cgp_component]`](macros/cgp_component.md) — turn a trait into a component (consumer trait, provider trait, blanket impls).
- [`#[cgp_impl]`](macros/cgp_impl.md) — write a provider for a component using consumer-trait-style syntax.
- [`#[cgp_provider]`](macros/cgp_provider.md) — write a provider by implementing the provider trait directly.
- [`#[cgp_new_provider]`](macros/cgp_new_provider.md) — `#[cgp_provider]` that also defines the provider struct.
- [`#[cgp_fn]`](macros/cgp_fn.md) — define a single-implementation capability as a blanket-impl trait from a function.
- [`#[async_trait]`](macros/async_trait.md) — rewrite a trait's `async fn` declarations to `-> impl Future`, the lint-clean way to declare async CGP methods.
- [`#[cgp_type]`](macros/cgp_type.md) — define an abstract-type component.
- [`#[cgp_getter]`](macros/cgp_getter.md) — define a getter component wired through CGP.
- [`#[cgp_auto_getter]`](macros/cgp_auto_getter.md) — define a getter as a blanket impl over `HasField`.
- [`#[blanket_trait]`](macros/blanket_trait.md) — generate a blanket impl from a trait with default methods.
- [`#[cgp_computer]`](macros/cgp_computer.md) — define a `Computer` provider from a function.
- [`#[cgp_producer]`](macros/cgp_producer.md) — define a `Producer` provider from a function.
- [`#[cgp_auto_dispatch]`](macros/cgp_auto_dispatch.md) — generate a handler that dispatches over an extensible-data input.

## Wiring and checking macros — [macros/](macros/)

These macros connect components to providers on a concrete context and verify the wiring at compile time.

- [`delegate_components!`](macros/delegate_components.md) — build a context's type-level table mapping components to providers.
- [`check_components!`](macros/check_components.md) — assert at compile time that a context's wiring is complete.
- [`delegate_and_check_components!`](macros/delegate_and_check_components.md) — delegate and check in one macro.
- [`#[cgp_namespace]`](macros/cgp_namespace.md) — group components under a namespace for presets and inheritance.

## Type-level construction macros — [macros/](macros/)

These macros construct the type-level vocabulary — strings, lists, sums, and paths — that the rest of CGP is built on.

- [`Symbol!`](macros/symbol.md) — type-level string, used for field names.
- [`Product!` / `product!`](macros/product.md) — type-level list type and value.
- [`Sum!`](macros/sum.md) — type-level sum (variant) type.
- [`Path!`](macros/path.md) — type-level path, used by namespaces and redirected lookups.

## Attribute modifiers — [attributes/](attributes/)

These attributes refine what the definition macros generate and are used inside `#[cgp_impl]`, `#[cgp_fn]`, and `#[cgp_component]`.

- [`#[implicit]`](attributes/implicit.md) — extract a function argument from a context field automatically.
- [`#[uses]`](attributes/uses.md) — import other CGP capabilities as `Self` bounds.
- [`#[use_type]`](attributes/use_type.md) — import an abstract associated type with fully-qualified rewriting.
- [`#[use_provider]`](attributes/use_provider.md) — complete an inner provider's bound in higher-order providers.
- [`#[extend]`](attributes/extend.md) — add supertrait bounds to a generated trait.
- [`#[extend_where]`](attributes/extend_where.md) — add `where` clauses to a generated trait definition.
- [`#[derive_delegate]`](attributes/derive_delegate.md) — generate `UseDelegate` providers that dispatch on a generic parameter.

## Data derives — [derives/](derives/)

These derive macros generate the field-access and extensible-data machinery for structs and enums.

- [`#[derive(HasField)]`](derives/derive_has_field.md) — per-field accessors keyed by `Symbol!`/`Index`.
- [`#[derive(HasFields)]`](derives/derive_has_fields.md) — whole-struct/enum field-list view.
- [`#[derive(CgpData)]`](derives/derive_cgp_data.md) — full extensible-data derivation.
- [`#[derive(CgpRecord)]`](derives/derive_cgp_record.md) — extensible record (struct) derivation.
- [`#[derive(CgpVariant)]`](derives/derive_cgp_variant.md) — extensible variant (enum) derivation.
- [`#[derive(BuildField)]`](derives/derive_build_field.md) — builder support for records.
- [`#[derive(ExtractField)]`](derives/derive_extract_field.md) — extractor support for variants.
- [`#[derive(FromVariant)]`](derives/derive_from_variant.md) — variant-construction support.

## Built-in components — [components/](components/)

These are the full CGP components CGP ships with — each a consumer trait, provider trait, and `…Component` marker — that an application wires through `delegate_components!` like any component it defines itself.

- [`HasType` / `TypeProvider`](components/has_type.md) — CGP's built-in abstract-type component.
- [`HasErrorType`](components/has_error_type.md) — the abstract error type component.
- [`CanRaiseError` / `CanWrapError`](components/can_raise_error.md) — raising and wrapping source errors into the abstract error type.
- [`Computer` / `CanCompute`](components/computer.md) — the synchronous computation component and its by-reference and async variants.
- [`TryComputer` / `CanTryCompute`](components/try_computer.md) — the fallible computation component.
- [`Handler` / `CanHandle`](components/handler.md) — the general async, fallible, error-aware computation component.
- [`Producer` / `CanProduce`](components/producer.md) — the input-free production component.
- [`CanRun` / `CanSendRun`](components/runner.md) — the task-running components.
- [`HasRuntime` / `HasRuntimeType`](components/has_runtime.md) — the abstract runtime type and accessor components.

## Providers — [providers/](providers/)

These are the zero-sized provider structs a context delegates components to. They carry no runtime value and exist only at the type level.

- [`UseContext`](providers/use_context.md) — satisfy a provider trait by routing back through the context's own consumer-trait impl.
- [`UseDelegate`](providers/use_delegate.md) — dispatch on a generic parameter through an inner type-level table.
- [`UseDelegatedType`](providers/use_delegated_type.md) — resolve an abstract type through an inner table.
- [`UseField`](providers/use_field.md) — implement a getter by reading a named context field.
- [`UseFieldRef`](providers/use_field_ref.md) — implement a getter by reading a field through `AsRef`/`AsMut`.
- [`UseFields`](providers/use_fields.md) — getter provider keyed by the method name.
- [`UseType`](providers/use_type.md) — supply a concrete type for an abstract-type component.
- [`UseDefault`](providers/use_default.md) — marker provider selecting default implementations.
- [`WithProvider`](providers/with_provider.md) — adapt a foundational provider into a component (and the `WithContext`/`WithType`/`WithField` aliases).
- [`RedirectLookup`](providers/redirect_lookup.md) — re-route a lookup along a type-level path; the namespace mechanism.
- [`ChainGetters`](providers/chain_getters.md) — chain field getters to reach into nested contexts.
- [Handler combinators](providers/handler_combinators.md) — `ComposeHandlers`, `PipeHandlers`, `ReturnInput`, and the `Promote*` adapters that build and lift handlers.
- [Dispatch combinators](providers/dispatch_combinators.md) — `MatchWithHandlers`, `MatchWithValueHandlers`, `ExtractFieldAndHandle`, and the rest of the cgp-dispatch routing providers.
- [Monad providers](providers/monad_providers.md) — `PipeMonadic`, `BindOk`, `BindErr`, and the identity/ok/err monad markers.
- [Error providers](providers/error_providers.md) — `DebugError`, `DisplayError`, `RaiseFrom`, `ReturnError`, and the other backends for the error components.

## Runtime traits — [traits/](traits/)

These are the capability and mechanism traits the macros expand into — the traits a programmer rarely writes by hand but must understand to read generated code.

- [`DelegateComponent`](traits/delegate_component.md) — the per-context type-level table mapping a component key to a provider.
- [`IsProviderFor`](traits/is_provider_for.md) — the marker supertrait that surfaces missing-dependency errors.
- [`CanUseComponent`](traits/can_use_component.md) — the consumer-side check that a context can use a component.
- [`HasField`](traits/has_field.md) — tag-keyed field access (with `HasFieldMut` and the provider-side `FieldGetter`).
- [`HasFields`](traits/has_fields.md) — the whole-shape field representation and its conversions.
- [`HasBuilder`](traits/has_builder.md) — the incremental-builder trait family (`BuildField`, `UpdateField`, `FinalizeBuild`, …).
- [`ExtractField`](traits/extract_field.md) — the incremental-extractor trait family (`HasExtractor`, `FinalizeExtract`, …).
- [`FromVariant`](traits/from_variant.md) — generic construction of an enum from a named variant.
- [`MapType`](traits/map_type.md) — the present/absent/void type-mapping markers (`IsPresent`, `IsNothing`, …) and transforms.
- [`AppendProduct`](traits/product_ops.md) — type-level product operations (`AppendProduct`, `ConcatProduct`, `MapFields`).
- [`CanUpcast`](traits/cast.md) — structural casts between records and variants (`CanUpcast`, `CanDowncast`, `CanBuildFrom`).
- [`DefaultNamespace`](traits/default_namespace.md) — the namespace/preset default-resolution traits.
- [`StaticFormat`](traits/static_format.md) — runtime formatting of type-level strings and path concatenation.
- [Monad traits](traits/monad.md) — `MonadicTrans`, `MonadicBind`, `LiftValue`, and `ContainsValue`, the trait layer behind monadic handler composition.
- [Optional fields](traits/optional_fields.md) — the cgp-field-extra builder/extractor traits for optional and defaulted fields.

## Type-level types — [types/](types/)

These are the type-level building-block types the macros and traits operate on.

- [`Field`](types/field.md) — a value paired with its type-level name tag.
- [`Index`](types/index.md) — a type-level natural number, used to tag tuple-struct fields.
- [`Cons` / `Nil`](types/cons.md) — the product (record) list spine.
- [`Either` / `Void`](types/either.md) — the sum (variant) list spine.
- [`Chars`](types/chars.md) — the type-level character list behind `Symbol`.
- [`PathCons`](types/path_cons.md) — the type-level path list behind `Path!`.
- [`Life`](types/life.md) — a lifetime lifted into a type.
- [`MRef`](types/mref.md) — an owned-or-borrowed value.
