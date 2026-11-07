# AGENTS.md

Read this file before working in this repository. The
[knowledge base's AGENTS.md](https://github.com/contextgeneric/cgp-knowledge-base/blob/main/AGENTS.md)
carries the canonical wording of the universal rules. Change a universal rule there first, then
update this block to match.

## Universal rules

- Prefer the local sibling checkout when reading another project.
- Write a cross-project link as a GitHub URL on `main`.
- Name official release versions only, never a pre-release.
- Commit only when the user explicitly asks, on the current branch.
- Verify a claim against the source before writing it.
- Keep every view of the same truth in sync in the same change.
- Ask the user when a decision is ambiguous.
- Apply point-first writing to internal prose.

## Versions

- Document this tree as v0.8.0.
- Treat v0.7.0 as a separate, older line; do not reconcile this tree's syntax against it.

## Before any task

- Invoke the `/cgp` skill before reading, writing, reviewing, or debugging CGP code.
- Re-invoke it when you move into an unfamiliar construct.
- Read the knowledge base's [cgp README](https://github.com/contextgeneric/cgp-knowledge-base/blob/main/cgp/README.md),
  the [construct reference](https://github.com/contextgeneric/cgp-knowledge-base/blob/main/cgp/reference/README.md),
  and the [implementation reference](https://github.com/contextgeneric/cgp-knowledge-base/blob/main/cgp/implementation/README.md)
  when the task needs them.
- Load the `/point-first-writing` skill before editing Markdown or inline comments.
- Start from `cgp::prelude` and the prelude re-exports when a name is unclear.

## Code rules

- Keep every crate `no_std`-compatible unless it already requires `std`; gate `std` and `alloc` behind
  features.
- Place functionality in the lowest layer that makes sense and re-export it upward.
- Do not add a cross-layer dependency that skips the hierarchy.
- Keep all crate versions in lockstep at the workspace level.
- Declare inter-crate dependencies once in `[workspace.dependencies]` and reference them with
  `{ workspace = true }`.
- Mark a test-only crate `publish = false`.
- Treat [CHANGELOG.md](CHANGELOG.md) as the record of which macro forms are current and which are
  removed.
- Add a `///` to every public item that lacks one, and keep inline docs brief.
- Run the repository's checks before committing; the commands are in [README.md](README.md).
- Put a post-codegen compile failure in a `cargo-cgp` UI fixture, not in this repository, and update the
  matching fixture when a construct change alters a diagnostic.

## Tests

- A compile-time wiring test passes when the crate compiles; still run the suite for the runtime tests.
- Update the expansion snapshots when macro codegen changes, and review the expanded code.

## Macro review workflow

Review one macro at a time when the user asks for a macro review, and continue until no further issue
is found. Follow the detailed procedure in
[cgp-knowledge-base/cgp/implementation/review.md](https://github.com/contextgeneric/cgp-knowledge-base/blob/main/cgp/implementation/review.md).

- Orient first: read the macro's reference and implementation documents, its source, and its tests.
- Fix bugs and corner cases; close test gaps; verify each test exercises what it claims; deduplicate
  the tests; improve the documentation and inline docs.
- Scrutinize the codegen across attributes, parsing, expansion, generics, hygiene, and spans.
- Capture an accepted input whose expansion fails to compile as a `cargo-cgp` UI fixture, and catalog
  the class.
- Keep the reference, implementation documents, snapshots, and skill in sync in the same change.
- Run the format, clippy, and test checks for the affected crates before finishing.
- Ask when a behavior, corner case, or design choice is ambiguous.

## Sibling projects

- Read [sibling-projects.md](sibling-projects.md) to find a sibling and the revision to read.
- Use the local checkout first; fetch the file from the repository when it is absent.
- Write a link to another project as a GitHub URL on `main`; keep a bare checkout path relative.
- Keep the code dependency one-way: no `cgp` crate may depend on a `cargo-cgp` crate.
- Prefer `cargo-cgp` over raw `cargo check` when diagnosing a wiring failure.
- Propagate a construct change to the matching skill sub-skill and to `cargo-cgp` when it changes a
  diagnostic.

## AGENTS.md files

- Add a nested `AGENTS.md` only when its directory has significant rules of its own; otherwise extend
  this file.

## Committing

- Never commit as a side effect, suggest a commit unprompted, or treat a request as standing
  authorization.
