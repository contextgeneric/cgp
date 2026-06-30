# CGP Agent Skills

This directory holds the [agent skills](https://agentskills.io/) built from the CGP knowledge base — self-contained guides an LLM coding agent loads to read, write, debug, and explain Context-Generic Programming code. Where the [reference/](../reference/README.md), [concepts/](../concepts/README.md), and [examples/](../examples/README.md) directories are the exhaustive, version-controlled record, a skill is the distilled working subset of that record: enough for an agent to become proficient without loading everything, organized for progressive disclosure so the agent reads only the parts a task needs.

## How a skill differs from the rest of the knowledge base

A skill is a *teaching artifact optimized for an agent's context window*, whereas a reference document is an *exhaustive per-construct record* and a concept document is a *cross-cutting overview*. The skill draws on all three but reproduces none of them wholesale — it carries the mental model, the common constructs, and worked examples in enough depth to act, and points to the online knowledge base for the corner cases it deliberately omits. The relationship is one-directional: the knowledge base is the source of truth, and the skill is a synthesis kept in sync with it.

A skill is also deployed differently. It is copied out of this repository and run on its own, so it cannot rely on any file outside its own directory — every cross-link is either to a sibling sub-skill (a plain relative filename) or to the online knowledge base on GitHub. The authoring and synchronization rules that govern this, including how to keep a skill current when a construct changes, live in [../CLAUDE.md](../CLAUDE.md) under "The skills directory."

## The catalog

- [`cgp`](cgp/SKILL.md) — the foundational skill for working with CGP in Rust. Its `SKILL.md` establishes the paradigm (the consumer/provider trait split, wiring, impl-side dependencies) and routes to a set of topic sub-skills under [cgp/references/](cgp/references/) covering components, wiring, checking, functions and getters, abstract types, higher-order providers, error handling, handlers, extensible data, namespaces, the type-level primitives, and the modularity hierarchy. It is the skill `/cgp` resolves to.
