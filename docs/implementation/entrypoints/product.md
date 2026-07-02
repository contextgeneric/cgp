# `Product!` and `product!` — implementation

`Product!` and `product!` are function-like macros that expand a comma-separated list into a `Cons`/`Nil` chain — a type at the type level for `Product!`, a matching value for `product!`. This document covers how each parses its list and emits the chain; for the accepted syntax and the full expansion a user sees, read the reference document [reference/macros/product.md](../../reference/macros/product.md).

## Entry point

The two macros are driven by the `Product` and `product` functions in [cgp-macro-lib/src/product.rs](../../../crates/macros/cgp-macro-lib/src/product.rs), which follow the same shape: parse the body into an AST type, call its `eval`, and emit the result.

```rust
// Product!
let product_type: ProductType = parse2(body)?;
Ok(product_type.eval()?.to_token_stream())

// product!
let product_expr: ProductExpr = parse2(body)?;
Ok(product_expr.eval()?.to_token_stream())
```

Both parse the body as a comma-separated list, so a malformed element fails at `parse2`. The two AST types are documented together in the [`product` AST stack](../asts/product.md).

## Pipeline

Each macro is a parse-then-`eval` step with no further stages. `Product!`'s `Parse` impl reads a `Punctuated<Type, Comma>` and `product!`'s reads a `Punctuated<Expr, Comma>`; each `eval` folds its list into the chain and re-parses the result through [`parse_internal!`](../macros/parse_internal.md), validating `Product!`'s output as a `syn::Type` and `product!`'s as a `syn::Expr`. The [`product` AST document](../asts/product.md) covers both types.

## Generated items

`Product!` emits a single type: a right-nested chain of `Cons` terminated by `Nil`, one `Cons` per element.

```rust
// Product![A, B, C]
Cons<A, Cons<B, Cons<C, Nil>>>
```

`product!` emits a value of that type using the `Cons` tuple-struct constructor rather than the type form, so the value's type is exactly what `Product!` builds over the corresponding element types:

```rust
// product![a, b, c]
Cons(a, Cons(b, Cons(c, Nil)))
```

The chain is built by folding right-to-left onto `Nil`, so an empty `Product![]` (or `product![]`) is just `Nil`. `Cons` and `Nil` are emitted through the [export markers](../../../crates/macros/cgp-macro-core/src/exports.rs).

## Behavior and corner cases

The two macros parse at the level they emit at: `Product!` reads its elements as types and `product!` reads them as expressions, so a `product!` element may be any expression — a literal, a method call, an arithmetic expression — and not just a path that also parses as a type. Parsing and re-parsing at the right level is what keeps each macro in its position: `product!`'s `syn::Expr` output is valid in expression context and `Product!`'s `syn::Type` output in type context. A trailing comma is accepted on both because the list is parsed with `parse_terminated`, and an empty body is valid and yields `Nil` (a value for `product!`, a type for `Product!`).

Because `eval` re-parses its output through `parse_internal!`, a fold that produced malformed tokens would surface as a spanned `syn::Error` rather than raw token garbage — though with `Cons`/`Nil` and well-formed elements this path does not normally fail.

## Tests

`Product!`/`product!` have no snapshot macro of their own; the type-level chain they build is exercised wherever a struct's field list is derived, since `#[derive(HasFields)]` emits a `Product!` of `Field<Tag, Value>` entries, and the value-level form has a dedicated behavioral test.

- [extensible_records/product_value.rs](../../../crates/tests/cgp-tests/tests/extensible_records/product_value.rs) exercises the value-level `product!` directly: that expression items build the nested `Cons(..)`/`Nil` value, that its type is the matching `Product!`, and that the empty and trailing-comma forms work.
- [extensible_records/person_record.rs](../../../crates/tests/cgp-tests/tests/extensible_records/person_record.rs) and [extensible_records/record_derive.rs](../../../crates/tests/cgp-tests/tests/extensible_records/record_derive.rs) pin, through `snapshot_derive_cgp_data!` goldens, the `Product!` field spine a record derives, so the `Cons`/`Nil` shape is checked as embedded output.
- [handlers/pipe_handlers.rs](../../../crates/tests/cgp-tests/tests/handlers/pipe_handlers.rs) uses `Product![…]` to write a handler pipeline, exercising the type-level form as a list of provider types rather than fields.

## Source

- Entry points: `Product` and `product` in [cgp-macro-lib/src/product.rs](../../../crates/macros/cgp-macro-lib/src/product.rs).
- `ProductType` and `ProductExpr` AST types: [cgp-macro-core/src/types/product/](../../../crates/macros/cgp-macro-core/src/types/product/), documented in [asts/product.md](../asts/product.md).
- The fold re-parses through [parse_internal!](../macros/parse_internal.md).
- Runtime types `Cons<Head, Tail>` and `Nil`: defined in [cgp-base-types](../../../crates/core/cgp-base-types/src/types/).
