# CGP Construct Reference

This directory documents every CGP construct — one self-contained document per construct, each explaining its purpose, syntax, expansion, examples, related constructs, and source. The documents are written for agents who need precise per-construct semantics; for the conceptual framing that connects constructs, read the `/cgp` skill alongside them. The authoring rules, document template, and the requirement to keep these documents in sync with the code live in [../CLAUDE.md](../CLAUDE.md).

The index below is the catalog of constructs and tracks which are documented. When you add, remove, or rename a construct, update both its document and this index in the same change. A construct listed as *pending* is a known gap and a good candidate for the next document to write; a cross-link in any document that points at a pending construct is expected to dangle until that document exists.

## Component definition macros

These macros define CGP components and the providers that implement them — the core act of writing CGP code.

- [`#[cgp_component]`](cgp_component.md) — turn a trait into a component (consumer trait, provider trait, blanket impls).
- [`#[cgp_impl]`](cgp_impl.md) — write a provider for a component using consumer-trait-style syntax.
- [`#[cgp_provider]`](cgp_provider.md) — write a provider by implementing the provider trait directly.
- [`#[cgp_new_provider]`](cgp_new_provider.md) — `#[cgp_provider]` that also defines the provider struct.
- [`#[cgp_fn]`](cgp_fn.md) — define a single-implementation capability as a blanket-impl trait from a function.
- [`#[cgp_type]`](cgp_type.md) — define an abstract-type component.
- [`#[cgp_getter]`](cgp_getter.md) — define a getter component wired through CGP.
- [`#[cgp_auto_getter]`](cgp_auto_getter.md) — define a getter as a blanket impl over `HasField`.
- [`#[blanket_trait]`](blanket_trait.md) — generate a blanket impl from a trait with default methods.

## Wiring and checking macros

These macros connect components to providers on a concrete context and verify the wiring at compile time.

- [`delegate_components!`](delegate_components.md) — build a context's type-level table mapping components to providers.
- [`check_components!`](check_components.md) — assert at compile time that a context's wiring is complete.
- [`delegate_and_check_components!`](delegate_and_check_components.md) — delegate and check in one macro.
- [`#[cgp_namespace]`](cgp_namespace.md) — group components under a namespace for presets and inheritance.

## Attribute modifiers

These attributes refine what the definition macros generate and are used inside `#[cgp_impl]`, `#[cgp_fn]`, and `#[cgp_component]`.

- [`#[implicit]`](implicit.md) — extract a function argument from a context field automatically.
- [`#[uses]`](uses.md) — import other CGP capabilities as `Self` bounds.
- [`#[use_type]`](use_type.md) — import an abstract associated type with fully-qualified rewriting.
- [`#[use_provider]`](use_provider.md) — dispatch a call to a named provider in higher-order providers.
- [`#[extend]`](extend.md) — add supertrait bounds to a generated trait.
- [`#[extend_where]`](extend_where.md) — add `where` clauses to a generated trait definition.
- [`#[derive_delegate]`](derive_delegate.md) — generate `UseDelegate` providers that dispatch on a generic parameter.

## Type-level and data macros

These macros construct the type-level vocabulary (strings, lists, sums) and derive the field-access machinery for extensible data.

- [`Symbol!`](symbol.md) — type-level string, used for field names.
- [`Product!` / `product!`](product.md) — type-level list type and value.
- [`Sum!`](sum.md) — type-level sum (variant) type.
- [`#[derive(HasField)]`](derive_has_field.md) — per-field accessors keyed by `Symbol!`/`Index`.
- [`#[derive(HasFields)]`](derive_has_fields.md) — whole-struct/enum field-list view.
- [`#[derive(CgpData)]`](derive_cgp_data.md) — full extensible-data derivation.
- [`#[derive(CgpRecord)]`](derive_cgp_record.md) — extensible record (struct) derivation.
- [`#[derive(CgpVariant)]`](derive_cgp_variant.md) — extensible variant (enum) derivation.
- [`#[derive(BuildField)]`](derive_build_field.md) — builder support for records.
- [`#[derive(ExtractField)]`](derive_extract_field.md) — extractor support for variants.
- [`#[derive(FromVariant)]`](derive_from_variant.md) — variant-construction support.

## Runtime constructs (pending)

These are the library traits, providers, and types that the macros expand into. Their semantics are already described within the macro documents above, and several documents link to them in anticipation, so those cross-links dangle until the documents below exist. They are the next batch to write; each entry names the filename a future document should claim.

The wiring and checking mechanism traits are the first priority, since nearly every macro document references them: `delegate_component.md` (`DelegateComponent`), `is_provider_for.md` (`IsProviderFor`), and `can_use_component.md` (`CanUseComponent`).

The standard providers come next, as they appear directly in user wiring: `use_context.md` (`UseContext`), `use_delegate.md` (`UseDelegate`), `use_field.md` (`UseField`), `use_type_provider.md` (the `UseType` provider/type, distinct from the [`#[use_type]`](use_type.md) attribute), and a document for `WithProvider`.

The foundational capability traits and type-level types round out the batch: `has_field.md` (`HasField`), `has_fields.md` (`HasFields`), `has_type.md` (`HasType` / `TypeProvider`), a document for `HasErrorType` / `CanRaiseError`, `field.md` (the `Field` type), `index.md` (the `Index` type), and `redirect_lookup.md` (`RedirectLookup`).
