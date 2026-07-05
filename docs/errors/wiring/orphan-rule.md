# Orphan-rule violation

A generated impl targets a foreign trait and foreign types, which Rust's orphan rule forbids, so the expansion fails with `E0210`/`E0117` — most often when a prefixed `#[default_impl]` is registered from a crate that owns neither the namespace nor the path.

> **Status: planned.** This class is scaffolded but not yet fully written. The scope and backing below are recorded so the campaign can fill it in; see [../README.md](../README.md) and [../AGENTS.md](../AGENTS.md).

## Intended scope

This document covers the case where a `#[default_impl(@path in Namespace)]` on a *prefixed* component expands to `impl Namespace<_> for PathCons<..>`, whose `Self` type is built entirely from the `cgp`-owned `PathCons`/`Symbol` types and a foreign marker. A downstream crate owns neither the foreign trait nor any element of the foreign path, so the orphan rule rejects the impl. The document should record:

- **The diagnostic** — `E0210` (or `E0117`) naming the foreign trait and the fully type-built `Self`.
- **Where the root cause is** — present in the error, but the actionable insight (register from the namespace's own crate, or use a *local* component key whose marker is a local type) is the fix rather than something the diagnostic states.
- **Notes for tooling** — a tool should recognize the `PathCons<..>` `Self` type and point the user at the crate-ownership fix, since the raw orphan error does not explain the CGP-specific remedy.

## Backing fixtures

- [acceptable/cgp_namespace/default_impl_foreign_prefix_path.rs](../../../crates/tests/cgp-compile-fail-tests/tests/acceptable/cgp_namespace/default_impl_foreign_prefix_path.rs) — a prefixed default impl registered from a foreign crate, rejected by the orphan rule.

## Related

- [Conflicting wiring](conflicting-wiring.md), [Wiring cycle](wiring-cycle.md), [Unconstrained generic](unconstrained-generic.md) — the sibling structural classes.
- [Debugging CGP compile errors](../../guides/debugging.md) — the `E0210`/`E0117` entry in the decoder.
- [`DefaultNamespace`](../../reference/traits/default_namespace.md) and [`#[cgp_namespace]`](../../reference/macros/cgp_namespace.md).
