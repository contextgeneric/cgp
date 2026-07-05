# Wiring cycle

A delegation chases its own tail — a component wired to a provider whose resolution routes back to that same component — so the trait solver overflows with `E0275`.

> **Status: planned.** This class is scaffolded but not yet fully written. The scope and backing below are recorded so the campaign can fill it in; see [../README.md](../README.md) and [../AGENTS.md](../AGENTS.md).

## Intended scope

This document covers the recursion failures of wiring resolution. The classic case is delegating a component to [`UseContext`](../../reference/providers/use_context.md) when the context's only implementation of that component *is* that delegation, so the lookup loops. The document should record:

- **The diagnostic** — `E0275` "overflow evaluating the requirement …", naming a requirement that recurses through the same component/provider pair.
- **Where the root cause is** — the cycle is visible in the repeated requirement, but the overflow message truncates and the actionable fix (break the cycle by wiring the component to a concrete provider) is not stated.
- **Notes for tooling** — a tool should detect the repeating requirement pattern and report the cycle as a cycle, rather than surfacing the raw overflow depth.

## Backing fixtures

This class needs a **new** fixture (a `UseContext` self-delegation, or an equivalent two-component mutual cycle) to pin the `E0275` shape; none exists yet. Add it under `acceptable/` per [../AGENTS.md](../AGENTS.md) when writing this document.

## Related

- [Conflicting wiring](conflicting-wiring.md), [Orphan-rule violation](orphan-rule.md), [Unconstrained generic](unconstrained-generic.md) — the sibling structural classes.
- [Debugging CGP compile errors](../../guides/debugging.md) — the `E0275` entry in the decoder.
- [`UseContext`](../../reference/providers/use_context.md) — the provider whose misuse most often causes the cycle.
