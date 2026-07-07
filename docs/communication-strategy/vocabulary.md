# Vocabulary and message discipline

This document is the canonical word list for public writing about CGP: the term to use for each concept, the plain-language gloss that introduces it to a cold reader, and the words and framings to avoid — so that everything written about CGP reads as one voice.

## Why a shared word list matters

CGP's public writing succeeds or fails on wording as much as on substance, so the section's "one voice" goal needs a single place that fixes the words rather than leaving each document to choose its own. The other documents each carry their own "say it like this / avoid this" guidance for their topic; this document consolidates those choices into a reference a writer checks before publishing, and it is the authority the others defer to — when a phrasing rule here and a phrasing rule elsewhere disagree, this document is where the disagreement is resolved. It is deliberately narrow: it governs *which word* to use, not *what to pitch* (that is [selling-points.md](selling-points.md)) or *how to answer an objection* (that is [skepticism.md](skepticism.md)).

The list divides into three parts, and the division is the whole method. There are terms to **use** — the established CGP vocabulary, each paired with the one-line gloss that makes it legible to a reader meeting it for the first time. There are terms to **defer** — accurate internal words that intimidate a newcomer and should be withheld or introduced by analogy until the reader is ready, per [technical-barriers.md](technical-barriers.md). And there are words and framings to **avoid** — the ones that reliably trigger the misreadings [skepticism.md](skepticism.md) exists to prevent. A writer keeps the code and construct names accurate throughout, because a factual slip is shown to the audience most able to catch it, but chooses which of the accurate terms a given reader is ready for.

## Terms to use, and how to introduce each

The established CGP vocabulary is the same in public writing as in the rest of the knowledge base, so a reader moving between a blog post and the reference never reconciles two dialects. What public writing adds is the gloss: a term should be introduced with a plain-language definition on first use, then used consistently. Prefer these terms and these introductions.

- **Context** — the concrete type an application wires and calls methods on. Introduce it as "the type that owns the wiring — your application, your test harness, your service," not as a bare "`Self`," because a cold reader has no reason to know the two coincide.
- **Component** — one capability, defined once, that can have many implementations. Introduce it as "an interface you can wire an implementation for," and reserve the internal detail that a component is a consumer trait plus a provider trait for when the reader asks how it works.
- **Consumer trait** and **provider trait** — the trait you *call* and the trait you *implement*. Introduce the pair only when a piece goes past the surface; for an introductory audience, "the trait you use" and "the code that implements it" carry the idea without the vocabulary.
- **Provider** — a named, swappable implementation of a component. Introduce it as "one implementation you can choose," which is the word's whole job; avoid explaining that it is a zero-sized marker type until the reader is reading generated code.
- **Wiring** — choosing which provider implements each component for a context. Introduce it as "a small table that says which implementation to use," and lean on the table image, since it is the single most load-bearing analogy in CGP writing.
- **Impl-side dependency** — a requirement a provider states in its own implementation rather than in the interface callers see. Introduce it as "the provider declares what it needs, and callers never see it," because the encapsulation benefit is the point and the phrase "impl-side" means nothing cold.
- **Context-generic programming** — the name of the paradigm, never the pitch. Introduce it only after a concrete capability has landed, always paired with a plain descriptor, per [tag-lines.md](tag-lines.md).

## Terms to defer, and how to reveal them

Some accurate terms are barriers, not vocabulary, for a reader below them on the [prerequisite ladder](technical-barriers.md), and leading with one loses the reader before the value lands. Defer these, and reveal each only when the reader has a reason to want it — matching the depth to the profile, so the advanced reader gets the real word immediately and the newcomer meets it late or by analogy.

- **Coherence** and the **orphan rule** — the reason the consumer/provider split exists, but a theory the reader does not need to adopt CGP. Defer them behind the concrete pain (implementing a trait for a type you don't own), and introduce them, when at all, through that pain rather than as a rule to learn; a piece should never open on coherence, as [tag-lines.md](tag-lines.md) warns.
- **Blanket implementation**, **monomorphization**, **higher-ranked trait bound**, **`PhantomData`** — the machinery under the ergonomic surface. Withhold them from introductory material; when the runtime-cost question makes monomorphization worth naming, introduce it as "the compiler generates a direct call," not as jargon.
- **`DelegateComponent`**, **`IsProviderFor`**, the **type-level table** — the mechanism, not the model. Keep them out of a beginner's view entirely and describe the table with the settings-map analogy; bring the real traits in only for a reader reading an expansion or a compiler error, per [technical-barriers.md](technical-barriers.md).

## Words and framings to avoid

A handful of words reliably create the misunderstandings the section spends its effort preventing, and they should be avoided by default, each with a truer replacement that is also more persuasive to this audience. The through-line is that vague or grand wording lets the reader supply the worst reading, while a precise, smaller claim forecloses it and survives scrutiny — which, with the Rust audience, persuades *because* it survives. The pairs below consolidate the guidance scattered across the section into one checklist.

- Avoid **"magic"**, even admiringly — this audience reads it as a warning. Say **"explicit,"** and point at the wiring table and the declared dependencies.
- Avoid **"automatically resolves," "finds," or "figures out"** the implementation — the single most common misframing, and the one that undercuts the coherence-freedom story. Say the provider is **"named explicitly, in one readable place."**
- Avoid flatly calling CGP **"a DI framework," "a reflection system,"** or **"an effect system"** — each makes the reader expect runtime behavior CGP does not have. Qualify each — **"compile-time, reflection-free dependency injection,"** **"compile-time structural reflection,"** **"the dynamic-binding fragment of effect handlers"** — and state the distinguishing limit in the same breath.
- Avoid implying **any runtime component**, even "a fast one." Say the wiring is **"resolved at compile time and compiled to a direct call."**
- Avoid **"zero-cost abstraction"** as a lead — it is accurate but worn. Prefer the concrete **"compiled to a direct call, with no vtable and nothing in the binary for a provider you don't use."**
- Avoid **"blazingly fast"** and any **unbenchmarked "faster than."** Say **"there is no runtime cost to compare,"** which is the honest and stronger claim.
- Avoid **"no boilerplate."** Say CGP **"moves the wiring into one readable place,"** because there is wiring and the reader will find it.
- Avoid **"replaces traits"** or **"a new language."** Say **"a superset of ordinary traits"** and **"a library on stable Rust," "an extension,"** never "a language of its own," per [tag-lines.md](tag-lines.md).
- Avoid **"just"** in a competitor's description — "just macros," "just another DI framework" are the reader's dismissals, not ours, and echoing them concedes the frame.
- Avoid overstating maturity — **"works on stable Rust today"** is true and worth saying, while **"production-proven at scale"** needs evidence the [evaluator](reader-profiles.md) will notice is missing.

## The name, and the community's bridge terms

The project's own name needs its own rule, because it is the term most likely to be misused as a pitch. Never ship **"context-generic programming"** as a standalone hook: it is opaque on first contact, and this is observed rather than hypothetical — readers in CGP's own release discussions said the phrase obscures more than it conveys and reached instead for "structural typing" or "duck typing for statically-typed code" to name what they thought it was ([attention-and-engagement.md](attention-and-engagement.md)). Always pair the name with a plain descriptor, and treat those community-supplied phrases as *bridge terms* for body copy — useful for meeting a reader where they are, but qualified, because CGP is nominal-and-wired rather than truly structural, and a precise reader will catch an unqualified "structural typing." Keep the name as the owned category term, per [tag-lines.md](tag-lines.md), and let a concrete capability carry the hook.

## Keeping the list in sync

Because this document consolidates wording rules the other documents also carry, it must stay consistent with them, and the coupling runs both ways. When a phrasing rule changes here, check the "say it like this / avoid this" lists in [selling-points.md](selling-points.md) and [skepticism.md](skepticism.md) and the title rules in [key-features.md](key-features.md) and [tag-lines.md](tag-lines.md), because a writer told to prefer a phrase in one place must never be warned against it in another. And when a CGP construct is renamed or a capability changes, the terms here are bound by the synchronization rule exactly as a reference document is: verify each against the source and the `/cgp` skill, and prefer the modern idioms the skill and the [guides](../guides/README.md) teach over the legacy forms a reader will still meet in older code.
