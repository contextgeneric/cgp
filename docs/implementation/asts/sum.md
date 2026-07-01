# The `sum` AST stack

The `sum` stack is a single AST type, `SumType`, behind the `Sum!` macro. It parses a comma-separated list of types and folds it into an `Either`/`Void` chain — the same one-type, parse-then-`eval` shape as [`ProductType`](product.md), differing only in the spine it folds onto. The [entrypoint document](../entrypoints/sum.md) covers what the macro produces; this document covers the type.

## `SumType`

`SumType` holds the parsed element list (`types: Punctuated<Type, Comma>`), and its `Parse` impl reads that list with `parse_terminated`, so a trailing comma is accepted and an empty body yields an empty list. Its `eval` method folds the elements right-to-left onto `Void`, wrapping each in `Either<ty, acc>`, then re-parses the accumulated tokens into a `syn::Type` through [`parse_internal!`](../macros/parse_internal.md):

```rust
// Sum![A, B, C] evals to
Either<A, Either<B, Either<C, Void>>>
// Sum![] evals to
Void
```

The only thing that distinguishes this from `ProductType` is the terminator: a sum folds onto `Void` — the uninhabited empty enum — rather than `Nil`, so an empty sum is a type with no values while an empty product is the unit-like `Nil`. `Either` and `Void` come from the [export markers](../../../crates/macros/cgp-macro-core/src/exports.rs).

## Tests

- [extensible_variants/sum_macro.rs](../../../crates/tests/cgp-tests/tests/extensible_variants/sum_macro.rs) asserts `Sum![u32, String, bool]` is the identical type to the hand-written `Either<u32, Either<String, Either<bool, Void>>>`, pinning the fold directly.
- [extensible_variants/has_fields_enum.rs](../../../crates/tests/cgp-tests/tests/extensible_variants/has_fields_enum.rs) exercises the `Sum!` variant spine an enum derives through `#[derive(HasFields)]`.

## Source

- The type lives in [cgp-macro-core/src/types/sum.rs](../../../crates/macros/cgp-macro-core/src/types/sum.rs), re-parsing its fold through [parse_internal!](../macros/parse_internal.md).
- The runtime types `Either<Head, Tail>` and the uninhabited `Void` are defined in [cgp-field](../../../crates/core/cgp-field/src/types/sum.rs).
