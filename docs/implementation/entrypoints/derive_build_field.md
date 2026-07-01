# `#[derive(BuildField)]` — implementation

`#[derive(BuildField)]` emits just the incremental-builder slice of the record machinery: a `__Partial{Name}` companion struct plus the `HasBuilder`, `IntoBuilder`, `PartialData`, `FinalizeBuild`, per-field `UpdateField`, and per-field `HasField` impls that assemble a struct one field at a time. This document covers how that codegen works; for the accepted syntax and the full expansion, read the reference document [reference/derives/derive_build_field.md](../../reference/derives/derive_build_field.md).

## Entry point

The macro is driven by the `derive_build_field` function in [cgp-macro-lib/src/derive_build_field.rs](../../../crates/macros/cgp-macro-lib/src/derive_build_field.rs). It parses the input into a `syn::ItemStruct`, wraps it in an `ItemCgpRecord`, and calls `to_build_field_items` — the same method the record path of `#[derive(CgpData)]` uses for its builder slice:

```rust
let record = ItemCgpRecord { item_struct };
let items = record.to_build_field_items()?;
```

Applying the derive to a non-struct item fails at `syn::parse2`.

## Pipeline

There is no multi-stage transform. `ItemCgpRecord::to_build_field_items` names the companion struct `__Partial{ContextName}` and composes the helpers in the [`derive_builder/`](../../../crates/macros/cgp-macro-core/src/types/cgp_data/derive_builder/) submodule in a fixed order: `derive_builder_struct`, `derive_has_builder_impl`, `derive_into_builder_impl`, `derive_partial_data_impl_from_struct`, `derive_finalize_build_impl`, `derive_update_field_impls`, and `derive_has_field_impls`. The [`cgp_data` AST stack](../asts/cgp_data.md) documents `ItemCgpRecord` and the field-tag types.

## Generated items

The derive centers on the partial companion struct `__Partial{Name}`: a clone of the input struct that gains one `MapType` type parameter per field and wraps each field type in that parameter's `Map`. The marker decides how a field is stored — `IsPresent` maps `T` to `T`, `IsNothing` maps it to the unit `()`, and `IsVoid` maps it to the empty `Void` — so a field's present/absent state is encoded in the type:

```rust
pub struct __PartialPerson<__F0__: MapType, __F1__: MapType> {
    pub first_name: <__F0__ as MapType>::Map<String>,
    pub last_name: <__F1__ as MapType>::Map<String>,
}
```

Around that struct the derive emits the entry and exit points. `HasBuilder` starts an empty builder at `__PartialPerson<IsNothing, IsNothing>`; `IntoBuilder` turns an existing value into a fully-present builder at `<IsPresent, IsPresent>`; `PartialData` records that any configuration targets the original struct; and `FinalizeBuild` reconstructs the struct but is implemented only for the all-`IsPresent` configuration — so an incomplete build cannot be finalized. It then emits, per field, an `UpdateField<Tag, M>` impl that flips one field's marker to `M` and returns the old value alongside the rebuilt partial struct, and a `HasField` impl on the partial type that is in scope only when that field's marker is `IsPresent`:

```rust
impl<__F1__: MapType> HasField<Symbol!("first_name")> for __PartialPerson<IsPresent, __F1__> {
    type Value = String;
    fn get_field(&self, _: PhantomData<Symbol!("first_name")>) -> &String { &self.first_name }
}
```

The `BuildField` trait itself is *not* emitted. It is a blanket impl in the field crate over any `UpdateField<Tag, IsPresent, Mapper = IsNothing>`, so `build_field` is sugar over the generated `update_field` that transitions a field from absent to present; `FinalizeBuild` is likewise a field-crate subtrait of `PartialData`, and the derive supplies only the all-present impl.

## Behavior and corner cases

A named field is keyed by [`Symbol!`](../../reference/macros/symbol.md) and a tuple field by [`Index<N>`](../../reference/types/index.md), following the whole family's tagging rule, so a tuple struct produces `UpdateField<Index<N>, _>` impls. The struct's generic parameters and `where` clause are preserved and carried onto the companion struct and every impl, with the field markers added on top and `PartialData::Target` naming the original struct with its own generics.

An **empty struct** produces a `__Partial{Name}` with no field markers, a trivial `HasBuilder`/`FinalizeBuild`, and no `UpdateField`/`HasField` impls. This derive emits no `HasField`/`HasFieldMut` getters on the original struct and no `HasFields` representation impls — those come from [`#[derive(HasField)]`](derive_has_field.md) and [`#[derive(HasFields)]`](derive_has_fields.md); `BuildField` is purely the construction slice, included wholesale by [`#[derive(CgpRecord)]`](derive_cgp_record.md) and [`#[derive(CgpData)]`](derive_cgp_data.md).

## Tests

`#[derive(BuildField)]` has no snapshot macro of its own; the builder items it emits are part of the record expansion pinned by the `snapshot_derive_cgp_data!` snapshots indexed in [derive_cgp_data.md's Snapshots section](derive_cgp_data.md#snapshots). The behavioral builder tests in [crates/tests/cgp-tests/tests/extensible_records/](../../../crates/tests/cgp-tests/tests/extensible_records/) exercise the machinery:

- [record_build_from.rs](../../../crates/tests/cgp-tests/tests/extensible_records/record_build_from.rs) drives `builder`/`build_from`/`build_field`/`finalize_build`.
- [optional_builder.rs](../../../crates/tests/cgp-tests/tests/extensible_records/optional_builder.rs) drives the optional-builder path.
- [point_cast.rs](../../../crates/tests/cgp-tests/tests/extensible_records/point_cast.rs) drives the `build_with_default` cast.

## Source

- Entry point: `derive_build_field` in [cgp-macro-lib/src/derive_build_field.rs](../../../crates/macros/cgp-macro-lib/src/derive_build_field.rs).
- Codegen: `ItemCgpRecord::to_build_field_items` in [cgp-macro-core/src/types/cgp_data/record.rs](../../../crates/macros/cgp-macro-core/src/types/cgp_data/record.rs), which composes the [derive_builder/](../../../crates/macros/cgp-macro-core/src/types/cgp_data/derive_builder/) helpers (`builder_struct.rs`, `has_builder_impl.rs`, `into_builder_impl.rs`, `partial_data.rs`, `finalize_build_impl.rs`, `update_field_impls.rs`, `has_field_impls.rs`); the AST types are documented in [asts/cgp_data.md](../asts/cgp_data.md).
- The `BuildField`, `FinalizeBuild`, `UpdateField`, `HasBuilder`, `IntoBuilder`, and `PartialData` traits and the `MapType` markers live under [crates/core/cgp-field/src/](../../../crates/core/cgp-field/src/).
