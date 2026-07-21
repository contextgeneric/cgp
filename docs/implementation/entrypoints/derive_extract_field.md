# `#[derive(ExtractField)]` — implementation

`#[derive(ExtractField)]` emits just the incremental-extractor slice of the variant machinery: the owned and borrowed `__Partial{Name}` companion enums plus the `HasExtractor`/`HasExtractorRef`/`HasExtractorMut`, `PartialData`, `FinalizeExtract`, and per-variant `ExtractField` impls that peel an enum apart one variant at a time. This document covers how that codegen works; for the accepted syntax and the full expansion, read the reference document [reference/derives/derive_extract_field.md](../../reference/derives/derive_extract_field.md).

## Entry point

The macro is driven by the `derive_extract_field` function in [cgp-macro-lib/src/derive_extract_field.rs](../../../crates/macros/cgp-macro-lib/src/derive_extract_field.rs). It parses the input into a `syn::ItemEnum`, wraps it in an `ItemCgpVariant`, and calls `to_extract_field_items` — the same method the enum path of `#[derive(CgpData)]` uses for its extractor slice:

```rust
let variant = ItemCgpVariant { item_enum };
let items = variant.to_extract_field_items()?;
```

Applying the derive to a non-enum item fails at `syn::parse2`.

## Pipeline

There is no multi-stage transform. `ItemCgpVariant::to_extract_field_items` names two companion enums — `__Partial{ContextName}` (owned) and `__PartialRef{ContextName}` (borrowed) — and composes the helpers in the [`derive_extractor/`](../../../crates/macros/cgp-macro-core/src/types/cgp_data/derive_extractor/) submodule. Most helpers run twice, once for each companion enum, selected by a `bool` that adds the borrowing `'__a`/`MapTypeRef` generics for the ref form. The [`cgp_data` AST stack](../asts/cgp_data.md) documents `ItemCgpVariant` and the field-tag types.

## Generated items

The derive centers on two partial companion enums. `__Partial{Name}` is a clone of the input enum that gains one `MapType` parameter per variant and wraps each payload in that parameter's `Map`; `IsPresent` keeps the payload and `IsVoid` maps it to the empty `Void`, so a variant's remaining-possibility is encoded in the type. `__PartialRef{Name}` adds a `'__a__` lifetime and a `MapTypeRef` parameter that selects a shared or mutable borrow of each payload:

```rust
pub enum __PartialShape<__F0__: MapType, __F1__: MapType> {
    Circle(<__F0__ as MapType>::Map<Circle>),
    Rectangle(<__F1__ as MapType>::Map<Rectangle>),
}
```

Around the enums the derive emits: `PartialData` for each (both target the original enum); `HasExtractor` (owned, all variants `IsPresent`) with `to_extractor`/`from_extractor` that map each concrete variant across, plus `HasExtractorRef` (over `__PartialRef…<'__a__, IsRef, …>`) and `HasExtractorMut` (over `IsMut`); a `FinalizeExtract` impl for the all-`IsVoid` configuration of each enum, whose body is `match self {}` because that configuration is uninhabited; and, per variant, an `ExtractField<Tag>` impl in scope only when that variant's marker is `IsPresent`. The extract impl returns `Ok(value)` on a match and `Err(remainder)` on a miss, where the remainder flips that variant's marker to `IsVoid`:

```rust
impl<__F1__: MapType> ExtractField<Symbol!("Circle")> for __PartialShape<IsPresent, __F1__> {
    type Value = Circle;
    type Remainder = __PartialShape<IsVoid, __F1__>;
    fn extract_field(self, _: PhantomData<Symbol!("Circle")>) -> Result<Circle, Self::Remainder> { /* … */ }
}
```

Each failed extraction narrows the remainder by one `IsVoid`, so a chain of `extract_field` calls becomes a provably exhaustive match: once every marker is `IsVoid`, the value inhabits `FinalizeExtract` and can be discharged without a wildcard. The `FinalizeExtract` and `FinalizeExtractResult` traits are defined in the field crate; the derive supplies only the all-void impl.

## Behavior and corner cases

A variant's name is keyed by the [`Symbol!`](../../reference/macros/symbol.md) of its identifier, and the enum's generic parameters are threaded onto every impl and onto both companion enums, with the ref enum additionally bounding the type parameters by its `'__a__` lifetime. The reserved `'__a__` name (rather than a bare `'a`) is what lets the derive apply to an enum whose own lifetime parameter is named `'a` without the two colliding. The `HasExtractorRef`/`HasExtractorMut` associated types carry the `where Self: '__a__` bound that a borrowed extractor needs.

A variantless enum is special-cased so its degenerate expansion still compiles. Because such an enum is uninhabited and borrows nothing, the borrowed partial enum `__PartialRef{Name}` is emitted as a bare empty enum with neither the `'__a__` lifetime nor the `__R__: MapTypeRef` selector — leaving them in would make both unused parameters (`E0392`) — and every borrowed accessor (`extractor_ref`/`extractor_mut`, and the sibling `HasFields` `to_fields_ref`) matches the dereferenced place with `match *self {}` rather than `match self {}`, since a bare match over `&Self` is non-exhaustive when `Self` is uninhabited (a reference is always considered inhabited, `E0004`). The owned side needs no special case: `to_extractor`, `from_extractor`, and the owned `FinalizeExtract` all match owned uninhabited values directly. The `HasExtractorRef`/`HasExtractorMut` GATs keep their `'__a__` parameter because the trait declares it, but the empty partial enum on the right-hand side takes no arguments.

This derive emits no `HasFields` representation impls and no `FromVariant` constructors — those come from [`#[derive(HasFields)]`](derive_has_fields.md) and [`#[derive(FromVariant)]`](derive_from_variant.md); `ExtractField` is purely the deconstruction slice, included wholesale by [`#[derive(CgpVariant)]`](derive_cgp_variant.md) and [`#[derive(CgpData)]`](derive_cgp_data.md).

## Error spans

Each generated impl is re-spanned onto the token it derives from, so a compiler error points at that token rather than at the whole `#[derive(ExtractField)]`. The per-variant `ExtractField` impls are aimed at the variant they match, and the whole-enum `HasExtractor`/`HasExtractorRef`/`HasExtractorMut`/`FinalizeExtract`/`PartialData` impls at the enum name. Each goes through [`override_item_span`](../README.md#spans-aim-generated-items-at-the-token-the-user-wrote), moving only the `impl`/`{ … }` boundary — the mechanism the [`#[derive(HasField)]`](derive_has_field.md#error-spans) doc explains in full. The `__Partial{Name}`/`__PartialRef{Name}` companion enums are cloned from the user's own enum, so their tokens already carry meaningful spans and need no re-spanning.

## Known issues

The extractor codegen requires every variant to be a single-unnamed-field tuple variant (enforced by `get_variant_type` in the `derive_extractor/utils.rs` helper). A fieldless variant like `Empty`, a multi-field variant like `Pair(A, B)`, or a struct-style variant like `Named { x: A }` makes the macro fail with "Expected variant to contain exactly one unnamed field." There is no per-variant opt-out, so an enum mixing variant shapes cannot derive the extractor at all; the same requirement applies to [`#[derive(FromVariant)]`](derive_from_variant.md) and therefore to `#[derive(CgpVariant)]`/`#[derive(CgpData)]` on such an enum. The reference document records the user-visible form of this limitation in its own Known issues.

## Tests

`#[derive(ExtractField)]` has no snapshot macro of its own; the extractor items it emits are part of the variant expansion pinned by the `snapshot_derive_cgp_data!` snapshots indexed in [derive_cgp_data.md's Snapshots section](derive_cgp_data.md#snapshots). The behavioral extractor tests in [crates/tests/cgp-tests/tests/extensible_variants/](../../../crates/tests/cgp-tests/tests/extensible_variants/) exercise the machinery:

- [shape_dispatch.rs](../../../crates/tests/cgp-tests/tests/extensible_variants/shape_dispatch.rs) drives the owned extractor.
- [shape_dispatch_ref.rs](../../../crates/tests/cgp-tests/tests/extensible_variants/shape_dispatch_ref.rs) drives the borrowed extractor.
- [variant_dispatch.rs](../../../crates/tests/cgp-tests/tests/extensible_variants/variant_dispatch.rs) drives the extract-and-dispatch flow.
- [derive_cgp_data_empty.rs](../../../crates/tests/cgp-tests/tests/extensible_variants/derive_cgp_data_empty.rs) snapshots the variantless-enum expansion, pinning the empty-enum special case: bare `__PartialNever`/`__PartialRefNever` enums with no parameters and `match *self {}` in the borrowed accessors.
- The single-unnamed-field requirement (Known issues) has no dedicated failure case in `cgp-macro-tests`, but is covered end-to-end through the variant derives by [parser_rejections/derive_from_variant.rs](../../../crates/tests/cgp-macro-tests/tests/parser_rejections/derive_from_variant.rs).
- The reserved-`'__a__` lifetime is exercised against a lifetime-parameterized enum by [derive_cgp_data_lifetime.rs](../../../crates/tests/cgp-tests/tests/extensible_variants/derive_cgp_data_lifetime.rs), which derives `#[derive(CgpData)]` on an `enum Message<'a>` and drives the owned, borrowed, and mutable extractors.

## Source

- Entry point: `derive_extract_field` in [cgp-macro-lib/src/derive_extract_field.rs](../../../crates/macros/cgp-macro-lib/src/derive_extract_field.rs).
- Codegen: `ItemCgpVariant::to_extract_field_items` in [cgp-macro-core/src/types/cgp_data/variant.rs](../../../crates/macros/cgp-macro-core/src/types/cgp_data/variant.rs), which composes the [derive_extractor/](../../../crates/macros/cgp-macro-core/src/types/cgp_data/derive_extractor/) helpers (`extractor_enum.rs`, `partial_data.rs`, `has_extractor_impl.rs`, `finalize_extract_impl.rs`, `extract_field_impls.rs`, `utils.rs`); the AST types are documented in [asts/cgp_data.md](../asts/cgp_data.md).
- The `ExtractField`, `HasExtractor`/`HasExtractorRef`/`HasExtractorMut`, `FinalizeExtract`, `FinalizeExtractResult`, and `PartialData` traits and the `MapType`/`MapTypeRef` markers live under [crates/core/cgp-field/src/](../../../crates/core/cgp-field/src/).
