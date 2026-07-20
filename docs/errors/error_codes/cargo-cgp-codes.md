# cargo-cgp `[CGP-Exxx]` codes

This entry catalogs the `[CGP-Exxx]` codes [`cargo-cgp`](https://github.com/contextgeneric/cargo-cgp) stamps on a diagnostic it rewrites into a recognized CGP class, so a class document's *How cargo-cgp presents it* section can cite a code by number. Unlike the sibling `rustc`-code entries, these codes are **owned by `cargo-cgp`, not by CGP** — this file is a pointer, and the authoritative definitions (what triggers each code and how to fix it) live in `cargo-cgp`'s [error-code catalog](https://github.com/contextgeneric/cargo-cgp/blob/main/docs/error-code.md). When the two disagree, that catalog wins; keep this list in sync with it under the [cross-project rule](../../../AGENTS.md#the-two-projects-cgp-and-cargo-cgp).

## What a code is

A code is `CGP-E` plus three digits, shown in square brackets at the start of a rewritten message — for example `error[E0277]: [CGP-E001] the consumer trait \`CanCalculateArea\` is not implemented for context \`Rectangle\``. The diagnostic keeps its own `rustc` code (`E0277`, `E0599`, `E0271`, `E0275`), so `rustc --explain` still works and the entries in this directory still apply; the `[CGP-Exxx]` tag rides *inside* the message text. A code is assigned only when `cargo-cgp` both rewrote the message and identified it as a CGP class — everything else is left uncoded by design.

The three-digit space is split by *what* a code classifies.

## Main-message headlines — `CGP-E0xx`

These lead a rewritten main message. `E001`–`E003` and `E009` come from the typed resolver and carry a `root cause:` note; `E004`–`E008` classify a duplicate-key `E0119` conflict (no root-cause note, both `rustc` carets kept); `E010` is a wiring-overflow rewrite of `E0275`.

- **CGP-E001** — the consumer trait is not implemented for the context (missing wiring, or an unmet transitive dependency).
- **CGP-E002** — the provider trait is not implemented for the provider (its impl-side `where` bounds do not hold).
- **CGP-E003** — a context field has the wrong type (the `HasField` bound holds but its `::Value` projection fails).
- **CGP-E004** — duplicate wiring: the same key mapped twice.
- **CGP-E005** — overlapping wiring: a key already set through another (generic over specific, `@`-path over a namespace forwarding, or a prefix path).
- **CGP-E006** — multiple namespaces used for one target in `delegate_components!`.
- **CGP-E007** — redirect collision: a direct wiring collides with a redirect of the same key.
- **CGP-E008** — duplicate redirect: the same key redirected more than once.
- **CGP-E009** — a hand-written wrapper trait (not a CGP consumer) fails because a CGP component it depends on fails.
- **CGP-E010** — the wiring never resolves (an `E0275` overflow whose requirement is a `CanUseComponent` bound; usually a `UseContext` cycle).

## Dependency-tree entries — `CGP-E1xx`

One per rendering template, riding at the start of each entry in a `root cause:` dependency tree.

- **CGP-E101** — consumer trait impl hop.
- **CGP-E102** — provider trait impl hop.
- **CGP-E103** — non-terminal `HasField` accessor hop.
- **CGP-E104** — redirect-lookup hop (a namespace or `open` `RedirectLookup`).
- **CGP-E105** — a hop through any other trait impl (a user capability, or an ordinary bound restated).
- **CGP-E106** — leaf: a genuinely missing context field.
- **CGP-E107** — leaf: the context wires no provider for a component (or terminates no namespace path).
- **CGP-E108** — leaf: the struct has the field but did not derive `HasField`.
- **CGP-E109** — leaf: a field has the wrong type.
- **CGP-E110** — leaf: a non-context delegation table (aggregate provider, `UseDelegate`/`UseInputDelegate`) is missing a key.
- **CGP-E111** — leaf: a non-provider was wired into a provider slot.

## Root-cause leads — `CGP-E2xx`

The `root cause:` line reuses the terminal leaf's `CGP-E1xx` code where it has one; this range exists for the one case that needs its own.

- **CGP-E201** — the failure bottoms out on an ordinary (non-CGP) trait bound.

## Where CGP classes cite these

A class document's *How cargo-cgp presents it* section names the specific code(s) the tool produces for that class and links to the [authoritative catalog](https://github.com/contextgeneric/cargo-cgp/blob/main/docs/error-code.md). For example, [check-trait-failure](../checks/check-trait-failure.md) is stamped `[CGP-E001]` with a `root cause:` tree whose leaf is `[CGP-E106]` (missing field) or `[CGP-E108]` (underived); the [conflicting-wiring](../wiring/conflicting-wiring.md) family maps to `[CGP-E004]`–`[CGP-E008]`; and [wiring-cycle](../wiring/wiring-cycle.md) maps to `[CGP-E010]`.
