# AGENTS.md

`cgp-fork-extra-macro-core` is the pure-logic core of CGP's extra-feature macros:
`#[cgp_computer]`, `#[cgp_producer]`, and `#[cgp_auto_dispatch]`. It is to
`cgp-fork-extra-macro-lib` what [`cgp-fork-macro-core`](../cgp-fork-macro-core) is to
`cgp-fork-macro-lib`, and it depends on `cgp-fork-macro-core` so it can reuse that crate's
AST types and helpers rather than duplicating them. **Read
[cgp-fork-macro-core/AGENTS.md](../cgp-fork-macro-core/AGENTS.md) first**: its conventions
(`Parse`/`ToTokens` construct types, `parse_internal!`, `exports` markers, staged
pipelines, `define_keyword!`, `VisitMut` visitors, and brief inline docs) all
apply here unchanged. This file adds only what is specific to this crate.

## The pipeline is `crate → cgp-fork-macro-core`

The proc-macro pipeline is **`cgp-fork-extra-macro` (entrypoints) →
`cgp-fork-extra-macro-lib` (per-macro glue) → `cgp-fork-extra-macro-core` (this crate) →
`cgp-fork-macro-core`**. Like `cgp-fork-macro-core`, this crate has no `#[proc_macro]`
entrypoints and does not depend on `proc-macro`, so its stages are unit-testable
as plain functions.

## Lower through another macro's IR

These macros build on CGP's core macros: a computer is a `#[cgp_new_provider]`
impl plus a `delegate_components!` table, and a dispatch trait adds a
`#[cgp_computer]` per method. Rather than emitting those invocations for the
compiler to expand again, each macro's `eval()` stage returns the **AST nodes
those macros parse into**, its intermediate representation (IR):
`cgp_fork_macro_core::types::cgp_provider::ItemCgpProvider`,
`cgp_fork_macro_core::types::delegate_component::DelegateTable`, or this crate's own
`ItemCgpComputer`. The entrypoint then lowers that IR with the owning type's own
method (`ItemCgpProvider::lower`, `DelegateTable::eval`), so the expansion is the
final code in one step and every CGP name in it is qualified through an `exports`
marker. `#[cgp_impl]` lowers through `ItemCgpProvider` the same way. Build each IR
node with `parse_internal!` from quoted tokens, as any other AST node.

## Module map

- **[src/types/](src/types/)**: one submodule per macro, each a staged pipeline
  of `Item…` (raw input) → `.preprocess()` → `Preprocessed…` → `.eval()` → the
  IR, plus `handler_fn`, the `EvaluatedHandlerFn` IR the computer and producer
  macros share.
- **[src/exports.rs](src/exports.rs)**: the `exports` markers for the handler and
  dispatch items the expansions name, declared with `cgp-fork-macro-core`'s
  `export_constructs!`. Reuse `cgp_fork_macro_core::exports` for any item already
  declared there, such as `HasExtractor`.
- **[src/functions/](src/functions/)**: free helpers the stages share, such as
  the default provider and computer names (`derive_provider_ident`,
  `derive_computer_ident`, which unraw a raw identifier) and `return_type`.
- **[src/visitors/](src/visitors/)**: the `syn` passes over types:
  `ElaborateElidedLifetimes` (a `VisitMut` naming a dispatch method's elided
  lifetimes by the elision rules), `collect_lifetimes`, and `find_impl_trait`.
