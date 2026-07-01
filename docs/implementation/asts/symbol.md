# The `Symbol` AST

The `Symbol` stack is a single AST type: `Symbol`, which parses a string literal and emits the type-level `Symbol<LEN, Chars<…>>` string. It has no intermediate stages — one `Parse` reads the literal and one `ToTokens` emits the type — so the whole of `Symbol!`'s logic lives here. The [entrypoint document](../entrypoints/symbol.md) covers what the macro produces; this document covers the type.

## `Symbol`

`Symbol` holds the captured string and its span (`ident: String`, `span: Span`); it does not retain the `syn` literal. Its `Parse` impl reads a single `LitStr` and stores the literal's value and span, so a non-literal body is rejected there.

Its `ToTokens` impl is where the type-level string is built. It folds the string's characters right-to-left onto `Nil`, wrapping each in a `Chars<char, tail>` node, then wraps the resulting chain in `Symbol<LEN, …>` where `LEN` is the string's byte length from `str::len()`:

```rust
// "ab" folds to
Chars<'a', Chars<'b', Nil>>
// then wraps to
Symbol<2, Chars<'a', Chars<'b', Nil>>>
```

Every emitted token carries the stored span via `quote_spanned!`, so a type error on the produced `Symbol` points back at the original literal. The `Symbol`, `Chars`, and `Nil` names come from the [export markers](../../../crates/macros/cgp-macro-core/src/exports.rs) so the output is hygienic.

Beyond parsing a literal, `Symbol` is also constructed from a bare identifier through `Symbol::from_ident`, which the [`path` stack](path.md) uses to turn a lowercase path segment into a symbol; that constructor stores the identifier's text and span and reuses the same `ToTokens` emission, bypassing the `LitStr` parser. `Symbol` also implements the internal `ToType` trait, so a caller that needs a `syn::Type` rather than raw tokens can obtain one.

## Tests

- [field_access/symbol.rs](../../../crates/tests/cgp-tests/tests/field_access/symbol.rs) pins the runtime round-trip — a `Symbol!` value `Display`s back to its string and `StaticString::VALUE` recovers the literal — across the empty string, ASCII, and multi-byte Unicode, confirming the char chain rather than `LEN` drives the reconstruction.
- The `Symbol<N, Chars<…>>` emission is checked as embedded output by the record and field derive snapshots indexed in the [entrypoint document's Tests section](../entrypoints/symbol.md).

## Source

- The type lives in [cgp-macro-core/src/types/field/symbol.rs](../../../crates/macros/cgp-macro-core/src/types/field/symbol.rs).
- The runtime types `Symbol<const LEN: usize, Chars>`, `Chars<const CHAR: char, Tail>`, and `Nil` are defined in [cgp-base-types](../../../crates/core/cgp-base-types/src/types/).
