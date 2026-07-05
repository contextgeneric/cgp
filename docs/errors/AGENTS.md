# AGENTS.md — the CGP error catalog

This directory catalogs the compiler errors CGP produces *after* codegen, organized by the kind of error. Read [README.md](README.md) for what the catalog is and the hidden-versus-surfaced axis it is built around. The rules below govern how to keep these documents correct and useful, and they assume you have read the knowledge-base-wide rules in [../AGENTS.md](../AGENTS.md); this file adds the rules specific to the error catalog.

Invoke the `/cgp` skill before writing or revising any document here, and write in the dual-reader prose style (the `/dual-reader-prose` skill). Each document is read both by an agent scanning for one error class and by a tool author reading a class start to finish, so every section opens with a self-contained topic sentence and frames any list with a sentence before and after.

## What these documents are for

An error document records the *anatomy* of one class of post-codegen compile failure, so that a tool author, a debugging agent, or an error-extraction sub-agent can recognize the class and act on it without re-deriving it from a raw error dump. Where the [debugging guide](../guides/debugging.md) is prescriptive — "trace it like this" — an error document is descriptive: it states what the mistake is, what the compiler prints, whether the root cause is in that output, and where. It is the reference the guide and the tooling index into.

The catalog is the **canonical documentation for the post-codegen compile-fail fixtures** in [cgp-compile-fail-tests](../../crates/tests/cgp-compile-fail-tests). A macro's [implementation document](../implementation/README.md) still owns the failures the macro raises by *rejecting* its input (the `assert_macro_rejects` cases in `cgp-macro-tests`), but a failure that lands on the *compiler* checking the macro's output is documented here, and its fixture cross-links here. See [What belongs here](README.md#what-belongs-here-and-what-does-not) for the dividing line.

## The synchronization rule applies here too

An error document must stay in sync with the code and its fixtures, and keeping it in sync is part of the change. The source of truth is the diagnostic the pinned toolchain actually emits, captured in a fixture's `.stderr`. When a change alters what the compiler prints for a class — a new error code, a root cause that moves from hidden to surfaced, a caret that lands elsewhere — revise the matching document in the same change; when you add, move, or reclassify a compile-fail fixture, update the document that catalogs its class so the catalog stays the canonical index. Verify a claim against a real compilation, not memory: regenerate the fixture's `.stderr` with `TRYBUILD=overwrite` (see [crates/tests/AGENTS.md](../../crates/tests/AGENTS.md)) and read what the compiler prints before you describe it.

Document the present, not the history, following [../AGENTS.md](../AGENTS.md): describe the diagnostic as it is emitted today, and delete superseded wording outright rather than leaving "previously" traces.

## Never record verbatim error output

The defining rule of this catalog is that **a document summarizes the kind of error a class produces; it never pastes the verbatim output.** CGP errors are long — a single mistake in a deep dependency graph can print one failure per transitively dependent provider — and a pasted cascade would bloat the document and rot the instant a diagnostic shifts. Reproduce only the short *fragment* that carries a point (the one line naming the failing trait, or the shape of the near-contradiction between an error and its `help:` note), the way an implementation document quotes a slice of an expansion rather than the whole file.

In place of the output, every document must record three facts, because these are what a tool and a debugging agent actually need:

- **The kind of diagnostic** — the error code or codes, the trait or traits named (`IsProviderFor`, `DelegateComponent`, `CanUseComponent`, a consumer trait), and the recognizable shape (a lone `E0599` "method exists but bounds unsatisfied", an `E0277` note chain, an error immediately contradicted by a `help:` note, a repeated per-provider cascade).
- **Whether the root cause is present** — surfaced somewhere in the output, or hidden by the compiler's heuristics and absent entirely. This is the field that decides which subdirectory the class lives in and whether a reader should even look for a cause.
- **Where the root cause sits when present** — which note carries it, and its position in the output. Record when it is near the last or second-to-last message of a cascade, when it hides behind an elided `...` whose full form is written to the `long-type-….txt` file the compiler names, and any reliable landmark that lets a reader or tool jump to it.

## Document structure

Each error document follows the same shape so a reader can navigate any of them by habit. Open with a level-one heading naming the class and a one-sentence summary of the mistake and the error code it produces. Then proceed through these sections, using the same headings:

- **What triggers it** — the CGP mistake that produces the class, with a short, self-contained code snippet of the failing input. Name the construct involved and link to its reference.
- **The diagnostic** — the *kind* of error the compiler emits, per the rule above: the code(s), the trait(s) named, and the shape. State plainly whether this is a hidden or a surfaced class.
- **Where the root cause is** — whether the true cause appears in the output and, if so, its position; if hidden, say so and explain briefly *why* the compiler suppresses it (the blanket-impl-plus-candidates heuristic, an elided type, cascade depth).
- **Resolving it** — the fix for the underlying mistake, and — for a hidden class — how to *promote* the error into a surfaced one (typically a `check_components!` at the wiring site) so the cause becomes visible.
- **Notes for tooling** — what a `cargo-cgp`-style post-processor should do for this class: which fragment to extract as the headline, what to suppress as noise, and whether recovering the cause needs compiler-internal introspection (`rustc_driver`) because the ordinary output hides it. Omit the heading when a class needs no special tool handling beyond the general rules.
- **Backing fixtures** — a bullet list of the [cgp-compile-fail-tests](../../crates/tests/cgp-compile-fail-tests) fixtures that pin this class, each linked with a one-line note of what it exercises. This is where the catalog's test pointers live; every documented class must have at least one fixture.
- **Related** — links to the relevant reference documents, the [debugging guide](../guides/debugging.md) section that handles the class, the sibling error classes it contrasts with, and — for a problematic/defect class — the owning macro's `## Known issues`.

## Backing every class with a fixture, and organizing the fixtures

Every error class must be backed by at least one compile-fail fixture, and reciprocally every post-codegen compile-fail fixture must be cataloged by a class here. When you write a class that has no fixture yet, add one under [cgp-compile-fail-tests](../../crates/tests/cgp-compile-fail-tests) following [crates/tests/AGENTS.md](../../crates/tests/AGENTS.md): place it under `acceptable/` or `problematic/` by the split above, write a header comment stating what it exercises and why it must not compile, and cross-link the fixture's header to the class document here (and, for a problematic case, to the owning macro's Known issues as well). Regenerate the `.stderr` and review it before committing.

Because the catalog is expected to grow a large number of fixtures, **keep each fixture directory small — no more than roughly a dozen cases — and split into further nested subdirectories when one grows past that.** The compile-fail driver globs the tree with `**`, so a new nesting level needs no registration. This mirrors the "split a category before it sprawls" rule the main suite already follows; the difference is only that the compile-fail tree groups by the failing macro's expansion rather than by concept, so a nested split subdivides within an owning macro's directory.

## Handling the acceptable / problematic / rejection split

A class's home is decided by the same three-way split the fixtures use, and getting it right keeps the catalog from either duplicating or losing information. An **acceptable** failure — one CGP intentionally defers to the compiler — is documented here in full; nothing about it belongs in a macro's Known issues, because it is not a defect. A **problematic** failure — a CGP defect — is cataloged here for the *observable error a user hits*, but the explanation of why it is a defect and what the correct behavior would be stays under the owning macro's `## Known issues`; link the two so neither drifts, and when the defect is fixed, update both in the same change. A failure the macro catches by **rejecting its input** is not cataloged here at all — it stays with the macro, per [What belongs here](README.md#what-belongs-here-and-what-does-not).

## Gathering an error with a sub-agent

Writing or verifying a document here means reading real compiler output, and that output is often long enough to waste the context of the agent doing the writing. Delegate the reading. The [error-extraction sub-skill](../skills/cgp/references/error-extraction.md) defines how a sub-agent compiles a fixture or a scratch reproduction, captures the diagnostic, and returns only the compact anatomy this catalog records — the class, whether the root cause is present, and its position — rather than the raw dump. Spawn a sub-agent with that skill to gather the facts for a class, then write the document from its summary. The same delegation applies in an ordinary debugging session: when a CGP error is too long to read inline, hand it to a sub-agent and act on the summary. Keeping the extraction technique in the skill and the extracted facts in the catalog means the two describe the same anatomy from two directions.
