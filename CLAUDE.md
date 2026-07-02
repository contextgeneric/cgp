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

## Macro review workflow

This section defines the standing process for reviewing one CGP macro implementation at a time,
hardening it until no further issue is found. The goal of an iteration is a macro whose
implementation, tests, and documentation are correct, complete, mutually consistent, and as simple
as the behavior allows.

### Orient before touching anything

Load the fundamentals first, every iteration. Invoke the `/cgp` skill to reload CGP's mental model
and vocabulary, and the `/dual-reader-prose` skill to reload the writing convention the docs must
follow. Re-invoke `/cgp` whenever the review moves into an unfamiliar construct — the macros and
core traits are the ground truth the skill describes, so read the two together.

Then read the documentation for the macro under review, in [docs/](docs). Read its reference
document under [docs/reference/](docs/reference), its implementation documents under
[docs/implementation/](docs/implementation) (the `entrypoints/` document, the `asts/` stack it
drives, and any `functions/` helpers it relies on), and the governing `CLAUDE.md` files that define
how those documents stay in sync with the code: [docs/CLAUDE.md](docs/CLAUDE.md),
[docs/implementation/CLAUDE.md](docs/implementation/CLAUDE.md), and
[crates/macros/cgp-macro-core/CLAUDE.md](crates/macros/cgp-macro-core/CLAUDE.md). These establish
that the source is the single source of truth and that reference, implementation, snapshot, and
skill are four views of it that must never drift.

Next, study the implementation itself in [crates/macros/](crates/macros). Start from the
`cgp-macro-lib` entry function, follow it into the `cgp-macro-core` `types/<construct>/` AST stack
and the `functions/` helpers it calls, and read closely enough to reason about corner cases, not
just the happy path. Finally, study the tests in [crates/tests/](crates/tests) — the behavioral
tests in `cgp-tests` and the failure cases and expansion snapshots in `cgp-macro-tests` — and read
[crates/tests/CLAUDE.md](crates/tests/CLAUDE.md) to learn how the suite is organized and how to run
and update it.

### Harden the implementation and its tests

With the macro understood, work through the review in these areas. Each is a distinct concern;
treat correctness as non-negotiable and simplification as a judgment call, and never let a
readability edit introduce a behavioral change.

- **Fix bugs and corner cases.** Identify potential bugs and unhandled corner cases in the
  implementation and fix them. When a corner case cannot be fixed in this iteration, capture it as a
  failure case in `cgp-macro-tests` and record it under the construct's Known issues, per
  [crates/tests/CLAUDE.md](crates/tests/CLAUDE.md).
- **Close test gaps.** Add tests for corner cases that are not yet covered, placing each in the
  concept target that owns the behavior and snapshotting only in the macro's owning target.
- **Verify existing tests.** Confirm each existing test really exercises the behavior it claims to,
  that the corner case it checks makes sense, and that it makes appropriate assertions wherever an
  assertion is possible rather than relying on compilation alone.
- **Deduplicate and simplify tests.** Merge or remove tests that check the same or overlapping
  behavior, and factor common boilerplate into shared test helpers.
- **Improve the documentation and inline docs.** Update the reference document, the implementation
  documents, and any README when they are inconsistent with the code or when something is worth
  explaining or clarifying; add a brief `///` to any public struct, trait, or function that lacks
  one; and simplify existing inline docs, removing facts that are obvious from reading the code.

### Scrutinize the macro codegen

A CGP macro is only as correct as the code it emits, so review the implementation against the ways
its input can be parsed and its output expanded, not just the happy path its tests exercise. Cover
every one of these concerns, since a gap in any of them is a latent miscompilation waiting for the
right input:

- **Review every supported attribute.** Enumerate the attributes the macro accepts and confirm each
  is parsed, validated, mutually constrained, and rejected-when-unknown exactly as documented — an
  unrecognized attribute should fail with a spanned error, a mutually exclusive pair should error
  when both appear, and a duplicate should not be silently accepted (or silently accepted for one
  attribute while rejected for another).
- **Prefer `parse_internal!`/`parse_internal` over `parse2` or `parse_quote!`.** When constructing a
  `syn` node from quasi-quoted tokens, build it with `parse_internal!` so a malformed fragment fails
  with an error naming the target type and the offending tokens (prelude prefix stripped) rather than
  a bare parse error. Reserve `parse2` for re-parsing tokens already known to be valid (a span
  override, say), and treat every `parse_quote!` as an assertion that parsing can never fail.
- **Return `syn::Result` wherever parsing can fail.** A function that parses anything should thread
  `syn::Result` and propagate the error, rather than `parse_quote!`-ing and risking a panic that
  aborts the compiler with no usable diagnostic. Use the panicking `parse_quote!` only when it is
  trivially obvious — from the surrounding, fully-controlled tokens — that the parse cannot fail.
- **Enumerate every way the input can be parsed.** For each parser and `parse_internal!` call, think
  through the full range of inputs a user could write — path-qualified types, generic and lifetime
  parameters, arrays, tuples, empty lists, turbofish, associated-type bindings — and confirm none
  reaches a parser that fails with a confusing internal error. Reject malformed or unsupported input
  early, at the macro's own parse stage with a clear spanned message, rather than letting the failure
  surface deep inside internal fragment parsing or in the expanded code.
- **Enumerate every way the output can expand.** Walk the shapes the expansion can take across the
  whole input space and confirm none can produce invalid Rust — no duplicate or conflicting `impl`
  blocks from a cartesian expansion, no unbound or doubly-declared generic parameter, no empty
  expansion that silently checks nothing, and no clash on a generated identifier.
- **Scrutinize generics with care.** Generic parameters take many forms — lifetimes, types, consts,
  and the distinction between *impl* generics (`impl<T>`) and *type* generics (the `<T>` in
  `Foo<T>`) — and mixing them produces subtly wrong output. Confirm the macro keeps the kinds
  separate, renders each in the right position, merges parameters from different sources without
  colliding, and binds every parameter that appears in the generated header so nothing is left free.
- **Fully qualify every CGP construct in the expansion.** Any CGP item the expansion references must
  be emitted through the `crate::exports` markers so it resolves as `::cgp::macro_prelude::<Name>`,
  never as a bare or hand-written path — this is what lets a user with only `cgp` in scope compile
  the output. Grep the codegen for any CGP name that is not interpolated from an `exports` marker.

Beyond these, weigh the concerns that recur across the macro suite: the hygiene of the reserved
identifiers the expansion introduces (`__Component__`, `__Context__`, and the like), the span
attached to each generated item so a downstream type error points at the token the user actually
wrote, and the idempotency of the expansion when the same entry is listed more than once.

### Keep every view in sync and verify

Every change propagates to all four views in the same change, per the synchronization rule. When
you alter the macro's behavior, syntax, expansion, or defaults, update the reference document's
Expansion, the implementation document's Pipeline and Generated items, the affected snapshots, and
the `/cgp` skill. When you move or rename a test, update the implementation document's Tests or
Snapshots section. Then verify the work: run `cargo +nightly fmt --all`, the clippy invocations, and
`cargo nextest run` for the affected crates (and `cargo insta` to review any snapshot diffs) as
described in the Commands section above, confirming a green suite before considering the iteration
done.

### Ask when in doubt

During the review, ask the user for clarification whenever something should be settled before the
next step is taken — an ambiguous intended behavior, a corner case whose correct outcome is unclear,
or a design choice with more than one defensible answer. Surface the question rather than guessing,
since a wrong assumption baked into the source, tests, and four documentation views is expensive to
unwind.
