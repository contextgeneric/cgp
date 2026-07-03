# `cgp-compile-fail-tests`

Compile-fail tests for the CGP macros, driven by
[`trybuild`](https://docs.rs/trybuild).

Each test is a standalone `.rs` fixture under `tests/` that a CGP macro
**accepts** but whose **expansion** then fails to type- or borrow-check — the
failure lands on the emitted Rust, not inside the macro. `trybuild` compiles
each fixture as its own throwaway crate and compares the compiler's output
against a committed `.stderr` file, so a test passes only when compilation fails
*with the pinned diagnostic*. Because the driver is an ordinary integration test
(`tests/compile_fail_tests.rs`), `cargo test` and `cargo nextest run` both
execute it — unlike the `compile_fail` doctests this crate previously held, which
`cargo nextest` silently skipped.

## What belongs here

A `trybuild` fixture is reserved for input that a CGP macro **accepts** but whose
**expansion** fails to compile — the case a macro cannot reject because it lacks
the whole-program view the borrow/coherence check needs, or a documented bug
where the macro emits code it should not. Input that a macro **rejects** during
expansion (it returns `Err`) does not belong here; test it by driving the
entrypoint directly in `cgp-macro-tests` with the `assert_macro_rejects` helper,
which pins the macro's own diagnostic.

## Two categories of failure

The fixtures are split by *whose fault the failure is*, because the two
categories carry opposite messages about CGP's health.

**`tests/acceptable/`** holds failures CGP **intentionally delegates to the Rust
compiler**. CGP is working as designed: it cannot see the whole program, so it
lowers the input faithfully and lets `rustc` reject it (overlapping
`delegate_components!` entries becoming conflicting impls, a lazily-wired
provider whose impl-side dependency the context does not meet, an ill-formed
per-entry generic the compiler rejects as unconstrained). The pinned `.stderr`
documents that the failure is the compiler doing its job, and its diagnostic is
the one a user should expect. Each fixture is cross-linked to the
`## Failure modes` section of the owning macro's implementation document.

**`tests/problematic/`** holds failures that are a **CGP defect**: input a macro
should have rejected with a spanned error, or that a macro expanded into invalid
Rust. The pinned `.stderr` captures the confusing downstream error a user
currently hits; each fixture is cross-linked to the `## Known issues` section of
the owning macro's implementation document, and its `.stderr` should improve
(ideally become a clean macro-time rejection) when the defect is fixed.

## Organization

Under each category directory, fixtures are grouped into one subdirectory per
**owning macro** — the macro whose expansion produces the failure and whose
implementation document documents it (`acceptable/delegate_components/`, or a
`problematic/<macro>/` when a defect is pinned). This mirrors the per-entrypoint
layout of the implementation docs, so a fixture and the document that indexes it
share a name. Within a subdirectory, write one fixture file per case, named for
the failure mode it probes (`duplicate_key.rs`, `missing_dependency.rs`), and
open each with a comment stating what it exercises and why it must not compile.
The driver `tests/compile_fail_tests.rs` globs both trees with `**`, so a new
fixture is picked up with no registration.

## Regenerating the `.stderr` snapshots

A fixture's committed `.stderr` is the golden output. After adding a fixture or
when an intended change alters a diagnostic, regenerate the snapshots with
`TRYBUILD=overwrite cargo test -p cgp-compile-fail-tests`, then review the diff
before committing — an unexpected change to an `acceptable/` diagnostic, or a
`problematic/` fixture that stops failing, is a signal worth reading closely.
