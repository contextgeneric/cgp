# Rust error-code reference

This directory is a compact reference for the handful of `rustc` error codes CGP's post-codegen failures surface under, so a reader who has an error code in hand can look up what it means in plain Rust — the message `rustc` prints, the language rule behind it, and the RFC or issue that defines that rule — before turning to the CGP-specific class that produced it. It is the *forward* index (error code → meaning → the CGP classes that emit it) that complements the [catalog's class documents](../README.md), which run the other way (a CGP mistake → the diagnostic it produces).

## Why this exists

The per-class documents in the catalog each explain one CGP failure, and several of them lean on the same underlying Rust rule — coherence, the orphan rule, the trait-solver recursion limit. Rather than restate that rule in every class that touches it, each code has one entry here that states it once, grounded in the official Rust documentation and the RFCs and issues that define it, and the class documents link to that entry. This keeps the Rust-language facts in one verified place and the CGP-specific anatomy in the class docs, so neither drifts and neither repeats the other.

Each entry records the same four things: the message `rustc` prints, when the compiler emits it in ordinary Rust, the rule it enforces and where that rule is defined, and which CGP error classes produce it. The facts here are verified against the official [error index](https://doc.rust-lang.org/error_codes/), the [Rust reference](https://doc.rust-lang.org/reference/), and the linked RFCs and `rust-lang/rust` issues, not against memory; the class documents remain the source of truth for the *exact* diagnostic a CGP mistake produces, pinned by their [`cargo-cgp` UI fixtures](https://github.com/contextgeneric/cargo-cgp/blob/main/tests/README.md).

## The codes

The catalog surfaces nine codes, split by the kind of rule they enforce. Three are **coherence and orphan** rules — the compiler refusing overlapping or foreign impls:

- [`E0119`](e0119.md) — conflicting implementations of a trait for a type.
- [`E0210`](e0210.md) — orphan rule: an uncovered type parameter in a foreign-trait impl.
- [`E0117`](e0117.md) — orphan rule: a foreign trait implemented for a foreign (arbitrary) type.

One is a **well-formedness** rule on impl parameters:

- [`E0207`](e0207.md) — an unconstrained type parameter on an impl.

Two are **trait-solving** outcomes — the solver failing to satisfy or terminate a bound:

- [`E0277`](e0277.md) — a trait bound is not satisfied (including the `Sized` special case).
- [`E0275`](e0275.md) — overflow evaluating a requirement (a recursion-limit or cycle failure).

Three are **name-resolution and method-probe** diagnostics:

- [`E0428`](e0428.md) — a name is defined more than once in one scope.
- [`E0576`](e0576.md) — an associated item is named that the trait or type does not declare.
- [`E0599`](e0599.md) — a method exists but its trait bounds are not satisfied.

Beyond the `rustc` codes, `cargo-cgp` stamps its own `[CGP-Exxx]` codes on a message it rewrites into a recognized CGP class. Those are `cargo-cgp`'s, not CGP's, so they are cataloged in one pointer entry rather than mixed in above:

- [cargo-cgp-codes.md](cargo-cgp-codes.md) — the headline (`CGP-E0xx`), dependency-tree (`CGP-E1xx`), and root-cause-lead (`CGP-E2xx`) codes, each with a one-line meaning and a link to `cargo-cgp`'s authoritative [error-code catalog](https://github.com/contextgeneric/cargo-cgp/blob/main/docs/error-code.md).

## Relationship to the rest of the catalog

These entries are a supporting reference, not a class of error: they describe Rust, and the [class documents](../README.md) describe CGP. A code entry's "Where CGP produces it" section links out to every class that emits it, and each class links back to the code entry in place of the raw `doc.rust-lang.org` URL, so the official links live here and the classes cite this directory. The rules for keeping these entries accurate — verifying wording against a real compilation and the official docs rather than memory — are the same [synchronization rules](../AGENTS.md) that govern the whole catalog.
