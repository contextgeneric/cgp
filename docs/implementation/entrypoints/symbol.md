# `Symbol!` — implementation

`Symbol!` is a function-like macro that expands a string literal into a type-level string — the `Symbol<LEN, Chars<…>>` type CGP uses to name a field at compile time. This document covers how the macro parses the literal and emits that type; for the accepted syntax and the full expansion a user sees, read the reference document [reference/macros/symbol.md](../../reference/macros/symbol.md).

## Entry point

The macro is driven by the thin `Symbol` function in [cgp-macro-lib/src/symbol.rs](../../../crates/macros/cgp-macro-lib/src/symbol.rs), which parses the body into a `Symbol` construct and returns its token stream directly:

```rust
let symbol: Symbol = parse2(body)?;
Ok(symbol.to_token_stream())
```

There is no multi-stage pipeline here — a single `Parse` reads the literal and a single `ToTokens` emits the type, so all the logic lives in the one [`Symbol` AST type](../asts/symbol.md). A body that is not a string literal fails while parsing `Symbol`, since its `Parse` impl expects a `LitStr`.

## Pipeline

The macro is a single parse-then-emit step with no intermediate stages. `Symbol::parse` captures the literal's value and span, and `Symbol::to_tokens` folds that value into the `Symbol<LEN, Chars<…>>` type. The [`Symbol` AST document](../asts/symbol.md) covers the type.

## Generated items

`Symbol!` emits a single type: the `Symbol` wrapper around a right-nested `Chars` chain terminated by `Nil`, with the string's byte length baked in as the leading const argument. The chain has one `Chars` node per character:

```rust
// Symbol!("abc")
Symbol<3, Chars<'a', Chars<'b', Chars<'c', Nil>>>>
```

The chain is built by folding the characters right-to-left onto `Nil`, so an empty `Symbol!("")` collapses to `Symbol<0, Nil>`. The `Symbol`, `Chars`, and `Nil` names are emitted through the [export markers](../../../crates/macros/cgp-macro-core/src/exports.rs) so the expansion is hygienic.

## Behavior and corner cases

The leading `LEN` argument is the string's **byte** length, taken from `str::len()`, not its character count. For an ASCII string the two coincide, but a multi-byte string diverges: `Symbol!("世界你好")` records `12`, while the `Chars` chain has one node per Unicode scalar value, so four `Chars` nodes. This split is deliberate — the char count lives in the chain's shape and the byte length lives in `LEN`.

Every emitted token carries the literal's span (via `quote_spanned!`), so a downstream type error points back at the `Symbol!` invocation rather than at the macro internals.

The `Symbol` type is also constructed from a bare identifier rather than a literal by the [`Path!` stack](../asts/path.md): a lowercase path segment becomes a `Symbol` through `Symbol::from_ident`, which reuses the same `ToTokens` emission. That path does not go through this macro's `Parse` impl, which only accepts a `LitStr`.

## Known issues

The `LEN` const argument exists to work around stable Rust's inability to compute the length of a `Chars` chain inside a const-generic context. Rather than deriving the length from the character list at the type level, the macro precomputes it and bakes it in as a separate parameter. This is why the length appears redundantly in every `Symbol` type and why it is a byte length rather than a character count; it is a limitation of the encoding, not a bug.

## Tests

The `Symbol!` expansion has no snapshot macro of its own; its behavior is exercised through runtime round-trip tests and, indirectly, through the field-derive snapshots that embed `Symbol` in their output.

- [field_access/symbol.rs](../../../crates/tests/cgp-tests/tests/field_access/symbol.rs) checks that a `Symbol!` value `Display`s back to its string and that `StaticString::VALUE` recovers the original literal, covering the empty string, a single character, a multi-word string, and a multi-byte Unicode string — the last pinning that the char chain, not `LEN`, drives the reconstruction.
- [extensible_records/person_record.rs](../../../crates/tests/cgp-tests/tests/extensible_records/person_record.rs) pins, through a `snapshot_derive_cgp_data!` golden, how a multi-character field name such as `first_name` expands into its `Symbol<N, Chars<…>>` spine with the leading length.

## Source

- Entry point: `Symbol` in [cgp-macro-lib/src/symbol.rs](../../../crates/macros/cgp-macro-lib/src/symbol.rs).
- `Symbol` AST type: [cgp-macro-core/src/types/field/symbol.rs](../../../crates/macros/cgp-macro-core/src/types/field/symbol.rs), documented in [asts/symbol.md](../asts/symbol.md).
- Runtime types `Symbol<const LEN: usize, Chars>`, `Chars<const CHAR: char, Tail>`, and `Nil`: defined in [cgp-base-types](../../../crates/core/cgp-base-types/src/types/).
