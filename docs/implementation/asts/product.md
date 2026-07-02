# The `product` AST stack

The `product` stack is the pair of AST types behind `Product!` and `product!`: `ProductType` for the type-level macro and `ProductExpr` for the value-level one. Both parse a comma-separated list and fold it into a `Cons`/`Nil` chain, but they parse and emit at different levels: `ProductType` reads a list of types and folds it into the type `Cons<…>`, while `ProductExpr` reads a list of expressions and folds it into the value `Cons(…)`. The [entrypoint document](../entrypoints/product.md) covers what the macros produce; this document covers the types.

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

`ProductExpr` is the value-level form. It holds the parsed element list as `exprs: Punctuated<Expr, Comma>` and parses with `parse_terminated`, so each element is a full Rust expression — a literal, a method call, an arithmetic expression — not merely a path that also happens to parse as a type. Its `eval` folds the elements right-to-left onto `Nil` with the tuple-struct constructor form `Cons(expr, acc)`, then re-parses the accumulated tokens into a `syn::Expr` through [`parse_internal!`](../macros/parse_internal.md):

```rust
// product![a, b, c] evals to
Cons(a, Cons(b, Cons(c, Nil)))
```

Parsing the items as `Expr` and re-parsing the fold as `Expr` is what keeps `product!` in expression position: the macro emits a value, so its output is dropped into an expression context, exactly as `ProductType`'s `syn::Type` output is dropped into a type context.

## Tests

- The `Cons`/`Nil` field spine is pinned as embedded output by the record derive snapshots ([extensible_records/person_record.rs](../../../crates/tests/cgp-tests/tests/extensible_records/person_record.rs), [extensible_records/record_derive.rs](../../../crates/tests/cgp-tests/tests/extensible_records/record_derive.rs)), which emit a `Product!` of `Field<Tag, Value>` entries.
- [handlers/pipe_handlers.rs](../../../crates/tests/cgp-tests/tests/handlers/pipe_handlers.rs) exercises the type-level `Product![…]` as a list of provider types in a handler pipeline.
- [extensible_records/product_value.rs](../../../crates/tests/cgp-tests/tests/extensible_records/product_value.rs) exercises the value-level `product!`: that expression items (a method call, an arithmetic expression) build the nested `Cons(..)`/`Nil` value, that the value's type is the matching `Product!`, and that the empty and trailing-comma forms work.

## Source

- The stack lives in [cgp-macro-core/src/types/product/](../../../crates/macros/cgp-macro-core/src/types/product/): `ProductType` in `product_type.rs` and `ProductExpr` in `product_expr.rs`, both re-parsing their fold through [parse_internal!](../macros/parse_internal.md).
- The runtime types `Cons<Head, Tail>` and `Nil` are defined in [cgp-base-types](../../../crates/core/cgp-base-types/src/types/).
