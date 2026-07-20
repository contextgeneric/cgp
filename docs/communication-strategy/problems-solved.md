# Problems solved

This document catalogs the concrete problems CGP removes, written as short before-and-after stories a writer can lead with, so that any piece opens on a pain the reader already feels rather than on the paradigm.

## How to use these problems

The whole strategy is problem-first, and this is where the problems live: each entry is a familiar Rust pain, the awkward workaround a developer reaches for today, and the CGP version that removes it, sized down to the smallest honest before/after. This document is the third axis of the section, alongside the capabilities in [selling-points.md](selling-points.md) and the objections in [skepticism.md](skepticism.md) — a selling point says what CGP *can do*, an objection says what a reader *fears*, and a problem says what *hurts now*, which is the one a skeptic will actually follow. Reach for the problem that matches the reader before reaching for any selling point, because "here is the thing you fight every week, gone" persuades where "look what CGP can do" does not.

Three rules govern how to use an entry. Lead with the **pain, not the mechanism** — open on the workaround the reader recognizes, and let the CGP version arrive as relief rather than as a new thing to learn. Keep the **before/after small and honest** — a five-line diff on ordinary-looking code outperforms a full example, and the "before" must be code the reader would genuinely write, not a strawman that makes CGP look better than it is. And **name the limit in the same breath**, because every entry below is also a place a reader could over-apply CGP, and the honest boundary is what makes the win believable; when the plainer tool is the right one, [positioning.md](positioning.md) says so.

Each problem names the reader profile it lands with, the selling point and skepticism it maps to, and the honest cost, so a writer can move from a problem straight to the wording that sells it and the objection to defuse. The code shown uses the modern idioms the `/cgp` skill teaches — a provider written with [`#[cgp_impl]`](../reference/macros/cgp_impl.md), values read with [`#[implicit]`](../reference/attributes/implicit.md), wiring with [`delegate_components!`](../reference/macros/delegate_components.md) — because a "before/after" that showed dated CGP would teach the reader a dialect they must later unlearn.

## Mock in tests, run the real thing in production — without `dyn` or a framework

The most universal pain CGP removes is swapping a real implementation for a fake one across tests and production, which every Rust developer has solved awkwardly. The usual routes each cost something: a `Box<dyn EmailSender>` field pays runtime dispatch and infects the type with a trait object; a generic `<E: EmailSender>` parameter threads through every layer that touches it and multiplies as more dependencies join; and a dependency-injection crate brings machinery the Rust community broadly distrusts, as [attention-and-engagement.md](attention-and-engagement.md) records. CGP makes the swap a single wiring line, monomorphized to a direct call.

The capability becomes a component, and each environment wires the provider it wants:

```rust
#[cgp_component(EmailSender)]
pub trait CanSendEmail {
    fn send_email(&self, to: &str, body: &str);
}

#[cgp_impl(new SendViaSmtp)]
impl EmailSender { /* connect and send over SMTP */ }

#[cgp_impl(new RecordEmails)]
impl EmailSender { /* push to a Vec so a test can assert on it */ }

delegate_components! { App     { EmailSenderComponent: SendViaSmtp } }
delegate_components! { TestApp { EmailSenderComponent: RecordEmails } }
```

`App` sends real mail and `TestApp` records it; neither pays for `dyn`, the swap is one greppable line, and code that calls `self.send_email(..)` never changes. This is the entry to lead with for the **working developer** ([reader-profiles.md](reader-profiles.md)), it maps to the [swap-implementations selling point](selling-points.md), and it defuses the ["why not just use traits"](skepticism.md) reflex by showing the case where a plain generic would have proliferated. The honest limit: the choice is a line *you* write, not one CGP infers, and for a dependency with exactly one implementation a plain trait is still the right tool.

## Implement a trait for a type you don't own — no newtype dance

A pain sharp enough that Rust developers hand-roll CGP's own mechanism to escape it is the orphan rule: you cannot implement a foreign trait for a foreign type, so adding behavior to a type from another crate means wrapping it in a newtype and re-exposing every method you still need — tedious, and it clutters the code with a wrapper the rest of the program must thread around. This is not a theoretical annoyance: there is a repository cataloguing the rule's design problems and a steady stream of posts on the workaround, and one Rust author [independently reinvented CGP's exact marker-struct pattern](attention-and-engagement.md) to get around it, calling the hand-rolled version "3 extra lines to link things together."

CGP dissolves the orphan rule because a provider implements the *provider* trait for its own zero-sized marker, not the target trait for the foreign type, so coherence never bites — a crate can add a capability to a type it does not own with no wrapper. The mechanics are the subject of [bypassing coherence](../concepts/coherence.md), and the pitch's power is recognition: the reader has written the three-line workaround and would rather not maintain it. This lands with the **working and advanced developer** ([reader-profiles.md](reader-profiles.md)), maps to the [overlapping-and-orphan selling point](selling-points.md), and answers the ["coherence exists for a reason"](skepticism.md) objection by locating CGP's discipline. The honest limit: CGP does not repeal coherence globally; it keeps each choice explicit and per-context, and where a program genuinely wants one instance program-wide, a coherent trait is the better fit.

## Give one interface many implementations that would otherwise collide

Closely related but distinct is wanting *several* implementations of one capability at once — the case Rust forbids outright, because two blanket impls that could overlap are rejected even in theory. A developer who wants to serialize a value three ways, or handle an error type by several strategies, hits a wall the language will not let them past without the marker-struct contortion. CGP is built for exactly this: overlapping providers coexist because each is a distinct marker type, and a context selects one explicitly, so the choice is unambiguous locally without global uniqueness.

The clearest illustration is per-value dispatch, where two applications encode the same type differently, each coherent within itself:

```rust
delegate_components! {
    AppA {
        open ValueSerializerComponent;
        @ValueSerializerComponent.Vec<u8>: SerializeHex,
    }
}

delegate_components! {
    AppB {
        open ValueSerializerComponent;
        @ValueSerializerComponent.Vec<u8>: SerializeBase64,
    }
}
```

`AppA` serializes bytes as hex and `AppB` as base64, and the overlapping providers never conflict because each context names one. This is the standout for the **type-system and functional-programming reader** ([reader-profiles.md](reader-profiles.md)), it maps to the [overlapping-and-orphan selling point](selling-points.md) and its "type classes without the orphan rule" one-liner, and it is the case [type classes](../related-work/type-classes.md) frames in full. The honest limit: this is genuinely more machinery than a single trait needs, so it earns its place only when the several implementations are real — the boundary [positioning.md](positioning.md) draws.

## Swap your error type or runtime by changing one line

A quieter pain, felt hardest in libraries and reusable cores, is that fallible code commits to a concrete error type early and then cannot easily change it, so a decision to move from `anyhow::Error` to a custom enum, or from one async runtime to another, ripples through every signature. CGP makes the error type — or the runtime, or any cross-cutting type — abstract, chosen by the same wiring that selects behavior, so the concrete choice lives in one place and generic code never names it.

Code written against an abstract [`HasErrorType`](../reference/components/has_error_type.md) works unchanged whichever error a context wires:

```rust
delegate_components! {
    App {
        ErrorTypeProviderComponent: UseType<anyhow::Error>,
    }
}
```

Switching that line to a custom `AppError` retargets every fallible provider in the context, touching no logic. This resonates with the **ML-module and systems reader** ([reader-profiles.md](reader-profiles.md)) and underpins CGP's [modular error handling](../concepts/modular-error-handling.md); it maps to the [abstract-types selling point](selling-points.md) and attaches to the live `anyhow`-versus-`thiserror` conversation ([attention-and-engagement.md](attention-and-engagement.md)). The honest limit: CGP defers and configures the type, it does not seal a representation the way an ML module does — hiding a representation is still Rust's module privacy, and the wiring keys live under `cgp::core::error`, not the prelude, so a piece that shows this should import them.

## Break up a trait that grew into a monolith

A pain that arrives with a codebase's age rather than its domain is the trait that accreted responsibilities until every implementor must supply the whole surface and every change touches all of them — the "god trait" that resists decomposition because splitting it breaks impls and threads new generic parameters everywhere. CGP lets one large capability become a set of independently wired components, each with its own providers, so an implementor supplies only what it uses and a context assembles the pieces without a single monolithic impl.

Because each component is wired separately, a context can compose a behavior from many small providers and reuse each across contexts, and adding a capability is adding a wiring line rather than editing a shared trait every type implements. This is the decoupling pitch for the **framework author and the evaluator weighing maintainability** ([reader-profiles.md](reader-profiles.md)); it maps to the [gradual-adoption selling point](selling-points.md) and the decoupling benefits, and it rests on the [consumer/provider split](../concepts/consumer-and-provider-traits.md). The honest limit: decomposition is a judgment call, and a small, stable trait with one implementation should stay a plain trait — the [modularity hierarchy](../concepts/modularity-hierarchy.md) is the map of when the split pays, and [positioning.md](positioning.md) draws the line for a marketing piece.

## Write a framework over any type's structure — without runtime reflection

For the author of a serialization library, a builder, an ORM, or a configuration system, the recurring pain is code that must work over the *shape* of a user's type without a general reflection facility Rust lacks, which pushes them toward per-type derive macros, stringly-typed field access, or a reflection crate that walks a descriptor at runtime. CGP encodes a type's fields and variants as type-level data the trait system resolves against, so a framework written once recurses over any type that opts in with a derive, fully checked when written and with nothing walked on each call.

A type opts in with a derive, its structure becomes a type-level list, and generic code processes it with static checking and no runtime introspection — the payoff of reflection without its cost, developed in [extensible records](../concepts/extensible-records.md) and [extensible variants](../concepts/extensible-variants.md). This is the entry for the **framework and tooling author** ([reader-profiles.md](reader-profiles.md)), it maps to the [generic-over-structure selling point](selling-points.md), and it sits in the live reflection-and-comptime conversation ([attention-and-engagement.md](attention-and-engagement.md), [reflection](../related-work/reflection.md)). The honest limit, stated in the same breath: it works only on types that derive the shape and does no runtime introspection, so a reader must not be sold "reflection for Rust" and then look for an API to query a type — the precise frame is "compile-time structural reflection encoded in types."

## Keep a provider's dependencies out of your public API

A subtler pain, felt by library authors, is that trait-based dependency injection leaks: because any type named in a public trait's method signature must itself be public, exposing a dependency through a generic trait forces internal types into the public API and breaks encapsulation, a cost [documented with its source](attention-and-engagement.md) in the Rust community. CGP keeps a provider's dependencies as impl-side constraints — declared where the implementation lives, through [`#[uses]`](../reference/attributes/uses.md) and [`#[implicit]`](../reference/attributes/implicit.md), rather than in the consumer trait's signature — so the public interface stays clean while the requirements are still explicit and compiler-checked.

A caller who bounds on the consumer trait sees only the capability, never the dependency types the provider happens to need, so a library can require an internal helper without publishing it. This is a targeted pitch for the **library author** and complements the [explicit-and-compiler-checked-dependencies selling point](selling-points.md), which now carries the encapsulation angle; it rests on [impl-side dependencies](../concepts/impl-side-dependencies.md). The honest limit: this is a narrower, more advanced benefit than the others here, so it belongs in a piece aimed at library authors rather than in a general hook, where it would land as abstract.

## Read a CGP compile error without decoding a wall of generated types

The pain that has done CGP the most adoption damage lives not in the language but in its errors: a small wiring mistake — a context missing one field a provider needs — expands, through the generated code, into screens of diagnostics naming `IsProviderFor`, `CanUseComponent`, and a nested `Symbol<…>` spine the programmer never wrote, with the real cause buried in the machinery or, in the worst class, suppressed entirely. A developer who meets that wall on their first mis-wire often concludes CGP is unusable and leaves, which is why this is the pain most worth showing removed.

The "before" is a `Rectangle` context wired to an area calculator whose provider reads a `height` field the struct does not have. Under plain `cargo check`, the failure surfaces as an `E0277`/`E0599` cascade that names the consumer and provider traits but not the missing field — the [hidden unsatisfied-dependency class](../errors/hidden/unsatisfied-dependency.md), where the default trait solver omits the cause rather than merely burying it. The "after" is the same mistake through CGP's error toolchain:

```text
$ cargo cgp check
error[E0277]: [CGP-E001] the consumer trait `CanCalculateArea` is not implemented for context `Rectangle`
   = note: root cause: [CGP-E106] missing field `height` on `Rectangle`
           this is required through the dependency chain:
             [CGP-E101] consumer trait impl `CanCalculateArea` for context `Rectangle`
             └─ [CGP-E102] provider trait impl `AreaCalculator` with context `Rectangle` for provider `RectangleArea`
               └─ [CGP-E106] missing field `height` on `Rectangle`
```

[`cargo-cgp`](https://github.com/contextgeneric/cargo-cgp) turns on Rust's next-generation trait solver to un-hide the suppressed cause, then leads with it — naming the missing `height` field and drawing the dependency chain that reaches it as a `cargo tree`-style tree, each line tagged with a `[CGP-Exxx]` code. This maps to the [tooling selling point](selling-points.md) and answers the ["wall of generated types"](skepticism.md) objection head-on, for every reader but most sharply the **working developer and the evaluator** ([reader-profiles.md](reader-profiles.md)) weighing whether a team can live with the diagnostics. The honest limit, stated in the same breath: `cargo-cgp` is v0.1.0-alpha and reshapes the classes it recognizes — the core wiring errors — while some, orphan-rule errors among them, still pass through as the compiler wrote them, so the honest claim is that the error experience is dramatically better and improving, not that it is solved. The reference [`cargo-cgp.md`](../reference/cargo-cgp.md) and the [error catalog](../errors/README.md) carry the fuller picture.

## Choosing which problem to lead with

The last decision is which of these to open on, and it is settled by the reader, not by which problem is most impressive. Name the one or two dominant profiles for the piece and its channel first, per [reader-profiles.md](reader-profiles.md), then pick the problem that reader feels most sharply: the mock-in-tests swap for the pragmatic majority, the orphan-rule escape for the trait-heavy developer, overlapping implementations for the type-system reader, the error-type swap for the systems and ML-module reader, the monolith decomposition for the evaluator, and the structure-generic framework for the tooling author — and the readable-errors story for the evaluator or any reader who has heard CGP's diagnostics are unusable. When a piece must serve a broad public audience, lead with the mock-in-tests swap or the orphan-rule escape, because they are the most widely felt and the least likely to read as astronaut architecture — and hold the more advanced problems for the channels where their reader gathers.
