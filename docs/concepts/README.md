# CGP Concepts

This directory holds the high-level conceptual overviews that tie the CGP constructs together — the consumer/provider duality, dependency injection, namespaces, handlers, and so on. Each document explains one cross-cutting idea and points down into the [reference documents](../reference/README.md) for the per-construct mechanics, so a reader can grasp the shape of an idea here and follow the links for the precise semantics.

## How concepts differ from reference documents and examples

A concept document explains an *idea that spans several constructs*, whereas a [reference document](../reference/README.md) explains a *single construct completely* and an [example](../examples/README.md) develops a *single use case end to end*. The three are complementary. The reference for `delegate_components!` states its syntax and exact expansion; the [coherence](coherence.md) concept explains *why* CGP wires components through such a table at all, and the [modular serialization](../examples/modular-serialization.md) example shows the pattern solving a real problem. A concept document carries only enough mechanism to make the idea legible and links to the reference for the rest, so a reader who wants the detail follows the link while a reader who wants the framing stays here.

## The catalog

The authoring rules for concept documents, including when a cross-cutting idea earns its own page, live in [../CLAUDE.md](../CLAUDE.md). These documents explain the ideas that connect the constructs, each linking down to the per-construct references for the detail.

- [Bypassing coherence](coherence.md) — what Rust's coherence rules forbid, and the incoherent-impl-plus-local-wiring strategy CGP uses to work around them.
- [Modularity hierarchy](modularity-hierarchy.md) — the ladder from a single blanket impl to per-type-per-provider wiring, and how to pick the lowest rung a use case needs.
- [Modern idioms: a migration guide](modern-idioms.md) — the preferred higher-level forms for providers, dependencies, abstract types, and dispatch, mapped from the explicit forms they replace.
- [Consumer and provider traits](consumer-and-provider-traits.md) — the trait duality at the heart of CGP and how it sidesteps coherence.
- [Impl-side dependencies](impl-side-dependencies.md) — dependency injection through the `where` clause of blanket impls.
- [Implicit arguments](implicit-arguments.md) — writing providers as ordinary functions whose arguments come from context fields.
- [Higher-order providers](higher-order-providers.md) — providers parameterized by other providers.
- [Check traits](check-traits.md) — why wiring is lazy and how to verify it at compile time.
- [Abstract types](abstract-types.md) — abstract associated types shared and swapped across contexts.
- [Modular error handling](modular-error-handling.md) — an abstract error type plus raising and wrapping capabilities, with the error type and construction strategy chosen by wiring.
- [Namespaces](namespaces.md) — reusable, inheritable wiring tables and preset-style configuration.
- [Handlers](handlers.md) — the Computer/Producer/Handler family of computation components and their sync/async/fallible/by-reference variants.
- [Extensible records](extensible-records.md) — building and reading a struct by its named fields, and the extensible builder pattern.
- [Extensible variants](extensible-variants.md) — constructing and deconstructing an enum by its named variants, and the extensible visitor pattern.
- [Dispatching](dispatching.md) — routing extensible-data inputs to per-field and per-variant handlers.
- [Monadic handlers](monadic-handlers.md) — composing handlers through the identity/ok/err monads.
- [Type-level DSLs](type-level-dsls.md) — encoding a small language as types and interpreting it at compile time through CGP wiring.
- [Recovering `Send` bounds](send-bounds.md) — restoring the `Send` guarantee an async trait method drops, as a stand-in for the Return Type Notation stable Rust lacks.
