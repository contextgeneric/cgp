# `Sum!` — implementation

`Sum!` is a function-like macro that expands a comma-separated list of types into an `Either`/`Void` chain — the type-level coproduct CGP uses to represent an enum's variants. This document covers how the macro parses its list and emits the chain; for the accepted syntax and the full expansion a user sees, read the reference document [reference/macros/sum.md](../../reference/macros/sum.md).

## Entry point

The macro is driven by the thin `Sum` function in [cgp-macro-lib/src/sum.rs](../../../crates/macros/cgp-macro-lib/src/sum.rs), which parses the body into a `SumType`, calls its `eval`, and emits the result:

```rust
let sum_type: SumType = parse2(body)?;
Ok(sum_type.eval()?.to_token_stream())
```

The body is parsed as a comma-separated list of types, so a malformed element fails at `parse2`. The single AST type is documented in the [`sum` AST stack](../asts/sum.md).

## Pipeline

The macro is a parse-then-`eval` step with no further stages, mirroring `Product!` exactly but over a different spine. `SumType::parse` reads a `Punctuated<Type, Comma>`, and `eval` folds it into the chain and re-parses through [`parse_internal!`](../macros/parse_internal.md). The [`sum` AST document](../asts/sum.md) covers the type.

## Generated items

`Sum!` emits a single type: a right-nested chain of `Either` terminated by `Void`, one `Either` per element.

```rust
// Sum![A, B, C]
Either<A, Either<B, Either<C, Void>>>
```

The chain is built by folding right-to-left onto `Void`, so an empty `Sum![]` is just `Void`. `Either` and `Void` are emitted through the [export markers](../../../crates/macros/cgp-macro-core/src/exports.rs).

## Behavior and corner cases

The only structural difference from `Product!` is the terminator: a sum folds onto `Void` rather than `Nil`. This is what makes an empty sum uninhabited — `Void` is an empty enum with no values, so an empty choice cannot be constructed, whereas an empty product (`Nil`) is a valid unit-like value. Otherwise the parsing is identical: a trailing comma is accepted through `parse_terminated`, and an empty body is valid.

## Tests

`Sum!` has no snapshot macro of its own; its expansion is exercised by a dedicated same-type assertion and by the enum field-derive that emits a `Sum!` of `Field<Tag, Value>` branches.

- [extensible_variants/sum_macro.rs](../../../crates/tests/cgp-tests/tests/extensible_variants/sum_macro.rs) writes `Sum![u32, String, bool]` and asserts it is the identical type to the hand-written `Either<u32, Either<String, Either<bool, Void>>>`, pinning the expansion directly, then builds values by nesting `Left`/`Right` to select each branch.
- [extensible_variants/has_fields_enum.rs](../../../crates/tests/cgp-tests/tests/extensible_variants/has_fields_enum.rs) exercises the `Sum!` variant spine an enum derives through `#[derive(HasFields)]`, where each branch is a `Field<Symbol!("Variant"), Payload>`.

## Source

- Entry point: `Sum` in [cgp-macro-lib/src/sum.rs](../../../crates/macros/cgp-macro-lib/src/sum.rs).
- `SumType` AST type: [cgp-macro-core/src/types/sum.rs](../../../crates/macros/cgp-macro-core/src/types/sum.rs), documented in [asts/sum.md](../asts/sum.md).
- The fold re-parses through [parse_internal!](../macros/parse_internal.md).
- Runtime types `Either<Head, Tail>` and the uninhabited `Void`: defined in [cgp-field](../../../crates/core/cgp-field/src/types/sum.rs).
