# Skepticism

This document catalogs the skepticisms a reader brings to CGP, judges honestly whether each is justified, and prescribes the wording that answers it without triggering the misunderstanding that fed it.

## How to use this document

Skepticism is the default reaction to CGP, not the exception, so a writer's job is less to avoid objections than to meet them before the reader raises them. The skepticisms come from three sources, and telling them apart decides how to respond. Some are **imported** from a paradigm CGP resembles — the reader was burned by dependency-injection magic or spooky implicit resolution and transfers that wound to CGP on sight. Some are **native** to the Rust community's culture, which is proudly wary of complexity, macros, and abstraction that does not earn its keep. And some point at **genuine costs** CGP really carries, which no wording can spin away and which the writer must concede plainly. The worst mistake is treating a justified skepticism as a misunderstanding to argue down; the reader knows the difference, and the attempt destroys the trust the [selling-points](selling-points.md) rest on.

For each skepticism below, the document names where it comes from, judges whether it is justified — fully, partly, or only through a misunderstanding — and gives the wording that responds. Two response patterns recur. When a skepticism rests on a **misunderstanding** (usually an assumption that CGP behaves at runtime like the thing it resembles), the fix is precise wording that heads off the wrong mental model before it forms. When a skepticism rests on a **real cost**, the fix is to concede it, then reframe it as the deliberate price of a benefit the reader values — the trade the reader would make anyway if they saw it whole. This document is the mirror of [selling-points.md](selling-points.md); most objections here answer a claim there, and the two must stay consistent so a writer is never told to say a phrase that the other warns against.

## Skepticisms imported from other paradigms

Many readers meet CGP already carrying a grievance against a concept it resembles, and their skepticism is really about that concept. The related-work documents record these grievances in depth and with citations; the task here is to recognize which imported wound is firing and to word CGP so the reader does not simply re-live it. In almost every case the reader's original complaint is *justified about the other tool* and *misapplied to CGP*, and the wording must grant the first while correcting the second — never by dismissing their experience, always by showing where CGP's design diverges from the thing that hurt them.

### "This is dependency injection, and DI is heavy, magic, and fails at runtime"

This skepticism comes from the enterprise reader's real scars: reflection-based containers that hide dependencies, fail at startup with an exception, cost time scanning a classpath, and behave as "magic" no one can debug. The complaint is thoroughly justified about the frameworks that provoked it, and entirely misapplied to CGP, which has no container, no reflection, and no runtime graph — its wiring is resolved by the compiler and compiled away, and a missing dependency is a compile error at the wiring site. The danger is that flatly calling CGP "dependency injection" makes the reader import the whole runtime picture, cost and failure mode included.

Word it by naming the difference before the category. Say "compile-time, reflection-free dependency injection" rather than "dependency injection," and defuse the runtime-container assumption explicitly: there is no object holding a graph, the container is the type system, and resolution happens during compilation. Lead with the pains they resent — hidden dependencies, the startup `NullPointerException`, the "magic" they cannot trace — and show CGP answers each by construction. The framing that lands, per [dependency injection](../related-work/dependency-injection.md), is "Dagger, taken further": a reader who accepts Dagger's compile-time-checked, reflection-free injection already accepts CGP's bargain.

### "Implicit resolution is spooky — values appear from nowhere"

This comes from the Scala or Haskell reader who has spent an afternoon tracking down which `given` was selected or why an expected instance was not, and it is justified about implicit resolution: the mechanism's generality is exactly what makes it hard to follow. It does *not* transfer to CGP, but for a reason the writer must state carefully, because the obvious reassurance is a trap. CGP does not have spooky resolution because it does not resolve automatically at all — the provider is named explicitly in a wiring table. The misunderstanding to prevent here is the opposite of the usual one: the reader may hope CGP "just finds" the implementation, and it does not.

Word it as "implicits without the mystery," then immediately anchor the honesty: the dependency still arrives without being threaded through every call, but *which* implementation supplies it is a line you can read, not the outcome of a search. Never promise automatic resolution — a reader sold on "CGP finds the right one" feels misled at the first `delegate_components!` entry, and the misframing also undercuts the coherence-freedom selling point that depends on there being no global search. The [implicit parameters](../related-work/implicit-parameters.md) comparison frames the trade: explicit wiring is the feature, not a shortfall.

### "Coherence exists for a reason — incoherent instances are dangerous"

This is the informed skepticism of the Haskell or advanced-Rust reader who knows that global coherence is what keeps a `Set` from being corrupted by two orderings, and that Haskell's `INCOHERENT` and Lean's diamonds let a wrong instance be chosen silently. It is justified in general — dropping coherence naively *is* dangerous — and it is the objection most worth engaging on its merits rather than deflecting, because the reader raising it is precisely the one who can become an advocate or a detractor. CGP's answer is that it drops global coherence but not safety: because every provider choice is explicit and per-context, incoherence never means the silent, searched-and-wrong selection that makes the extensions dangerous.

Word it by conceding the general point and locating CGP's discipline precisely. Say that CGP is deliberately incoherent at the definition level and disciplined at the use site — overlapping providers coexist, but a context names exactly one, explicitly and locally, so the choice can never be silently derailed by overlap. Do not claim CGP is coherent, and do not promise program-wide uniqueness; say plainly that uniqueness is per context, and frame that scoping as the point. The [type classes](../related-work/type-classes.md) and [bypassing coherence](../concepts/coherence.md) documents carry the full argument, including the honest case where genuine global uniqueness is what a program wants and coherent type classes are the better tool.

### "Extensible records and rows mean gigantic error messages and specialist complexity"

This comes from the PureScript or Gleam reader who has seen row-unification errors clog a terminal and watched extensible records earn a reputation as a tool you regret reaching for. It is *partly justified* against CGP, and this is one to concede rather than wave away: CGP's structural operations, when mis-wired, do surface as long, generated-type-heavy trait errors — the CGP-idiom echo of the row-unification message that pushed two languages away from rows. What CGP changes is that the shape is opt-in per type rather than pervasive, and that [`check_components!`](../reference/macros/check_components.md) forces a missing field or variant to be named at the wiring site instead of deep inside a use.

Word it by owning the diagnostic cost and bounding it. Say that CGP inherits verbose trait errors, that a check localizes them to the wiring site, and that because the structural view is opt-in you pay the complexity and the error-message tax only where you use the power — not across the whole program, as a row kind would impose. Do not claim the errors are clean; the reader has seen this movie and will trust the writer who admits the ending. The [row polymorphism](../related-work/row-polymorphism.md) comparison frames it as extensibility brought to a nominal language, paid for only where used.

### "Isn't this just an effect system / a reflection system?"

These two come from readers fluent in algebraic effects or in reflection, and both are misunderstandings that a careless pitch invites. The effects reader will look for continuations — `resume`, multi-shot handlers, generators, async — and CGP has none; a provider is an ordinary function that returns exactly once. The reflection reader will look for an API to query a type at runtime, and CGP has none; the structure is types the compiler resolves, and only for types that opted in via a derive. Neither skepticism is a real deficiency, because CGP never claimed to be either thing — but a writer who calls it "an effect system" or "reflection for Rust" creates the false expectation and then owns the reader's disappointment.

Word it by naming what CGP is with precision and stating the limit up front. For effects: "the exactly-once, resume-in-place fragment of effect handlers — dynamic binding, made static and per-context," with the plain admission that continuations, generators, and async are outside it (see [algebraic effects](../related-work/algebraic-effects.md)). For reflection: "compile-time structural reflection encoded in the type system," with the plain admission that it does no runtime introspection and works only on types that derive the shape (see [reflection](../related-work/reflection.md)). In both cases the honesty is the persuasion: a reader who knows how much machinery exists to tame continuations, or how much runtime cost reflection carries, hears "we don't do that part" as a considered trade.

### "Dynamic dispatch is flexible — won't a static version lose what makes it useful?"

This comes from the dynamic-language reader who values runtime openness: heterogeneous collections, plugins loaded at startup, live redefinition, metaprogramming. Their skepticism is *justified and must be granted fully*, because CGP genuinely cannot do these things — it resolves at compile time, so there is no runtime object graph to mutate and no runtime dispatch to intercept. What CGP offers instead is the decoupling of dynamic dispatch with none of its cost or its runtime failure modes, which is a different bargain, not a replacement for runtime dynamism.

Word it by drawing the line honestly and selling what is on CGP's side of it. Say plainly that runtime openness — plugins, heterogeneous collections, monkey-patching — lives on the runtime side, where Rust's own `dyn Trait` and the dynamic languages remain the right tools, and that CGP is for the case where the set of implementations is known at build time. Then offer the pair this reader most wishes for: "duck typing that can't throw at runtime" and "dynamic dispatch that costs nothing." Frame CGP's late binding as late to the *wiring site*, not to runtime — the flexibility is real but spent at compile time. The [dynamic dispatch](../related-work/dynamic-dispatch.md) comparison holds the full mapping.

## Skepticisms native to the Rust audience

Beyond the imported wounds, the Rust community brings its own well-earned wariness, and these skepticisms fire even for a reader with no functional-programming or enterprise background. This culture has watched abstraction ruin codebases and macros obscure behavior, and it treats "clever" as a criticism. Several of these skepticisms are partly or fully justified, and the writing must earn trust by conceding the true part rather than by insisting the complexity is free.

### "This is too clever / over-engineered / astronaut architecture"

This is the reflex of the pragmatic majority, and it is a reasonable prior rather than a misunderstanding: most abstractions that announce a new paradigm do not earn their cost, and the reader is right to demand that CGP prove it does. The skepticism is justified as a *default to be overcome with evidence*, not as a verdict. The failure mode is answering it with more enthusiasm about the paradigm, which confirms exactly the fear.

Word the response as problem-first restraint. Lead with a concrete, familiar pain and show it gone in a before/after on ordinary-looking code; let the modularity justify itself by removing something real rather than by being impressive. Say explicitly when *not* to reach for CGP — a capability with one implementation does not need it — because naming the boundary is what proves the tool is not being oversold. The reader who sees the author decline to apply CGP everywhere believes the author about where it does belong.

### "Macros are magic — I can't see what they generate or debug it"

This skepticism is partly justified: CGP is macro-driven, and generated code is genuinely harder to inspect and step through than hand-written code, which the macro-wary Rust reader is right to weigh. What tempers it is that the expansions are documented and deterministic, the generated traits and impls are ordinary Rust once emitted, and a provider can be written and read without ever seeing the macro output. But the honest position concedes that debugging generated code is a real cost.

Word it by replacing "magic" with "explicit" and pointing at the seams. Never use the word "magic" approvingly — this audience reads it as a warning — and instead emphasize that the wiring is a table you write and read, the dependencies are declared, and the expansion is specified. Concede that generated code is harder to trace, and offer the mitigations honestly: the expansions are documented, and the vanilla-looking idioms keep most code readable without expansion. Overclaiming transparency here loses the exact reader you are addressing.

### "I can't tell which code actually runs on a method call"

This is a specific, well-earned worry distinct from the macro complaint, and it appeared verbatim in CGP's own community discussion (see [attention-and-engagement.md](attention-and-engagement.md)): a reader fears that with wiring indirection it becomes "basically impossible to tell which particular bit of code will be entered on a method call," which makes a codebase hard to navigate. It is *partly justified* — CGP does add a layer between a consumer-trait call and the provider that answers it, so a reader tracing execution has one more hop to follow than with a direct call, and denying that indirection would be dishonest.

Word it by conceding the hop and pointing at the map. The indirection is real, but unlike runtime dynamic dispatch it is *statically resolved and explicit*: the [`delegate_components!`](../reference/macros/delegate_components.md) table is a single, greppable place that names exactly one provider for each component, so "which code runs" is a table lookup that a reader — or an IDE's go-to-definition — follows at compile time, not a runtime search over registered handlers. Frame the wiring table as the navigation aid it is, the one location that answers "what implements this here," rather than pretending the indirection is absent. This is the same explicitness that answers the implicit-resolution and "magic" objections and that the [swap-implementations selling point](selling-points.md) calls "one greppable place": the choice is written down, in one spot, and can never be silently redirected.

### "What does this do to compile times?"

This is a justified and specific concern, not a reflex: trait-resolution-heavy code and macro expansion do add compile-time work, and a reader who has waited on a slow Rust build is right to ask. Pretending otherwise is the kind of dodge that ends trust.

Word it by conceding the direction and declining to invent a magnitude. Acknowledge that CGP adds compile-time work, do not quote a number you cannot cite, and reframe against the runtime it replaces: the resolution that costs at compile time is resolution that would otherwise cost at runtime or not be checked at all. If a concrete measurement exists for the piece at hand, cite it; if not, keep the claim qualitative and honest rather than reassuring and unsupported.

### "The error messages are a wall of generated types"

This is justified, and it is the cost most likely to bite a real user, so it must be conceded without flinching. A mis-wired context can produce long, generated-type-heavy diagnostics, and a reader who has hit one will not believe a writer who pretends the experience is pleasant.

Word it by owning the problem and naming the mitigation precisely. Say that CGP's errors can be verbose, that [`check_components!`](../reference/macros/check_components.md) exists specifically to force the real cause — a missing field, type, or dependency — to be named at the wiring site rather than surfacing far away, and that this localizes the failure even though it does not make the message short. This is honest and still reassuring: the tool has a known sharp edge and a known way to blunt it. Claiming the diagnostics are good is the fastest way to lose the technical reader.

### "Rust doesn't need dependency injection — that's a Java-ism"

This is distinct from the enterprise reader's imported fear of runtime DI magic, and it is the *native* Rust version: the pragmatist who has read that idiomatic Rust already does dependency injection with plain traits and generics, and who therefore treats the whole category — and any crate that markets itself with the term — as an unwanted import from the Java and Spring world. The sentiment is real and widespread, and it is *partly justified*: for a great many cases, passing a trait-bounded generic or a trait object genuinely is the right tool, and a DI *framework* would be over-engineering, which is why Rust's own DI crates stay niche (see [attention-and-engagement.md](attention-and-engagement.md)). The trap is that flatly pitching CGP as "dependency injection" invites this reader to file it under "solution to a problem Rust doesn't have" and move on.

Word it by agreeing, not arguing. Concede that Rust does not need a DI *framework* and that CGP is not one — it is the traits-and-generics approach the reader already endorses, carried to the cases where doing it by hand stops scaling. Then locate those cases concretely: when a capability needs several interchangeable implementations chosen per context, plain generics hit the coherence wall that forces the marker-struct workaround developers already hand-roll ([attention-and-engagement.md](attention-and-engagement.md)); when the dependency lives on a type you do not own, the orphan rule blocks it; and when a public trait carries its dependencies in its signature, it leaks internal types into the public API, the encapsulation cost documented in [selling-points.md](selling-points.md). CGP earns its place at exactly those boundaries, and nowhere below them. The honest frame, per [dependency injection](../related-work/dependency-injection.md), is that CGP is the trait-based approach this reader already prefers, with the boilerplate and the coherence contortions removed — not a framework asking them to think like a Java developer.

### "Why not just use traits, generics, or an enum?"

This is the sharp, fair challenge of a competent Rust developer, and often the right question: for many problems, plain traits or an enum genuinely are the better tool, and CGP would be over-engineering. The skepticism is justified whenever the problem has one implementation or a closed, small set of them, and the writing must not pretend CGP wins universally.

Word it by conceding the range where the plainer tool wins and drawing the line where CGP starts to pay. Say that a single implementation belongs in an ordinary trait, a closed variant set in an enum, and that CGP earns its cost when a capability needs several interchangeable implementations chosen per context, or an implementation for a type you do not own, or coherence-free overlap — the cases plain traits handle awkwardly or not at all. The [modularity hierarchy](../concepts/modularity-hierarchy.md) is the honest map of when to climb from a plain trait to a full component, and pointing to it signals that CGP is a rung to reach for deliberately, not a default.

### "It's immature — can I bet a codebase on it?"

This is the evaluator's central, justified concern, and it is about risk the reader would own: a young paradigm, an evolving ecosystem, and the cost of a team learning something novel. It cannot be answered by enthusiasm, and it should not be minimized.

Word it with candor and with the escape hatches. Be honest that CGP is young and that adoption is a real decision, then lower the stakes truthfully: it is a superset of ordinary traits, so it can be adopted incrementally in one corner and stepped back from without rewriting a codebase, and it imposes no runtime, so it does not lock a project into a framework's lifecycle. Address the "can my team learn this" question directly rather than letting it linger. The evaluator discounts hype by profession, per [reader-profiles.md](reader-profiles.md), so the only register that works is level.

### "There's a learning curve"

This is justified and should simply be granted: the consumer/provider split, wiring, and coherence-bypassing are a genuine conceptual load, and a reader who suspects a ramp-up is right. Denying it insults the reader; the useful move is to shrink the *first* step rather than the whole curve.

Word it by separating the on-ramp from the depth. Say that the first useful thing — a capability defined as a function, wired once — takes only ordinary Rust knowledge, and that the deeper machinery can be learned as needed rather than up front. Point the newcomer at the gentle path, not the full reference. The honest message is that the curve is real but not a cliff, and that a reader can be productive well before they understand the whole system.

## A careful-wording checklist

The skepticisms above share a small set of wording failures that reliably create or inflame them, and a writer can catch most problems by scanning a draft against this list before publishing. The through-line is that vague or grand wording invites the reader to supply the worst interpretation, while precise wording forecloses it — and that conceding a real cost in plain words buys more trust than any amount of polish.

The words and moves to avoid, each paired with what to do instead, are worth keeping in view:

- Do not say CGP "automatically resolves," "finds," or "figures out" an implementation. Say the provider is named explicitly, in one readable place — the explicitness is the feature.
- Do not use "magic," even admiringly. Say "explicit," and point at the wiring table and the declared dependencies.
- Do not flatly call CGP "a DI framework," "a reflection system," or "an effect system." Qualify each — "compile-time, reflection-free dependency injection," "compile-time structural reflection," "the dynamic-binding fragment of effect handlers" — and state the distinguishing limit in the same breath.
- Do not imply any runtime component. Say the wiring is resolved at compile time and erased; a wired call is a direct call.
- Do not claim "no boilerplate," "replaces traits," or an unbenchmarked "faster." Say "moves boilerplate into one place," "a superset of traits," and "no runtime cost."
- Do not pretend the diagnostics are clean or the compile-time cost is nil. Concede both, and name [`check_components!`](../reference/macros/check_components.md) and the compile-time-versus-runtime trade as the honest mitigations.
- Do not sell CGP as universally better than plain traits, or promise runtime dynamism it lacks. Name where the plainer tool or a runtime mechanism wins; the concession is what makes the rest believable.
