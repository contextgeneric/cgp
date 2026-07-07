# CGP Communication Strategy

This directory holds the communication-strategy documents of the CGP knowledge base — guidance for LLM agents writing anything public-facing about CGP, from a landing page or a tutorial to a blog post, an article, or a social-media thread. Where the rest of the knowledge base records what CGP *is*, this section records how to *present* it: which ideas to lead with, how to frame the features that will feel unfamiliar, which misunderstandings to head off, and what tone to keep so that everything written about CGP reads as one consistent voice.

## Why this exists

CGP adds language-level capabilities that many Rust developers have never seen — a consumer/provider trait split, per-context wiring, impl-side dependencies, type-level tables — and the same true sentence about them can excite one reader and alienate another. A functional programmer hears "overlapping instances made safe" as a gift; a pragmatic Rust engineer hears the same pitch as a warning that the crate is too clever. Public-facing writing therefore succeeds or fails on framing as much as on accuracy, and framing depends on knowing who is reading and what they already believe. This section captures that knowledge once, so an agent preparing public writing starts from a shared understanding of the audience and a shared set of positioning decisions, rather than re-deriving them for every post — and so the whole body of CGP's public material speaks with one voice.

The purpose is twofold: to help writing **gain attention**, by naming which content and hooks draw the most interest, and to help it **minimize misunderstanding**, by naming the misreadings CGP reliably provokes and how to preempt them. Both goals run through the reader: attention is won by leading with what a specific audience cares about, and misunderstanding is avoided by defusing the specific misreading that audience is prone to.

## Relationship to related work

This section is the natural companion to [related-work/](../related-work/README.md), and the two are read together when preparing public writing. A related-work document explains one external idea — dependency injection, type classes, reflection — faithfully, records what its users like and dislike, and positions CGP against it; a communication-strategy document generalizes across those comparisons into audience-level guidance about which readers exist, what they already believe, and how a piece should be shaped for them. When a related-work document records a sentiment — that Rust developers reach for Dagger to escape reflection's runtime cost, say — this section turns it into a reader trait an author can plan around. Read the matching related-work document for the depth of a comparison; read here for the shape of the audience.

## The catalog

The documents below inform how CGP is presented to the public; register a new one here in the same change that adds it. The authoring rules for the section — including the marketing-director and developer-relations roles an agent takes on here — live in [AGENTS.md](AGENTS.md).

- [Reader profiles](reader-profiles.md) — the kinds of readers public-facing CGP writing must serve, from newcomers to Rust through advanced developers and across the backgrounds they arrive from, with what each reader already knows, is excited by, grows skeptical of, and needs from a piece written for them.
- [Selling points](selling-points.md) — the true capabilities CGP should advertise, each with the phrasings that make it land and the phrasings that backfire, plus audience-tuned one-liners keyed to the reader profiles and the related-work comparisons.
- [Skepticism](skepticism.md) — the objections a reader brings to CGP, whether imported from a paradigm they distrust or native to the Rust community, judged for whether they are justified and answered with wording that convinces without triggering the misunderstanding that fed them.
- [Tag lines](tag-lines.md) — a brainstorm of the candidate one-line descriptions of CGP, from the incumbent "modular programming paradigm" through the "language extension" and feature-first framings, each weighed for attention, skepticism, and honest feasibility, with a recommended shortlist to validate.
- [Key features](key-features.md) — the short, curated headline set of the few best selling points to put on the front page, with titles and one-line copy chosen for breadth, honesty, and the phrasing lessons of the rest of the section.
- [Technical barriers](technical-barriers.md) — the comprehension barriers a reader hits when learning CGP, from unfamiliarity with generics and traits upward, and the design affordances and progressive-disclosure teaching moves that lower each one.
