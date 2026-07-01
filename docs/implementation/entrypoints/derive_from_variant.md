# `#[derive(FromVariant)]` — implementation

`#[derive(FromVariant)]` emits just the variant-construction slice of the variant machinery: one `FromVariant` impl per variant, so an enum can be built generically from any single variant addressed by a type-level tag. This document covers how that codegen works; for the accepted syntax and the full expansion, read the reference document [reference/derives/derive_from_variant.md](../../reference/derives/derive_from_variant.md).

## Entry point

The macro is driven by the `derive_from_variant` function in [cgp-macro-lib/src/derive_from_variant.rs](../../../crates/macros/cgp-macro-lib/src/derive_from_variant.rs). It parses the input into a `syn::ItemEnum`, wraps it in an `ItemCgpVariant`, and calls `to_from_variant_impls` — the same method the enum path of `#[derive(CgpData)]` uses for its constructor slice:

```rust
let variant = ItemCgpVariant { item_enum };
let item_impls = variant.to_from_variant_impls()?;
```

Applying the derive to a non-enum item fails at `syn::parse2`.

## Pipeline

There is no multi-stage transform. `ItemCgpVariant::to_from_variant_impls` forwards to the single codegen helper `derive_from_variant_from_enum`, which walks the enum's variants and emits one constructor impl each. The [`cgp_data` AST stack](../asts/cgp_data.md) documents `ItemCgpVariant` and the `Symbol` field-tag type.

## Generated items

The derive emits one [`FromVariant`](../../reference/traits/from_variant.md) impl per variant and nothing else — no companion type and no presence tracking, making it the simplest derive in the family. Each impl is keyed by the [`Symbol!`](../../reference/macros/symbol.md) of the variant's name, takes the variant's payload as the associated `Value`, and wraps it in the variant:

```rust
impl FromVariant<Symbol!("Circle")> for Shape {
    type Value = Circle;
    fn from_variant(_tag: PhantomData<Symbol!("Circle")>, value: Self::Value) -> Self { Self::Circle(value) }
}
```

The `PhantomData<Tag>` argument exists only to let a caller select which variant to build when several `FromVariant` impls are in scope. The `FromVariant` trait itself is defined in the field crate; the derive supplies only the per-variant impls.

## Behavior and corner cases

The enum's generic parameters are threaded onto every impl. This derive emits no `HasFields` representation impls and no extractor — those come from [`#[derive(HasFields)]`](derive_has_fields.md) and [`#[derive(ExtractField)]`](derive_extract_field.md); `FromVariant` is purely the construction slice, included wholesale by [`#[derive(CgpVariant)]`](derive_cgp_variant.md) and [`#[derive(CgpData)]`](derive_cgp_data.md).

## Known issues

Like the extractor derive, `#[derive(FromVariant)]` requires every variant to be a single-unnamed-field tuple variant. A fieldless, multi-field, or struct-style variant makes the macro fail with "Expected variant to contain exactly one unnamed field," with no per-variant opt-out. The requirement is described alongside the extractor's in [`derive_extract_field`](derive_extract_field.md#known-issues), and the reference document records its user-visible form.

## Tests

`#[derive(FromVariant)]` has no snapshot macro of its own; the constructor impls it emits are part of the variant expansion pinned by the `snapshot_derive_cgp_data!` snapshots indexed in [derive_cgp_data.md's Snapshots section](derive_cgp_data.md#snapshots).

- The behavioral variant tests in [crates/tests/cgp-tests/tests/extensible_variants/](../../../crates/tests/cgp-tests/tests/extensible_variants/) — notably [variant_dispatch.rs](../../../crates/tests/cgp-tests/tests/extensible_variants/variant_dispatch.rs) — construct enums through the generated `from_variant`.
- The single-unnamed-field requirement has no dedicated failure case in `cgp-macro-tests` and is a candidate for one.

## Source

- Entry point: `derive_from_variant` in [cgp-macro-lib/src/derive_from_variant.rs](../../../crates/macros/cgp-macro-lib/src/derive_from_variant.rs).
- Codegen: `ItemCgpVariant::to_from_variant_impls` in [cgp-macro-core/src/types/cgp_data/variant.rs](../../../crates/macros/cgp-macro-core/src/types/cgp_data/variant.rs), which delegates to `derive_from_variant_from_enum` in [cgp-macro-core/src/types/cgp_data/derive_from_variant.rs](../../../crates/macros/cgp-macro-core/src/types/cgp_data/derive_from_variant.rs); the AST types are documented in [asts/cgp_data.md](../asts/cgp_data.md).
- The `FromVariant` trait is defined in [crates/core/cgp-field/src/traits/from_variant.rs](../../../crates/core/cgp-field/src/traits/from_variant.rs).
