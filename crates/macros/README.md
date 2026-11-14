# CGP macro crates: the procedural macros and their rules

This directory holds the crates that implement CGP's procedural macros. Read the rules below before
changing any of them; the knowledge base's
[implementation documents](https://github.com/contextgeneric/cgp-knowledge-base/blob/main/cgp/implementation/README.md)
describe each crate's module map, its pipeline, and what each macro generates.

## The crates

The macros are split into a shared core, the implementation of each macro, and the thin crates that
export them:

- **`cgp-macro-core`:** the AST types, parsers, visitors, and code-generation helpers the macro
  crates build on. It has no `#[proc_macro]` entrypoints and does not depend on `proc-macro`.
- **`cgp-macro-lib`** and **`cgp-macro`:** the implementation of CGP's macros, one module per macro,
  and the crate that exports them.
- **`cgp-macro-extra-core`**, **`cgp-macro-extra-lib`**, and **`cgp-macro-extra`:** the same three
  layers for the extra macros `#[cgp_computer]`, `#[cgp_producer]`, and `#[cgp_auto_dispatch]`,
  which lower through `cgp-macro-core`'s AST types.
- **`cgp-async-macro`:** `#[async_trait]`, which rewrites a trait's `async fn` to return
  `impl Future`.
- **`cgp-macro-test-util-lib`** and **`cgp-macro-test-util`:** the snapshot-testing macros, which
  emit the real generated code and pin it as an inline `insta` snapshot; their
  [README](cgp-macro-test-util/README.md) describes them.

## Rules for every macro crate

Every macro parses, builds, and emits code the same way, so its expansion stays hygienic and its
errors stay spanned:

- Parse input by implementing `syn::parse::Parse`; emit code by implementing `quote::ToTokens`.
- Build AST nodes with `parse_internal!`, inside a `syn::Result`-returning function.
- Emit every CGP item through an `exports` marker; add a new item to `export_constructs!`.
- Keep the staged `preprocess` → `eval` → `to_items` pipeline.
- Define a custom keyword with `define_keyword!` and `IsKeyword`.
- Rewrite ASTs with `VisitMut` visitors, never with string manipulation.
- Put parser corner cases and expansion snapshots in `cgp-macro-tests`, following the
  [test suite's rules](../tests/README.md#rules-for-the-test-suite).

## Rules for the extra macros

The extra macros reuse `cgp-macro-core` rather than generating input for the core macros:

- Lower through `cgp-macro-core`'s intermediate representation, not by emitting a nested macro call.
- Reuse `cgp-macro-core`'s `exports` markers rather than declaring a duplicate.
- Build each intermediate node with `parse_internal!`.
