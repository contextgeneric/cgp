# AGENTS.md

This tree is **cgp-fork** at **v0.9.0**. Every crate is `0.9.0`, named `cgp-fork` or `cgp-fork-*`. The facade package is `cgp-fork`; its Rust path is `cgp_fork`. Upstream crates.io **v0.7.0** is a separate line; keep this tree's syntax. Test crates under [crates/tests/](crates/tests) are `publish = false`. The [knowledge base](https://github.com/Just-Replicant/cgp-knowledge-base-fork/tree/main/cgp) still says v0.8.0; shipping v0.9.0 needs a version edit there.

## Siblings

[sibling-projects.md](sibling-projects.md) lists each sibling, the revision to read, and how to find a checkout. Look at `../<project>` first. A committed link is a GitHub URL on `main`. A filesystem path, and a doc pointer in a source comment, stay relative.

- **[cgp-knowledge-base-fork](https://github.com/Just-Replicant/cgp-knowledge-base-fork)** (`cgp/`): the docs. There is no `docs/` here. A behavior change carries its documentation change. When the checkout is missing, say what needs updating there.
- **[cargo-cgp](https://github.com/contextgeneric/cargo-cgp)**: rewrites compiler diagnostics into CGP errors. Prefer it over `cargo check` for a wiring failure. A diagnostic change updates its UI fixture and the [error catalog](https://github.com/Just-Replicant/cgp-knowledge-base-fork/blob/main/cgp/errors/README.md) in the same change. No `cgp-fork` crate depends on a `cargo-cgp` crate.
- **[cgp-skills](https://github.com/contextgeneric/cgp-skills)**: the `/cgp` skill. A change to syntax, expansion, defaults, or recommended form updates the matching sub-skill in the same change.

## Before any task

Prefer the local `../cgp-knowledge-base-fork` checkout. [summary.md](https://github.com/Just-Replicant/cgp-knowledge-base-fork/blob/main/summary.md) lists every document.

- Invoke `/cgp`, and again for an unfamiliar construct. The macros and core traits here are the ground truth.
- Read the [`cgp` README](https://github.com/Just-Replicant/cgp-knowledge-base-fork/blob/main/cgp/README.md), then the README of the part the task covers.
- Read the [construct reference](https://github.com/Just-Replicant/cgp-knowledge-base-fork/blob/main/cgp/reference/README.md) for meaning, syntax, or expansion.
- Read the [implementation reference](https://github.com/Just-Replicant/cgp-knowledge-base-fork/blob/main/cgp/implementation/README.md) when reading or changing source.
- Load `/dual-reader-prose` when editing markdown or inline comments.

The user-facing surface is `cgp_fork::prelude` ([crates/main/cgp-fork/src/prelude.rs](crates/main/cgp-fork/src/prelude.rs)). When a name is unclear, start from [crates/main/cgp-fork/src/core/prelude.rs](crates/main/cgp-fork/src/core/prelude.rs).

## Commands

Cargo workspace, edition 2024, resolver 3, toolchain **1.98.1** ([rust-toolchain.toml](rust-toolchain.toml)). Keep new code `#![no_std]`: `core`/`alloc`, with `std` behind features.

- **Format:** `cargo +nightly fmt --all`
- **Lint:** `cargo clippy --all-features --all-targets -- -D warnings` and `cargo clippy --no-default-features --all-targets -- -D warnings`
- **Test:** `cargo nextest run --all-features --no-fail-fast --workspace`. Behavior: `cargo nextest run -p cgp-fork --all-features`. Parsers: `cargo nextest run -p cgp-fork-macro`.
- **Release:** a push to `main` runs [release-plz](https://release-plz.dev). A `feat`, `fix`, `perf`, or breaking commit opens a release pull request that bumps the shared version and [CHANGELOG.md](CHANGELOG.md). Merging that pull request publishes the crates and tags `v<version>`.
- Post-codegen compile failures are UI fixtures in `cargo-cgp`. See [crates/tests/AGENTS.md](crates/tests/AGENTS.md).
- A wiring check or expansion snapshot passes when it compiles.

## Layout

Change the library in [crates/main/cgp-fork](crates/main/cgp-fork) and the macros in [crates/macros](crates/macros). The shared version is `[workspace.package].version` in the root [Cargo.toml](Cargo.toml). Every crate sets `version.workspace = true`, and `[workspace.dependencies]` repeats that version on each path dependency. Add a crate to `members`, that table, and the `cgp-fork` version group in [release-plz.toml](release-plz.toml) together. Put new functionality in the lowest layer and re-export it upward. [CHANGELOG.md](CHANGELOG.md) records current macro forms.

- **`crates/main/cgp-fork/`** — the published library. Core, extra, and the anyhow, eyre, and std error backends are modules. The backends are the `anyhow`, `eyre`, and `std-error` features. Behavior tests and expansion snapshots live in `tests/`.
- **`crates/macros/cgp-fork-macro/`** — the published proc-macro crate. It compiles the shared implementation under `crates/macros/shared/`. Parser tests live in `src/macro_tests/`. The `snapshot_*` macros are `#[doc(hidden)]` and compile only for those tests or with the `snapshot` feature.
- **`crates/tests/`** — the guide for both test trees. A codegen change updates the snapshots.

## Macro review

Review one macro until its implementation, tests, and docs agree. A readability edit must not change behavior.

Read the construct's [reference](https://github.com/Just-Replicant/cgp-knowledge-base-fork/tree/main/cgp/reference) and [implementation](https://github.com/Just-Replicant/cgp-knowledge-base-fork/tree/main/cgp/implementation) docs, plus [cgp/AGENTS.md](https://github.com/Just-Replicant/cgp-knowledge-base-fork/blob/main/cgp/AGENTS.md), [implementation/AGENTS.md](https://github.com/Just-Replicant/cgp-knowledge-base-fork/blob/main/cgp/implementation/AGENTS.md), and [cgp-fork-macro-core/AGENTS.md](crates/macros/shared/macro_core/AGENTS.md). Follow the `cgp-fork-macro` entry into `crates/macros/shared/macro_core` `types/<construct>/` and `functions/`, then the tests in [crates/tests/](crates/tests).

A change to behavior, syntax, expansion, or defaults updates, in the same change, the reference Expansion, the implementation Pipeline and Generated items, the snapshots, and `/cgp`. A moved or renamed test updates the implementation document's Tests or Snapshots section. Then run fmt, both clippy invocations, and `cargo nextest run` for the affected crates, and review snapshot diffs with `cargo insta`.

Ask before the next step when the intended behavior, a corner-case outcome, or a design choice is unsettled.

Harden in order: fix bugs and corner cases; record one you cannot fix as a `cgp-fork-macro` parser-test failure and under Known issues; close gaps in the owning concept target and snapshot only there; confirm each test asserts what it claims; merge overlaps; update docs that disagree with the code; add a brief `///` to any public item that lacks one.

The [cross-cutting notes](https://github.com/Just-Replicant/cgp-knowledge-base-fork/blob/main/cgp/implementation/README.md#cross-cutting-implementation-notes) explain these checks:

- **Attributes.** An unknown, duplicate, or mutually exclusive attribute fails with a spanned error.
- **Parsing.** Build `syn` nodes with [`parse_internal!`](https://github.com/Just-Replicant/cgp-knowledge-base-fork/blob/main/cgp/implementation/macros/parse_internal.md) and thread `syn::Result`. Reject shapes `syn` accepts that CGP does not.
- **Expansion.** Valid Rust only: no conflicting impls, unbound or double generics, empty expansions, or identifier clashes. A compile failure becomes a [`cargo-cgp` UI fixture](https://github.com/contextgeneric/cargo-cgp/blob/main/tests/README.md) and an [error-catalog](https://github.com/Just-Replicant/cgp-knowledge-base-fork/blob/main/cgp/errors/README.md) entry. A defect the macro should have rejected also goes under `## Known issues`.
- **Generics.** Keep kinds and roles distinct, merge parameters without collisions, and bind every parameter a generated header uses.
- **Hygiene.** Emit every CGP item through a `crate::exports` marker, resolving as `::cgp_fork::macro_prelude::<Name>`. Reserved identifiers use `__Context__`, `__Provider__`, `__Component__`. Listing the same entry twice stays idempotent.
- **Spans.** A caret on a generated header points at the entry the user wrote, via [`override_item_span`](crates/macros/shared/macro_core/functions/override_span.rs) or a carried span. A resolvable reference is not spanned onto a user token. A derived name spans the user identifier it comes from. Caret position is pinned by a `trybuild` `.stderr` fixture.
