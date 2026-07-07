# AGENTS.md

This file provides guidance to LLM agents when working with code in this repository.

This directory holds the **related-work** documents of the CGP knowledge base — one document per external concept, framework, or language feature that solves a problem CGP also solves, or that resembles a CGP construct closely enough that a reader coming from it can be met on familiar ground. Read the knowledge-base [README.md](../README.md) for the background on the whole base, and the governing [../AGENTS.md](../AGENTS.md) for the rules every section shares. The rules below are specific to related work.

## Why this section exists

A related-work document exists to serve *future user-facing documentation*, not to teach CGP directly. The rest of the knowledge base explains CGP on its own terms; this section records how a mainstream idea — dependency injection, implicit parameters, type classes, and so on — actually works, what its users value and resent about it, and where CGP lands relative to it. An agent later asked to write a tutorial, a blog post, or a landing page *for readers who already know that idea* reads the matching related-work document first, then leans on the reader's existing intuition to make the CGP explanation land. The audience of the eventual writing is a practitioner of the related concept; the audience of the document itself is the agent preparing to address them.

This purpose sets the bar for the content. A related-work document is worth writing only if it captures the concept faithfully enough that an agent could explain it to that concept's own community without embarrassment, and honestly enough that the comparison to CGP survives a skeptic who prefers the other tool. Shallow praise of CGP and strawman versions of the related work both defeat the point: the reader we are ultimately writing for will spot either one immediately.

## What every related-work document must cover

Each document explains one related concept in genuine depth and then positions CGP against it. The explanation comes first and stands on its own — a reader learns the related work here, not just a caricature of it — and the CGP comparison follows once the concept is on the table. Every document works through the same obligations:

- **Explain the concept in real detail.** Cover the important sub-concepts of the related work, not just its headline. Define its vocabulary, show how it is actually used, and explain the mechanism behind it, at the depth a practitioner of it would recognize as correct.
- **Show it in code.** Demonstrate each important sub-concept with a code snippet in the related work's own language and idiom, and — where CGP has a counterpart — show the same thing written in CGP right beside it, so the two can be read against each other. Not every snippet has a CGP equivalent; say so when it does not.
- **Give both the pros and the cons.** State plainly what users *like* about the concept and what they *dislike* — the pain points, the footguns, the recurring complaints — drawing on real community sentiment rather than invented objections. A document that lists only weaknesses is as useless as one that lists only strengths.
- **Compare how CGP solves the problem differently.** Explain where CGP takes the same approach, where it diverges, and *why* — what CGP's design buys and what it costs relative to the related work. Be fair: name the cases where the related work is the better fit.
- **Analyze how to present CGP positively to someone who knows the concept.** Close with concrete guidance for the future writer: which of the reader's existing intuitions to build on, which analogies land and which mislead, which CGP advantages will resonate with this particular audience, and which of their expectations CGP will violate and must therefore address head-on. This positioning analysis is the section the rest of the document exists to support.

## Sourcing and citations

**Unlike the [examples](../examples/) directory, related-work documents cite their sources.** An example is re-derived as native knowledge-base material with no pointer to where the idea came from; a related-work document is the opposite — its credibility rests on being a faithful account of an external thing, so it must be traceable to that thing. Research the concept from its authoritative documentation, primary references, and representative community discussion before writing, and record what you used.

Study the research literature directly when the concept comes out of programming-language research, as row polymorphism, type classes, effect systems, and their kin do. For these ideas the online resources to study are not only the language documentation that popularized them but the academic papers and the serious technical articles that define and analyze them: read the foundational paper a feature descends from and, where it exists, the current research that generalizes or corrects it, alongside the blog posts and talks that make the theory legible. The primary literature is where a PL concept's precise definition, formal properties, and design space actually live — often stated nowhere else — so treat it as a first-class source to study and to cite, not an optional supplement to the documentation.

Ground every factual claim about the related work in a real source. Cite official language or framework documentation for how a feature behaves, primary papers for a concept's origin or formal properties, and reputable community writing for sentiment about what users like or dislike — attributing opinion as opinion, not as fact. Collect the citations in a **Sources** section at the end of the document, as a framed list of links with a short note on what each supports, and reference them inline where a specific claim leans on one. Do not invent quotations, statistics, or version-specific behavior; when unsure whether a detail is current, verify it against the source rather than trusting memory.

Keep the account current and neutral. Describe the related work as it exists now, note the version or edition when behavior is version-specific (Scala 2 `implicit` versus Scala 3 `given`/`using`, for instance), and represent the concept as its proponents would recognize it before critiquing it.

## The CGP side must obey the synchronization rule

Every CGP snippet in a related-work document is bound by the [synchronization rule](../AGENTS.md) exactly as a reference document's Expansion section is. A CGP comparison that shows syntax the macros no longer accept, or an expansion the code no longer produces, is a bug in the change that made it stale — and a especially damaging one here, because it will be quoted into user-facing material and shown to the very audience most likely to scrutinize it. Verify each CGP snippet against the source and the current macro behavior, invoke the `/cgp` skill before writing any CGP code, and prefer the modern idioms the skill and the [guides](../guides/) recommend. Draw CGP snippets from the [examples](../examples/) and the running scenarios the rest of the base already uses, rather than inventing fresh contexts, so the CGP side of every comparison speaks the knowledge base's shared vocabulary.

## Document structure

Each related-work document follows the same shape so readers can navigate any of them by habit. Open with a level-one heading naming the concept and a one-sentence summary of what it is and why it is worth comparing to CGP. Then proceed through these sections, using the same headings:

- **Purpose** — what problem the concept solves, and why a CGP reader should care that it exists. One or two paragraphs of framing.
- **The concept in depth** — the faithful explanation of the related work: its sub-concepts, its vocabulary, and its mechanism, each illustrated with a code snippet in the related work's own language. This is the section that must satisfy a practitioner of the concept. Use titled subsections when the concept has several distinct parts.
- **How CGP expresses it** — the same problems written in CGP, shown against the related-work snippets above, with prose explaining where the two align and where they part ways.
- **What users like and dislike** — the honest pros and cons of the related work, drawn from real sentiment and cited.
- **How CGP compares** — the design-level comparison: what CGP's approach buys, what it costs, and where the related work remains the better choice.
- **Presenting CGP to someone who knows this** — the positioning guidance for future user-facing writing: intuitions to build on, analogies that land or mislead, advantages that resonate, and expectations to address before they trip the reader.
- **Sources** — the framed list of citations described above.

Follow the dual-reader prose style (the `/dual-reader-prose` skill) throughout: open every section with a self-contained topic sentence, frame every list, and let the prose carry the meaning around each code block. Prefer plain language and the knowledge base's established CGP vocabulary — consumer trait, provider trait, provider, wiring, impl-side dependency, context — so a reader moving between this section and the rest never reconciles two dialects.

Register every new document in the catalog in [README.md](README.md) in the same change that adds it, and cross-link generously: to the [concepts](../concepts/README.md) for the CGP idea a comparison rests on, to the [reference](../reference/README.md) for the exact syntax of any construct shown, and to sibling related-work documents when two concepts are themselves related.
