# CGP test suite

This directory holds the test suite for Context-Generic Programming, organized **by
CGP concept** — basic delegation, abstract types, implicit arguments, namespaces,
and so on — rather than by the macro that implements each concept, because a single
macro such as `delegate_components!` serves many concepts at once. This README is
the map of what is here and how to run it; [AGENTS.md](AGENTS.md) is the
authoritative guide to the conventions, and you should read it before adding,
moving, or refactoring a test.

## The crates

The suite is split into two crates, each with a distinct job.

**`cgp-tests`** is the main suite: realistic example code that must compile and
run. Because much of CGP is compile-time wiring, a test here often passes simply
by compiling. It is also where the user-facing macros are exercised end-to-end and
where the canonical macro-expansion snapshots live.

**`cgp-macro-tests`** tests the macro internals directly against `cgp-macro-core`
(the parsers and AST types), and is the home for **rejection cases** — inputs CGP
refuses during expansion — and for pinning the invalid tokens a macro currently
emits.

A third category lives in another repository: the cases where a macro *accepts*
input whose *expansion* then fails to compile are UI fixtures in
[`cargo-cgp`](https://github.com/contextgeneric/cargo-cgp/blob/main/tests/README.md),
so each is pinned as the readable error the tool renders for it. AGENTS.md's
"Adding a failure case" says which of the three a new case belongs in.

## How the tests are laid out

Inside `cgp-tests`, each concept is one **integration test target**, which Cargo
compiles as its own crate — and therefore its own coherence scope. A target is an
entrypoint file `tests/<concept>_tests.rs` plus a module directory
`tests/<concept>/` holding one `.rs` file per unit test, each self-contained so the
type-level wiring of one test never leaks into another. `tests/basic_delegation/`
is the reference example of the layout.

The concept targets currently cover: basic delegation, impl-side dependencies,
implicit arguments, higher-order providers, generic components, abstract types,
getters, field access, extensible records, extensible variants, checking,
dispatching, namespaces, handlers, monadic handlers, async and Send bounds, and
blanket traits. The set grows and subdivides over time. `cgp-macro-tests` follows
the same shape, with `ident_with_type_params` for parser corner cases and the
failure-case targets `parser_rejections` and `invalid_expansion`.

## Running the tests

```
cargo nextest run -p cgp-tests                  # the main suite
cargo nextest run -p cgp-macro-tests            # macro internals + rejection cases
cargo nextest run --workspace                   # everything

cargo insta test -p cgp-tests --review          # review snapshot diffs interactively
cargo insta test -p cgp-tests --accept          # accept intended snapshot changes
```

Many tests assert the exact code a macro generates, through the `snapshot_*!`
macros from `cgp-macro-test-util`: each emits the real generated code into the
module *and* generates a `#[test]` asserting a pretty-printed inline `insta`
snapshot of it. So a failing snapshot prints a diff of the generated code — accept
it with `cargo insta` only after confirming the change is intended. Which target
owns a given macro's snapshot is a convention AGENTS.md sets out.
