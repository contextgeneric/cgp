# `#[derive(HasField)]` — implementation

`#[derive(HasField)]` gives a struct tag-keyed field access by emitting one `HasField` impl and one `HasFieldMut` impl per field, each keyed by the field's type-level name. This document covers how that codegen works; for the accepted syntax and the full expansion a user sees, read the reference document [reference/derives/derive_has_field.md](../../reference/derives/derive_has_field.md).

## Entry point

The macro is driven by the thin `derive_has_field` function in [cgp-macro-lib/src/derive_has_field.rs](../../../crates/macros/cgp-macro-lib/src/derive_has_field.rs). It parses the input into a `syn::ItemStruct`, wraps it in an `ItemCgpRecord`, and calls `to_has_field_impls`, which returns the getter impls to emit:

```rust
let record = ItemCgpRecord { item_struct };
let item_impls = record.to_has_field_impls()?;
```

Because it parses straight into `ItemStruct`, applying the derive to a non-struct item fails at `syn::parse2`. All codegen lives in `cgp-macro-core`; the entry function only concatenates the returned impls.

## Pipeline

There is no multi-stage transform. `ItemCgpRecord::to_has_field_impls` forwards to the single codegen helper `derive_has_field_impls_from_struct`, which walks the struct's fields and emits the getter impls. The [`cgp_data` AST stack](../asts/cgp_data.md) documents `ItemCgpRecord` and the `Symbol`/`Index` field-tag types the helper uses.

## Generated items

The derive emits two impls per field — a `HasField` read accessor and a `HasFieldMut` mutable accessor — and leaves the struct definition untouched. A named field is keyed by the [`Symbol!`](../../reference/macros/symbol.md) of its identifier; a tuple field is keyed by its positional [`Index<N>`](../../reference/types/index.md). The field's declared type becomes the associated `Value`, and the body simply borrows the corresponding field:

```rust
// named field
impl HasField<Symbol!("name")> for Person {
    type Value = String;
    fn get_field(&self, key: PhantomData<Symbol!("name")>) -> &Self::Value { &self.name }
}
// tuple field — same shape, Index<N> tag, &self.0 body
impl HasField<Index<0>> for Rectangle {
    type Value = f64;
    fn get_field(&self, key: PhantomData<Index<0>>) -> &Self::Value { &self.0 }
}
```

The `HasFieldMut` impl for each field mirrors the read impl, returning `&mut Self::Value` from `&mut self.<field>`. Whether a field's tag is a `Symbol` or an `Index` is decided by mapping the field's `syn::Member` to a [`FieldName`](../asts/cgp_data.md#symbol-index-and-fieldname), whose `ToTokens` picks the right type-level spelling.

## Behavior and corner cases

The struct's **generic parameters** are threaded onto every impl: the helper splits the generics into impl-generics, type-generics, and a `where` clause, so `struct Wrapper<T> { value: T }` yields `impl<T> HasField<Symbol!("value")> for Wrapper<T>` with `Value = T`, and a lifetime field carries the struct's lifetime through (`impl<'a> HasField<…> for Context<'a>` with `Value = &'a str`).

A **unit struct** has no fields, so the derive emits nothing rather than erroring. The helper only handles the named-field and tuple-field cases; there is no whole-struct output here — the aggregate `HasFields` view comes from [`#[derive(HasFields)]`](derive_has_fields.md) instead.

A **raw-identifier field** such as `r#type` is tagged by its logical name: `Symbol::from_ident` calls `Ident::unraw`, so the field expands to `HasField<Symbol!("type")>`, not `HasField<Symbol!("r#type")>`. Without this, the tag would encode the literal string `r#type` (a length-6 symbol whose second character is `#`) and no `Symbol!("type")` bound could match it. The accessor body still borrows through the raw identifier (`&self.r#type`), since that is the field's real name. This is exercised by the [field_access/raw_ident.rs](../../../crates/tests/cgp-tests/tests/field_access/raw_ident.rs) snapshot.

Field access through **smart pointers** is not the derive's doing: `HasField`/`HasFieldMut` have blanket impls over `Deref`/`DerefMut` targets defined in the field crate, so a `Box<Person>` resolves through to the inner struct without the derive generating anything for the pointer type.

## Error spans

Each generated impl is re-spanned onto the field it derives from, so a compiler error about one field's `HasField`/`HasFieldMut` impl points at that field rather than at the whole `#[derive(HasField)]`. The impls are built with `parse_internal!`, whose tokens all carry the macro's `call_site` span — the entire derived struct — so without a re-span every such error would underline the derive and say nothing about which field is involved. `derive_has_field_impls_from_struct` therefore passes each finished impl through [`override_item_span`](../README.md#spans-aim-generated-items-at-the-token-the-user-wrote), moving only its boundary tokens (the `impl` keyword and the `{ … }` body) onto the field's own span — the field identifier for a named field, the whole `syn::Field` for a tuple field, which has no identifier. This is the same technique the [`delegate_components!`](delegate_components.md#error-spans) impls use for their per-entry keys.

Two diagnostics show the difference. A **coherence conflict** (`E0119`) — a hand-written `HasField<Symbol!("name")>` impl clashing with the one the derive emits — now lands its "conflicting implementation" caret on the `name` field instead of on the derive. More common in practice is the **near-impl hint** inside a missing-field check error: when a provider needs a field the struct lacks, `rustc` reports the unmet `HasField` bound and adds "but trait `HasField<…>` is implemented for it," pointing at the *nearest existing* field impl. That caret now lands on the field whose impl is cited, so a struct that derives `HasField` for several fields shows each near-miss on its own field rather than collapsing them all onto the derive attribute — where the encoded `Symbol<len, …>` tags were the reader's only way to tell them apart. See the [check-trait-failure](../../errors/checks/check-trait-failure.md) class for the full diagnostic.

Only the boundary tokens move; every interior token — the `HasField` reference, the `Symbol!`/`Index` tag, the `Value` type — keeps its own span. That is what keeps the field navigable in an editor: rust-analyzer maps a source token to its expansion by source range, so re-spanning a resolvable reference onto the field would hijack go-to-definition on the field, whereas a keyword and a delimiter cannot. The caret half is pinned by the raw `.rust.stderr` baselines of the `cargo-cgp` UI fixtures that exercise a derived context — [`fields/base_area_1`](https://github.com/contextgeneric/cargo-cgp/blob/main/tests/ui/acceptable/fields/base_area_1.rs) and the multi-field [`duplication/density_3`](https://github.com/contextgeneric/cargo-cgp/blob/main/tests/ui/acceptable/duplication/density_3.rs) among them — so a regression that drops the re-span changes those snapshots.

## Snapshots

Every `snapshot_derive_has_field!` invocation across the suite is indexed here, since these snapshots all belong to this entrypoint. They live in the `field_access` target, which owns the `#[derive(HasField)]` expansion:

- [field_access/index.rs](../../../crates/tests/cgp-tests/tests/field_access/index.rs) — a tuple struct, each field keyed by `Index<0>`/`Index<1>` rather than a `Symbol!`.
- [field_access/lifetime_field.rs](../../../crates/tests/cgp-tests/tests/field_access/lifetime_field.rs) — a struct lifetime lifted onto the impls, with a borrowed field type (`&'a str`) kept as `Value`.
- [field_access/chain.rs](../../../crates/tests/cgp-tests/tests/field_access/chain.rs) — the canonical named-field expansion, over two owned structs.
- [field_access/raw_ident.rs](../../../crates/tests/cgp-tests/tests/field_access/raw_ident.rs) — a raw-identifier field (`r#type`), pinning that the tag is the unrawed `Symbol!("type")` while the body borrows `&self.r#type`.
- [field_access/chain_inner_life.rs](../../../crates/tests/cgp-tests/tests/field_access/chain_inner_life.rs) — the inner struct carries a lifetime, threaded onto its impls.
- [field_access/chain_outer_life.rs](../../../crates/tests/cgp-tests/tests/field_access/chain_outer_life.rs) — the outer struct borrows the inner one, with the outer lifetime on its impls.
- [field_access/chain_deeply_nested.rs](../../../crates/tests/cgp-tests/tests/field_access/chain_deeply_nested.rs) — five structs each deriving `HasField`, pinning the plain expansion repeated across a deep chain.

## Tests

The behavioral tests confirm the generated getters read the right fields:

- [field_access/index.rs](../../../crates/tests/cgp-tests/tests/field_access/index.rs) reads a tuple struct's fields at run time through `get_field` with `Index<0>`/`Index<1>` tags.
- [field_access/lifetime_field.rs](../../../crates/tests/cgp-tests/tests/field_access/lifetime_field.rs) reads a lifetime-carrying field back out.
- [field_access/chain.rs](../../../crates/tests/cgp-tests/tests/field_access/chain.rs), [chain_inner_life.rs](../../../crates/tests/cgp-tests/tests/field_access/chain_inner_life.rs), [chain_outer_life.rs](../../../crates/tests/cgp-tests/tests/field_access/chain_outer_life.rs), and [chain_deeply_nested.rs](../../../crates/tests/cgp-tests/tests/field_access/chain_deeply_nested.rs) compose the generated getters through `ChainGetters` to read a nested field in one hop.
- [field_access/symbol.rs](../../../crates/tests/cgp-tests/tests/field_access/symbol.rs) and [field_access/index_display.rs](../../../crates/tests/cgp-tests/tests/field_access/index_display.rs) exercise the `Symbol!`/`Index<N>` tag types the derive emits.
- [field_access/raw_ident.rs](../../../crates/tests/cgp-tests/tests/field_access/raw_ident.rs) reads a raw-identifier field back through `get_field` with the unrawed `Symbol!("type")` tag, confirming the `r#` prefix is dropped from the tag.

## Source

- Entry point: `derive_has_field` in [cgp-macro-lib/src/derive_has_field.rs](../../../crates/macros/cgp-macro-lib/src/derive_has_field.rs).
- It calls `ItemCgpRecord::to_has_field_impls` in [cgp-macro-core/src/types/cgp_data/record.rs](../../../crates/macros/cgp-macro-core/src/types/cgp_data/record.rs), whose codegen is `derive_has_field_impls_from_struct` in [cgp-macro-core/src/types/cgp_data/derive_has_field.rs](../../../crates/macros/cgp-macro-core/src/types/cgp_data/derive_has_field.rs); the AST types are documented in [asts/cgp_data.md](../asts/cgp_data.md).
- The `HasField`/`HasFieldMut` traits are defined in [crates/core/cgp-field/src/traits/](../../../crates/core/cgp-field/src/traits/).
