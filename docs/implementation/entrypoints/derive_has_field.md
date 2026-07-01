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

Field access through **smart pointers** is not the derive's doing: `HasField`/`HasFieldMut` have blanket impls over `Deref`/`DerefMut` targets defined in the field crate, so a `Box<Person>` resolves through to the inner struct without the derive generating anything for the pointer type.

## Snapshots

Every `snapshot_derive_has_field!` invocation across the suite is indexed here, since these snapshots all belong to this entrypoint. They live in the `field_access` target, which owns the `#[derive(HasField)]` expansion:

- [field_access/index.rs](../../../crates/tests/cgp-tests/tests/field_access/index.rs) — a tuple struct, each field keyed by `Index<0>`/`Index<1>` rather than a `Symbol!`.
- [field_access/lifetime_field.rs](../../../crates/tests/cgp-tests/tests/field_access/lifetime_field.rs) — a struct lifetime lifted onto the impls, with a borrowed field type (`&'a str`) kept as `Value`.
- [field_access/chain.rs](../../../crates/tests/cgp-tests/tests/field_access/chain.rs) — the canonical named-field expansion, over two owned structs.
- [field_access/chain_inner_life.rs](../../../crates/tests/cgp-tests/tests/field_access/chain_inner_life.rs) — the inner struct carries a lifetime, threaded onto its impls.
- [field_access/chain_outer_life.rs](../../../crates/tests/cgp-tests/tests/field_access/chain_outer_life.rs) — the outer struct borrows the inner one, with the outer lifetime on its impls.
- [field_access/chain_deeply_nested.rs](../../../crates/tests/cgp-tests/tests/field_access/chain_deeply_nested.rs) — five structs each deriving `HasField`, pinning the plain expansion repeated across a deep chain.

## Tests

The behavioral tests confirm the generated getters read the right fields:

- [field_access/index.rs](../../../crates/tests/cgp-tests/tests/field_access/index.rs) reads a tuple struct's fields at run time through `get_field` with `Index<0>`/`Index<1>` tags.
- [field_access/lifetime_field.rs](../../../crates/tests/cgp-tests/tests/field_access/lifetime_field.rs) reads a lifetime-carrying field back out.
- [field_access/chain.rs](../../../crates/tests/cgp-tests/tests/field_access/chain.rs), [chain_inner_life.rs](../../../crates/tests/cgp-tests/tests/field_access/chain_inner_life.rs), [chain_outer_life.rs](../../../crates/tests/cgp-tests/tests/field_access/chain_outer_life.rs), and [chain_deeply_nested.rs](../../../crates/tests/cgp-tests/tests/field_access/chain_deeply_nested.rs) compose the generated getters through `ChainGetters` to read a nested field in one hop.
- [field_access/symbol.rs](../../../crates/tests/cgp-tests/tests/field_access/symbol.rs) and [field_access/index_display.rs](../../../crates/tests/cgp-tests/tests/field_access/index_display.rs) exercise the `Symbol!`/`Index<N>` tag types the derive emits.

## Source

- Entry point: `derive_has_field` in [cgp-macro-lib/src/derive_has_field.rs](../../../crates/macros/cgp-macro-lib/src/derive_has_field.rs).
- It calls `ItemCgpRecord::to_has_field_impls` in [cgp-macro-core/src/types/cgp_data/record.rs](../../../crates/macros/cgp-macro-core/src/types/cgp_data/record.rs), whose codegen is `derive_has_field_impls_from_struct` in [cgp-macro-core/src/types/cgp_data/derive_has_field.rs](../../../crates/macros/cgp-macro-core/src/types/cgp_data/derive_has_field.rs); the AST types are documented in [asts/cgp_data.md](../asts/cgp_data.md).
- The `HasField`/`HasFieldMut` traits are defined in [crates/core/cgp-field/src/traits/](../../../crates/core/cgp-field/src/traits/).
