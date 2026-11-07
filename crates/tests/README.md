# CGP test suite: the tests organized by CGP concept

This directory holds the test suite for Context-Generic Programming, organized **by
CGP concept**, such as basic delegation, abstract types, implicit arguments, or namespaces,
rather than by the macro that implements each concept, because a single
macro such as `delegate_components!` serves many concepts at once. This README is
the map of what is here and how to run it; [AGENTS.md](AGENTS.md) is the
authoritative guide to the conventions, and you should read it before adding,
moving, or refactoring a test.

**[Website](https://contextgeneric.dev/) · [Reference](https://contextgeneric.dev/docs/reference/) · [Implementation record](https://github.com/contextgeneric/cgp-knowledge-base/blob/main/cgp/implementation/README.md)**

## The crates

The suite is split into two crates, each with a distinct job.

**`cgp-tests`** is the main suite: realistic example code that must compile and
run. Because much of CGP is compile-time wiring, a test here often passes simply
by compiling. It is also where the user-facing macros are exercised end-to-end and
where the canonical macro-expansion snapshots live.

**`cgp-macro-tests`** tests the macro internals directly against `cgp-macro-core`
(the parsers and AST types), and is the home for **rejection cases**, the inputs CGP
refuses during expansion, and for pinning the invalid tokens a macro currently
emits.

**`cgp-no-std-tests`** builds the `cgp` crate for a target without `std`, so a `std` or `alloc` leak
in the dependency tree fails the build. It is `#![no_std]`, uses the macros, and gates an alloc-using
provider behind its `alloc` feature.

A fourth category lives in another repository: the cases where a macro *accepts*
input whose *expansion* then fails to compile are UI fixtures in
[`cargo-cgp`](https://github.com/contextgeneric/cargo-cgp/blob/main/tests/README.md),
so each is pinned as the readable error the tool renders for it. AGENTS.md's
"Adding a failure case" says which of the three a new case belongs in.

## How the tests are laid out

Inside `cgp-tests`, each concept is one **integration test target**, which Cargo
compiles as its own crate, and therefore its own coherence scope. A target is an
entrypoint file `tests/<concept>_tests.rs` plus a module directory
`tests/<concept>/` holding one `.rs` file per unit test, each self-contained so the
type-level wiring of one test never leaks into another. `tests/basic_delegation/`
is the reference example of the layout.

The concept targets currently cover: basic delegation, impl-side dependencies,
implicit arguments, higher-order providers, generic components, abstract types,
getters, field access, extensible records, extensible variants, checking,
dispatching, namespaces, handlers, monadic handlers, async and Send bounds,
blanket traits, and the standalone error backends (`error_backends`). The crate's build script
turns the example in each error backend's README into a test module, which the `readme_*.rs` files
in `error_backends` include, so those examples are tested without being copied.

The library crate's `generated_docs` module denies `missing_docs` while using every macro, so a
generated item that loses its doc line fails the build. The `implicit_arguments` and
`basic_delegation` targets pin where a user's own doc comment lands.

The set grows and subdivides over time. `cgp-macro-tests` follows
the same shape, with `ident_with_type_params` and `shape_macro_parsing` for parser corner cases,
`companion_attrs` for the attributes the data derives copy onto their companion types, and the
failure-case targets `parser_rejections` and `invalid_expansion`.

## Running the tests

```
cargo nextest run -p cgp-tests                  # the main suite
cargo nextest run -p cgp-macro-tests            # macro internals + rejection cases
cargo nextest run --workspace                   # everything

cargo insta test -p cgp-tests --review          # review snapshot diffs interactively
cargo insta test -p cgp-tests --accept          # accept intended snapshot changes
```

The `no_std` check builds for a target whose sysroot has no `std`, so it fails if any crate in the
tree needs one. Install the target once, then run both paths:

```sh
rustup target add x86_64-unknown-none
cargo check -p cgp-no-std-tests --target x86_64-unknown-none
cargo check -p cgp-no-std-tests --features alloc --target x86_64-unknown-none
```

The first proves that `cgp` with default features needs neither `std` nor `alloc`; the second turns on
the `alloc` feature and compiles the provider that needs an allocator. A `std` use planted in the
crate fails the first command with "cannot find module or crate `std`".

Many tests assert the exact code a macro generates, through the `snapshot_*!`
macros from `cgp-macro-test-util`: each emits the real generated code into the
module *and* generates a `#[test]` asserting a pretty-printed inline `insta`
snapshot of it. A failing snapshot prints a diff of the generated code, so accept
it with `cargo insta` only after confirming the change is intended. Which target
owns a given macro's snapshot is a convention AGENTS.md sets out.
