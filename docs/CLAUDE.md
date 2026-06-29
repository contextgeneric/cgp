# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

This directory is the CGP knowledge base — agent-maintained documentation whose job is to record the full semantics of every CGP construct. Read [README.md](README.md) for the background and motivation behind it. The rules below govern how to keep it correct.

## The synchronization rule

**Documentation here must stay in sync with the code, and keeping it in sync is part of the change, not a follow-up.** Whenever you modify a CGP construct — its accepted syntax, the code it expands to, its defaults, its error behavior, or its relationships to other constructs — you must update the matching reference document in the same change. A reference document that describes behavior the code no longer has is actively harmful: the next agent will trust it and be misled. Treat a stale document as a bug in the change that made it stale.

This rule applies in both directions. If you add a new construct, add its reference document and register it in the [reference index](reference/README.md). If you remove a construct, remove or supersede its document and update the index. If you change a construct's expansion, revise the "Expansion" section of its document so the desugaring shown still matches what the macro emits.

The implementation in [crates/macros/cgp-macro-core](../crates/macros/cgp-macro-core), the expansion snapshots in [crates/tests/cgp-macro-tests](../crates/tests/cgp-macro-tests), and the reference documents here are three views of the same truth. When they disagree, that disagreement is a defect. The snapshots are the most mechanical check on whether a document's "Expansion" section is honest — when you doubt what a macro emits, generate or read the snapshot rather than guessing.

## Authoring conventions

Invoke the `/cgp` skill before writing or revising any reference document. The skill is the authoritative source for CGP semantics and terminology; the reference documents must use the same vocabulary (consumer trait, provider trait, provider, wiring, impl-side dependency, and so on) so that a reader moving between the skill and the documents never has to reconcile two dialects.

Write every reference document in the dual-reader prose style (the `/dual-reader-prose` skill). Each document is read both by agents scanning for one specific fact and by agents reading start-to-finish for complete understanding, so every section opens with a self-contained topic sentence that states its point, followed by elaboration. Avoid orphaned bullet lists; frame any list with a sentence before and after. Use code blocks freely — showing the exact expansion is the whole point — but the prose around them must carry the meaning on its own.

Verify against the source before writing, not from memory. Read the construct's implementation in `cgp-macro-core` (its `types/<construct>/` module and the `cgp-macro-lib` entry that drives it) and any tests that exercise it. The "Expansion" section is a claim about generated code; it must reflect what the macro actually produces today, including the real default identifiers (for example, `#[cgp_component]` defaults the context type to `__Context__` and the component name to `{Provider}Component`), not idealized names used for teaching.

## Document structure

Each reference document follows the same shape so readers can navigate any of them by habit. Open with a level-one heading naming the construct and a one-sentence summary of what it is. Then proceed through these sections, using the same headings:

- **Purpose** — the problem the construct solves and why it exists, in prose.
- **Syntax** — the accepted forms of the construct, with the meaning of each argument and option.
- **Expansion** — the exact code the construct desugars to, shown with before/after code blocks. This is the heart of the document and the part most likely to drift; keep it faithful to the current macro output.
- **Examples** — at least one realistic, self-contained example showing the construct in use.
- **Related constructs** — links to the reference documents for constructs commonly used with this one, with a phrase explaining each relationship.
- **Source** — pointers to the implementing modules in `cgp-macro-core` and the relevant tests, so a reader can drop from prose into code.

Place each document in the subdirectory that matches what the construct is. The reference is organized into `macros/` (procedural macros a programmer invokes, including the type-level construction macros), `derives/` (the `#[derive(...)]` family), `attributes/` (modifier attributes consumed by a host macro), `traits/` (runtime capability and mechanism traits), and `types/` (runtime providers and type-level types). The [reference index](reference/README.md) describes the layout in full and is the catalog you register a new document in. Because documents live in different subdirectories, a cross-link between two of them is a relative path — a sibling in the same directory is `name.md`, and a document in another directory is `../that-dir/name.md`.

Cross-link generously. When a document mentions another construct, link to its reference document so a reader can follow the thread. A mention of a construct that is not yet documented is a useful signal of what to write next; record it as a gap in the [reference index](reference/README.md) — under the `traits/` or `types/` pending sections, for example — rather than leaving a dangling link with no home.
