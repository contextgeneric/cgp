# CGP Guides

This directory holds the *guides* to writing Context-Generic Programming code — documents that direct the **choices** an author makes, rather than explaining what a construct is. Where the [reference](../reference/README.md) tells you what a construct means and the [concepts](../concepts/README.md) explain the ideas that tie constructs together, a guide answers the question that comes up once you already understand the pieces: *given several ways to express something, which should I use, and how do I evolve code from one form to another?*

## How guides differ from concepts, reference, and examples

The four sections answer four different questions, and a reader in a hurry can pick the one that matches their need. A [reference document](../reference/README.md) answers "what does `#[prefix]` mean and what does it expand to?" A [concept document](../concepts/README.md) answers "what is a namespace, as an idea?" An [example](../examples/README.md) answers "show me a namespace solving a real problem end to end." A **guide** answers "my `delegate_components!` table has grown unwieldy — what should I do about it, and in what order?" A guide is prescriptive: it recommends a default, names the trade-offs of the alternatives, and often walks a concrete before/after refactoring so the recommendation is grounded in real code rather than stated in the abstract.

A guide leans on the other three rather than restating them. It links to the reference for the exact syntax of each construct it recommends, to the concepts for the mechanism behind a recommendation, and to the examples for a fuller worked scenario. Keep the guide focused on the decision and the migration path; when a guide finds itself explaining what a construct *is* at length, that explanation belongs in the reference or a concept, linked from the guide.

## The catalog

The authoring rules for these documents live in [../AGENTS.md](../AGENTS.md). Each guide below names a decision you face when writing CGP and walks through how to make it.

- [Organizing wiring with namespaces and prefixes](namespaces-and-prefixes.md) — how to keep a growing `delegate_components!` table short: grouping components under path prefixes with `#[prefix]`, binding providers to a namespace with `#[default_impl]`, and merging multiple providers into one flattened table, worked as a refactoring of a real application.
- [Debugging CGP compile errors](debugging.md) — the playbook for tracing a wiring failure back to its cause: reading the error's shape, moving the error to the wiring site with checks, reducing to a minimal reproduction, inspecting the macro expansion, and a decoder for the errors you actually see.

The **modern idioms** are a family of small, related choices — each a shift from an explicit form to a vanilla-looking one — so they are grouped under a hub with a focused guide per idiom. Start at the hub for the framing and the map, or go straight to the idiom you are deciding on:

- [Modern idioms: a migration guide](modern-idioms.md) — the hub, with the explicit-to-modern framing and the list of cases where an explicit form is still right.
- [Writing providers the modern way](writing-providers.md) — `#[cgp_impl]` in consumer-trait shape, omitting the context parameter.
- [Declaring a provider's dependencies](declaring-dependencies.md) — `#[uses]` and `#[use_provider]` instead of hand-written `where` bounds.
- [Reading context fields](reading-context-fields.md) — `#[implicit]` arguments instead of getter traits.
- [Importing abstract types](importing-abstract-types.md) — `#[use_type]` aliases and the concrete-type equality form.
- [Adding capability supertraits](capability-supertraits.md) — `#[extend]` instead of native `:` supertrait syntax.
- [Dispatching a component per type](dispatching-per-type.md) — the `open` statement or a namespace instead of a `UseDelegate` table.
