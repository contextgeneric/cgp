# Selling points

This document catalogs the selling points CGP should advertise and, for each, the phrasings that make it land and the phrasings that backfire, so a writer can pick the right pitch for the right reader and word it in a way this audience will believe.

## How to use these selling points

A selling point is a true CGP capability, stated in the reader's own language, that answers a problem the reader already feels. That definition carries three obligations at once, and dropping any one turns a selling point into the kind of claim the Rust audience punishes. It must be **true**, because the readers most worth winning are the ones most able to catch an overclaim; it must be **framed for a reader**, because the same capability excites one audience and alarms another, as [reader-profiles.md](reader-profiles.md) lays out; and it must be **anchored to a pain**, because "look what CGP can do" persuades no one, while "here is the problem you have, gone" persuades the skeptic. Lead every piece from a concrete problem, reach for the selling point that removes it, and state it in the words below.

Two habits make the difference between a pitch that lands and one that invites a pile-on. First, pair the advantage with its cost whenever the audience is a skeptic — the [skepticism](skepticism.md) document is the companion to this one, and a selling point delivered without acknowledging its matching objection reads as spin. Second, prefer the concrete over the grand: a one-line before/after on familiar code outperforms any adjective, and "there is no runtime cost" outperforms "blazingly fast." The phrasing guidance in each section below is not decoration; the wrong word on the right capability is how CGP gets pattern-matched to something the reader already dismisses.

## The headline pitch

CGP's one-line identity should say what it *is* and what it *costs* in a single breath, because the reader's first question is "what is this" and their immediate second is "what does it cost me." The value proposition that does both is that **CGP is modular programming for Rust: write many interchangeable implementations of an interface and choose between them per context, resolved entirely at compile time with zero runtime cost.** That sentence leads with the benefit (many implementations, chosen per context), names the mechanism honestly (it is a way of writing Rust, not a runtime), and closes the escape hatch a skeptic reaches for (no runtime cost). Shorter openers — "swappable implementations, wired per context, erased before it runs" — keep the same three beats.

Resist the urge to lead with the machinery. Opening on the consumer/provider trait split, `DelegateComponent`, or coherence-bypassing loses every reader but the advanced enthusiast, and even they respond better to a problem first. The internals are what a curious reader graduates *into*, not what they should meet in the first sentence. Reserve the phrase "context-generic programming" for after the value has landed; as an opener it is a name the reader cannot yet decode, and a name is not a pitch.

## Zero runtime cost

The strongest broad selling point is that all of CGP's flexibility is resolved at compile time and erased before the program runs, so the modularity costs nothing where it matters. There is no container holding a graph, no reflection walking types, no vtable, and no dynamic dispatch; a wired call monomorphizes to a direct function call, and a provider a context does not use is not in the binary. This is the selling point that answers the working developer's first suspicion — that abstraction means overhead — and the one that separates CGP from the runtime dependency-injection and reflection tools other languages rely on, as the [dependency injection](../related-work/dependency-injection.md) and [reflection](../related-work/reflection.md) comparisons detail.

Say it like this:

- "Resolved at compile time and compiled away — a wired call is a direct call."
- "No runtime container, no reflection, no vtable; nothing about the wiring survives into the running program."
- "Dependency injection that costs nothing at runtime."
- "Unused providers are not in your binary."

Avoid the phrasings that overreach or read as slogans. Skip "blazingly fast" and any comparative speed claim without a benchmark to cite; the honest and stronger claim is that there is no cost to compare, not that CGP wins a race. "Zero-cost abstraction" is accurate but worn, so prefer the concrete "compiled to a direct call." And never imply there is a runtime component "but a fast one" — the point is that there is none, and blurring that invites the exact runtime-cost objection the selling point exists to remove.

## Swap implementations without the runtime penalty

CGP's central capability is that one interface can have many interchangeable implementations, chosen per context — per deployment, per test, per environment — which is the everyday problem that drives developers to trait objects or dependency-injection frameworks in the first place. A context wires a production implementation; a test wires a mock; a second application wires a third choice; none of them pay for `dyn` dispatch, and none of them collide. For the working developer this is the most legible benefit, because the mock-in-tests, real-in-production split is a problem they have solved awkwardly before, and CGP solves it with a swap of one wiring line.

Say it like this:

- "One interface, many implementations — the context picks which one, and the choice is a line you can read."
- "Mock it in tests, run the real thing in production, with no trait objects and no runtime dispatch."
- "The same component resolves to different implementations in different contexts, with no conflict between them."

Avoid implying the choice is automatic. CGP does not find the implementation for you — you name it in a wiring table — and a pitch that says "CGP picks the right one" sets up the reader to feel misled the first time they write a `delegate_components!` entry. Frame the explicitness as the feature it is: the choice is one greppable place, not a resolution search you have to reverse-engineer. This is the wording that also defuses the implicit-resolution skepticism covered in [skepticism](skepticism.md).

## Dependencies that are explicit and compiler-checked

CGP makes what a provider needs from its context explicit in its declarations and enforces it at compile time, which answers the loudest complaint against dependency-injection frameworks directly. A provider states its dependencies through `#[uses]` and `#[implicit]` rather than hiding them, so a reader sees what a component requires without spelunking, and a context that fails to satisfy a dependency is a compile error at the wiring site — not a startup exception or a `NullPointerException` in production. This is "the explicitness the Spring community learned to prefer, by default," and it lands hardest with the enterprise and dependency-injection reader profiled in [reader-profiles.md](reader-profiles.md). It also answers a pain the pure-Rust reader feels without any framework at all: because a provider's dependencies live in its own implementation rather than in a public trait's method signature, they never force internal types to become public — the encapsulation leak that hand-rolled trait-based dependency injection is prone to, documented with its source in [attention-and-engagement.md](attention-and-engagement.md).

Say it like this:

- "A provider's dependencies are stated where the implementation lives and checked by the compiler."
- "A missing dependency is a compile error at the wiring site, not a runtime surprise."
- "No hidden dependencies: what a component needs is in its signature, not buried in its body."
- "[`check_components!`](../reference/macros/check_components.md) is your container's startup validation — run at compile time instead of at boot."
- "Dependencies stay in the implementation, not the public interface, so they never leak internal types into your API the way a public generic trait does."

Avoid overselling the verification as effortless. The check is something you write, and its error messages, though they name the missing dependency, are verbose — say so when the audience is technical, because pretending the diagnostics are pretty is exactly the kind of small dishonesty that costs trust. Frame the trade honestly: you write a check line, and in return the whole class of runtime wiring failures cannot occur.

## It reads like ordinary Rust, and you adopt it gradually

A decisive selling point for the cautious majority is that CGP code can read like ordinary functions and plain trait impls, and that it is a superset of normal traits you can adopt one piece at a time. An `#[implicit]` argument looks like a function parameter; a provider written with `#[cgp_impl]` looks like a trait impl; a consumer trait can be implemented directly on a type with no CGP machinery at all. Nothing forces a project to swallow the whole paradigm at once, and a codebase can use a single component in one corner and stay otherwise vanilla. This is the selling point that disarms the "too clever / all-or-nothing" fear the Rust community brings to any new abstraction.

Say it like this:

- "It's a superset of ordinary traits — start with one component and leave the rest of your code unchanged."
- "Providers read like normal impls; implicit arguments read like normal parameters."
- "Adopt it gradually; you never have to buy the whole paradigm to use part of it."

Avoid two opposite mistakes. Do not claim "no boilerplate" — there is wiring, and the honest framing is that CGP *moves* boilerplate into one readable place, not that it erases it. And do not undersell by leading with the machinery that makes the gradual on-ramp possible; the reader wants to hear "it looks like the Rust you already write," not a tour of the desugaring that achieves it.

## Overlapping and orphan implementations, made safe

For the type-system and functional-programming audience, the standout selling point is that CGP makes legal the overlapping and orphan implementations their languages forbid or make fragile — and makes them safe by keeping every choice explicit and local. Because a provider implements a provider trait for its own marker type rather than for the context, the orphan rule and the overlap rule do not bite: a crate can define many implementations of one capability, and implement a capability for a type it does not own, without the newtype dance or the coherence contortions. The per-context wiring table is what keeps this safe rather than chaotic, so incoherence never means the indeterminism that makes Haskell's `INCOHERENT` a footgun. The full comparison lives in [type classes](../related-work/type-classes.md) and [bypassing coherence](../concepts/coherence.md).

This selling point has a rare asset behind it: developers already reinvent CGP's mechanism by hand. Rust programmers who hit the "no two blanket impls may overlap" wall reach independently for the same zero-sized-marker-plus-helper-trait pattern CGP is built on, as [attention-and-engagement.md](attention-and-engagement.md) documents — so the pitch reminds a reader of a workaround they have written and would rather not maintain, not a capability they must be talked into wanting. Leading with that recognition ("you have written the three-line version of this") disarms the "over-engineered" reflex faster than any claim about expressiveness.

Say it like this:

- "Type classes without the orphan rule — overlapping instances made legal and safe."
- "Implement a capability for a type you don't own, with no newtype wrapper."
- "Overlapping implementations coexist because each is a distinct provider; the context picks one, explicitly and locally."
- "The incoherence is deliberate at the definition level and disciplined at the use site — every choice is a line in a table, never a silent resolution."

Avoid claiming CGP is coherent, or promising global uniqueness. It deliberately drops global coherence for per-context choice, and a reader who expects "one instance per type, program-wide" will look for a guarantee CGP scopes rather than globalizes. Say plainly that uniqueness is *per context*, and frame the scoping as the point — it is what lets two contexts treat the same type two ways without conflict.

## Generic over a type's structure, checked and free

CGP can write code that is generic over a type's fields and variants — serialization, builders, visitors, conversions — without runtime reflection, which is the reflection payoff without the reflection cost. A type opts in with a derive, its shape becomes type-level data the trait system resolves against, and generic code recurses over that shape with full static checking and no runtime introspection, no stringly-typed field access, and nothing walked on every call. This is the selling point for the framework and tooling author, and the [reflection](../related-work/reflection.md) comparison carries the detail.

Say it like this:

- "Reflection's payoff without the runtime cost or the stringly-typed failures."
- "Write a framework once over any type that derives the shape — checked when you write it, not when it runs."
- "A type's structure becomes types the compiler resolves, not a descriptor you walk at runtime."

Avoid calling CGP "a reflection system" or "reflection for Rust." A reader sold on that will look for an API to query a type and find trait bounds instead. The precise framing is "compile-time structural reflection encoded in the type system," and the honest limit — it works only on types that opted in via a derive, and does no runtime introspection — belongs in the same breath, per [skepticism](skepticism.md).

## Abstract types chosen per context

A quieter but powerful selling point is that CGP lets generic code name a type — an error type, a scalar, a runtime — that each context fills in for itself, through the same wiring that selects behavior. Code written against an abstract `Error` or `Runtime` works unchanged whether a context chooses `anyhow::Error` or a custom enum, Tokio or a mock, and swapping the concrete type is a wiring change that touches no provider. This unifies type selection and implementation selection into one mechanism, which resonates with the ML-module and abstract-type audience and underpins CGP's modular error handling.

Say it like this:

- "Generic code names the type; the context chooses it — the same wiring that picks behavior picks types."
- "Swap your error type or your runtime by changing one wiring line, touching no logic."
- "Abstract over the error type so fallible code never commits to a concrete one."

Avoid conflating this with sealing or representation hiding. CGP's abstract types defer and configure a type; they do not hide a representation behind a type-system boundary the way an ML module's sealed type does. An ML-module reader will expect sealing, so say plainly that representation hiding is Rust's module privacy, separate from this feature — the [ML modules](../related-work/ml-modules.md) comparison makes the distinction.

## It works on stable Rust today

A plain but load-bearing selling point is that CGP is a library on stable Rust, not a language fork, an experimental feature, or a nightly-only trick. Much of what it delivers — type-class-style modularity, functor-style assembly, extensible data, the exactly-once fragment of effect handlers — corresponds to features that other ecosystems ship only in research languages or unmerged proposals, and CGP provides them as a crate you can add to a project now. This matters most to the evaluator weighing adoption risk and to the functional-programming reader who assumes this level of expressiveness requires a different language.

Say it like this:

- "A library on stable Rust — not a fork, not a nightly feature, not a proposal."
- "The expressiveness you'd reach for another language to get, available as a crate today."

Avoid overstating maturity while making this point. "Works on stable Rust today" is true and worth saying; "production-proven at scale" is a different claim that needs evidence, and the evaluator reader will hear the gap. Keep this selling point to what it is — availability without a toolchain gamble — and let the maturity conversation happen honestly where [skepticism](skepticism.md) handles it.

## Audience-tuned one-liners

The sharpest phrasings are the ones that translate a selling point into the exact idiom of a reader's background, because they let the reader reuse everything they already know and spend their attention only on what is new. Each of the following is calibrated to one profile from [reader-profiles.md](reader-profiles.md) and grounded in the matching [related-work](../related-work/README.md) comparison; reach for the one that fits the piece's audience and avoid using it on a different audience, where it will misfire.

- For the **Scala or implicits reader**: "implicits without the mystery — the dependency still arrives without threading it through every call, but which implementation supplies it is a line in a wiring table, not a resolution search." See [implicit parameters](../related-work/implicit-parameters.md).
- For the **Haskell or type-class reader**: "type classes without the orphan rule, and overlapping instances made legal." See [type classes](../related-work/type-classes.md).
- For the **Spring, Guice, or Dagger reader**: "Dagger, taken further — compile-time-checked injection, with per-context choice a single global binding graph can't express." See [dependency injection](../related-work/dependency-injection.md).
- For the **dynamic-language reader**: "duck typing that can't blow up at runtime, and dynamic dispatch that costs nothing." See [dynamic dispatch](../related-work/dynamic-dispatch.md).
- For the **OCaml or ML-module reader**: "functors with the plumbing automated — a declarative table instead of a hand-ordered functor chain." See [ML modules](../related-work/ml-modules.md).
- For the **algebraic-effects reader**: "effect handlers minus the continuation, resolved by type instead of dynamic scope, and extended to abstract types." See [algebraic effects](../related-work/algebraic-effects.md).
- For the **reflection or framework-author reader**: "compile-time reflection encoded in the type system — generic over a type's structure, checked when you write it, erased before it runs." See [reflection](../related-work/reflection.md).
- For the **PureScript or row-types reader**: "row polymorphism's extensibility, in a nominal systems language, opt-in per type." See [row polymorphism](../related-work/row-polymorphism.md).

## Phrasing principles

Across every selling point, a few wording rules hold, and they consolidate the per-section guidance into a checklist a writer can run before publishing. Prefer the concrete capability to the adjective — "compiled to a direct call" over "fast," "one interface with many implementations" over "flexible," "checked at compile time" over "safe" — because the concrete version is both more believable and more informative. State the honest limit in the same breath as the claim when the audience is skeptical, since the skeptic is scanning for the omission and trusts the writer who volunteers it.

Certain words reliably cause the misunderstandings the [skepticism](skepticism.md) document exists to prevent, and they should be avoided by default. Do not say CGP "automatically resolves," "finds," or "figures out" the implementation — it is explicit, and this is the single most common misframing. Do not call it "magic," a word this audience treats as a warning rather than a wonder. Do not flatly call it "a DI framework," "a reflection system," or "an effect system" — each invites the reader to expect runtime behavior CGP does not have; qualify each with "compile-time" and the distinguishing limit. Do not promise "no boilerplate," "replaces traits," or any unbenchmarked "faster than"; the true, smaller claims — "moves boilerplate into one place," "a superset of traits," "no runtime cost" — are the ones that survive scrutiny and, with this audience, persuade more for surviving it.
