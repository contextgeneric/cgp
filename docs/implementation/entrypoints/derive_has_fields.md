# `#[derive(HasFields)]` — implementation

`#[derive(HasFields)]` gives a struct or enum a whole-shape view by emitting the representation impls — `HasFields`, `HasFieldsRef`, `FromFields`, `ToFields`, `ToFieldsRef` — that describe the type as a single type-level product (for a struct) or sum (for an enum). This document covers how that codegen works; for the accepted syntax and the full expansion, read the reference document [reference/derives/derive_has_fields.md](../../reference/derives/derive_has_fields.md).

## Entry point

The macro is driven by the `derive_has_fields` function in [cgp-macro-lib/src/derive_has_fields.rs](../../../crates/macros/cgp-macro-lib/src/derive_has_fields.rs). Unlike the other data derives it dispatches on shape at the entry point: it parses the input as a `syn::Item` and branches on `struct` versus `enum`, rejecting anything else with a spanned "expect body to be either a struct or enum" error:

```rust
let impls = match item {
    Item::Struct(item_struct) => ItemCgpRecord { item_struct }.to_has_fields_impls()?,
    Item::Enum(item_enum)     => derive_has_fields_impls_from_enum(&item_enum)?,
    _ => return Err(/* struct or enum */),
};
```

The struct path goes through `ItemCgpRecord`; the enum path calls the enum codegen helper directly.

## Pipeline

There is no multi-stage transform. Both paths call a single codegen helper — `derive_has_fields_impls_from_struct` for a struct, `derive_has_fields_impls_from_enum` for an enum — that emits the five representation impls. The [`cgp_data` AST stack](../asts/cgp_data.md) documents `ItemCgpRecord` and the `Symbol`/`Index` field-tag types.

## Generated items

The derive emits five impls and leaves the type definition untouched. The load-bearing part is the `Fields` associated type: a struct's fields become a [`Product`](../../reference/macros/product.md) of [`Field<Tag, Value>`](../../reference/types/field.md) entries over the `Cons`/`Nil` spine, and an enum's variants become a [`Sum`](../../reference/macros/sum.md) of `Field<Symbol!("Variant"), Payload>` entries over the `Either`/`Void` spine. Named fields and variant names are keyed by [`Symbol!`](../../reference/macros/symbol.md); tuple fields by [`Index<N>`](../../reference/types/index.md).

```rust
// struct → product
impl HasFields for Person {
    type Fields = Cons<Field<Symbol!("name"), String>, Cons<Field<Symbol!("age"), u8>, Nil>>;
}
// enum → sum, terminated by Void
impl HasFields for Shape {
    type Fields = Either<Field<Symbol!("Circle"), Circle>, Either<Field<Symbol!("Rectangle"), Rectangle>, Void>>;
}
```

Alongside the shape type, the derive emits `HasFieldsRef` (the same product/sum with each value borrowed under a fresh `'__a` lifetime) and the three conversions: `ToFields` builds the product/sum from a value, `FromFields` destructures it back, and `ToFieldsRef` builds the borrowed form. For a struct the conversions read `self.<field>.into()` into a `Cons(…)` chain and pattern-match `Cons(…)` back into `Self { … }`; for an enum they match each concrete variant to its `Either` arm and back. The product spine is built by `item_fields_to_product_type` in the `product.rs` submodule and the sum spine by `variants_to_sum_type` in `sum.rs`; the entries are chained right-associatively.

## Behavior and corner cases

A **single-field tuple struct** (a newtype) is special-cased: its `Fields` is the inner type directly, not a one-element `Cons<Field<Index<0>, _>, Nil>`, and the conversions pass the single value straight through. A tuple struct with more than one field is not special-cased — its fields are keyed by `Index<N>` and chained into the usual product.

A **unit struct** produces `Nil` as its `Fields`, and its conversions round-trip through the empty product.

The type's **generic parameters and `where` clause** are threaded onto all five impls. A borrowed field type appears verbatim in the product, and `HasFieldsRef` layers its own `'__a` borrow on top — so a field of type `&'a Name` becomes `&'__a &'a Name` in `FieldsRef<'__a>`. The `HasFieldsRef` associated type carries the `where Self: '__a` bound that every borrowed representation needs.

An **enum** always maps each variant's payload into a `Field` entry regardless of the payload's own shape; the `HasFields` enum path does not impose the single-unnamed-field requirement that the extractor and `FromVariant` derives do, because it only names the payload type rather than deconstructing it.

## Snapshots

Every `snapshot_derive_has_fields!` invocation across the suite is indexed here, since these snapshots all belong to this entrypoint. The struct expansion is owned by the `extensible_records` target and the enum expansion by `extensible_variants`:

- [extensible_records/struct_single_named_field.rs](../../../crates/tests/cgp-tests/tests/extensible_records/struct_single_named_field.rs) — the canonical struct expansion, one named field.
- [extensible_records/struct_two_named_fields.rs](../../../crates/tests/cgp-tests/tests/extensible_records/struct_two_named_fields.rs) — a two-field `Cons<_, Cons<_, Nil>>` product.
- [extensible_records/struct_single_unnamed_field.rs](../../../crates/tests/cgp-tests/tests/extensible_records/struct_single_unnamed_field.rs) — the newtype special case, `Fields` = the inner type.
- [extensible_records/struct_tuple_fields.rs](../../../crates/tests/cgp-tests/tests/extensible_records/struct_tuple_fields.rs) — a multi-field tuple struct keyed by `Index<N>`.
- [extensible_records/struct_generic.rs](../../../crates/tests/cgp-tests/tests/extensible_records/struct_generic.rs) — a generic struct with a `where` clause threaded onto each impl.
- [extensible_records/struct_generic_lifetime.rs](../../../crates/tests/cgp-tests/tests/extensible_records/struct_generic_lifetime.rs) — a lifetime plus type parameter, with the layered `&'__a &'a` borrow in `FieldsRef`.
- [extensible_variants/has_fields_enum.rs](../../../crates/tests/cgp-tests/tests/extensible_variants/has_fields_enum.rs) — the canonical enum expansion, a `Sum!` of `Field<Symbol!("Variant"), Payload>`.
- [extensible_variants/has_fields_enum_generic.rs](../../../crates/tests/cgp-tests/tests/extensible_variants/has_fields_enum_generic.rs) — a generic enum with a lifetime and a reference-typed payload.

## Tests

The snapshot tests above double as the coverage:

- Each pins one field-shape and, where paired with runtime assertions, round-trips a value through `to_fields`/`from_fields`.
- The `struct_single_unnamed_field` snapshot is the guard on the newtype special case.
- `struct_generic`/`struct_generic_lifetime` and `has_fields_enum_generic` guard the generic-threading behavior.

## Source

- Entry point: `derive_has_fields` in [cgp-macro-lib/src/derive_has_fields.rs](../../../crates/macros/cgp-macro-lib/src/derive_has_fields.rs).
- The struct path calls `ItemCgpRecord::to_has_fields_impls` in [cgp-macro-core/src/types/cgp_data/record.rs](../../../crates/macros/cgp-macro-core/src/types/cgp_data/record.rs); both paths land in the [derive_has_fields/](../../../crates/macros/cgp-macro-core/src/types/cgp_data/derive_has_fields/) submodule, where `derive_struct.rs`/`derive_enum.rs` drive the five impls, `product.rs` (`item_fields_to_product_type`) builds the struct product, `sum.rs` (`variants_to_sum_type`) builds the enum sum, and the `from_fields_*`/`to_fields_*`/`to_fields_ref_*` files build the conversions. The AST types are documented in [asts/cgp_data.md](../asts/cgp_data.md).
- The `HasFields`, `HasFieldsRef`, `FromFields`, `ToFields`, and `ToFieldsRef` traits and the `Field`/`Either`/`Void` building blocks live under [crates/core/cgp-field/src/](../../../crates/core/cgp-field/src/).
