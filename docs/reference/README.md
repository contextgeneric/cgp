# CGP Construct Reference

This directory documents every CGP construct — one self-contained document per construct, each explaining its purpose, syntax or definition, expansion or behavior, examples, related constructs, and source. The documents are written for agents who need precise per-construct semantics, and they point only at library source, never at a test. The high-level conceptual framing that connects the constructs lives in the sibling [concepts/](../concepts/README.md) directory; the internal mechanics of each macro — its pipeline, the functions that synthesize its output, and every pointer into the test suite — live in the sibling [implementation/](../implementation/README.md) directory, to which each reference document's Source section links; the `/cgp` skill remains a complementary teaching aid. The authoring rules, document template, and the requirement to keep these documents in sync with the code live in [../AGENTS.md](../AGENTS.md).

## Summary

This section summarizes every documented construct, grouped by the job it does and ordered from the constructs almost every CGP task needs down to the specialized and reference-only ones. Each group says when to reach for it and links to the per-construct document that carries the full grammar, expansion, and examples. The summary tells you only what a construct is *for*, not how it behaves in every corner, so read the linked document before writing, changing, reviewing, or debugging code that uses the construct — that document is the ground truth, and this catalog exists to route you to the right one.

### Core constructs

These are the constructs behind almost every CGP program: defining a component, writing a provider for it, and wiring a context to the provider it uses. [`#[cgp_component]`](macros/cgp_component.md) turns one trait into a component — the consumer trait callers invoke, the provider trait implementers target, and the blanket impls plus component-name marker that connect them. A provider is then written with [`#[cgp_impl]`](macros/cgp_impl.md), the idiomatic form that keeps `self`/`Self` and consumer-trait signatures and desugars into the inside-out provider-trait shape; its lower-level layers [`#[cgp_provider]`](macros/cgp_provider.md) and [`#[cgp_new_provider]`](macros/cgp_new_provider.md) implement the provider trait directly (the latter also declaring the provider struct) and are mostly what you read rather than write. When a capability has a single implementation and needs no wiring, [`#[cgp_fn]`](macros/cgp_fn.md) generates a blanket-impl trait straight from a function, and [`#[blanket_trait]`](macros/blanket_trait.md) does the same from a trait with default methods and supertrait dependencies; [`#[async_trait]`](macros/async_trait.md) rewrites a trait's `async fn` declarations into the lint-clean `-> impl Future` form that CGP's async methods use.

Wiring is where a concrete context chooses its providers. [`delegate_components!`](macros/delegate_components.md) builds a context's type-level table mapping each component to a provider — read its document for the array-key, generic-list, aggregate-provider (`new`), and per-type `open` forms — and that table is the [`DelegateComponent`](traits/delegate_component.md) trait underneath, a compile-time key→value map that both ordinary wiring and inner dispatch tables share. Two providers appear directly in wiring: [`UseContext`](providers/use_context.md) satisfies a provider trait by routing back through the context's own consumer-trait impl (and is the default inner provider of a higher-order provider), and [`UseDefault`](providers/use_default.md) selects a component's default method bodies. To dispatch one component per value of a generic parameter, prefer the `open` statement of `delegate_components!`; the legacy path is the [`UseDelegate`](providers/use_delegate.md) provider and the [`#[derive_delegate]`](attributes/derive_delegate.md) attribute that generates its dispatch impl, which you still meet in existing code and in CGP's own error and handler components.

### Basic field access

Reach for these whenever a provider needs a value out of its context — the most common form of dependency injection in CGP. [`#[derive(HasField)]`](derives/derive_has_field.md) gives a struct per-field getters keyed by a type-level tag, implementing the [`HasField`](traits/has_field.md) trait (and its mutable and provider-side mirrors) that all value injection stands on; the tags are [`Symbol!`](macros/symbol.md) type-level strings for named fields and [`Index`](types/index.md) type-level numbers for tuple fields. The default way to read such a field is an [`#[implicit]`](attributes/implicit.md) argument, which looks like an ordinary function parameter but is sourced from a same-named context field, and whose access rules (`.clone()` for owned, `.as_str()` for `&str`, and the option/slice/mutable forms) live in its document.

A getter trait is the sparing alternative for the cases an implicit argument cannot reach — a field on another type, a named shared capability, or a getter carrying an inferred associated type. [`#[cgp_auto_getter]`](macros/cgp_auto_getter.md) generates a blanket getter impl over `HasField` keyed by the method name, while the advanced [`#[cgp_getter]`](macros/cgp_getter.md) makes the getter a full component so its source field is chosen at wiring time through the [`UseField`](providers/use_field.md) provider — with [`UseFieldRef`](providers/use_field_ref.md) for `AsRef`/`AsMut` access and [`UseFields`](providers/use_fields.md) for the method-name convention. [`ChainGetters`](providers/chain_getters.md) composes getters to reach a field on a nested context, and [`MRef`](types/mref.md) is the owned-or-borrowed getter return type for a getter that may lend or produce its value.

### Imports

These attributes declare a provider's dependencies as if importing them, keeping the constraints off the public trait interface. [`#[uses]`](attributes/uses.md) adds `Self:` capability bounds that read like a `use` statement — the idiomatic replacement for a hand-written `where Self: Trait` clause — and [`#[use_provider]`](attributes/use_provider.md) completes an inner provider's bound in a higher-order provider by filling in the stray `<Self>` context argument the provider trait requires. [`#[extend]`](attributes/extend.md) adds a supertrait to the generated trait (the `pub use` counterpart of `#[uses]`, and the only way to add a supertrait in `#[cgp_fn]`), while [`#[extend_where]`](attributes/extend_where.md) promotes a `where` predicate onto a `#[cgp_fn]` trait's own definition rather than only its impl. The `#[impl_generics(...)]` attribute, which adds a bounded generic parameter to a `#[cgp_fn]` impl alone, is documented within [`#[cgp_fn]`](macros/cgp_fn.md).

### Abstract types

Use these when generic code must name a type — an error type, a scalar, a runtime — that each context chooses for itself. [`#[cgp_type]`](macros/cgp_type.md) defines an abstract-type component from a trait with one associated type, layering a [`UseType`](providers/use_type.md) blanket impl on top of [`#[cgp_component]`](macros/cgp_component.md) so a context binds the concrete type by wiring the component to `UseType<T>`; that machinery rests on CGP's single built-in abstract-type component, [`HasType` / `TypeProvider`](components/has_type.md). The [`#[use_type]`](attributes/use_type.md) attribute — distinct from the `UseType` provider despite the shared name — imports an abstract type into a `#[cgp_fn]`/`#[cgp_impl]`/`#[cgp_component]` definition, rewriting a bare `Error` or `Scalar` into its fully-qualified `<Self as Trait>::Type` form and adding the supertrait or bound, and it also carries the equality form that pins an abstract type to a concrete one. [`UseDelegatedType`](providers/use_delegated_type.md) resolves an abstract type through a lookup table instead of a fixed type, and [`WithProvider`](providers/with_provider.md) (with its `WithType`/`WithField`/`WithContext`/`WithDelegatedType` aliases) is the adapter that lets a foundational `TypeProvider` or `FieldGetter` stand in as a named component's provider.

### Error handling

CGP makes the error type abstract so fallible generic code never names a concrete error, and these components carry that strategy. [`HasErrorType`](components/has_error_type.md) gives a context one shared `Error` type (an abstract-type component, so wired with `UseType`), and [`CanRaiseError` / `CanWrapError`](components/can_raise_error.md) construct that error from a source error and attach detail to it, dispatching per source or detail type. The interchangeable strategies that satisfy them — the [error providers](providers/error_providers.md) `RaiseFrom`, `ReturnError`, `RaiseInfallible`, `DebugError`/`DisplayError`, `DiscardDetail`, and `PanicOnError` — stay generic over the context's error type, while the concrete backends (`cgp-error-anyhow`, `cgp-error-eyre`, `cgp-error-std`) are opt-in and named in their own crates. The wiring keys and backend providers are deliberately not in the prelude and must be imported from `cgp::core::error` / `cgp::extra::error`.

### Checks and debugging

CGP wiring is lazy, so a context can compile while wired wrong; these constructs force that failure to surface, readably, at the wiring site. [`check_components!`](macros/check_components.md) asserts at compile time that a context can use each listed component, and [`delegate_and_check_components!`](macros/delegate_and_check_components.md) fuses wiring and checking for basic contexts (but never for an aggregate provider). Both build on two marker traits: [`CanUseComponent`](traits/can_use_component.md), the context-side check that a context both delegates a component and its provider's dependencies hold, and [`IsProviderFor`](traits/is_provider_for.md), the supertrait every provider trait carries that re-exposes a provider's `where` bounds so the compiler names the actual missing dependency instead of a bare "trait not implemented". The `#[check_providers(...)]` form of `check_components!` asserts `IsProviderFor` on each provider directly, which is how a broken layer of a higher-order provider stack is localized.

### Namespaces

Namespaces are reusable, inheritable wiring tables — CGP's preset mechanism — for keeping top-level wiring short as component counts grow. [`cgp_namespace!`](macros/cgp_namespace.md) defines a namespace (optionally inheriting a parent); a context joins it with a `namespace` header inside [`delegate_components!`](macros/delegate_components.md), a component registers into one with the `#[prefix(...)]` attribute, and a provider registers as a per-type default with `#[default_impl(...)]` — the last two documented within `cgp_namespace!` and [`DefaultNamespace`](traits/default_namespace.md). The mechanism underneath is the [`RedirectLookup`](providers/redirect_lookup.md) provider, which re-routes a lookup along a type-level [`Path!`](macros/path.md) / [`PathCons`](types/path_cons.md), together with the [`DefaultNamespace` / `DefaultImpls`](traits/default_namespace.md) traits that resolve inherited and per-type defaults. The lightweight `open` statement of `delegate_components!` is a special case of the same redirection for per-type dispatch wired directly on one context.

### Handlers and computation

This family models computation as swappable components along the axes of sync/async, fallible/infallible, and input/input-free — reach for it for pipelines, I/O, and type-level interpreters. [`Computer`](components/computer.md) is the pure synchronous transform (with by-reference and async variants), [`TryComputer`](components/try_computer.md) adds fallibility, [`Handler`](components/handler.md) is the general async-and-fallible workhorse, and [`Producer`](components/producer.md) is the input-free case; [`CanRun` / `CanSendRun`](components/runner.md) run tasks and [`HasRuntime` / `HasRuntimeType`](components/has_runtime.md) supply the abstract runtime they execute against. A provider is written from a plain function with [`#[cgp_computer]`](macros/cgp_computer.md) or [`#[cgp_producer]`](macros/cgp_producer.md), which also wire the promotion tables that let one function answer the whole family, and [`#[cgp_auto_dispatch]`](macros/cgp_auto_dispatch.md) generates a variant-dispatching handler from a per-type trait. The providers that build and route handlers are the [handler combinators](providers/handler_combinators.md) (`ComposeHandlers`, `PipeHandlers`, `ReturnInput`, the `Promote*` lifts, and `UseInputDelegate`), the [dispatch combinators](providers/dispatch_combinators.md) that match enum variants and assemble records, and the [monad providers](providers/monad_providers.md) (`PipeMonadic` with the ident/ok/err monads) built on the [monad traits](traits/monad.md) for short-circuiting pipelines.

### Extensible data types

These derives and traits let generic code build and read structs and enums by their named fields and variants without naming the concrete type. [`#[derive(CgpData)]`](derives/derive_cgp_data.md) is the umbrella derive; its struct- and enum-specific faces are [`#[derive(CgpRecord)]`](derives/derive_cgp_record.md) and [`#[derive(CgpVariant)]`](derives/derive_cgp_variant.md), and its individual slices are [`#[derive(HasFields)]`](derives/derive_has_fields.md) (the whole-shape view), [`#[derive(BuildField)]`](derives/derive_build_field.md) (the record builder), [`#[derive(ExtractField)]`](derives/derive_extract_field.md) (the variant extractor), and [`#[derive(FromVariant)]`](derives/derive_from_variant.md) (variant construction). The traits behind them are [`HasFields`](traits/has_fields.md), the incremental-builder family in [`HasBuilder`](traits/has_builder.md), the extractor family in [`ExtractField`](traits/extract_field.md), [`FromVariant`](traits/from_variant.md), the presence markers of [`MapType`](traits/map_type.md), the list algebra of [`AppendProduct` / `ConcatProduct` / `MapFields`](traits/product_ops.md), the structural casts [`CanUpcast` / `CanDowncast` / `CanBuildFrom`](traits/cast.md), and the [optional-field extensions](traits/optional_fields.md) for defaulted and optional fields. Each entry in the shape is a [`Field`](types/field.md) pairing a value with its type-level name tag.

### Type-level primitives

These are the type-level building blocks the rest of CGP is constructed from — mostly written through sugar, and otherwise needing only to be recognized in expansions and error messages. The construction macros are [`Symbol!`](macros/symbol.md) (a type-level string, for field names), [`Product!`](macros/product.md) and [`Sum!`](macros/sum.md) (type-level record and variant lists), and [`Path!`](macros/path.md) (a routing path); their expanded spines are [`Cons` / `Nil`](types/cons.md) for products, [`Either` / `Void`](types/either.md) for sums, [`Chars`](types/chars.md) for the string behind `Symbol`, and [`PathCons`](types/path_cons.md) for paths. Two further lifts make non-type things addressable in trait resolution — [`Index`](types/index.md) for a tuple-field position and [`Life`](types/life.md) for a lifetime — and [`MRef`](types/mref.md) is the owned-or-borrowed getter value. The [`StaticFormat` / `StaticString` / `ConcatPath`](traits/static_format.md) traits recover these type-level strings and paths back into runtime data.

## Directory layout

The documents are grouped into subdirectories by the *kind* of construct, so a reader looking for "the macro I invoke", "the trait the macro generates", or "the provider I wire" each has an obvious place to start. A new document goes in the subdirectory that matches what the construct is; when you add one, place it accordingly and register it in the matching section below. The high-level conceptual overviews that tie multiple constructs together — the consumer/provider duality, dependency injection, namespaces, handlers, and so on — live in the sibling [concepts/](../concepts/README.md) directory rather than here, each pointing into these per-construct documents for the mechanics.

The [macros/](macros/) directory holds the procedural macros a programmer invokes directly: the attribute macros that define components and providers, the function-like macros that wire and check them, and the type-level construction macros (`Symbol!`, `Product!`, `Sum!`, `Path!`). The [derives/](derives/) directory holds the `#[derive(...)]` macros, a distinct family large enough to warrant its own space. The [attributes/](attributes/) directory holds the modifier attributes that refine what the definition macros generate — they are not standalone macros but options consumed by a host macro such as `#[cgp_fn]` or `#[cgp_impl]`.

The remaining directories hold the runtime library constructs the macros expand into. The [components/](components/) directory documents the built-in CGP components CGP ships with — full consumer/provider trait pairs such as `HasType`, `HasErrorType`, and the handler family — that an application consumes and wires like any component it defines itself. The [providers/](providers/) directory documents the zero-sized provider structs that appear in wiring — `UseField`, `UseType`, `UseDelegate`, `UseContext`, and the rest — the values a context delegates a component to. The [traits/](traits/) directory documents the capability and mechanism traits that are *not* themselves components: the wiring traits (`DelegateComponent`, `IsProviderFor`, `CanUseComponent`), the field and type capabilities (`HasField`, `HasFields`), the extensible-data builder and extractor families, and the type-level operations. The [types/](types/) directory documents the type-level building-block types the rest of CGP is constructed from (`Field`, `Index`, the `Cons`/`Nil` product spine, the `Either`/`Void` sum spine, and the `Chars`/`PathCons` lists).

The distinction between [components/](components/) and [traits/](traits/) is whether the trait is a CGP component: a document belongs in `components/` when its trait is defined with `#[cgp_component]`, `#[cgp_type]`, or `#[cgp_getter]` and therefore has a generated provider trait and `…Component` marker that contexts wire; it belongs in `traits/` when it is an ordinary capability or mechanism trait that the machinery uses but no one delegates.

This index is the catalog of constructs. When you add, remove, or rename a construct, update both its document and this index in the same change. Because documents live in different subdirectories, a cross-link between two of them is a relative path — a sibling in the same directory is `name.md`, and a document in another directory is `../that-dir/name.md`.

## Tooling — [cargo-cgp.md](cargo-cgp.md)

One document here describes a tool rather than a construct, and it is the exception to the per-construct rule above: it has no subdirectory and no Syntax/Expansion shape. It is registered here because a reader looking up how to *read* a CGP error belongs in the reference.

- [`cargo-cgp`](cargo-cgp.md) — CGP's first-class error toolchain: the cargo subcommand that rewrites CGP compiler errors into a readable, root-cause-first form, how to install and run it, how its `[CGP-Exxx]` output maps to the [error catalog](../errors/README.md), and its companion `expand` command, which shows the ordinary Rust a target's CGP macros generate with the type-level constructs resugared. Recommend it for building, checking, and debugging CGP code.

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

Three further modifier attributes do not yet have their own page and are documented inside their host construct's document; each is a candidate for a dedicated page here.

- `#[impl_generics(...)]` — add bounded generic parameters to a `#[cgp_fn]`'s impl only (not its trait); documented in [`#[cgp_fn]`](macros/cgp_fn.md).
- `#[prefix(...)]` — register a `#[cgp_component]` trait into a namespace under a path; documented in [`#[cgp_namespace]`](macros/cgp_namespace.md).
- `#[default_impl(...)]` — register a `#[cgp_impl]` provider as a namespace's per-type default; documented in [`DefaultNamespace`](traits/default_namespace.md).

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
