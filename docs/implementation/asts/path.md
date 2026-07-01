# The `path` AST stack

The `path` stack is the family of AST types that parse and emit CGP's type-level paths. `Path!` itself drives only two of them — `UniPath`, which parses the `@`-path and emits the `PathCons` chain, and `PathElement`, which classifies each segment as a symbol or a named type — but the same segment machinery is shared by the richer path forms the namespace and prefix constructs need. This document covers the whole stack in the order the data flows, starting with the two `Path!` drives; the [entrypoint document](../entrypoints/path.md) covers what the macro produces.

## `PathElement`

`PathElement` is one segment of a path, either a named `Type` or a `Symbol`. Its `Parse` impl first reads the segment as a full Rust `Type`, then reclassifies: if the type reduces to a single identifier, begins with a lowercase ASCII letter, and is not a primitive type name, it becomes a `Symbol` (built via `Symbol::from_ident`, see the [`Symbol` AST](symbol.md)); otherwise it stays the named `Type`.

```rust
app                 // -> Symbol (lowercase, non-primitive)
ErrorRaiserComponent // -> Type (capitalized)
u32                 // -> Type (lowercase but primitive)
```

The primitive check recognizes the `iN`/`uN`/`fN` numeric families plus `char`, `bool`, `usize`, `isize`, and `str`. `PathElement` emits itself through `ToTokens` (delegating to the symbol or the type) and also implements the internal `ToType` trait for callers that need a `syn::Type`.

## `UniPath`

`UniPath` is the whole `@`-path — the type `Path!` parses into and the one `PathElement` list its `ToTokens` folds. Its `Parse` impl consumes a leading `At` token and then a dot-separated, non-empty run of `PathElement`s via `parse_separated_nonempty`, so a body without the `@` or with no segments is rejected. Its `ToTokens` right-folds the segments onto `Nil`, wrapping each in `PathCons<element, tail>`:

```rust
// @app.error.ErrorRaiserComponent folds to
PathCons<Symbol!("app"), PathCons<Symbol!("error"), PathCons<ErrorRaiserComponent, Nil>>>
```

`UniPath` also carries helpers the namespace machinery uses — `append_type` to push a trailing named segment and `to_prefix` to convert into a `PrefixPath` — but `Path!` uses only the parse-and-emit path. `PathCons` and `Nil` come from the [export markers](../../../crates/macros/cgp-macro-core/src/exports.rs).

## `PrefixPath`

`PrefixPath` is a `UniPath` whose chain terminates in a named suffix type rather than `Nil`, used by the `#[prefix(...)]` attribute to route a component key through a namespace path. It holds the segment list plus a `suffix: Type`, and its `ToTokens` folds the segments onto that suffix instead of `Nil`, so `@bar.baz` with a `FooProviderComponent` suffix becomes `PathCons<Symbol!("bar"), PathCons<Symbol!("baz"), PathCons<FooProviderComponent, Nil>>>` once the suffix itself expands. `Path!` never produces a `PrefixPath` directly; it is reached through `UniPath::to_prefix`.

## `PathHead` and the branching forms

The remaining types support the multi-path, generic-carrying grammar that `#[cgp_namespace]` and the `open` statement accept, which `Path!` does not: `PathHead` is a recursive parser for a path that can branch (a brace group of alternative sub-paths), carry a per-value group (a bracketed set sharing a tail), or attach generics, and its `into_paths` flattens such a tree into a list of `(ImplGenerics, UniPath)` pairs. `PathElementWithGenerics` pairs a `PathElement` with leading `ImplGenerics` so a segment can introduce a lifetime or type parameter. `PathHeadOrType` and `UniPathOrType` are the entry parsers that decide, by peeking for a leading `@`, whether an input is a path or a plain type — the disambiguation the namespace and wiring grammars need where either is allowed. These belong to the path stack because they reuse `PathElement` and the same `PathCons` fold, but they are driven by the namespace and wiring macros rather than by `Path!`.

## Tests

`Path!` is rarely written directly, so the stack is exercised mainly through the namespace and prefix machinery that reuses it.

- [namespaces/redirect_lookup.rs](../../../crates/tests/cgp-tests/tests/namespaces/redirect_lookup.rs) pins, through a `snapshot_cgp_component!` golden, the `PathCons` chain a `#[prefix(@bar.baz in DefaultNamespace)]` attribute produces via `PrefixPath`.
- [namespaces/namespace_symbol_path.rs](../../../crates/tests/cgp-tests/tests/namespaces/namespace_symbol_path.rs) and [namespaces/namespace_type_path.rs](../../../crates/tests/cgp-tests/tests/namespaces/namespace_type_path.rs) exercise the lowercase-symbol and capitalized-type segment classification of `PathElement` through `#[cgp_namespace]` `@`-paths.

## Source

- The stack lives in [cgp-macro-core/src/types/path/](../../../crates/macros/cgp-macro-core/src/types/path/): `PathElement` in `path_element.rs`, `UniPath` in `unipath.rs`, `PrefixPath` in `prefix.rs`, `PathHead` in `path_head.rs`, `PathElementWithGenerics` in `path_element_with_generics.rs`, and the `PathHeadOrType`/`UniPathOrType` disambiguators in `path_head_or_type.rs` and `unipath_or_type.rs`.
- The runtime spine `PathCons` is defined in [cgp-base-types](../../../crates/core/cgp-base-types/src/types/path.rs).
