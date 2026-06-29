# CGP Examples

This directory holds self-contained worked examples of CGP in use. Each document develops one realistic use case end to end — the contexts, the components and providers, and the wiring that connects them — so that the example reads as a coherent program rather than a list of isolated snippets. The examples are the preferred source of code snippets for the rest of the knowledge base: when a [reference document](../reference/README.md) needs a worked illustration, it should draw on the patterns shown here so that the same vocabulary and the same running example appear everywhere a reader looks.

## How examples differ from reference documents

An example demonstrates a *use case and the design patterns that solve it*, whereas a reference document explains a *single construct completely*. The two are complementary. A reference document for `#[cgp_fn]` states its syntax, its exact expansion, and its corner cases; an example shows `#[cgp_fn]` working alongside `delegate_components!` and higher-order providers to actually compute something. Because the constructs are documented in full in the reference, an example does not re-explain them. It carries only enough prose to make the code legible, leaves a short note on which CGP concept each step demonstrates, and links to the reference document that owns that concept. A reader who wants the mechanics follows the link; a reader who wants the shape of a solution stays in the example.

The audience for these examples is an agent writing expanded documentation — a tutorial, a guide, a how-to. An example is the raw material such an agent draws on: a verified, idiomatic progression it can quote, adapt, and elaborate, confident that the code reflects current CGP. The examples therefore optimize for being *quotable and correct* rather than exhaustive.

## The catalog

The authoring rules for examples, including how to add a new one, live in [../CLAUDE.md](../CLAUDE.md).

- [Area calculation](area-calculation.md) — computing the area of several shapes, progressing from field-driven functions to a wireable area component with composable higher-order providers.
- [Shell-scripting DSL](shell-scripting-dsl.md) — a type-level DSL whose programs are types interpreted at compile time, progressing from a fixed CLI program through the handler component and its namespace wiring to a custom context and a language extension.
</content>
</invoke>
