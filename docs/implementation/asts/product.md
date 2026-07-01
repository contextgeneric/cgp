# The `product` AST stack

The `product` stack is the pair of AST types behind `Product!` and `product!`: `ProductType` for the type-level macro and `ProductExpr` for the value-level one. Both parse a comma-separated list into a `Punctuated<Type, Comma>` and fold it into a `Cons`/`Nil` chain, differing only in whether they emit the type form `Cons<…>` or the constructor form `Cons(…)`. The [entrypoint document](../entrypoints/product.md) covers what the macros produce; this document covers the types.

## `ProductType`

`ProductType` is the type-level form. It holds the parsed element list (`types: Punctuated<Type, Comma>`), and its `Parse` impl reads that list with `parse_terminated`, so a trailing comma is accepted and an empty body yields an empty list. Its `eval` method folds the elements right-to-left onto `Nil`, wrapping each in the type form `Cons<ty, acc>`, then re-parses the accumulated tokens into a `syn::Type` through [`parse_internal!`](../macros/parse_internal.md):

```rust
// Product![A, B, C] evals to
Cons<A, Cons<B, Cons<C, Nil>>>
// Product![] evals to
Nil
```

Returning a validated `syn::Type` rather than a raw token stream is what lets the entry function drop the result straight into `to_token_stream()`. `Cons` and `Nil` come from the [export markers](../../../crates/macros/cgp-macro-core/src/exports.rs).

## `ProductExpr`

`ProductExpr` is the value-level form and is structurally identical: it too holds a `Punctuated<Type, Comma>` and parses with `parse_terminated`. The one difference is in `eval`, which wraps each element in the tuple-struct constructor form `Cons(ty, acc)` rather than the type form, folding onto `Nil` the same way:

```rust
// product![a, b, c] evals to
Cons(a, Cons(b, Cons(c, Nil)))
```

Note that its field is still named and typed as a list of `Type`, not `Expr` — the value macro reuses the type parser and only changes the emission shape, so the two forms cannot diverge in how they read their input.

## Tests

- The `Cons`/`Nil` field spine is pinned as embedded output by the record derive snapshots ([extensible_records/person_record.rs](../../../crates/tests/cgp-tests/tests/extensible_records/person_record.rs), [extensible_records/record_derive.rs](../../../crates/tests/cgp-tests/tests/extensible_records/record_derive.rs)), which emit a `Product!` of `Field<Tag, Value>` entries.
- [handlers/pipe_handlers.rs](../../../crates/tests/cgp-tests/tests/handlers/pipe_handlers.rs) exercises the type-level `Product![…]` as a list of provider types in a handler pipeline.

## Source

- The stack lives in [cgp-macro-core/src/types/product/](../../../crates/macros/cgp-macro-core/src/types/product/): `ProductType` in `product_type.rs` and `ProductExpr` in `product_expr.rs`, both re-parsing their fold through [parse_internal!](../macros/parse_internal.md).
- The runtime types `Cons<Head, Tail>` and `Nil` are defined in [cgp-base-types](../../../crates/core/cgp-base-types/src/types/).
