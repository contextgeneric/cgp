# AGENTS.md

This tree is **v0.9.0**. Every crate is `0.9.0`. crates.io **v0.7.0** is a separate line; keep this tree's syntax. Test crates under [crates/tests/](crates/tests) are `publish = false`. The [knowledge base](https://github.com/contextgeneric/cgp-knowledge-base/tree/main/cgp) still says v0.8.0; shipping v0.9.0 needs a version edit there.

## Siblings

[sibling-projects.md](sibling-projects.md) lists each sibling, the revision to read, and how to find a checkout. Look at `../<project>` first. A committed link is a GitHub URL on `main`. A filesystem path, and a doc pointer in a source comment, stay relative.

- **[cgp-knowledge-base](https://github.com/contextgeneric/cgp-knowledge-base)** (`cgp/`): the docs. There is no `docs/` here. A behavior change carries its documentation change. When the checkout is missing, say what needs updating there.
- **[cargo-cgp](https://github.com/contextgeneric/cargo-cgp)**: rewrites compiler diagnostics into CGP errors. Prefer it over `cargo check` for a wiring failure. A diagnostic change updates its UI fixture and the [error catalog](https://github.com/contextgeneric/cgp-knowledge-base/blob/main/cgp/errors/README.md) in the same change. No `cgp` crate depends on a `cargo-cgp` crate.
- **[cgp-skills](https://github.com/contextgeneric/cgp-skills)**: the `/cgp` skill. A change to syntax, expansion, defaults, or recommended form updates the matching sub-skill in the same change.

## Before any task

Prefer the local `../cgp-knowledge-base` checkout. [summary.md](https://github.com/contextgeneric/cgp-knowledge-base/blob/main/summary.md) lists every document.

- Invoke `/cgp`, and again for an unfamiliar construct. The macros and core traits here are the ground truth.
- Read the [`cgp` README](https://github.com/contextgeneric/cgp-knowledge-base/blob/main/cgp/README.md), then the README of the part the task covers.
- Read the [construct reference](https://github.com/contextgeneric/cgp-knowledge-base/blob/main/cgp/reference/README.md) for meaning, syntax, or expansion.
- Read the [implementation reference](https://github.com/contextgeneric/cgp-knowledge-base/blob/main/cgp/implementation/README.md) when reading or changing source.
- Load `/dual-reader-prose` when editing markdown or inline comments.

The user-facing surface is `cgp::prelude` ([crates/main/cgp/src/prelude.rs](crates/main/cgp/src/prelude.rs)). When a name is unclear, start from [crates/main/cgp-core/src/prelude.rs](crates/main/cgp-core/src/prelude.rs).

## Commands

Cargo workspace, edition 2024, resolver 3, toolchain **1.98.1** ([rust-toolchain.toml](rust-toolchain.toml)). Keep new code `#![no_std]`: `core`/`alloc`, with `std` behind features.

- **Format:** `cargo +nightly fmt --all`
- **Lint:** `cargo clippy --all-features --all-targets -- -D warnings` and `cargo clippy --no-default-features --all-targets -- -D warnings`
- **Test:** `cargo nextest run --all-features --no-fail-fast --workspace`. One crate: `cargo nextest run -p cgp-tests`.
- Post-codegen compile failures are UI fixtures in `cargo-cgp`. See [crates/tests/AGENTS.md](crates/tests/AGENTS.md).
- A wiring check or expansion snapshot passes when it compiles.

## Layout

Change fundamentals in core and macros. Change [crates/main/](crates/main) only for the public surface. Versions stay `0.9.0` via root [Cargo.toml](Cargo.toml) `[workspace.dependencies]`. Add a crate to `members` and that table together. Put new functionality in the lowest layer and re-export it upward. [CHANGELOG.md](CHANGELOG.md) records current macro forms.

- **`crates/macros/`** — `cgp-macro` → `cgp-macro-lib` → `cgp-macro-core` (parse, AST, codegen). `cgp-async-macro` is `#[async_trait]`. `cgp-extra-macro{,-lib}` host the extra macros.
- **`crates/core/`** — `cgp-component`, `cgp-type`, `cgp-field`, `cgp-error`, `cgp-base-types`.
- **`crates/extra/`** — `cgp-handler`, `cgp-dispatch`, `cgp-monad`, `cgp-run`, `cgp-runtime`, `cgp-log`, `cgp-field-extra`, `cgp-error-extra`.
- **`crates/main/`** — facades. Users depend on `cgp`.
- **`crates/standalone/error/`** — `anyhow`, `eyre`, `std`. Tests are the `error_backends` target of `cgp-tests`.
- **`crates/tests/`** — `cgp-tests` for behavior and expansion snapshots; `cgp-macro-tests` for internals and rejections. A codegen change updates those snapshots.

## Macro review

Review one macro until its implementation, tests, and docs agree. A readability edit must not change behavior.

Read the construct's [reference](https://github.com/contextgeneric/cgp-knowledge-base/tree/main/cgp/reference) and [implementation](https://github.com/contextgeneric/cgp-knowledge-base/tree/main/cgp/implementation) docs, plus [cgp/AGENTS.md](https://github.com/contextgeneric/cgp-knowledge-base/blob/main/cgp/AGENTS.md), [implementation/AGENTS.md](https://github.com/contextgeneric/cgp-knowledge-base/blob/main/cgp/implementation/AGENTS.md), and [cgp-macro-core/AGENTS.md](crates/macros/cgp-macro-core/AGENTS.md). Follow the `cgp-macro-lib` entry into `cgp-macro-core` `types/<construct>/` and `functions/`, then the tests in [crates/tests/](crates/tests).

A change to behavior, syntax, expansion, or defaults updates, in the same change, the reference Expansion, the implementation Pipeline and Generated items, the snapshots, and `/cgp`. A moved or renamed test updates the implementation document's Tests or Snapshots section. Then run fmt, both clippy invocations, and `cargo nextest run` for the affected crates, and review snapshot diffs with `cargo insta`.

Ask before the next step when the intended behavior, a corner-case outcome, or a design choice is unsettled.

Harden in order: fix bugs and corner cases; record one you cannot fix as a `cgp-macro-tests` failure and under Known issues; close gaps in the owning concept target and snapshot only there; confirm each test asserts what it claims; merge overlaps; update docs that disagree with the code; add a brief `///` to any public item that lacks one.

The [cross-cutting notes](https://github.com/contextgeneric/cgp-knowledge-base/blob/main/cgp/implementation/README.md#cross-cutting-implementation-notes) explain these checks:

- **Attributes.** An unknown, duplicate, or mutually exclusive attribute fails with a spanned error.
- **Parsing.** Build `syn` nodes with [`parse_internal!`](https://github.com/contextgeneric/cgp-knowledge-base/blob/main/cgp/implementation/macros/parse_internal.md) and thread `syn::Result`. Reject shapes `syn` accepts that CGP does not.
- **Expansion.** Valid Rust only: no conflicting impls, unbound or double generics, empty expansions, or identifier clashes. A compile failure becomes a [`cargo-cgp` UI fixture](https://github.com/contextgeneric/cargo-cgp/blob/main/tests/README.md) and an [error-catalog](https://github.com/contextgeneric/cgp-knowledge-base/blob/main/cgp/errors/README.md) entry. A defect the macro should have rejected also goes under `## Known issues`.
- **Generics.** Keep kinds and roles distinct, merge parameters without collisions, and bind every parameter a generated header uses.
- **Hygiene.** Emit every CGP item through a `crate::exports` marker, resolving as `::cgp::macro_prelude::<Name>`. Reserved identifiers use `__Context__`, `__Provider__`, `__Component__`. Listing the same entry twice stays idempotent.
- **Spans.** A caret on a generated header points at the entry the user wrote, via [`override_item_span`](crates/macros/cgp-macro-core/src/functions/override_span.rs) or a carried span. A resolvable reference is not spanned onto a user token. A derived name spans the user identifier it comes from. Caret position is pinned by a `trybuild` `.stderr` fixture.
