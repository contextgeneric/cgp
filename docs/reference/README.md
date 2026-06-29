# CGP Construct Reference

This directory documents every CGP construct — one self-contained document per construct, each explaining its purpose, syntax, expansion, examples, related constructs, and source. The documents are written for agents who need precise per-construct semantics; for the conceptual framing that connects constructs, read the `/cgp` skill alongside them. The authoring rules, document template, and the requirement to keep these documents in sync with the code live in [../CLAUDE.md](../CLAUDE.md).

## Directory layout

The documents are grouped into subdirectories by the *kind* of construct, so a reader looking for "the macro I invoke" and a reader looking for "the trait the macro generates" each have an obvious place to start. A new document goes in the subdirectory that matches what the construct is; when you add one, place it accordingly and register it in the matching section below.

The [macros/](macros/) directory holds the procedural macros a programmer invokes directly — the attribute macros that define components and providers, the function-like macros that wire and check them, and the type-level construction macros (`Symbol!`, `Product!`, `Sum!`). The [derives/](derives/) directory holds the `#[derive(...)]` macros, which are also macros but form a distinct family large enough to warrant their own space. The [attributes/](attributes/) directory holds the modifier attributes that refine what the definition macros generate — they are not standalone macros but options consumed by a host macro such as `#[cgp_fn]` or `#[cgp_impl]`. The [traits/](traits/) and [types/](types/) directories hold the runtime library constructs the macros expand into: `traits/` for the capability and mechanism traits, `types/` for the providers and type-level types. Both are pending their first documents.

This index is the catalog of constructs and tracks which are documented. When you add, remove, or rename a construct, update both its document and this index in the same change. A construct listed as *pending* is a known gap and a good candidate for the next document to write; a cross-link in any document that points at a pending construct is expected to dangle until that document exists.

## Component definition macros — [macros/](macros/)

These macros define CGP components and the providers that implement them — the core act of writing CGP code.

- [`#[cgp_component]`](macros/cgp_component.md) — turn a trait into a component (consumer trait, provider trait, blanket impls).
- [`#[cgp_impl]`](macros/cgp_impl.md) — write a provider for a component using consumer-trait-style syntax.
- [`#[cgp_provider]`](macros/cgp_provider.md) — write a provider by implementing the provider trait directly.
- [`#[cgp_new_provider]`](macros/cgp_new_provider.md) — `#[cgp_provider]` that also defines the provider struct.
- [`#[cgp_fn]`](macros/cgp_fn.md) — define a single-implementation capability as a blanket-impl trait from a function.
- [`#[cgp_type]`](macros/cgp_type.md) — define an abstract-type component.
- [`#[cgp_getter]`](macros/cgp_getter.md) — define a getter component wired through CGP.
- [`#[cgp_auto_getter]`](macros/cgp_auto_getter.md) — define a getter as a blanket impl over `HasField`.
- [`#[blanket_trait]`](macros/blanket_trait.md) — generate a blanket impl from a trait with default methods.

## Wiring and checking macros — [macros/](macros/)

These macros connect components to providers on a concrete context and verify the wiring at compile time.

- [`delegate_components!`](macros/delegate_components.md) — build a context's type-level table mapping components to providers.
- [`check_components!`](macros/check_components.md) — assert at compile time that a context's wiring is complete.
- [`delegate_and_check_components!`](macros/delegate_and_check_components.md) — delegate and check in one macro.
- [`#[cgp_namespace]`](macros/cgp_namespace.md) — group components under a namespace for presets and inheritance.

## Type-level construction macros — [macros/](macros/)

These macros construct the type-level vocabulary — strings, lists, and sums — that the rest of CGP is built on.

- [`Symbol!`](macros/symbol.md) — type-level string, used for field names.
- [`Product!` / `product!`](macros/product.md) — type-level list type and value.
- [`Sum!`](macros/sum.md) — type-level sum (variant) type.

## Attribute modifiers — [attributes/](attributes/)

These attributes refine what the definition macros generate and are used inside `#[cgp_impl]`, `#[cgp_fn]`, and `#[cgp_component]`.

- [`#[implicit]`](attributes/implicit.md) — extract a function argument from a context field automatically.
- [`#[uses]`](attributes/uses.md) — import other CGP capabilities as `Self` bounds.
- [`#[use_type]`](attributes/use_type.md) — import an abstract associated type with fully-qualified rewriting.
- [`#[use_provider]`](attributes/use_provider.md) — dispatch a call to a named provider in higher-order providers.
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

## Runtime traits — [traits/](traits/) (pending)

These are the capability and mechanism traits the macros expand into. Their semantics are already described within the macro documents above, and several documents link to them in anticipation, so those cross-links dangle until the documents below exist. Each entry names the filename a future document should claim, inside `traits/`.

The wiring and checking mechanism traits are the first priority, since nearly every macro document references them: `delegate_component.md` (`DelegateComponent`), `is_provider_for.md` (`IsProviderFor`), and `can_use_component.md` (`CanUseComponent`). The foundational capability traits follow: `has_field.md` (`HasField`), `has_fields.md` (`HasFields`), `has_type.md` (`HasType` / `TypeProvider`), and `has_error_type.md` (`HasErrorType` / `CanRaiseError`).

## Runtime types and providers — [types/](types/) (pending)

These are the providers and type-level types the macros expand into, also referenced by dangling cross-links until written. Each entry names the filename a future document should claim, inside `types/`.

The standard providers come first, as they appear directly in user wiring: `use_context.md` (`UseContext`), `use_delegate.md` (`UseDelegate`), `use_field.md` (`UseField`), `use_type_provider.md` (the `UseType` provider/type, distinct from the [`#[use_type]`](attributes/use_type.md) attribute), `with_provider.md` (`WithProvider`), and `redirect_lookup.md` (`RedirectLookup`). The type-level types round out the batch: `field.md` (`Field`) and `index.md` (`Index`).
