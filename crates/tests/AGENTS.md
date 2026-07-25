# AGENTS.md — maintaining the CGP test suite

This file governs the test crates under `crates/tests`. Read it before adding,
moving, or refactoring any test here. Invoke the `/cgp` skill first — every test
in this tree is CGP code, and the skill is the authoritative source for CGP
semantics and vocabulary.

This repository's documentation lives in the sibling
[`cgp-knowledge-base`](https://github.com/contextgeneric/cgp-knowledge-base) repository, under its
`cgp/` directory, so a doc pointer below names a path there (`cgp-knowledge-base/cgp/…`) rather than a
local one. See [../../sibling-projects.md](../../sibling-projects.md) for finding that checkout.

The test suite has two jobs, split across crates:

- **`cgp-tests`** is the main suite: realistic example code that must **compile and
  run**. A passing test is often just successful compilation, because much of CGP
  is compile-time wiring. This is where behavior is verified and where the
  user-facing macros are exercised end-to-end.
- **`cgp-macro-tests`** tests the **internals** of the CGP macros by calling the
  functions in `cgp-macro-core` directly (parsers, AST types), and is the home for
  **inputs a macro rejects** (via `assert_macro_rejects`) and for **pinning the exact
  invalid tokens** a macro emits (`invalid_expansion` string snapshots).

**Post-codegen compile failures are tested in `cargo-cgp`, not here.** The cases where a
macro *accepts* input but its *expansion* then fails to compile — and the cross-crate
coherence and orphan-rule fixtures that once lived in `cgp-test-crate-a`/`-b` — were
migrated to `cargo-cgp`'s UI test suite, which pins the readable errors the tool renders
for each class. `cargo-cgp` is CGP's first-class error toolchain, so those diagnostics
belong where the tool that improves them lives; see "Adding a failure case" for the new
workflow.

## Organize by concept, not by construct

Group tests by the **CGP concept or feature** under test, never by the macro that
happens to appear. A single construct such as `delegate_components!` serves many
concepts — basic delegation, `open` dispatch, namespace headers, `UseDelegate`
tables — so a bucket named after the construct mixes unrelated concerns and hides
what is actually being verified. Name each group for the concept: `basic_delegation`,
`abstract_types`, `implicit_arguments`, `namespaces`, `higher_order_providers`, and
so on.

The right granularity is driven by the feature, its implementation complexity, and
how many cases are needed to cover it exhaustively — **not** by mirroring the
concept documents under `cgp-knowledge-base/cgp/concepts/`. The names may coincide, but the split is
chosen for coverage. **When a category accumulates too many test cases to stay
coherent, split it into finer categories** rather than letting it sprawl; prefer
splitting early.

## A test target is a "sub-crate"

Each concept is one **integration test target**, which Cargo compiles as its own
crate — so each concept has its own coherence scope, exactly like a separate crate.
A target is two things:

- an **entrypoint file** `tests/<concept>_tests.rs` — the `_tests` suffix marks it
  as the target root; it carries a module doc comment, `#![allow(dead_code)]` when
  the target is mostly compile-time wiring, and a single `pub mod <concept>;`;
- a **module directory** `tests/<concept>/` — the clean concept name — whose
  `mod.rs` lists the unit-test modules, one `pub mod` per file.

`basic_delegation` is the reference implementation of this layout — copy its shape
when adding a concept.

## One unit test per file

Put each unit test in its own `.rs` file under the concept directory, and make the
file **self-contained**: define its own components, providers, and context types at
module scope. Do **not** separate unrelated units with `#[test]` functions or nested
`mod`s inside one file. CGP tests are dominated by type-level constructs and
compile-time wiring that live at module scope and cannot be isolated by a function
boundary; separate files are the only reliable isolation within a target. A file may
still contain a `#[test]` fn for its runtime assertions, plus the module-scope items
that test exercises.

## Explain what each test covers

Open every test file with a brief comment stating **what behavior it exercises**,
and annotate individual tricky cases inline. Link to the owning **implementation
document** — the one under `cgp-knowledge-base/cgp/implementation/` whose Tests and Snapshots
sections index this test (for example `// see cgp-knowledge-base/cgp/implementation/entrypoints/cgp_impl.md`);
that document is where test pointers live, since a reference document never links
to a test (per the knowledge base's `cgp/AGENTS.md`). You may additionally link to a reference
document when a reader needs the user-facing semantics. Tests link **to** the
documentation; the reference documents never link back to a test.

## Use macro snapshots sparingly

`cgp-macro-test-util` provides `snapshot_*!` macros (`snapshot_cgp_component!`,
`snapshot_cgp_impl!`, `snapshot_delegate_components!`, …). Each **emits the real
generated code** into the module *and* generates a `#[test]` that asserts a
pretty-printed inline `insta` snapshot of it — so adding or removing a snapshot
never changes the compile/runtime coverage, only the golden assertion. Always keep
the snapshot string **inline** in the file (`@"…"`).

The rule for when to snapshot: **snapshot a macro only in the concept target that
owns that macro's feature; everywhere else invoke the macro plainly.** Concretely,
each macro has one canonical full-expansion snapshot (plus snapshots for its
genuinely distinct variants) in its owning target, and nowhere else:

| Macro | Owning target(s) |
| --- | --- |
| `#[cgp_component]` | `basic_delegation` (+ generic variant in `generic_components`) |
| `#[cgp_impl]` | `basic_delegation` (+ `higher_order_providers`, `implicit_arguments` variants) |
| `#[cgp_type]` | `abstract_types` |
| `#[cgp_getter]` / `#[cgp_auto_getter]` | `getters` |
| `#[cgp_fn]` | `implicit_arguments`, `impl_side_dependencies` |
| `delegate_components!` | `basic_delegation` (basic), `namespaces` (open/namespace), `dispatching` (`UseDelegate`) |
| `check_components!` / `delegate_and_check_components!` | `checking` |
| `cgp_namespace!` | `namespaces` |
| `#[blanket_trait]` | `blanket_traits` |
| `#[derive(HasField)]` / `HasFields` / `CgpData` | `field_access` / `extensible_records` / `extensible_variants` |

When a file uses one of these macros as **incidental scaffolding** — a
`#[cgp_component]` needed to set up a `delegate_components!` test, say — write the
plain macro, not the snapshot form. The expansion is already pinned in the owning
target, and a redundant snapshot only adds golden output that breaks on unrelated
macro changes.

## Adding a failure case

CGP will have corner cases it does not yet handle. Do **not** try to fix them inline
while refactoring; capture them as failing-behavior tests instead. The mechanism
depends on *where* the failure lands — whether the macro refuses the input, or
accepts it and emits Rust that then fails to compile.

**Input a macro rejects — test the entrypoint in `cgp-macro-tests`.** When a macro
itself refuses the input by returning `Err` during expansion, assert it with the
`assert_macro_rejects` helper in `cgp-macro-tests` (see `parser_rejections`). This
drives the `cgp-macro-lib` entrypoint directly and checks the internal `Result`,
which is enough to pin a rejection and gives a precise check of the macro's own
diagnostic. This is the right tool for a structural error the macro is expected to
catch, and such a case does **not** also need a compile-fail test.

**Input a macro accepts whose expansion fails to compile — a UI fixture in
`cargo-cgp`.** Reserve these for input a CGP macro *accepts* but whose *expansion* then
fails to type- or borrow-check — the failure lands on the emitted Rust, not inside the
macro. These **no longer live in this repository**: they are UI fixtures in `cargo-cgp`,
the tool that rewrites such errors into readable form, so the pinned snapshot is what a
user actually sees rather than the raw cascade. Add the fixture there — a standalone
`.rs` program with `fn main`, plus a `//@aux-build: <crate>` directive when the case is
cross-crate — following
[cargo-cgp's UI-test guide](https://github.com/contextgeneric/cargo-cgp/blob/main/tests/README.md);
its harness records both `cargo-cgp`'s output (`.cgp.stderr`) and the raw compiler
baseline (`.rust.stderr`). cargo-cgp files a fixture by the **quality of the output** —
`acceptable/` when the tool already leads with the root cause, `usability/` when the
cause is present but buried, `ok/` for a clean compile — rather than by the
acceptable/problematic distinction this repository formerly used. Then catalog the class
and link the fixture (below).

**Pinning the exact invalid output** is a separate, rarer need that *does* stay here:
only when you must *inspect* the wrong tokens a macro emits (not merely assert they fail
to compile), capture the expanded code as an `insta` inline string snapshot in the
`invalid_expansion` target of `cgp-macro-tests` (the snapshot is a *string*, so it
compiles even though the code would not), with a comment explaining **why** the output
is wrong and **what the correct output should be**.

**Where a post-codegen class is documented.** Such a class is cataloged in the
[error catalog](https://github.com/contextgeneric/cgp-knowledge-base/blob/main/cgp/errors/README.md) under `cgp-knowledge-base/cgp/errors/`, the canonical
reader-facing documentation for these errors (see
[cgp-knowledge-base/cgp/errors/AGENTS.md](https://github.com/contextgeneric/cgp-knowledge-base/blob/main/cgp/errors/AGENTS.md)): the class doc describes the raw
diagnostic *and* how cargo-cgp presents it, and links the backing `cargo-cgp` UI fixture
as a GitHub URL. When a construct change alters such a diagnostic, the cross-project
[sync rule](../../AGENTS.md) applies — update the `cargo-cgp` fixture and the class doc
here together when both repos are checked out. A macro's **implementation document**
still records the *rejection* cases it catches and its behavioral tests (its `## Known
issues` and `## Tests` sections); the accept-then-fail classes are documented in the
error catalog, and a macro's `## Failure modes` section links out to the catalog class
and its `cargo-cgp` fixture rather than to a local fixture.

## Keep the docs in sync

This suite is one of the views of CGP's truth, alongside the macro implementation
in `cgp-macro-core`, the knowledge base's `cgp/reference` and `cgp/implementation`
documents, and the `/cgp` skill (see the knowledge base's `cgp/AGENTS.md`). The
implementation documents are the ones tightly coupled to this suite: each macro's implementation document has a `## Tests` section linking
every behavioral test and failure case that exercises it, and every entrypoint
document a `## Snapshots` section indexing the expansion snapshots and calling out
which variants are still missing. When a test reveals or pins a behavior worth
documenting, update the implementation document to explain that behavior directly —
and the reference document when the behavior is user-facing — without referring to
the test. When you add, move, or rename a test, update the implementation
document's Tests or Snapshots section in the same change.

## Running the suite

```
cargo nextest run -p cgp-tests                  # the main suite
cargo nextest run -p cgp-macro-tests            # macro internals + rejection/invalid-expansion cases
cargo nextest run --workspace                   # everything

cargo insta test -p cgp-tests --review          # review snapshot diffs
cargo insta test -p cgp-tests --accept          # accept intended snapshot changes
```

A snapshot test that fails prints a diff of the generated code; accept it with
`cargo insta` only after confirming the change is intended. When a change touches a
macro's accepted input or its *expansion*, also check the post-codegen behavior in
`cargo-cgp`'s UI suite (its fixtures compile CGP code through the tool), per the
cross-project [sync rule](../../AGENTS.md).

## Migration status

The suite was reorganized from a by-construct layout to this by-concept layout. As
categories grow, keep splitting them per the rule above, and keep expanding rejection
coverage in `cgp-macro-tests`. Post-codegen compile-fail coverage and cross-crate
coverage now grow in `cargo-cgp`'s UI suite rather than here (the former
`cgp-compile-fail-tests` and `cgp-test-crate-*` packages were migrated there).
