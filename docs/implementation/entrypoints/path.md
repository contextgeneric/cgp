# `Path!` — implementation

`Path!` is a function-like macro that expands an `@`-prefixed dotted name into a `PathCons` chain — the type-level route CGP's namespace and `RedirectLookup` machinery walks through nested delegation tables. This document covers how the macro parses the segments and emits the chain; for the accepted syntax and the full expansion a user sees, read the reference document [reference/macros/path.md](../../reference/macros/path.md).

## Entry point

The macro is driven by the thin `Path` function in [cgp-macro-lib/src/path.rs](../../../crates/macros/cgp-macro-lib/src/path.rs), which parses the body into a `UniPath` and emits its tokens directly:

```rust
let unipath: UniPath = parse2(body)?;
Ok(unipath.to_token_stream())
```

There is no `eval` step — the `UniPath` type both parses the `@`-path and, through its `ToTokens`, folds the parsed segments into the `PathCons` chain. The `UniPath` type and the segment types around it are documented in the [`path` AST stack](../asts/path.md).

## Pipeline

The macro is a single parse-then-emit step. `UniPath::parse` consumes the leading `@` and a dot-separated, non-empty run of segments, classifying each into a `PathElement` (a `Symbol` or a named `Type`); `UniPath::to_tokens` right-folds those segments into the `PathCons` chain. The classification is where the interesting decision lives, and it belongs to `PathElement` in the [`path` AST document](../asts/path.md).

## Generated items

`Path!` emits a single type: a right-nested chain of `PathCons` terminated by `Nil`, one `PathCons` per segment. Each lowercase, non-primitive segment is encoded as a `Symbol!` type-level string and every other segment is kept as the named type it spells:

```rust
// Path!(@app.error.ErrorRaiserComponent)
PathCons<
    Symbol!("app"),
    PathCons<Symbol!("error"), PathCons<ErrorRaiserComponent, Nil>>,
>
```

A lowercase segment expands further, since `Symbol!("app")` is itself a `Symbol<3, Chars<'a', …>>` chain. A single-segment path becomes `PathCons<Segment, Nil>`. `PathCons` and `Nil` are emitted through the [export markers](../../../crates/macros/cgp-macro-core/src/exports.rs).

## Behavior and corner cases

The leading `@` is required — `UniPath::parse` consumes an `At` token before anything else, so a body without it fails at parse time — and at least one segment must follow, because the segments are read with `parse_separated_nonempty`.

Each segment is first parsed as a full Rust `Type`, then reclassified: only a segment that reduces to a single identifier beginning with a lowercase ASCII letter, and is not a primitive type name, becomes a `Symbol`; everything else stays the named type. The primitive exception means a lowercase name like `u32`, `bool`, `usize`, or `str` is kept as the primitive type rather than turned into a symbol. This is the same convention `#[cgp_namespace]` entries and `#[prefix(...)]` attributes embed, and those constructs reuse the same segment and fold machinery rather than the `Path!` entry function.

## Tests

`Path!` has no snapshot macro of its own, and it is rarely written directly — the `@`-path syntax is almost always embedded inside `#[cgp_namespace]` entries and `#[prefix(...)]` attributes instead of the bare macro. Its expansion is therefore exercised indirectly through the namespace and prefix machinery that shares its parsing.

- [namespaces/redirect_lookup.rs](../../../crates/tests/cgp-tests/tests/namespaces/redirect_lookup.rs) pins, through a `snapshot_cgp_component!` golden, how a `#[prefix(@bar.baz in DefaultNamespace)]` attribute lowers a component lookup into a `RedirectLookup` over the `PathCons` chain the same segment fold produces.
- [namespaces/namespace_symbol_path.rs](../../../crates/tests/cgp-tests/tests/namespaces/namespace_symbol_path.rs) and [namespaces/namespace_type_path.rs](../../../crates/tests/cgp-tests/tests/namespaces/namespace_type_path.rs) exercise the lowercase-symbol and capitalized-type segment classifications through `#[cgp_namespace]` `@`-paths.

## Source

- Entry point: `Path` in [cgp-macro-lib/src/path.rs](../../../crates/macros/cgp-macro-lib/src/path.rs).
- The `UniPath`, `PathElement`, and the wider path stack: [cgp-macro-core/src/types/path/](../../../crates/macros/cgp-macro-core/src/types/path/), documented in [asts/path.md](../asts/path.md).
- Runtime spine `PathCons`: defined in [cgp-base-types](../../../crates/core/cgp-base-types/src/types/path.rs).
- The [`RedirectLookup`](../../reference/providers/redirect_lookup.md) provider that walks a path: [cgp-component](../../../crates/core/cgp-component/src/providers/redirect_lookup.rs).
