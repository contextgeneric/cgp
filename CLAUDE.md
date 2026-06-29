# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Understanding CGP first

This repository **is** the implementation of Context-Generic Programming (CGP). Before reading or
writing any CGP code here, **always invoke the `/cgp` skill** to load the fundamentals (consumer vs.
provider traits, `#[cgp_component]`/`#[cgp_impl]`/`#[cgp_fn]`, `delegate_components!`, `HasField`,
`UseDelegate`, check traits, etc.). Re-invoke it whenever you navigate into unfamiliar parts of the
codebase — the macros and core traits here are the ground truth that the skill describes, so the two
should always be read together.

The canonical export surface for users is `cgp::prelude` — see
[crates/main/cgp/src/prelude.rs](crates/main/cgp/src/prelude.rs), which re-exports
`cgp_core::prelude` + `cgp_extra::prelude`. When unsure what a name resolves to, start from the
prelude re-exports in [crates/main/cgp-core/src/prelude.rs](crates/main/cgp-core/src/prelude.rs).

## Commands

This is a Cargo workspace (edition 2024, resolver 3). Toolchain is pinned to **1.93** via
[rust-toolchain.toml](rust-toolchain.toml). Nearly every crate is `#![no_std]` — keep new code
`no_std`-compatible (use `core`/`alloc`, gate `std`/`alloc` usage behind features as existing crates
do).

- **Format** (requires nightly — `.rustfmt.toml` uses unstable `group_imports`/`imports_granularity`):
  `cargo +nightly fmt --all` (check: `cargo +nightly fmt --all -- --check`)
- **Lint:** `cargo clippy --all-features --all-targets -- -D warnings`
  and `cargo clippy --no-default-features --all-targets -- -D warnings`
- **Test** (uses `cargo-nextest`): `cargo nextest run --all-features --no-fail-fast --workspace`
- **Single test crate / test:** `cargo nextest run -p cgp-tests` or target one file with the
  standard test harness, e.g. `cargo test -p cgp-tests --test component`
- Many "tests" are **compile-time wiring checks** (`check_components!` /
  `delegate_and_check_components!`) and **macro-expansion snapshots** — for these, a successful
  `cargo build`/`cargo test` compilation *is* the passing test. A wiring mistake surfaces as a
  compile error, not a runtime failure.

## Architecture: layered micro-crates

Crates are organized so that low-level primitives have no knowledge of the high-level facade. Work
inward (core/macros) when changing fundamentals, outward (main) only to adjust the public surface.

- **`crates/macros/`** — the proc-macro pipeline. `cgp-macro` is a thin `#[proc_macro]` entrypoint
  that forwards to `cgp-macro-lib` (one module per macro), which in turn builds on
  **`cgp-macro-core`** — this is where the real parsing, AST types, and codegen live (see
  `cgp-macro-core/src/{types,functions,visitors,macros}/`). When a macro misbehaves, the logic to
  fix is almost always in `cgp-macro-core`, not the entrypoint crate. `cgp-async-macro` provides
  `#[async_trait]`; `cgp-extra-macro{,-lib}` host the extra-feature macros.

- **`crates/core/`** — the foundational runtime traits the macros expand into:
  - `cgp-component` — the wiring machinery: `DelegateComponent`, `IsProviderFor`,
    `CanUseComponent`, `UseContext`, `UseDelegate`, `UseField`, `WithProvider`, etc.
  - `cgp-type` — abstract types: `HasType`, `TypeProvider`, `UseType`.
  - `cgp-field` — `HasField` and extensible data: `Cons`/`Nil`, `Symbol`, `Index`, `Field`,
    builders/extractors for records and variants.
  - `cgp-error` — `HasErrorType`, `CanRaiseError`, `CanWrapError`.
  - `cgp-base-types` — the lowest-level type-level primitives (`Symbol`/`Chars`/`Cons`/`Nil`/path).

- **`crates/extra/`** — higher-level building blocks layered on core: `cgp-handler`,
  `cgp-dispatch`, `cgp-monad`, `cgp-run`, `cgp-runtime`, `cgp-field-extra`, `cgp-error-extra`.

- **`crates/main/`** — facade crates that only re-export. `cgp` is the crate users depend on
  (`cgp = core + extra`, exposing `cgp::prelude`). `cgp-core`/`cgp-extra`/`cgp-base`/
  `cgp-base-extra` are intermediate bundles. Changes here are almost always just re-export plumbing.

- **`crates/standalone/error/`** — pluggable error backends implementing the `cgp-error` traits:
  `cgp-error-anyhow`, `cgp-error-eyre`, `cgp-error-std`. These are opt-in and not part of the
  default `cgp` facade.

- **`crates/tests/`** — `cgp-tests` exercises real wiring and the user-facing macros end-to-end;
  `cgp-macro-tests` covers parser corner cases plus **expansion snapshots** via the `snapshot_*`
  proc macros in `cgp-macro-test-util{,-lib}` (which pretty-print generated code with
  `prettyplease`). When you change macro codegen, expect snapshot output to change — update and
  review the expanded code, since it is the contract users see.

## Conventions specific to this repo

- All versions are kept in lockstep at the workspace level (currently **0.7.0**); inter-crate
  dependencies are declared once in the root [Cargo.toml](Cargo.toml) `[workspace.dependencies]`
  and referenced with `{ workspace = true }`. Add new crates to the `members` list and the
  workspace dependency table together.
- The crate split is deliberate (coherence-friendly micro-crates). When adding functionality,
  place it in the lowest layer that makes sense and re-export upward through the facade crates,
  rather than adding cross-layer dependencies that skip the hierarchy.
- See [CHANGELOG.md](CHANGELOG.md) for the evolution of macro syntax — it is the most reliable
  record of which macro forms are current vs. removed (e.g. `#[cgp_context]` was removed,
  `ProvideType` → `TypeProvider`).
