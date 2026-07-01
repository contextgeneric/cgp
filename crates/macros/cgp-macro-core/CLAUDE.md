# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

`cgp-macro-core` is the pure-logic core of the CGP proc-macro suite. **Always invoke the `/cgp`
skill before working here** — every type in this crate exists to generate one of the CGP constructs
the skill describes (`#[cgp_component]`, `#[cgp_impl]`, `#[cgp_fn]`, `delegate_components!`,
`check_components!`, `#[derive(HasField)]`, …). You cannot correctly change the *output* of a macro
without knowing the *expansion* it is supposed to produce, and that expansion is documented by
`/cgp`. The workspace-level [../../../CLAUDE.md](../../../CLAUDE.md) covers build/test/lint commands
and the crate hierarchy.

## What this crate is (and isn't)

- It contains **no `#[proc_macro]` entrypoints** and does **not depend on `proc-macro`** — only
  `syn` (with `full`, `extra-traits`, `visit`, `visit-mut`), `quote`, `proc-macro2`, `itertools`.
  Everything operates on `proc_macro2::TokenStream` and `syn` AST.
- The proc-macro pipeline is **`cgp-macro` (entrypoints) → `cgp-macro-lib` (per-macro glue) →
  `cgp-macro-core` (this crate, all the logic)**. A `cgp-macro-lib` function is thin: `syn::parse2`
  the attr/item, build a `cgp_macro_core::types::<macro>::Item*`, run its transform pipeline, and
  wrap the result in `quote!`. See `cgp-macro-lib/src/cgp_component.rs` for the canonical shape.
- Because it is `proc-macro`-free, its parsers and codegen are **unit-testable as plain functions**.
  Tests live in the sibling `crates/tests/cgp-macro-tests` crate (parser corner cases like
  `IdentWithTypeArgs`, plus expansion **snapshots** via the `snapshot_*` macros). Prefer adding
  coverage there over ad-hoc verification.

## Module map

- **[src/types/](src/types/)** — the bulk of the crate. One submodule per user-facing macro
  (`cgp_component`, `cgp_impl`, `cgp_provider`, `cgp_fn`, `cgp_getter`, `cgp_auto_getter`,
  `cgp_type`, `cgp_data`, `delegate_component`, `check_components`,
  `delegate_and_check_components`, `namespace`, `product`, `sum`), plus shared building-block types:
  `attributes/` (parsing of `#[uses]`, `#[use_type]`, `#[use_provider]`, `#[derive_delegate]`,
  `#[default_impl]`), `generics/`, `field/`, `getter/`, `implicits/`, `ident/`, `path/`, `keyword`.
- **[src/functions/](src/functions/)** — free helper functions: identifier case conversion
  (`camel_case`/`snake_case`), `parse_internal`, generics merging, delegated-impl synthesis,
  field/getter/implicit-argument parsing, `strip`.
- **[src/visitors/](src/visitors/)** — `syn` `VisitMut` passes that rewrite ASTs. `replace_self`
  (receiver/type/value) is the heart of `#[cgp_impl]`: it rewrites `self`/`Self` into the explicit
  `context`/`Context`. Also `replace_provider`, `remove_self_path`, `self_assoc_type`, and
  `substitute_abstract_type` (the `#[use_type]` machinery). **Do AST rewriting with these visitors,
  never with string manipulation.**
- **[src/exports.rs](src/exports.rs)** — see "hygiene" below.
- **[src/traits/](src/traits/)** — small internal traits (`ToType`, `IsKeyword`, bound helpers).
- **[src/macros/](src/macros/)** — internal `macro_rules!`: `parse_internal!`, `export_construct(s)!`,
  `define_keyword!`.
- **[src/vendor.rs](src/vendor.rs)** — re-exports `quote::quote` so exported `macro_rules!` can use
  it from downstream crates.

## Conventions you must follow

**1. Construct = a type implementing `Parse` and/or `ToTokens`.** Input is parsed by implementing
`syn::parse::Parse`; code is emitted by implementing `quote::ToTokens`. Add new syntax by writing a
type with these impls rather than hand-rolling token munging.

**2. Build AST nodes with `parse_internal!`, not by hand.** `parse_internal!(#some_tokens ...)`
(in [src/functions/parse_internal.rs](src/functions/parse_internal.rs)) quasi-quotes tokens and
parses them into the target `syn` type, attaching a descriptive error (including the offending
tokens with the `::cgp::macro_prelude::` prefix stripped) on failure. It expands to a `?`
expression, so call it inside a `syn::Result`-returning function.

**3. Reference CGP items through `exports.rs`, never by hardcoded path.** Each name in
[src/exports.rs](src/exports.rs) (`IsProviderFor`, `DelegateComponent`, `UseField`, `HasField`, …)
is a zero-sized marker struct whose `ToTokens` emits the fully-qualified path
`::cgp::macro_prelude::<Name>`. Generated code interpolates these markers (e.g.
`use crate::exports::IsProviderFor;` then `#is_provider_for`) so the expansion is hygienic and the
user only needs `cgp` in scope. **To emit a new CGP item, add it to `export_constructs!` in
`exports.rs` and interpolate the marker** — don't write `::cgp::...` path literals inline.

**4. Codegen is a staged transform pipeline.** Each macro type moves through explicit stages rather
than emitting in one pass. The reference shape is `cgp_component`:
`ItemCgpComponent { args, item_trait }` → `.preprocess()` → `PreprocessedCgpComponent` →
`.eval()` → `Evaluated…` → `.to_items()` → `Vec<syn::Item>` (driven by
`cgp-macro-lib/src/cgp_component.rs` as `item.preprocess()?.eval()?.to_items()?`). Other macros use
their own stage names — look for `item.rs`, `preprocessed.rs`/`lowered.rs`, `evaluated/`, and
`to_*` methods within each `types/<macro>/` directory. When extending a macro, slot your change
into the existing stage rather than collapsing the pipeline.

**5. Custom keywords go through `define_keyword!` + `IsKeyword`** (see
[src/macros/keyword.rs](src/macros/keyword.rs) and `types/keyword*.rs`).

**6. Keep inline docs brief and current as you go.** When you review a file — whether to change it
or to write its [implementation document](../../../docs/implementation/README.md) — improve the
inline docs in the same pass. Add a one-line `///` to any public struct, trait, or function that
lacks one, saying what it is or does (for a pipeline stage, its role in the sequence); prefer naming
the *why* or a corner case over restating the signature. Fix a doc that no longer matches the code,
and clarify genuinely confusing code (for example, why `generic_params_to_path` keeps only type
parameters) with a short comment. Keep them terse: delete a comment that only restates obvious code,
and do not narrate line by line. Deep mechanics belong in the implementation document, not in a wall
of inline prose — link to it rather than duplicating it.
