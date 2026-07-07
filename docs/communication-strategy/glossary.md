# Glossary of the non-technical craft

This document defines, in plain language, the marketing, public-communication, and developer-relations terms of art that the rest of the section uses — each anchored to an intuition a systems programmer already holds — so a reader fluent in CGP but new to these disciplines never has to guess what a word means.

## How to use this glossary

This is the quick-reference companion to the principles in the [README](README.md), not a replacement for them. The README explains the *ideas* — positioning, framing, the funnel — as an argument you read straight through; this document is the *lookup* you scan when a term appears and you want its one-line meaning and a programmer's analogy for it. Where a term has a fuller treatment in the README's principles section or in a dedicated document, the entry here gives the short gloss and points you there for the depth.

A word on why the analogies matter. The fastest way to learn a concept from a discipline you have never studied is to map it onto one you already own, so every entry ties its term to something a systems programmer knows cold — a function signature, a cache miss, a fast path, an API boundary. Treat the analogy as a handhold rather than an equation: it is close enough to make the term stick, and the document it points to carries the precise version. None of this is difficult or secret, as the README stresses; it is a separate discipline with its own words, and this is the word list.

## The parts of a piece

Every piece of public writing is built from a small set of named parts, and naming them once is what lets the per-artifact playbooks in [formats.md](formats.md) stay short. These are the anatomy of a landing page, a launch post, or a talk.

- **Hook** — the opening line whose only job is to make the reader read the next line. It is the fast path of a piece: nearly every reader hits it, most go no further, so if it is slow or unclear nothing downstream ever runs. A good hook carries a concrete benefit or a striking before/after, never the paradigm name.
- **Tag line** — the compressed, one-line description of the whole project, usually the first thing a reader learns about it and sometimes the only thing. It is the project's `--help` summary line: a handful of words that must say what the thing is and why to care. The full treatment, with CGP's candidates weighed, is in [tag-lines.md](tag-lines.md).
- **Subline** — the second line beneath a hook or tag line that carries the honest mechanism the hook left out. The hook earns the click; the subline heads off the reader's first objection — "still Rust, checked at compile time, no runtime cost" — before it can form.
- **Call to action (CTA)** — the single next step the piece asks the reader to take, such as "try the quickstart" or "read the comparison." Offer exactly one, the way a good CLI has one obvious default action: a reader handed five next steps takes none. Matching the CTA to the reader's stage is the job of the conversion ladder in [formats.md](formats.md).
- **Above the fold** — what a reader sees before scrolling, a phrase borrowed from the top half of a folded newspaper's front page. On a README or landing page it is the first screen, and it must carry the tag line, the headline features, and ideally runnable code, because many readers judge from it alone and never scroll.

## Getting attention and choosing the category

Marketing supplies the words for how an idea earns a reader's attention and which category the reader files it under, developed in full in the README's [marketing principles](README.md#marketing-positioning-and-attention). These terms recur wherever the section discusses what to lead with.

- **Positioning** — the deliberate choice of the category a reader files you under, made before they choose a dismissive one for you ("oh, it's a DI framework"). A reader pattern-matches a new tool to the nearest thing they know within seconds, so positioning is picking that match for them rather than leaving it to chance. It is the subject of [positioning.md](positioning.md).
- **Framing** — the choice of angle and wording that decides how a reader reacts to a fact that is true either way. "Overlapping instances made safe" and "clever type-system trickery" can describe the same feature; the frame is the message, not a decoration on it.
- **Value proposition** — the one-sentence answer to "what do I get, and why should I care?", stated as a benefit rather than a feature list. It is the return value of your pitch: if a reader cannot state it after your opening, the call returned nothing.
- **Differentiation** — what CGP does that the obvious alternative cannot, the answer to "why this and not the thing I already use." A reader who sees no difference keeps what they have, so a pitch must name the gap out loud.
- **Elevator pitch** — the thirty-second spoken explanation you could finish before an elevator reaches its floor, the conversational cousin of the tag line. Its constraint is the same: one idea, delivered before attention runs out.

## Reaching people at scale, and sounding like one project

Public communication is the craft of being understood by readers you will never meet, and its terms are about clarity and consistency across a whole body of writing rather than any single piece. The README's [public-communication principles](README.md#public-communication-clarity-and-consistency) argue why these invert the habits technical writing trains.

- **Channel** — where a piece appears: Lobsters, the Rust subreddit, a conference talk, a social thread, the README. Each channel has its own audience, patience, and etiquette, so the channel is chosen before the words are; [attention-and-engagement.md](attention-and-engagement.md) records where CGP's readers actually gather.
- **Curse of knowledge** — the expert's built-in inability to feel what a beginner does not yet know, which makes deep CGP fluency the single biggest risk to explaining CGP well. It is the reason a compiler author writes the error message only they can read. It is the master principle behind nearly every warning in the section, stated in the [README](README.md#principles-from-marketing-public-communication-and-developer-relations) and answered construct by construct in [technical-barriers.md](technical-barriers.md).
- **Message discipline** (or **one voice**) — using the same word for the same idea in every piece, so scattered posts reinforce one another instead of reading as different projects. It is API stability for prose: rename the concept per post and the reader senses incoherence even when they cannot name it. The canonical word list is [vocabulary.md](vocabulary.md).
- **Clarity over completeness** — the rule that one idea a reader keeps beats five accurate ideas they forget, so a pitch says less, more clearly, and defers the nuance. It fights the specification writer's instinct, where completeness is the virtue; in a pitch, completeness is what buries the point.

## Earning and keeping trust

Developer relations is communication with an audience that has been marketed to badly its whole career and detects spin on sight, so its terms are about trust — how it is earned, and how one misstep spends it. The README's [developer-relations principles](README.md#developer-relations-trust-and-community) explain why honesty is the strategy rather than a limit on it.

- **Social proof** — evidence from other people, especially peers and real running systems, that persuades where self-praise cannot. A developer believes another developer's working code far more than any adjective you write; the section argues a flagship adopter is CGP's most valuable missing asset, in [attention-and-engagement.md](attention-and-engagement.md).
- **The funnel** (and **conversion**) — the staged path a reader travels from first hearing of CGP to advocating for it: heard-of, curious, trying, adopting, advocating. It is called a funnel because readers drop out at every stage, so fewer reach each next one, and a **conversion** is one reader taking the step to the next stage. Each stage wants different content and a different [call to action](#the-parts-of-a-piece) — the **conversion ladder** the [formats.md](formats.md) playbooks end on.
- **Show, don't tell** — demonstrate a capability with running code rather than assert it with an adjective, because this audience trusts what it can run and discounts what it is merely told. A before/after on real code outperforms every "flexible" and "powerful" combined.
- **The pile-on** (and **dunking**) — a public group dismissal, where a technical community turns on a post it reads as arrogant or dishonest and piles ridicule on it, often outliving the post. "Dunking" is the act of publicly ridiculing a claim, and a pile-on is the crowd version. The defense is the same as the honest move — claim only what is true, and never disparage another tool to elevate CGP, per the [README](README.md#developer-relations-trust-and-community).

## When a term is missing

If you meet a non-technical term in the section that is not defined here, treat that as a gap to close rather than a word to guess at. Add the term to this glossary in the same change and in the same shape — a plain definition and a programmer's anchor — so the next reader is not left guessing, and so this document stays the section's single home for the vocabulary of the craft, the way [attention-and-engagement.md](attention-and-engagement.md) is the single home for its external citations.
