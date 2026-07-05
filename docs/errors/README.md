# CGP Error Catalog

This directory is a catalog of the compiler errors a programmer encounters when using CGP, organized by the *kind* of error rather than by the macro that produced it. Each document describes one class of failure: the mistake that triggers it, the shape of the diagnostic the Rust compiler prints, whether that diagnostic contains the real root cause, and — when it does — where in the output to find it. The catalog is the fifth section of the knowledge base, alongside [reference/](../reference/README.md), [concepts/](../concepts/README.md), [examples/](../examples/README.md), and [guides/](../guides/README.md).

## Why this exists

A CGP macro expands to ordinary Rust, so many mistakes are not caught by the macro at all — they surface later, when the compiler type-checks the generated code. Those downstream errors are the subject of this catalog. They are shaped by CGP's machinery in ways that make them hard to read: a single mistake can produce a wall of errors naming generated types the user never wrote, and the root cause is often buried or, worse, suppressed entirely. Documenting these classes in one place serves three readers at once.

The first reader is a **tool author**. The long-term goal is specialized tooling — a `cargo-cgp` that post-processes `rustc`'s output into a compact, root-cause-first form, much as Clippy layers its own analysis on top of the compiler. Such a tool needs a complete map of which error classes CGP produces, which of them hide the root cause, and where the root cause sits when it is present, before it can decide what to extract, what to suppress, and how to re-present it. A tool that reaches into compiler internals through `rustc_driver` to recover *suppressed* information especially needs to know, class by class, which information the compiler hides. This catalog is that map.

The second reader is an **agent debugging CGP code**, in this repository or any project that uses CGP. Recognizing an error's class on sight — before decoding a single nested type — tells the agent what kind of mistake to look for and which technique will surface it. The [debugging guide](../guides/debugging.md) is the playbook; this catalog is the reference the playbook indexes into.

The third reader is a **sub-agent extracting an error message on the main agent's behalf**. CGP error output is frequently long enough that reading it wastes a main agent's context window, so the [error-extraction sub-skill](https://github.com/contextgeneric/cgp/blob/main/docs/skills/cgp/references/error-extraction.md) delegates the reading to a sub-agent that returns a compact summary. That summary is only useful if it reports the same facts every document here records — the class, whether the root cause is present, and its position — so the catalog and the sub-skill describe the same anatomy from two directions.

## What belongs here, and what does not

This catalog covers only **compilation failures that occur after CGP codegen** — input a macro *accepts* and lowers to Rust that then fails to type-, trait-, or borrow-check. This is exactly the set of failures pinned by the `trybuild` fixtures in [cgp-compile-fail-tests](../../crates/tests/cgp-compile-fail-tests), and the catalog is the canonical documentation for them: each documented class is backed by a fixture, and the fixtures cross-link back to the class they exercise.

Errors a macro raises by **rejecting its input** do not belong here. When a macro refuses malformed or unsupported input and returns a spanned `syn::Error` during expansion, that diagnostic is a property of the macro's parser, and it stays in the macro's [implementation document](../implementation/README.md) — described there and pinned by an `assert_macro_rejects` case in `cgp-macro-tests`. The dividing line is simple: if the macro produced the error, it is documented with the macro; if the *compiler* produced the error while checking the macro's output, it is documented here.

One nuance follows the [acceptable / problematic split](../implementation/AGENTS.md) that the compile-fail fixtures already use. An **acceptable** failure is one CGP intentionally defers to the compiler because it lacks the whole-program view the check needs; its documentation lives here in full. A **problematic** failure is a CGP *defect* — input a macro should have rejected, or an expansion that emits invalid Rust; the observable error a user hits is cataloged here like any other class, but the explanation of *why it is a defect and what the fix would be* stays under the owning macro's `## Known issues`, cross-linked both ways.

## The central axis: hidden versus surfaced

The single most important distinction in this catalog is whether the compiler **surfaces** or **hides** the root cause of an unsatisfied dependency, because the two produce completely different diagnostics from the very same mistake. CGP wiring is resolved lazily, so a provider whose impl-side dependency the context cannot meet is wired without complaint and only fails when the wiring is exercised. *How* it is exercised decides what the user sees.

When the failure is forced through a **check trait** — `check_components!` asserting `CanUseComponent`, which walks through [`IsProviderFor`](../reference/traits/is_provider_for.md) — the compiler evaluates the provider's `where` clause and reports the real unmet bound, naming the missing `HasField`, abstract type, or transitive dependency. The root cause is **surfaced** — the concrete missing bound is named in the diagnostic (for a single-component check, in the compiler's `help:` note), and a `required for …` note chain traces the dependency path from it back to the check. These errors are cataloged under [checks/](checks/).

When the same broken wiring is instead exercised by **calling the consumer-trait method directly** on the context, the compiler sees a blanket impl of the consumer trait alongside the other candidate impls, cannot commit to one, and falls back to a heuristic that reports only "the method exists but its trait bounds were not satisfied" — naming the consumer and provider traits but *not* descending into the dependency that actually failed. The root cause is **hidden**: it is absent from the output, not merely buried. Because these errors report nothing about the true cause, mixing them with the surfaced classes would mislead a reader into looking for a root cause that is not there. They are isolated under [hidden/](hidden/), and recovering their root cause requires either re-checking the wiring to promote the error into a surfaced one, or the kind of compiler-internal introspection a `cargo-cgp` tool would perform.

## The verbosity problem: many errors, one cause

A second recurring difficulty is volume: a single mistake deep in a dependency graph surfaces at *every* provider that transitively needs it, so the compiler prints one error per affected provider even though there is only one thing to fix. If a `FooProvider` needs a `foo` field, a `BarProvider` depends on `Foo`, and a `BazProvider` depends on `Bar`, then a missing `foo` field produces three separate failures naming `FooProvider`, `BarProvider`, and `BazProvider`. The count of errors reflects the depth of the graph, not the number of mistakes.

Because of this, **the documents here never record verbatim error output.** Reproducing a multi-screen cascade would bloat the catalog and rot the moment a diagnostic shifts. Instead each document summarizes the *kind* of message a class produces, states whether the root cause appears anywhere in it, and pins the *position* where it appears when it does — very often near the last or second-to-last error in a cascade, since the innermost failing bound is reported after the outer ones. Recording the position is what lets a tool, or an agent, jump to the cause instead of reading the noise. The [authoring conventions](AGENTS.md) make this rule precise.

## Organization

The catalog is divided into three subdirectories by the axis above, so a reader lands in the right class before decoding any type. Each document is registered in the catalog below in the same change that adds it.

The [hidden/](hidden/) directory holds the classes where the compiler **suppresses** the root cause — the errors a user meets by exercising broken wiring through a consumer trait rather than a check. These are isolated precisely because their diagnostics report nothing about the true cause, so a reader must know not to look for one.

The [checks/](checks/) directory holds the classes where a check trait **surfaces** the root cause through `IsProviderFor`, and the classes that are dominated by the *volume* of a surfaced cascade rather than by any single message.

The [wiring/](wiring/) directory holds the whole-program **structural** failures — coherence conflicts, orphan-rule violations, wiring cycles, and unconstrained generics — where the compiler reports a definite error code (`E0119`, `E0210`, `E0275`, `E0207`) and the difficulty is mapping that code back to the wiring mistake rather than a hidden or cascading cause.

## Catalog

This section indexes every error document, grouped by the subdirectory it lives in.

Hidden-cause errors — [hidden/](hidden/):

- [Unsatisfied dependency (hidden)](hidden/unsatisfied-dependency.md) — a provider's impl-side dependency is unmet, and the failure is triggered by a direct consumer-trait method call, so the compiler reports only that the method's bounds are unsatisfied (`E0599`/`E0277`) and hides the missing dependency.

Surfaced and cascading errors — [checks/](checks/):

- [Check-trait failure (surfaced)](checks/check-trait-failure.md) — the same unmet dependency forced through `check_components!`, where `IsProviderFor` surfaces the concrete missing bound at the wiring site.
- [Verbose dependency cascade](checks/verbose-cascade.md) — one deep mistake reported at every transitively dependent provider, and how to locate the single root cause among the repeats.
- [Unregistered namespace path](checks/unregistered-namespace-path.md) — a component routed through a joined namespace to a path that no entry ever binds, so the *lookup* finds no delegate; a check surfaces it as an `E0277` on the path-keyed `DefaultNamespace`/`DelegateComponent` bound.

Structural wiring errors — [wiring/](wiring/):

- [Conflicting wiring](wiring/conflicting-wiring.md) — the same key or name wired twice, producing coherence (`E0119`) or duplicate-definition (`E0428`) errors.
- [Orphan-rule violation](wiring/orphan-rule.md) — a generated impl for a foreign trait and a fully foreign type (`E0210`, or `E0117`), as when a prefixed `#[default_impl]` is registered from the wrong crate.
- [Wiring cycle](wiring/wiring-cycle.md) — a delegation that chases its own tail: an `E0275` overflow when forced through a check, but the hidden `E0599` when reached by a plain method call.
- [Namespace inheritance cycle](wiring/namespace-inheritance-cycle.md) — namespaces whose parent chain loops (`A: B`, `B: A`, or `A: A`), an `E0275` overflow caught *eagerly at the `cgp_namespace!` definitions* rather than lazily at a use site.
- [Unconstrained generic](wiring/unconstrained-generic.md) — a per-entry generic that never reaches the key, leaving an impl parameter unconstrained (`E0207`).

## Relationship to the rest of the knowledge base

This catalog is one of the views of CGP's truth and is bound by the [synchronization rule](../AGENTS.md): a class whose diagnostic the code no longer produces is a bug in the change that made it stale. It leans on the other sections rather than restating them. It links to the [reference](../reference/README.md) for the traits and macros a class involves, to the [check-traits concept](../concepts/check-traits.md) for why wiring is lazy, and above all to the [debugging guide](../guides/debugging.md), which is the prescriptive playbook this reference-style catalog supports. The rules for authoring and maintaining these documents — including how they stay in sync with the compile-fail fixtures and how a sub-agent extracts the errors they describe — live in [AGENTS.md](AGENTS.md).
