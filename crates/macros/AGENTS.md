# AGENTS.md

The root [cgp/AGENTS.md](../../AGENTS.md) rules apply. Invoke the `/cgp` skill before working here.

## Shared rules

- Parse input by implementing `syn::parse::Parse`; emit code by implementing `quote::ToTokens`.
- Build AST nodes with `parse_internal!`, inside a `syn::Result`-returning function.
- Emit every CGP item through an `exports` marker; add a new item to `export_constructs!`.
- Keep the staged `preprocess` → `eval` → `to_items` pipeline.
- Define a custom keyword with `define_keyword!` and `IsKeyword`.
- Rewrite ASTs with `VisitMut` visitors, never with string manipulation.
- Add a one-line `///` to every public struct, trait, and function that lacks one.
- Put parser corner cases and expansion snapshots in `cgp-macro-tests`.

## Core rules

- `cgp-macro-core` has no `#[proc_macro]` entrypoints and does not depend on `proc-macro`.
- The crate's module map and pipeline are documented in the knowledge base's implementation
  documents.

## Extra-core rules

- Lower through `cgp-macro-core`'s intermediate representation, not by emitting a nested macro call.
- Reuse `cgp-macro-core`'s `exports` markers rather than declaring a duplicate.
- Build each intermediate node with `parse_internal!`.
- The crate's module map is documented in the knowledge base's implementation documents.
