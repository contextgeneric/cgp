# CGP Knowledge Base

This directory is a knowledge base about Context-Generic Programming (CGP), written by and for AI coding agents. Its purpose is to give an agent everything it needs to understand the *full semantics* of CGP — what each construct means, what code it expands into, and how the pieces fit together — without having to re-derive that understanding from the macro implementation every time. The `/cgp` skill gives a fast orientation; this knowledge base is the durable, version-controlled record that goes deeper and stays in sync with the code.

## Why this exists

CGP is implemented almost entirely as procedural macros, and proc-macro source code is a poor place to learn semantics from. An agent reading [crates/macros/cgp-macro-core](../crates/macros/cgp-macro-core) sees token-stream manipulation and AST transforms, not the meaning those transforms produce. The meaning — "`#[cgp_component]` generates a consumer trait, a provider trait, and two blanket impls that connect them" — has to be reconstructed by mentally running the macro. That reconstruction is slow, error-prone, and gets repeated on every visit. This knowledge base captures the reconstruction once, in prose, so the next agent can read the conclusion instead of re-deriving it.

The knowledge base also serves as a contract. When an agent changes how a macro expands, the corresponding reference document is the place where the intended new behavior is stated in plain language. A reviewer can compare the prose against the code and against the expansion snapshots in [crates/tests/cgp-macro-tests](../crates/tests/cgp-macro-tests) to confirm that all three agree. Documentation that drifts out of sync with the code is worse than no documentation, so keeping these documents accurate is a hard requirement of any change — see [CLAUDE.md](CLAUDE.md) for the maintenance rules.

## How it is organized

The knowledge base is divided into three top-level sections, and will grow to contain more as the need arises.

The [reference/](reference/README.md) directory holds one document per CGP construct — one for `cgp_component`, one for `cgp_impl`, one for `delegate_components`, and so on. Each document is self-contained and explains a single construct completely: its purpose, its accepted syntax, the exact code it desugars to, worked examples, and links to the constructs it relates to. The [reference index](reference/README.md) lists every construct and tracks which ones are documented.

The [concepts/](concepts/README.md) directory holds the cross-cutting conceptual overviews that span multiple constructs — the consumer/provider duality, dependency injection, namespaces, the handler family, and so on — each explaining one idea and linking down into the reference documents for the mechanics. Where the reference explains the individual trees, the concepts explain the shape of the forest.

The [examples/](examples/README.md) directory holds self-contained worked examples, one realistic use case developed end to end per document, from its contexts and components through to the wiring that connects them. The examples are the canonical source of code snippets the reference and concept documents reuse, so the same running scenarios recur across the whole knowledge base.

## How to use it

An agent working on CGP should read the relevant reference document before changing a construct, and should consult the `/cgp` skill for the conceptual framing that ties constructs together. The reference documents assume familiarity with the vocabulary the `/cgp` skill establishes — consumer traits, provider traits, providers, wiring, and so on — and focus on precise per-construct semantics rather than re-teaching the paradigm. Read the two together: the skill for the shape of the forest, the reference for the individual trees.
