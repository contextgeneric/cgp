# CGP test suite

This directory holds the test suite for Context-Generic Programming. The tests
are organized **by CGP concept** — basic delegation, abstract types, implicit
arguments, namespaces, and so on — rather than by the macro that implements each
concept, because a single macro (for example `delegate_components!`) serves many
concepts at once. If you are maintaining or extending the suite, read
[CLAUDE.md](CLAUDE.md) first; it is the authoritative guide to the conventions.
This README is the map.

## The crates

The suite is split into three kinds of crate, each with a distinct job.

**`cgp-tests`** is the main suite: realistic example code that must compile and
run. Because much of CGP is compile-time wiring, a test here often passes simply
by compiling. It is also where the user-facing macros are exercised end-to-end and
where the canonical macro-expansion snapshots live.

**`cgp-macro-tests`** tests the macro internals directly against `cgp-macro-core`
(the parsers and AST types), and is the home for **failure cases** — inputs CGP
should reject, and cases where a macro currently emits invalid code.

**`cgp-test-crate-a`** and **`cgp-test-crate-b`** are auxiliary packages for
**cross-crate** behavior. Crate A defines components, a provider, and a namespaced
component; crate B (a downstream crate) wires them, supplies its own provider for a
foreign component, and participates in crate A's namespace — showing that CGP stays
within Rust's coherence and orphan rules across crate boundaries.

## How the tests are laid out

Inside `cgp-tests`, each concept is one **integration test target**, which Cargo
compiles as its own crate (its own coherence scope). A target is an entrypoint file
`tests/<concept>_tests.rs` plus a module directory `tests/<concept>/` holding one
`.rs` file per unit test. Each unit-test file is self-contained: it defines its own
components, providers, and context types at module scope, so the type-level wiring
of one test never leaks into another. `tests/basic_delegation/` is the reference
example of this layout.

The concept targets currently cover: basic delegation, impl-side dependencies,
implicit arguments, higher-order providers, generic components, abstract types,
getters, field access, extensible records, extensible variants, checking,
dispatching, namespaces, handlers, monadic handlers, async and Send bounds, and
blanket traits. This set grows and subdivides over time — when a concept
accumulates too many cases to stay coherent, it is split into finer targets.

`cgp-macro-tests` follows the same target/`_tests.rs` shape: `ident_with_type_params`
for parser corner cases, and the failure-case targets `parser_rejections` and
`invalid_expansion`.

## Snapshots

Many tests assert the exact code a macro generates, using the `snapshot_*!` macros
from `cgp-macro-test-util`. Each such macro emits the real generated code into the
module **and** generates a `#[test]` asserting a pretty-printed inline `insta`
snapshot of it. Snapshots are used deliberately: a macro's expansion is snapshotted
only in the concept target that owns that macro's feature, and written plainly
everywhere else (see [CLAUDE.md](CLAUDE.md) for the ownership rules).

## Running the tests

```
cargo nextest run -p cgp-tests            # the main suite
cargo nextest run -p cgp-macro-tests      # macro internals + failure cases
cargo nextest run --workspace             # everything, including the aux crates

cargo insta test -p cgp-tests --review    # review snapshot diffs interactively
cargo insta test -p cgp-tests --accept    # accept intended snapshot changes
```

When a snapshot test fails it prints a diff of the generated code; accept the new
output with `cargo insta` only after confirming the change is intended.
