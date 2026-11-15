# AGENTS.md: the CGP library

Follow these rules for every change to this repository. Read [README.md](README.md) for how the
crates are organized, and [sibling-projects.md](sibling-projects.md) for the related repositories.

## Universal rules

These rules hold in every CGP repository. The
[knowledge base's AGENTS.md](https://github.com/contextgeneric/cgp-knowledge-base/blob/main/AGENTS.md)
holds their canonical wording: read it first, and change a rule there before updating this block:

- Learn CGP and its tools from the `/cgp` skill and the knowledge base; never restate them in this
  file.
- Prefer the local sibling checkout when reading another project.
- Write a cross-project link as a GitHub URL on `main`.
- Name official release versions only, never a pre-release.
- Commit only when the user explicitly asks, on the current branch.
- Verify a claim against the source before writing it.
- Keep every view of the same truth in sync in the same change.
- Ask the user when a decision is ambiguous.
- Apply point-first writing to internal prose, and natural writing to public-facing documents
  such as READMEs.

## Task guides

Each kind of task has a guide that holds its rules. Read the guide before the task:

- **Changing a macro crate:** [crates/macros/README.md](crates/macros/README.md).
- **Adding, moving, or refactoring a test, or adding a failure case:**
  [crates/tests/README.md](crates/tests/README.md#rules-for-the-test-suite).
- **Reviewing a macro:** the knowledge base's
  [cgp/implementation/review.md](https://github.com/contextgeneric/cgp-knowledge-base/blob/main/cgp/implementation/review.md),
  as [Macro review workflow](#macro-review-workflow) says.
- **Adding a fixture for a post-codegen failure:** `cargo-cgp`'s
  [tests/README.md](https://github.com/contextgeneric/cargo-cgp/blob/main/tests/README.md).
- **Writing a reference or implementation document:** the knowledge base's
  [cgp/writing-guide.md](https://github.com/contextgeneric/cgp-knowledge-base/blob/main/cgp/writing-guide.md)
  and
  [cgp/implementation/writing-guide.md](https://github.com/contextgeneric/cgp-knowledge-base/blob/main/cgp/implementation/writing-guide.md).

## Orient before any task

Read what a task touches in the skill and the knowledge base before changing the code:

- Invoke the `/cgp` skill before reading, writing, reviewing, or debugging CGP code, and re-invoke it
  when you move into an unfamiliar construct.
- Read the knowledge base's [cgp README](https://github.com/contextgeneric/cgp-knowledge-base/blob/main/cgp/README.md),
  the [construct reference](https://github.com/contextgeneric/cgp-knowledge-base/blob/main/cgp/reference/README.md),
  and the [implementation reference](https://github.com/contextgeneric/cgp-knowledge-base/blob/main/cgp/implementation/README.md)
  when the task needs them.
- Find the module that exports a name in the `/cgp` skill's import table or the construct reference
  rather than assuming `cgp::prelude` carries it.
- Load the `/point-first-writing` skill before editing Markdown or inline comments.

## Code rules

The crates stay layered and `no_std`, and each change keeps its docs, snapshots, and checks current:

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
- Update the expansion snapshots when macro codegen changes, and review the expanded code.
- Run the repository's checks before committing;
  [`.github/workflows/tests.yml`](.github/workflows/tests.yml) lists them, and CI runs the same
  commands.

## Sibling projects

`cgp` sits below its siblings, and a construct change reaches each one that describes it:

- Keep the code dependency one-way: no `cgp` crate may depend on a `cargo-cgp` crate.
- Prefer `cargo-cgp` over raw `cargo check` when diagnosing a wiring failure.
- Propagate a construct change to the matching skill sub-skill, and to `cargo-cgp`'s UI fixtures when
  it changes a diagnostic.

## Macro review workflow

Follow the knowledge base's
[cgp/implementation/review.md](https://github.com/contextgeneric/cgp-knowledge-base/blob/main/cgp/implementation/review.md)
when the user asks for a macro review; it holds the procedure and its rules.
