# AGENTS.md — maintaining the CGP test suite

This file governs the test crates under `crates/tests`. Read it before adding,
moving, or refactoring any test here. Invoke the `/cgp` skill first — every test
in this tree is CGP code, and the skill is the authoritative source for CGP
semantics and vocabulary.

The test suite has four jobs, split across crates:

- **`cgp-tests`** is the main suite: realistic example code that must **compile and
  run**. A passing test is often just successful compilation, because much of CGP
  is compile-time wiring. This is where behavior is verified and where the
  user-facing macros are exercised end-to-end.
- **`cgp-macro-tests`** tests the **internals** of the CGP macros by calling the
  functions in `cgp-macro-core` directly (parsers, AST types), and is the home for
  **inputs a macro rejects** (via `assert_macro_rejects`) and for **pinning the exact
  invalid tokens** a macro emits (`invalid_expansion` string snapshots).
- **`cgp-compile-fail-tests`** holds the **`trybuild` compile-fail tests**: input a
  macro *accepts* but whose *expansion* then fails to compile. Each case is a
  standalone `.rs` fixture whose expected compiler output is pinned in a committed
  `.stderr` file, and the fixtures are split into **acceptable** and **problematic**
  failures (see "Adding a failure case" below). Because the driver is an ordinary
  integration test, `cargo nextest` runs it like any other test.
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
to a test (per `docs/AGENTS.md`). You may additionally link to a reference
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

**Input a macro accepts whose expansion fails to compile — a `trybuild` fixture in
`cgp-compile-fail-tests`.** Reserve these tests for input a CGP macro *accepts* but
whose *expansion* then fails to type- or borrow-check — the failure lands on the
emitted Rust, not inside the macro. This is the tool for a documented bug or known
limitation, and for the cases a macro cannot reject because it lacks the whole-program
view the check needs. Each case is a standalone `.rs` fixture (a complete program with
`fn main`) under the crate's `tests/` tree; `trybuild` compiles each as its own crate
and compares the compiler's output against a committed sibling `.stderr` file, so the
test passes only when compilation fails *with the pinned diagnostic*. The `.stderr`
snapshot is what proves *which* element causes the failure and *why* — it names the
exact error code and span — so it replaces the companion ```` ```rust ```` block the
old doctest form paired with each probe. Because the toolchain is pinned
([rust-toolchain.toml](../../rust-toolchain.toml)), the pinned diagnostics are stable
across runs.

**Split every fixture into acceptable and problematic.** A compile failure carries one
of two opposite meanings, and the directory it lives in records which:

- **`tests/acceptable/`** — the failure is one CGP **intentionally delegates to the
  Rust compiler**. CGP is working as designed: it cannot see the whole program, so it
  lowers the input faithfully and lets `rustc` reject it. Two separate
  `delegate_components!` blocks that delegate the same key, generic
  `delegate_components!` entries that expand to overlapping impls, and a lazily-wired
  provider whose impl-side dependency the context does not satisfy all belong here. The
  pinned `.stderr` documents that the diagnostic a user sees is the compiler doing its
  job, and it is the diagnostic they should expect.
- **`tests/problematic/`** — the failure is a **CGP defect**: input a macro should have
  rejected with a spanned error, or that a macro expanded into invalid Rust. The pinned
  `.stderr` captures the confusing downstream error a user currently hits. Every
  problematic fixture must be cross-linked (in its header comment) to the `## Known
  issues` section of the owning macro's implementation document, and its `.stderr`
  should improve — ideally becoming a clean macro-time rejection — when the defect is
  fixed. When it does, regenerate the snapshot and, if the failure moves into the macro
  itself, migrate the case to an `assert_macro_rejects` test in `cgp-macro-tests`.

Under each category directory, group fixtures into one subdirectory per **owning
macro** — the macro whose expansion produces the failure and whose implementation
document documents it (`acceptable/delegate_components/`, or a `problematic/<macro>/`
when a defect is pinned). This is the one place the suite groups by construct rather
than by concept, and deliberately so: a compile-fail case is defined by *which
macro's expansion* fails, and each cross-links to that macro's per-entrypoint
implementation document, so the fixture tree mirrors [docs/implementation/entrypoints/](../../docs/implementation/entrypoints).
Within a subdirectory, write one fixture file per case, named for the failure mode it
probes (`duplicate_key.rs`, `missing_dependency.rs`), and open each with a comment
stating what it exercises and why it must not compile — exactly as the main suite
requires. **Keep each fixture subdirectory small — no more than about a dozen cases —
and split into further nested subdirectories when one grows past that**; the driver's
`**` glob picks up the new level with no registration. The driver [tests/compile_fail_tests.rs](cgp-compile-fail-tests/tests/compile_fail_tests.rs)
globs both trees with `**`, so the two `t.compile_fail(...)` calls pick up a new
fixture with no per-file registration. A single `trybuild::TestCases` runs both globs
— do not split them across two `#[test]` functions, which would race on the shared
build directory.

Run the suite with `cargo test -p cgp-compile-fail-tests` or `cargo nextest run -p
cgp-compile-fail-tests`; both work because the driver is an ordinary integration test.
After adding a fixture, or when an intended change alters a diagnostic, regenerate the
golden output with `TRYBUILD=overwrite cargo test -p cgp-compile-fail-tests` and review
the diff before committing.

**Pinning the exact invalid output** is a separate, rarer need: only when you must
*inspect* the wrong tokens a macro emits (not merely assert they fail to compile),
capture the expanded code as an `insta` inline string snapshot in the
`invalid_expansion` target of `cgp-macro-tests` (the snapshot is a *string*, so it
compiles even though the code would not), with a comment explaining **why** the
output is wrong and **what the correct output should be**.

Post-codegen compile-fail cases are additionally cataloged in the [error catalog](../../docs/errors/README.md)
under `docs/errors/`, which is becoming the canonical, reader-facing documentation for
them (see [docs/errors/AGENTS.md](../../docs/errors/AGENTS.md)): a fixture cross-links to
the error *class* it exercises there, and the catalog indexes the fixture. This migration
proceeds class by class — until a class is migrated, its fixture continues to cross-link
to the owning macro's implementation document as described below.

Every failure case must also be recorded in the owning macro's **implementation
document** under `docs/implementation/`, and *which section* holds it is what the
acceptable/problematic split decides. An **acceptable** fixture documents *intended*
behavior — a failure CGP deliberately defers to the compiler — so it belongs in that
document's `## Failure modes` section (a dedicated section, kept out of Known issues so
it is not mistaken for a bug), with a short code snippet of the failing input, and is
indexed from `## Tests`. A **problematic** fixture (and every `invalid_expansion`
snapshot) documents a defect, so describe it in `## Known issues` alongside the
construct's other bugs, again with a snippet, and index it from `## Tests`; when the
defect has a user-visible consequence, note it in the reference document's `## Known
issues` section too and cross-link the two. In both cases describe the behavior in the
document's own words without referring to the test, and put a link from the fixture's
header comment back to the implementation document — to its Failure modes section for an
acceptable case, its Known issues section for a problematic one.

## Keep the docs in sync

This suite is one of the views of CGP's truth, alongside the macro implementation
in `cgp-macro-core`, the reference documents in `docs/reference`, the
implementation documents in `docs/implementation`, and the `/cgp` skill (see
`docs/AGENTS.md`). The implementation documents are the ones tightly coupled to
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
cargo nextest run -p cgp-tests                  # the main suite
cargo nextest run -p cgp-macro-tests            # macro internals + rejection/invalid-expansion cases
cargo nextest run -p cgp-compile-fail-tests     # trybuild compile-fail fixtures
cargo nextest run --workspace                   # everything

TRYBUILD=overwrite cargo test -p cgp-compile-fail-tests   # regenerate .stderr snapshots

cargo insta test -p cgp-tests --review          # review snapshot diffs
cargo insta test -p cgp-tests --accept          # accept intended snapshot changes
```

The `cgp-compile-fail-tests` fixtures run under both `cargo test` and `cargo nextest`,
because the `trybuild` driver is an ordinary integration test rather than a doctest.
Include the crate when verifying a change that touches a macro's accepted input or its
expansion, and regenerate its `.stderr` snapshots with `TRYBUILD=overwrite` whenever an
intended change alters a pinned diagnostic — then review the diff before committing.

A snapshot test that fails prints a diff of the generated code; accept it with
`cargo insta` only after confirming the change is intended.

## Migration status

The suite was reorganized from a by-construct layout to this by-concept layout. As
categories grow, keep splitting them per the rule above, and keep expanding failure
coverage in `cgp-macro-tests` and cross-crate coverage in the `cgp-test-crate-*`
packages — these were established with representative cases and are meant to grow.
