# Worked examples: annotated model drafts

This document assembles the section's guidance into finished pieces of public writing — a launch post, a README above the fold, a social thread — and annotates each so a writer can see every decision and trace it to the document that argues for it.

## How to read these drafts

These drafts exist because the rest of the section is prescriptive and abstract, and a marketing-naive expert learns faster from one finished model than from twelve documents of rules. Each is a model of one artifact from [formats.md](formats.md), written the way that format's playbook prescribes, then followed by a note that names each move and its source. Read the draft first as a reader would, front to back and fast; then read the annotations to see the machinery behind it. The point is not to copy the copy but to watch the whole apparatus operate at once, so you can run it yourself on your own piece.

Treat every draft as an illustration, not approved copy, because two honesty caveats travel with each one. The tag line they use is the chosen one from [tag-lines.md](tag-lines.md); it is settled for consistency's sake, but publication remains the real measurement of any hook, so treat the copy as a strong starting point rather than a guarantee — [attention-and-engagement.md](attention-and-engagement.md) explains why. And every CGP claim and code snippet is bound by the knowledge base's synchronization rule exactly as a selling point is: verify it against the source and the `/cgp` skill before it ships, because a stale claim in a model draft is copied straight into real copy.

The code uses the modern idioms the `/cgp` skill teaches, so a reader who borrows a draft learns the current idiom rather than a dialect to unlearn. A provider is written with [`#[cgp_impl]`](../reference/macros/cgp_impl.md), a value is read with [`#[cgp_auto_getter]`](../reference/macros/cgp_auto_getter.md) or an [`#[implicit]`](../reference/attributes/implicit.md) argument, and wiring is a [`delegate_components!`](../reference/macros/delegate_components.md) table — the same forms [problems-solved.md](problems-solved.md) uses, and for the same reason.

## The link-aggregator launch post

The launch post on Lobsters or the Rust subreddit is the format to get right first, because it is where CGP is actually discussed and its reader is the pragmatic skimmer who forms and broadcasts a snap judgment ([attention-and-engagement.md](attention-and-engagement.md)). The draft below leads with a pain, shows the workaround the reader already knows, and concedes the boundary in the same breath, per the [launch-post playbook](formats.md).

### The draft

> **Pluggable trait implementations for Rust, at compile-time**
>
> Every Rust project eventually needs to swap a real implementation for a fake one — the real email sender in production, a recording stub in tests. The usual routes each cost something. A `Box<dyn EmailSender>` pays for dynamic dispatch and spreads a trait object through your types. A generic `<E: EmailSender>` parameter has to be threaded through every layer that touches it, and it multiplies as more dependencies join. A dependency-injection crate brings machinery most of us would rather not.
>
> CGP makes the swap a single line. You define the capability once:

```rust
#[cgp_component(EmailSender)]
pub trait CanSendEmail {
    fn send_email(&self, to: &str, body: &str);
}
```

> write as many implementations as you need, each an ordinary-looking impl:

```rust
#[cgp_impl(new SendViaSmtp)]
impl EmailSender { /* connect and send over SMTP */ }

#[cgp_impl(new RecordEmails)]
impl EmailSender { /* record each message so a test can assert on it */ }
```

> and let each context pick the one it wants:

```rust
delegate_components! { App     { EmailSenderComponent: SendViaSmtp } }
delegate_components! { TestApp { EmailSenderComponent: RecordEmails } }
```

> `App` sends real mail; `TestApp` records it. Neither pays for `dyn`, the choice is one greppable line, and the code that calls `self.send_email(..)` never changes.
>
> To be clear about the cost: this is more than a single trait needs. For a dependency with one implementation, a plain trait is still the right tool, and CGP would be over-engineering. It earns its keep when the implementations multiply, when the choice must differ per context, or when you're implementing a trait for a type you don't own and the orphan rule blocks you. All the wiring is resolved at compile time and compiled to direct calls — there is no runtime container, no reflection, and nothing left in the binary for a provider you don't use.
>
> It's a library on stable Rust, and it's a superset of ordinary traits, so you can use it in one corner and leave the rest of your code unchanged.
>
> Quickstart: [link]

### How the draft applies its guidance

Every choice above is a rule from elsewhere in the section, and naming them shows how to reproduce the result rather than the wording. The moves, in reading order:

- **The title is a concrete capability, not the paradigm name.** "Pluggable trait implementations for Rust, at compile-time" is the concrete-capability half of the chosen tag line in [tag-lines.md](tag-lines.md), used over "context-generic programming" because the title is the whole pitch for most of this audience, per the [launch-post playbook](formats.md).
- **It opens on a pain, not the paradigm.** The mock-in-tests swap is the most universal problem CGP removes and the entry to lead with for the working developer, drawn straight from [problems-solved.md](problems-solved.md) and keyed to that [reader profile](reader-profiles.md).
- **It shows the workarounds the reader already writes.** Naming `Box<dyn>`, the threaded generic, and the DI crate lets the CGP version arrive as relief rather than as a new thing to learn — the before/after discipline of [problems-solved.md](problems-solved.md).
- **It concedes the cost and the boundary in plain words.** "This is more than a single trait needs … CGP would be over-engineering" defuses the "why not just traits" and "over-engineered" reflexes from [skepticism.md](skepticism.md) by drawing the line [positioning.md](positioning.md) draws — and the concession is what makes the rest believable to this audience.
- **It states the zero runtime cost precisely.** "Resolved at compile time … no runtime container, no reflection, nothing left in the binary" is the [zero-runtime-cost selling point](selling-points.md), worded to avoid implying any runtime component, per the [vocabulary](vocabulary.md) avoid-list.
- **It calls the wiring "one greppable line."** That answers the "which code actually runs" traceability worry from [skepticism.md](skepticism.md) before it is raised.
- **It closes on stable Rust and gradual adoption.** "A library on stable Rust … a superset of ordinary traits" pairs two selling points that lower the evaluator's adoption risk ([selling-points.md](selling-points.md), the "Still Ordinary Rust" [key feature](key-features.md)).
- **The call to action is the quickstart, not the paradigm.** The skimmer is ready only for a low-commitment look, so the [conversion ladder](formats.md) asks for that and nothing heavier.

It is as notable for what it avoids: no paradigm name in the opener, no "magic," no "automatically resolves," and no unqualified "DI framework" — each an entry on the [vocabulary](vocabulary.md) avoid-list because each invites a dismissal.

## The README above the fold

The README's first screen is where an evaluator and a skimmer both decide in seconds whether to continue, so it must carry the tag line, the headline features, and runnable code before the reader scrolls ([formats.md](formats.md)). The draft models only the part above the fold — the layout term the [glossary](glossary.md) defines — because that is where the decision is made.

### The draft

> **Context-Generic Programming**
>
> *A language extension for Rust, with pluggable trait implementations at compile-time.*
>
> Still ordinary Rust — no nightly, no fork, and zero runtime cost. `cargo add cgp`
>
> ---
>
> **One Interface, Many Implementations.** Write many interchangeable implementations of the same interface and choose between them per context, with the overlapping and orphan implementations Rust normally forbids made safe because every choice is explicit and local.
>
> **Zero-Cost Abstraction.** Everything is resolved at compile time and compiles down to direct calls, so the flexibility costs nothing at runtime and unused providers never reach the binary.
>
> **Still Ordinary Rust.** A superset of ordinary traits you adopt one piece at a time: providers read like normal impls and implicit arguments like normal parameters, so a codebase can use it in one corner and stay otherwise vanilla.
>
> *(The full headline set is in [key-features.md](key-features.md).)*

```rust
use cgp::prelude::*;

#[cgp_component(Greeter)]
pub trait CanGreet {
    fn greet(&self) -> String;
}

#[cgp_auto_getter]
pub trait HasName {
    fn name(&self) -> &str;
}

#[cgp_impl(new GreetHello)]
#[uses(HasName)]
impl Greeter {
    fn greet(&self) -> String {
        format!("Hello, {}!", self.name())
    }
}

#[derive(HasField)]
pub struct Person {
    pub name: String,
}

delegate_components! {
    Person {
        GreeterComponent: GreetHello,
    }
}

fn main() {
    let person = Person { name: "World".to_owned() };
    println!("{}", person.greet()); // Hello, World!
}
```

> **[Quickstart →]**  ·  **[Is it ready for production? — honest maturity notes →]**

### How the draft applies its guidance

The README makes different promises to two readers at once, and each element serves one or both. The moves:

- **The tag line is layered.** The owned name as the title, the chosen descriptor beneath it, and a reassurance line under that are the layered form from [tag-lines.md](tag-lines.md); the reassurance does the skeptic's work by saying "still ordinary Rust — no nightly, zero runtime cost" before the reader supplies a worse reading of "language extension" or "pluggable."
- **It states "a library on stable Rust" and the install line early.** The [evaluator](reader-profiles.md) is scanning for the toolchain gamble, so the reassurance and `cargo add cgp` come before the prose, per [formats.md](formats.md) and the stable-Rust [selling point](selling-points.md).
- **The feature set is the ruthless few, and each title avoids a repellent word.** Three of the canonical headline features from [key-features.md](key-features.md) appear, each title leading with a concrete capability or a recognized Rust term rather than "modular," "macros," or "magic," and each sentence carrying its honest qualifier ("at compile time," "made safe because … explicit and local").
- **Runnable code sits above the fold.** A reader wants to see the code before the prose, and CGP's own launch feedback asked for exactly a runnable example ([attention-and-engagement.md](attention-and-engagement.md)); the Hello World shows a component, a provider, a getter, and wiring in the smallest honest form.
- **There are two calls to action, matched to the two readers.** The quickstart is the skimmer's rung and the maturity discussion is the evaluator's, straight from the [conversion ladder](formats.md).

It also leaves out, deliberately, any feature titled "Dependency Injection": on a general front page that label imports the runtime-framework baggage catalogued in [skepticism.md](skepticism.md), so the DI value ships through "Type-Safe Wiring" and "Abstract Over Every Dependency" instead, as [key-features.md](key-features.md) argues.

## The social thread

A thread on Mastodon, Bluesky, or X has seconds of attention and one job — earn the click without inviting the dunk — so it leads with a single concrete pain, keeps jargon out of the first post, and lets one link carry the depth ([formats.md](formats.md)). The draft is three posts.

### The draft

> **1/** Swapping a real implementation for a mock in Rust usually means a `Box<dyn>` you pay for at runtime, or a generic type parameter you thread through every layer of your code. There's a third option that costs neither. 🧵

> **2/** Define the capability once, write as many implementations as you like, and let each context — your app, your test harness — pick one in a single line. The call site never changes, and it compiles down to a direct call. No trait objects, no runtime dispatch.

> **3/** It's a library on stable Rust and a superset of ordinary traits, so you can try it in one corner without rewriting anything. And if a dependency has just one implementation, keep using a plain trait — this earns its keep once the implementations multiply. [link]

### How the draft applies its guidance

A thread fails in two opposite ways — too jargon-heavy to parse, or so compressed it reads as an overclaim — and the draft is shaped to avoid both. The moves:

- **The first post is one concrete pain, with no jargon and no paradigm name.** It opens on the mock swap from [problems-solved.md](problems-solved.md), because the [first-contact skimmer](reader-profiles.md) gives the piece three seconds and pattern-matches anything abstract to a category they already dismiss.
- **It earns the click without inviting the dunk.** Conceding "keep using a plain trait" in the last post is the [positioning.md](positioning.md) move that disarms the "over-engineered" reflex ([skepticism.md](skepticism.md)); a thread that only sells is the one a technical crowd piles onto.
- **The mechanism is stated precisely, not hyped.** "Compiles down to a direct call" is the [vocabulary](vocabulary.md)-approved phrasing, chosen over "blazingly fast," which would read as the exact overclaim the skimmer broadcasts.
- **One link carries the depth, and it is the whole call to action.** A thread is a hook-delivery mechanism, not a place to explain, per [formats.md](formats.md).

## Adapting these to your own piece

These three are templates for the moves, not for the words, so adapting one to a real piece means re-running the decisions rather than find-and-replacing the copy. Start by naming the one or two dominant [reader profiles](reader-profiles.md) for your channel, then swap the anchor pain for the one that reader feels most sharply — the [orphan-rule escape](problems-solved.md) for the trait-heavy developer, the [error-type swap](problems-solved.md) for the systems reader, and so on — because the pain, not the tag line, is what decides whether the piece lands.

Two things must survive every adaptation. Keep the conceded cost: it is load-bearing, not optional politeness, and dropping it is how a piece that earned attention loses trust ([skepticism.md](skepticism.md), [positioning.md](positioning.md)). And re-verify every CGP claim and code snippet against the source and the `/cgp` skill before publishing, since a draft copied without that check propagates whatever has gone stale in it. When you find a move these three drafts do not cover, the right next step is a new draft here — a talk opener, a comparison table, a `This Week in Rust` blurb — added in the same shape, so this document grows into the section's library of worked models rather than a fixed set of three.
