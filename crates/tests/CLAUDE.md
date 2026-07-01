# CLAUDE.md — maintaining the CGP test suite

This file governs the test crates under `crates/tests`. Read it before adding,
moving, or refactoring any test here. Invoke the `/cgp` skill first — every test
in this tree is CGP code, and the skill is the authoritative source for CGP
semantics and vocabulary.

The test suite has three jobs, split across crates:

- **`cgp-tests`** is the main suite: realistic example code that must **compile and
  run**. A passing test is often just successful compilation, because much of CGP
  is compile-time wiring. This is where behavior is verified and where the
  user-facing macros are exercised end-to-end.
- **`cgp-macro-tests`** tests the **internals** of the CGP macros by calling the
  functions in `cgp-macro-core` directly (parsers, AST types), and is the home for
  **failure cases** — inputs CGP should reject, and cases where a macro currently
  emits invalid or wrong code.
- **`cgp-test-crate-a` / `cgp-test-crate-b`** are auxiliary packages for
  **cross-crate** behavior: whether a downstream crate can extend a namespace or
  provide a provider for a component defined elsewhere, under Rust's coherence and
  orphan rules.

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
concept documents under `docs/concepts/`. The names may coincide, but the split is
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
document** — the one under `docs/implementation/` whose Tests and Snapshots
sections index this test (for example `// see docs/implementation/entrypoints/cgp_impl.md`);
that document is where test pointers live, since a reference document never links
to a test (per `docs/CLAUDE.md`). You may additionally link to a reference
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

## Adding a failure case (in `cgp-macro-tests`)

CGP will have corner cases it does not yet handle. Do **not** try to fix them inline
while refactoring; capture them as failing-behavior tests instead, in a dedicated
failure-case target:

- **Input that should be rejected** — assert the `cgp-macro-core` parser rejects it,
  using the `assert_rejects` helper pattern (see `ident_with_type_params`).
- **A macro that emits invalid Rust** — capture the expanded code as an `insta`
  inline snapshot (the snapshot is a *string*, so it compiles even though the code
  would not), and add a code comment explaining **why** the output is wrong and
  **what the correct output should be**.

Every failure case must also be recorded in the construct's **implementation
document** under `docs/implementation/`, in its `## Known issues` section and
indexed from its `## Tests` section, describing the behavior without referring to
the test. When the failure has a user-visible consequence, note that in the
reference document's `## Known issues` section too and cross-link the two. Put a
link from the test's comment to the implementation document.

## Keep the docs in sync

This suite is one of the views of CGP's truth, alongside the macro implementation
in `cgp-macro-core`, the reference documents in `docs/reference`, the
implementation documents in `docs/implementation`, and the `/cgp` skill (see
`docs/CLAUDE.md`). The implementation documents are the ones tightly coupled to
this suite: each macro's implementation document has a `## Tests` section linking
every behavioral test and failure case that exercises it, and every entrypoint
document a `## Snapshots` section indexing the expansion snapshots and calling out
which variants are still missing. When a test reveals or pins a behavior worth
documenting, update the implementation document to explain that behavior directly —
and the reference document when the behavior is user-facing — without referring to
the test. When you add, move, or rename a test, update the implementation
document's Tests or Snapshots section in the same change.

## Running the suite

```
cargo nextest run -p cgp-tests            # the main suite
cargo nextest run -p cgp-macro-tests      # macro internals + failures
cargo nextest run --workspace             # everything

cargo insta test -p cgp-tests --review    # review snapshot diffs
cargo insta test -p cgp-tests --accept    # accept intended snapshot changes
```

A snapshot test that fails prints a diff of the generated code; accept it with
`cargo insta` only after confirming the change is intended.

## Migration status

The suite was reorganized from a by-construct layout to this by-concept layout. As
categories grow, keep splitting them per the rule above, and keep expanding failure
coverage in `cgp-macro-tests` and cross-crate coverage in the `cgp-test-crate-*`
packages — these were established with representative cases and are meant to grow.
