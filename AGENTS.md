# AGENTS.md

This file provides guidance to LLM agents when working with code in this repository.

## Project status

This working tree **is** the upcoming **v0.8.0** release. Every crate under [crates/](crates) is
versioned `0.8.0-alpha` in its `Cargo.toml` — the pre-release of v0.8.0 — and the documentation in the
[knowledge base](https://github.com/contextgeneric/cgp-knowledge-base/tree/main/cgp) already refers to
the current version as v0.8.0, so nothing needs a version edit when v0.8.0 ships.

The stable release on crates.io is still **v0.7.0**, and it is not compatible with this tree. v0.7.0
carries legacy features and syntax that v0.8.0 has removed or changed, so code or documentation
written against v0.7.0 will not compile here. Treat crates.io v0.7.0 as a separate, older line and
do not reconcile this tree's syntax against it.

Only the library crates are published. The five crates under [crates/tests/](crates/tests) are
marked `publish = false`, so `cargo publish --workspace` skips them and publishes just the real CGP
crates.

## The sibling projects

`cgp` is one repository in a family that is developed together, so a task here routinely reads — and
often updates — another one. [sibling-projects.md](sibling-projects.md) is the authoritative list:
where each project lives, which revision of it to read, and the rules for finding a sibling locally
versus linking to it. Three of them matter constantly.

**This repository's documentation lives in [`cgp-knowledge-base`](https://github.com/contextgeneric/cgp-knowledge-base).**
There is no `docs/` directory here. The construct reference, the concepts, the guides, the error
catalog, and the macro-implementation notes are all in that repository's
[`cgp/`](https://github.com/contextgeneric/cgp-knowledge-base/tree/main/cgp) directory, and the worked
examples sit at its top level. It is a *consolidated* base — `cargo-cgp`'s documentation is the
neighbouring section — which is why an error class here and the tool's handling of it are now one
directory apart. Treat the knowledge base as part of this repository's surface: **a change to a
construct's behavior carries its documentation change**, and when the checkout is missing, say plainly
what needs updating there.

**[`cargo-cgp`](https://github.com/contextgeneric/cargo-cgp) is CGP's first-class toolchain**: a cargo
subcommand that rewrites the compiler's diagnostics into readable, root-cause-first CGP errors. It is
the recommended way to build and check CGP code and to read a CGP compile error, so recommend it
wherever CGP errors are discussed, and prefer it over raw `cargo check` when diagnosing a wiring
failure here. When a construct change alters a diagnostic it has a UI fixture for, update the fixture
in the same change, following that project's own conventions.

**[`cgp-skills`](https://github.com/contextgeneric/cgp-skills) holds the `/cgp` skill**, the distilled
view of the knowledge base that every agent loads first. It is the easiest view to leave stale, so a
change to a construct's syntax, expansion, defaults, or recommended form propagates out to the matching
sub-skill in the same change.

**Finding a sibling.** Look first in the parent directory, at `../cargo-cgp`,
`../cgp-knowledge-base`, `../cgp-skills`. When the checkout is present, use it — it is the fastest
reference and reflects any uncommitted work. When it is absent, fetch the file you need from the
project's repository at the revision [sibling-projects.md](sibling-projects.md) records.

**Writing cross-project links.** A *link* from a committed file here to a document in another
project — a Markdown link or a URL — is **always written as a GitHub URL** on `main`
(`https://github.com/contextgeneric/<project>/blob/main/<path>`), never as a relative
`../cargo-cgp/...` link, so it resolves for a reader who has only this repository checked out. When
*reading* such a link yourself, prefer the local checkout. A bare mention of a checkout's location,
like the path `../cargo-cgp`, is a filesystem reference rather than a link and stays relative — as is
a doc pointer in a source comment, which names its knowledge-base path
(`cgp-knowledge-base/cgp/implementation/entrypoints/cgp_impl.md`).

**The code dependency stays one-way.** No `cgp` *crate* may depend on a `cargo-cgp` crate:
`cargo-cgp` reads `cgp`, never the reverse. Documentation is the deliberate exception — the knowledge
base and the skill both reference `cargo-cgp` as the recommended toolchain — so it is the *code* graph
that stays acyclic, not the documentation. The post-codegen compile-fail cases this repository used to
keep in `cgp-compile-fail-tests` now live as `cargo-cgp` UI fixtures, and the
[error catalog](https://github.com/contextgeneric/cgp-knowledge-base/blob/main/cgp/errors/README.md)
links each error class to the fixture that backs it.

## Orient before any task

This repository **is** the implementation of Context-Generic Programming (CGP), and its behavior is
recorded as much in the [knowledge base](https://github.com/contextgeneric/cgp-knowledge-base/tree/main/cgp)
as in the code. Before starting any task here — reading, writing, reviewing, debugging, or answering a
question — load the CGP mental model and the documentation that covers what you are about to touch.
The following steps are standing requirements: they apply to every task regardless of how small it
looks, not just to the macro review workflow below. Prefer the local `../cgp-knowledge-base` checkout
when reading them, and note that its [summary.md](https://github.com/contextgeneric/cgp-knowledge-base/blob/main/summary.md)
lists every document in one page, which is the fastest way to find the few a task needs.

- **Always invoke the `/cgp` skill** to load the fundamentals (consumer vs. provider traits,
  `#[cgp_component]`/`#[cgp_impl]`/`#[cgp_fn]`, `delegate_components!`, `HasField`, `UseDelegate`,
  check traits, and so on). Re-invoke it whenever you move into an unfamiliar construct — the macros
  and core traits here are the ground truth the skill describes, so read the two together.
- **Always read [cgp-knowledge-base/cgp/README.md](https://github.com/contextgeneric/cgp-knowledge-base/blob/main/cgp/README.md)** to orient in the knowledge base, then follow it
  into the README of whichever section covers your task.
- **Read [cgp-knowledge-base/cgp/reference/README.md](https://github.com/contextgeneric/cgp-knowledge-base/blob/main/cgp/reference/README.md) and the relevant reference documents
  whenever the task requires understanding a CGP construct** — what it means, what syntax it accepts,
  and what code it expands to.
- **Read [cgp-knowledge-base/cgp/implementation/README.md](https://github.com/contextgeneric/cgp-knowledge-base/blob/main/cgp/implementation/README.md) and the relevant
  implementation documents whenever the task involves reading or modifying the CGP source code** —
  they map each macro to its `cgp-macro-core`/`cgp-macro-lib` internals, corner cases, and tests.
- **Load the `/dual-reader-prose` skill whenever the task involves editing markdown documentation or
  inline code comments**, and follow its writing convention for any prose you add.

The canonical export surface for users is `cgp::prelude` — see
[crates/main/cgp/src/prelude.rs](crates/main/cgp/src/prelude.rs), which re-exports
`cgp_core::prelude` + `cgp_extra::prelude`. When unsure what a name resolves to, start from the
prelude re-exports in [crates/main/cgp-core/src/prelude.rs](crates/main/cgp-core/src/prelude.rs).

## Commands

This is a Cargo workspace (edition 2024, resolver 3). Toolchain is pinned to **1.96** via
[rust-toolchain.toml](rust-toolchain.toml). Nearly every crate is `#![no_std]` — keep new code
`no_std`-compatible (use `core`/`alloc`, gate `std`/`alloc` usage behind features as existing crates
do).

- **Format** (requires nightly — `.rustfmt.toml` uses unstable `group_imports`/`imports_granularity`):
  `cargo +nightly fmt --all` (check: `cargo +nightly fmt --all -- --check`)
- **Lint:** `cargo clippy --all-features --all-targets -- -D warnings`
  and `cargo clippy --no-default-features --all-targets -- -D warnings`
- **Test** (uses `cargo-nextest`): `cargo nextest run --all-features --no-fail-fast --workspace`.
  This runs the whole suite: `cgp-tests` (behavior + expansion snapshots) and `cgp-macro-tests`
  (macro internals, rejection cases, invalid-expansion snapshots).
- **Single test crate / test:** `cargo nextest run -p cgp-tests` or target one file with the
  standard test harness, e.g. `cargo test -p cgp-tests --test component`
- **Post-codegen compile failures** (input a macro accepts but whose expansion fails to compile) are
  no longer tested here: they are UI fixtures in [`cargo-cgp`](https://github.com/contextgeneric/cargo-cgp),
  the first-class error toolchain, and cataloged in the [error catalog](https://github.com/contextgeneric/cgp-knowledge-base/blob/main/cgp/errors/README.md). See
  [crates/tests/AGENTS.md](crates/tests/AGENTS.md) for the workflow.
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

- All versions are kept in lockstep at the workspace level (currently **0.8.0-alpha**); inter-crate
  dependencies are declared once in the root [Cargo.toml](Cargo.toml) `[workspace.dependencies]`
  and referenced with `{ workspace = true }`. Add new crates to the `members` list and the
  workspace dependency table together. A crate that exists only for testing also gets
  `publish = false` so `cargo publish --workspace` excludes it from the published release.
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

Perform the standing steps in [Orient before any task](#orient-before-any-task) first, every
iteration. Then read the documentation specific to the macro under review, in the [knowledge base](https://github.com/contextgeneric/cgp-knowledge-base/tree/main/cgp): its
reference document under [cgp-knowledge-base/cgp/reference/](https://github.com/contextgeneric/cgp-knowledge-base/tree/main/cgp/reference), its implementation documents under
[cgp-knowledge-base/cgp/implementation/](https://github.com/contextgeneric/cgp-knowledge-base/tree/main/cgp/implementation) (the `entrypoints/` document, the `asts/` stack it
drives, and any `functions/` helpers it relies on), and the governing `AGENTS.md` files that define
how those documents stay in sync with the code: [cgp-knowledge-base/cgp/AGENTS.md](https://github.com/contextgeneric/cgp-knowledge-base/blob/main/cgp/AGENTS.md),
[cgp-knowledge-base/cgp/implementation/AGENTS.md](https://github.com/contextgeneric/cgp-knowledge-base/blob/main/cgp/implementation/AGENTS.md), and
[crates/macros/cgp-macro-core/AGENTS.md](crates/macros/cgp-macro-core/AGENTS.md). These establish
that the source is the single source of truth and that reference, implementation, snapshot, and
skill are four views of it that must never drift.

Next, study the implementation itself in [crates/macros/](crates/macros). Start from the
`cgp-macro-lib` entry function, follow it into the `cgp-macro-core` `types/<construct>/` AST stack
and the `functions/` helpers it calls, and read closely enough to reason about corner cases, not
just the happy path. Finally, study the tests in [crates/tests/](crates/tests) — the behavioral
tests in `cgp-tests` and the failure cases and expansion snapshots in `cgp-macro-tests` — and read
[crates/tests/AGENTS.md](crates/tests/AGENTS.md) to learn how the suite is organized and how to run
and update it.

### Harden the implementation and its tests

With the macro understood, work through the review in these areas. Each is a distinct concern;
treat correctness as non-negotiable and simplification as a judgment call, and never let a
readability edit introduce a behavioral change.

- **Fix bugs and corner cases.** Identify potential bugs and unhandled corner cases in the
  implementation and fix them. When a corner case cannot be fixed in this iteration, capture it as a
  failure case in `cgp-macro-tests` and record it under the construct's Known issues, per
  [crates/tests/AGENTS.md](crates/tests/AGENTS.md).
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

A CGP macro is only as correct as the code it emits, so review it against the full space of inputs
it can parse and outputs it can expand, not the happy path its tests exercise. A gap in any area
below is a latent miscompilation — or a broken editor experience — waiting for the right input.
**Apply every area to every macro you review**, not only the ones that already have a fixture or a
worked example for it; these concerns recur across the whole suite, and a macro that has never been
audited for one of them (spans especially) most likely has the bug. The areas below are the *what to
check*; the [cross-cutting implementation notes](https://github.com/contextgeneric/cgp-knowledge-base/blob/main/cgp/implementation/README.md#cross-cutting-implementation-notes)
explain *why the code behaves this way*, and most areas link out to the note or process doc that
carries their full detail.

#### Attributes

Enumerate the attributes the macro accepts and confirm each is parsed, validated, mutually
constrained, and rejected when unknown: a bad attribute should fail with a spanned error, a mutually
exclusive pair should error when both appear, and a duplicate should not be silently accepted (nor
accepted for one attribute while rejected for another).

#### Parsing the input

Build every `syn` node from quasi-quoted tokens with
[`parse_internal!`](https://github.com/contextgeneric/cgp-knowledge-base/blob/main/cgp/implementation/macros/parse_internal.md) rather than `parse2`/`parse_quote!`,
and thread `syn::Result` so a malformed fragment propagates a named error instead of panicking;
reserve the panicking `parse_quote!` for tokens trivially guaranteed to parse and `parse2` for
re-parsing tokens already known valid. Reject malformed or unsupported input early, at the macro's
own parse stage with a spanned message, rather than letting it surface deep inside fragment parsing.
Walk the full input space — path-qualified types, generics, lifetimes, arrays, tuples, turbofish,
associated-type bindings — and confirm none reaches a parser that fails obscurely; since `syn` parses
far more leniently than Rust accepts, validate against CGP's restricted argument types. See
[Parsing](https://github.com/contextgeneric/cgp-knowledge-base/blob/main/cgp/implementation/README.md#parsing-build-with-parse_internal-and-distrust-syns-leniency).

#### Expanding the output

Walk the shapes the expansion can take across the whole input space and confirm none produces invalid
Rust — no conflicting `impl` blocks from a cartesian expansion, no unbound or doubly-declared generic,
no empty expansion that checks nothing, no clash on a generated identifier. When an expansion fails to
compile, capture it as a [`cargo-cgp` UI fixture](https://github.com/contextgeneric/cargo-cgp/blob/main/tests/README.md)
(post-codegen compile-fail cases live there now, filed by the quality of the output the tool produces)
and catalog the class in the [error catalog](https://github.com/contextgeneric/cgp-knowledge-base/blob/main/cgp/errors/README.md); when the failure is a CGP *defect*
the macro should have rejected, also record it under the macro's `## Known issues`, per
[crates/tests/AGENTS.md](crates/tests/AGENTS.md) and [cgp-knowledge-base/cgp/implementation/AGENTS.md](https://github.com/contextgeneric/cgp-knowledge-base/blob/main/cgp/implementation/AGENTS.md).

#### Generics

Keep the *kinds* (lifetime, type, const) and the *roles* (impl generics `impl<T>` versus type
generics, the `<T>` in `Foo<T>`) distinct, render each in the right position, merge parameters from
different sources without colliding, and bind every parameter that appears in a generated header so
nothing is left free. See
[Generic-parameter insertion](https://github.com/contextgeneric/cgp-knowledge-base/blob/main/cgp/implementation/README.md#generic-parameter-insertion-and-lifetime-ordering)
and [keeping the kinds distinct](https://github.com/contextgeneric/cgp-knowledge-base/blob/main/cgp/implementation/README.md#generics-keep-the-kinds-and-roles-distinct).

#### Qualified paths and hygiene

Emit every CGP item through a `crate::exports` marker so it resolves as `::cgp::macro_prelude::<Name>`,
never as a bare or hand-written path — grep the codegen for any CGP name not interpolated from an
`exports` marker. Give the reserved identifiers the expansion introduces the double-underscore form
(`__Context__`, `__Provider__`, `__Component__`, …) so they cannot clash with a user's names, and keep
the expansion idempotent when the same entry is listed more than once. See
[Hygiene](https://github.com/contextgeneric/cgp-knowledge-base/blob/main/cgp/implementation/README.md#hygiene-exports-markers-and-reserved-identifiers).

#### Spans: for the compiler *and* the IDE

Span placement is a correctness concern for two tools at once, and it applies to **every macro that
emits items** — `#[cgp_impl]`, `#[cgp_provider]`, `#[cgp_new_provider]`, `#[cgp_type]`,
`#[cgp_getter]`, and the derives — not just `delegate_components!`/`check_components!`. By default
`parse_internal!`/`quote!` stamp generated tokens with the macro's `call_site` span (the whole
invocation), which misleads both tools, so confirm both for each macro:

- **Compiler carets.** An error on a generated item's header — a coherence conflict (`E0119`), an
  unsatisfied bound, a name-resolution failure — must point at the entry the user wrote, not the whole
  macro block. Re-span each generated item onto its originating token: with
  [`override_item_span`](crates/macros/cgp-macro-core/src/functions/override_span.rs) for a generated
  impl, a carried span field where the origin token is synthesized (`EvaluatedCheckEntry.span`,
  `EvaluatedDelegateEntry.span`), or reuse of the user's own `for` token.
- **IDE go-to-definition.** rust-analyzer maps a source token to its expansion by source range alone,
  ignoring hygiene, so never drag a resolvable reference (`IsProviderFor`, `DelegateComponent`) onto a
  user token's range, and span a *derived name* (a component marker) on the user identifier it derives
  from, not `Span::call_site()`. A leaked derived name makes go-to-definition on a reference offer the
  defining macro alongside the item.

Only the caret half is pinned by a test (a `trybuild` `.stderr` fixture records each caret's exact
position, so a regression changes the snapshot). The IDE half has no fixture and must be checked by
hand in an editor — and cross-crate, since the derived-name leak is invisible from inside the defining
crate. See [Spans](https://github.com/contextgeneric/cgp-knowledge-base/blob/main/cgp/implementation/README.md#spans-aim-generated-items-at-the-token-the-user-wrote)
for the full mechanism and the `delegate_components!`/`#[cgp_impl]` worked examples.

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
