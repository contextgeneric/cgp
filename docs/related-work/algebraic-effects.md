# Algebraic effects and handlers

Algebraic effects and handlers are a way to define side-effecting *operations* — throwing, reading state, yielding, awaiting — separately from the *handlers* that interpret them, so that one piece of effectful code can be run under many different interpretations, with a handler free to capture the rest of the computation as a continuation and resume it zero, one, or many times. CGP shares the split between an operation and its interpretation, and even the idea of choosing the interpretation from the surrounding context, but it keeps only the fragment where the continuation is used exactly once and in place — which turns out to be dynamic binding, resolved statically per context rather than dynamically down a call stack.

## Purpose

Algebraic effects solve the problem of writing code that *does* something effectful without fixing *how* that effect is carried out. A function that needs to read a configuration value, log a message, throw an error, suspend for I/O, or make a nondeterministic choice normally has to commit to a concrete mechanism — a global, a `Result`, a monad, a callback — and that commitment leaks into its type and forces every caller to accommodate it. Algebraic effects let the function instead *perform an operation* named by an abstract effect, and defer the meaning of that operation to a *handler* installed somewhere up the call stack. The same code runs against a real logger in production, a collector in a test, and a no-op in a benchmark, with only the handler changing.

This is the same separation CGP draws between a [consumer trait and its providers](../concepts/consumer-and-provider-traits.md), which is why the comparison is worth drawing carefully rather than casually. Both paradigms take a capability, expose it as an interface a caller invokes without naming an implementation, and supply the implementation from the surroundings. Where they diverge is *what a handler is allowed to do* and *how the surroundings choose it*: an effect handler receives the continuation and wields real control-flow power, chosen by dynamic scope; a CGP provider is an ordinary function selected by the context's type at compile time. Showing a reader where that line falls — and why CGP sits on the side of it that coincides with dynamic binding — is the heart of this comparison, and it draws on the [row-polymorphism](row-polymorphism.md) account too, since the effect systems that type these operations do so with the very row types that document covers.

## The concept in depth

Algebraic effects come in layers that a reader should keep distinct: the *operations* that name an effect, the *equational theory* that classically justifies calling it "algebraic," the *handlers* that interpret operations by grabbing the continuation, the *effect system* that types which operations a computation may perform, and the concrete language designs — Koka, OCaml, Flix, Eff — that realize these to different degrees. The subsections below build up in that order, and the final one isolates the single fragment that CGP corresponds to.

### Operations, effects, and the equational theory

An *effect* is a signature of *operations*, and a computation produces the effect by *performing* one of them. The founding idea, due to Gordon Plotkin and John Power, is that impure behavior arises from a set of operations — `get` and `put` for mutable state, `read` and `print` for I/O, `raise` for exceptions — rather than from an opaque notion of "side effect" ([Plotkin & Pretnar, *Handling Algebraic Effects*](https://homepages.inf.ed.ac.uk/gdp/publications/handling-algebraic-effects.pdf)). What made the account *algebraic* is that each effect came with an *equational theory*: laws the operations obey, such as the state laws relating `get` and `put`, whose free model induces exactly the monad for that effect ([Plotkin & Pretnar 2013](https://lmcs.episciences.org/705); [Bauer & Pretnar, *Programming with Algebraic Effects and Handlers*](https://www.researchgate.net/publication/221671686_Programming_with_Algebraic_Effects_and_Handlers)). This theoretical grounding is where the name comes from, but it is largely vestigial in the practical languages below, which keep the operations-and-handlers structure and drop the laws — a point worth holding onto, because CGP keeps even less of the algebra than they do.

### Handlers and the continuation

A *handler* interprets the operations of an effect, and its defining power is that it receives the *continuation* — the suspended rest of the computation from the point the operation was performed. When a computation performs `op(arg)`, control transfers to the nearest enclosing handler for that operation, which receives both the argument and the continuation as a first-class, delimited resumption ([Pretnar, *An Introduction to Algebraic Effects and Handlers*](https://www.eff-lang.org/handlers-tutorial.pdf)). This generalizes an exception handler in one decisive way: an exception handler can only *abandon* the computation, whereas an effect handler can *resume* it. Plotkin and Pretnar gave this its semantics — a handler is a *model* of the effect's theory, and handling is the unique homomorphism from the free model into it ([Plotkin & Pretnar, *Handlers of Algebraic Effects*](https://homepages.inf.ed.ac.uk/gdp/publications/Effect_Handlers.pdf)).

How many times a handler invokes the continuation is what determines the effect it realizes, and the three cases are worth naming because they mark exactly where CGP can and cannot follow:

- **Zero times** — the handler discards the continuation and returns its own value, which is *exception* behavior: `raise` aborts to the handler and never comes back.
- **Once** — the handler resumes the continuation with a result, which is the *ordinary* case: reading state, dynamic binding, logging, a normal function-like call that yields a value and lets the caller carry on.
- **Many times** — the handler resumes the continuation more than once, which is what makes effects *powerful*: nondeterminism and backtracking (try both branches), generators (yield and later resume), cooperative scheduling and async/await (suspend, run something else, resume). This is *multi-shot* resumption, and it is impossible to express as an ordinary function return.

### Effect typing: rows, sets, or nothing

An *effect system* tracks, in a computation's type, which effects it may perform, so that unhandled effects can be caught statically — and languages differ sharply in whether they do this. Koka types effects as a *row* — `<exn,div>` in a signature means the function may throw and diverge — building directly on row polymorphism ([Leijen, *Algebraic Effects for Functional Programming*](https://www.microsoft.com/en-us/research/wp-content/uploads/2016/08/algeff-tr-2016-v2.pdf)); this is the same row machinery surveyed in [row polymorphism](row-polymorphism.md), applied to effects instead of records. Flix types effects as a *set* over a lattice and uses the result for *purity reflection*, letting the compiler know when code is pure enough to parallelize or eliminate ([Madsen et al., *Programming with Purity Reflection*](https://drops.dagstuhl.de/entities/document/10.4230/LIPIcs.ECOOP.2023.18)). At the other extreme, Eff and OCaml 5 deliberately do *not* track effects in types at all; an unhandled effect is a runtime failure rather than a type error ([OCaml manual, *Effect handlers*](https://ocaml.org/manual/5.5/effects.html)). The presence or absence of this static check is the sharpest axis of variation among the languages, and the one where CGP's `check_components!` has something precise to say.

### The concept in three languages

The same effect looks recognizably similar across the languages that implement it, which makes their differences legible. In **Koka**, an effect declares operations that are marked `ctl` (full control, resume any number of times), `fun` (tail-resumptive — resume exactly once, in place), or `val`; the effect appears as a row in the type, and a handler is installed with `with`:

```koka
effect ask
  fun ask() : int          // `fun`: tail-resumptive, resumes exactly once

fun add-twice() : ask int
  ask() + ask()

fun main() : console ()
  with fun ask() 21        // install a handler that supplies 21
  println( add-twice() )   // 42
```

A `ctl` operation is the multi-shot form — a `ctl flip()` handler may call `resume(True)` *and* `resume(False)`, resuming the continuation twice to explore both branches — and if an effect is never handled, Koka keeps it in the row and the type checker demands the enclosing function declare it, so an unhandled effect is ultimately a *type error*.

In **OCaml 5**, effects are declared by extending `Effect.t`, performed with `perform`, and handled by matching on the effect and its continuation `k`, which is resumed with `continue`:

```ocaml
type _ Effect.t += Ask : int Effect.t

let add_twice () = perform Ask + perform Ask

let () =
  let open Effect.Deep in
  try_with add_twice ()
    { effc = fun (type a) (eff : a Effect.t) ->
        match eff with
        | Ask -> Some (fun (k : (a, _) continuation) -> continue k 21)
        | _ -> None }
```

OCaml's continuations are **one-shot**: `k` may be resumed *at most once*, enforced by a dynamic check that raises `Continuation_already_resumed` on a second resume, a restriction chosen because one-shot continuations are far cheaper and suffice for the concurrency schedulers the feature was built for ([Sivaramakrishnan et al., *Retrofitting Effect Handlers onto OCaml*](https://kcsrk.info/slides/handlers_edinburgh.pdf)). OCaml tracks **no** effects in types, so a `perform` with no matching handler raises `Effect.Unhandled` at runtime.

In **Flix**, an effect is declared with `eff`, appears in a function type after a backslash, and is handled with `run … with handler`:

```flix
eff Ask {
    def ask(): Int32
}

def addTwice(): Int32 \ Ask =
    Ask.ask() + Ask.ask()

def main(): Int32 =
    run {
        addTwice()
    } with handler Ask {
        def ask(k) = k(21)
    }
```

Flix's set-based effect system means `\ Ask` is part of `addTwice`'s type, and the handler's continuation `k` makes the resume explicit, just as Koka's `resume` and OCaml's `continue` do.

### The fragment CGP corresponds to: tail-resumptive handlers *are* dynamic binding

The single fact that makes the CGP comparison precise is that a handler which resumes the continuation *exactly once, in tail position* is no longer doing anything control-theoretic — it is dynamic binding. A handler is *tail-resumptive* when every operation clause invokes the continuation in tail position, and the literature names its canonical example outright: **the canonical tail-resumptive handler is dynamic binding** ([Xie et al., *Effect Handlers, Evidently*](https://www.microsoft.com/en-us/research/wp-content/uploads/2020/07/evidently-5f0b7dbc1a998.pdf)). The reason this matters for implementation, and for CGP, is that such a handler never needs to capture the continuation at all: the compiler can run the operation *in place* on the current stack, replacing the dynamic search for a handler with a constant-offset lookup into an *evidence vector* of handlers passed down like a dictionary ([Xie & Leijen, *Generalized Evidence Passing for Effect Handlers*](https://xnning.github.io/papers/multip.pdf)). In other words, the efficient compilation of the exactly-once fragment of effect handlers *is* dictionary passing. That is the fragment CGP occupies — and it occupies it directly, with no dynamic search underneath and nothing else on top.

## How CGP expresses it

CGP reproduces the operations-and-handlers structure with its consumer/provider split, but every CGP "handler" is an ordinary function that returns once, so CGP realizes only the exactly-once fragment above. A [component](../reference/macros/cgp_component.md) is the effect signature, a [provider](../reference/macros/cgp_impl.md) is the handler, and [wiring](../reference/macros/delegate_components.md) installs the handlers on a context. The correspondence is exact for the tail-resumptive case and breaks cleanly wherever an effect would reach for the continuation.

### Components are effect signatures; providers are (tail-resumptive) handlers

A CGP component declares operations the way an effect declares them, and a provider interprets them the way a handler does — with the standing restriction that the interpretation is a plain function body. Declaring a component names a capability without giving it meaning:

```rust
#[cgp_component(Greeter)]
pub trait CanGreet {
    fn greet(&self);
}
```

`CanGreet` is the effect signature and `greet` the operation; a provider written with [`#[cgp_impl]`](../reference/macros/cgp_impl.md) is the handler, interpreting the operation for any context. The structural match is one-to-one — signature, operation, handler, installation — but the provider's body is not handed a continuation and cannot choose whether to resume. It computes a value and returns, and the caller resumes in place, exactly once. In effect-handler terms every CGP provider is a `fun` operation in Koka's sense, never a `ctl` one; there is no `resume` to call zero times or twice, because there is no reified continuation at all.

### Reading from the context is dynamic binding, exactly

The place the correspondence is not merely structural but *exact* is context-value access, because that is dynamic binding on both sides. Koka's `with fun ask() 21` installs a tail-resumptive handler that supplies a value to deeply nested code without threading it through every call; CGP's [getters](../concepts/implicit-arguments.md) and [`#[implicit]` arguments](../reference/attributes/implicit.md) do the same by reading a field from the context that is threaded through every provider as `self`:

```rust
#[cgp_fn]
pub fn greet(&self, #[implicit] name: &str) -> String {
    format!("Hello, {name}!")
}
```

The `name` argument is supplied not by the caller but from the context, precisely as Koka's `ask()` is supplied by the enclosing `ask` handler rather than by `add-twice`'s caller. This is the same dynamic-binding pattern the [implicit-parameters](implicit-parameters.md) document describes, and the equivalence is not a loose analogy: the reader effect *is* the canonical tail-resumptive handler, and reading a context field *is* CGP's whole realization of it. Where the two part ways is *how the value is found* — Koka searches the dynamic handler stack, CGP reads a field of a statically-known context — which is the same resolution split that separates CGP from every dynamically-scoped mechanism.

### Raising an error looks like the `raise` operation — but passes a value, not control

CGP's error handling is the sharpest illustration of the boundary, because it mimics the *selection* of an exception handler while doing none of the *control transfer*. [`CanRaiseError<SourceError>`](../reference/components/can_raise_error.md) reads like the `raise` operation, and a provider raises without knowing the concrete error type:

```rust
#[cgp_impl(new LoadOrFail)]
#[uses(CanRaiseError<String>)]
#[use_type(HasErrorType.Error)]
impl Loader {
    fn load(&self, path: &str) -> Result<String, Error> {
        if path.is_empty() {
            return Err(Self::raise_error("empty path".to_owned()));
        }
        Ok(format!("contents of {path}"))
    }
}
```

Because the raise-and-wrap components carry per-source-type dispatch, a context can even route each source error to a different strategy, which reads like installing one exception handler per exception type — `RaiseFrom` for a `String`, `DebugError` for a `ParseError` — through the [`open` statement](../reference/macros/delegate_components.md) exactly as [modular error handling](../concepts/modular-error-handling.md) describes. But the resemblance stops at *selection*. An effect `raise` is the *zero-resume* handler: it abandons the continuation and unwinds to the handler. CGP's `raise_error` does not unwind anything — it *constructs and returns a value* of the abstract `Self::Error` type, and the actual abort is Rust's own `return`/`?` on the `Result`, entirely outside the wiring. CGP selects the *interpretation* of the error (the handler-choice half) but leaves the *control flow* (the continuation-discarding half) to `Result`. This is why CGP's dispatch is not even one-shot like OCaml's — it is strictly exactly-once, because the zero-resume case never lives inside the capability system at all.

### Impl-side dependencies are the effect row; `check_components!` is "all effects handled"

CGP tracks which capabilities a provider needs, and verifies they are all supplied, in a way that lines up closely with an effect system typing a row and demanding it be discharged. A provider's needs are stated as [impl-side dependencies](../concepts/impl-side-dependencies.md) — the `#[uses(CanRaiseError<String>)]` above, and every bound in a `where` clause — which is the CGP counterpart of the effect row `<exn>` that Koka would infer for a function that performs `raise`. A generic provider over `Context` with such bounds is even effect-*polymorphic* in a loose sense: the context type variable plays a role like Koka's row variable, standing for "whatever else this context can do." And [`check_components!`](../reference/macros/check_components.md) is the discharge check:

```rust
check_components! {
    App {
        LoaderComponent,
    }
}
```

This asserts that `App` supplies every capability `LoadOrFail` transitively needs, and fails to compile naming the missing one if not — which is exactly Koka's guarantee that a program with an unhandled effect in its row is a type error, and the opposite of OCaml's runtime `Effect.Unhandled`. CGP lands on the statically-checked end of the effect-typing axis, reached through trait resolution rather than a dedicated effect system.

### Configuring abstract types through the same wiring

CGP extends the operations-and-handlers wiring to something effect systems do not touch: abstract *types*. Alongside choosing the provider for an operation, a context chooses the concrete type behind an [abstract type](../concepts/abstract-types.md) — its `Error`, its `Scalar`, its `Runtime` — through the same [`delegate_components!`](../reference/macros/delegate_components.md) table, by wiring a [`#[cgp_type]`](../reference/macros/cgp_type.md) component to [`UseType<T>`](../reference/providers/use_type.md):

```rust
delegate_components! {
    App {
        ErrorTypeProviderComponent: UseType<anyhow::Error>,
    }
}
```

An effect handler interprets operations, which are values and computations; it has no notion of supplying a *type member* the way `HasErrorType` supplies `Self::Error`. CGP unifies both under one wiring mechanism, so the same table that says "raise errors this way" also says "and the error type is `anyhow::Error`." This is a genuine capability beyond the effect-handler analogy, not a restatement of it.

### What CGP cannot express

The multi-shot power of effect handlers has no CGP analogue, and this is the honest limit of the comparison. Because a provider is an ordinary function that returns once, CGP cannot express any effect whose handler resumes the continuation zero times or more than once. Generators, backtracking search, cooperative scheduling, and async/await — the applications that motivate effect handlers in the first place ([Kammar, Lindley & Oury, *Handlers in Action*](https://denotational.co.uk/publications/kammar-lindley-oury-handlers-in-action.pdf)) — all require capturing the continuation and are simply outside CGP's model. CGP does have an [async handler family](../concepts/handlers.md) and [type-level DSLs](../concepts/type-level-dsls.md) that interpret a `Code` tag by dispatching to a provider, which is the closest CGP comes to the operations-and-interpreters shape; but even there the interpretation is a straight call chain resolved at compile time, not a captured continuation the handler controls. Async in CGP is Rust's own `async`/`await` threaded through provider calls, not a handler that suspends and resumes a computation.

## What users like and dislike

Algebraic effects are among the most admired ideas in current language research, and the reasons practitioners value them are consistent and concrete. The headline benefit is the elimination of *function coloring*: intermediate code between the operation and its handler needs no awareness of the effect, so effectful and pure code compose without the `async`/`sync` or `IO`-tainted split that plagues other approaches ([Abramov, *Algebraic Effects for the Rest of Us*](https://overreacted.io/algebraic-effects-for-the-rest-of-us/)). Users also prize the clean separation between an effect's *interface* — its operations — and its *semantics* — the handler — which makes the same code testable under a mock handler and runnable under a real one, and the *composability* of stacking several handlers to combine independent effects, widely contrasted favorably with monad transformers and their pain ([*Why Algebraic Effects?*, Ante](https://antelang.org/blog/why_effects/)). Where the effects are typed, as in Koka and Flix, the row or set in a signature is valued as honest documentation of what a function may do, and Flix turns that information into automatic parallelization and dead-code elimination.

The complaints are equally consistent and fall into three clusters. The loudest is *unfamiliarity and control-flow opacity*: the concept is new to most programmers, no mainstream language ships it, and — the recurring technical objection — effects "abstract away side effects to effect handlers by definition," so that following control from a `perform` to the handler that catches it is as hard as reasoning about a distant exception handler, the "which handler runs this?" problem ([Ante](https://antelang.org/blog/why_effects/)). The second is *performance*: general handlers must capture continuations, which costs, and while optimizing compilers recover much of it for the tail-resumptive case through evidence passing, non-tail handlers remain more expensive than native control flow ([Xie & Leijen 2021](https://xnning.github.io/papers/multip.pdf)). The third is language-specific and cuts against the untyped designs: OCaml's decision to omit effect typing means a function's signature says nothing about the effects it performs and an unhandled effect is a runtime crash rather than a compile error ([OCaml manual](https://ocaml.org/manual/5.5/effects.html)), and its one-shot restriction rules out the multi-shot uses outright. For readers who reach effects from Haskell, the nearest comparison is the type-class-based effect libraries (`mtl` and its successors), whose recurring frustration is the *n² instances* problem — every handler must supply instances delegating all the *other* effects — which handler-based systems avoid ([`fused-effects`](https://hackage.haskell.org/package/fused-effects); [`effet`](https://hackage.haskell.org/package/effet)).

## How CGP compares

CGP and algebraic effects make opposite choices on the two axes that define the effect-handler design space — *what a handler may do* and *how it is chosen* — and the comparison is cleanest stated as that pair of trades. On *handler power*, an effect handler holds the continuation and may resume it any number of times, which buys generators, backtracking, and async; a CGP provider is a plain function that returns once, which buys nothing control-theoretic but costs nothing either — the call monomorphizes to a direct jump with no continuation to capture. On *handler selection*, an effect handler is chosen by *dynamic scope* — the nearest handler on the runtime stack wins, and a program can install a fresh handler for the same effect at any point — while a CGP provider is chosen by the context's *type* at compile time, fixed once in a wiring table and resolved through the trait system. That second axis places CGP much closer to type classes and implicit parameters than to effects, which is why the [dependency-injection](dependency-injection.md) and [implicit-parameters](implicit-parameters.md) comparisons cover ground this one deliberately leaves to them. The bridge between the two paradigms is evidence passing: Koka *compiles* dynamically-scoped handlers down to dictionary passing for the tail-resumptive case, and CGP is what you get if that compiled form is the only form there ever was — evidence passing chosen by types, with no dynamic search beneath it.

Two further divergences follow from these and are worth stating plainly. First, an effect handler *stack* is ordered and nesting matters: the innermost handler for an operation wins, and reordering handlers changes results — state-over-nondeterminism and nondeterminism-over-state compute different things ([Kammar, Lindley & Oury 2013](https://denotational.co.uk/publications/kammar-lindley-oury-handlers-in-action.pdf)). CGP's wiring is a *flat* table: one provider per component, resolved by type, with no dynamic nesting to shadow an outer handler and no order to permute — and because the dispatch is exactly-once and resume-in-place, the non-commutativity that makes handler order significant simply does not arise. A CGP table is a set of handlers, not a stack of them. Second, CGP keeps even less of the "algebraic" than the practical effect languages do: it has no equational theory relating its operations, though since Koka, OCaml, and Flix mostly drop the laws too, this is a shared simplification rather than a CGP-specific gap.

Neither design dominates, and the honest positioning names where each wins. When a program needs the continuation — a scheduler, a generator, a backtracking solver, a suspendable coroutine — algebraic effects are the right and only tool of the two, and emulating them with CGP is not possible, not merely awkward. When a program needs many interchangeable implementations of a capability chosen per deployment, checked statically, compiled to zero-overhead direct calls, and extended to abstract types as well as operations — and needs none of the continuation power — CGP delivers that on stable Rust, where an effect system would require a language the platform does not have. The exactly-once fragment CGP restricts itself to is not a crippled effect system; it is dynamic binding and dictionary passing, which is a complete and useful thing in its own right, and CGP adds to it the static per-context selection and abstract-type configuration that effect handlers do not offer.

## Presenting CGP to someone who knows this

A reader fluent in algebraic effects arrives with most of CGP's structure already in mind, and the fastest way in is to map the vocabulary and then immediately mark the one boundary. A **component is an effect signature**, its methods are **operations**, a **provider is a handler**, and **wiring is installing the handlers** on a context; a provider's `where` bounds are the **effect row** it performs, and **`check_components!` is the guarantee that every effect is handled**, the static version of Koka's unhandled-effect type error rather than OCaml's runtime crash. Reading a context field is **dynamic binding** — and here the correspondence is exact, not approximate, because dynamic binding *is* the canonical tail-resumptive handler and reading a field is CGP's whole realization of it. Framed this way, CGP is not a foreign paradigm to this reader but the *tail-resumptive corner* of the one they know, made static and type-directed.

The boundary to draw at once, before it misleads, is the continuation. This reader will assume a CGP provider can resume, abort, or fork the computation the way a handler can, and it cannot: a provider is an ordinary function that returns exactly once, so there is no `resume` to call zero times or twice, and every multi-shot use they value — generators, async, backtracking — is outside CGP entirely. Present that not as a missing feature but as the deliberate location of CGP in the design space: it takes the fragment of effect handlers that the literature already identifies as dynamic binding, the fragment that compiles to direct calls with no continuation capture, and builds everything on it. The pitch that lands for this audience is "effect handlers minus the continuation, resolved by type instead of by dynamic scope, and extended to abstract types" — which reframes what could read as a limitation as a precise and defensible design choice. For the Koka or Flix reader specifically, lean on the shared row intuition: their effect row and CGP's impl-side dependencies are the same idea, and CGP's `check_components!` discharges it the same way their type checker does. For the OCaml reader, the resonant point is that CGP recovers the *static* discharge check their language chose to forgo, and does so without needing effect typing bolted onto the language. And for anyone who has fought monad transformers or `mtl`'s n² instances, the framing is that CGP composes capabilities without either — a flat table of per-context handlers, no stacking order to get wrong.

The analogy to avoid is calling CGP "an effect system." It is not — it has no continuations, no dynamic scope, and no effect kind in the type system — and a reader sold on that framing will look for `resume` and feel misled when it is not there. Say precisely what CGP is: the exactly-once, resume-in-place fragment of effect handlers, which is dynamic binding, made into a compile-time, per-context, type-directed wiring mechanism that also configures abstract types. A reader who knows how much of effect-handler machinery exists to tame the continuation will recognize that a paradigm which never captures one has bought real simplicity, and will hear the trade as a considered one rather than a shortfall.

## Sources

The account of the related work draws on the primary research literature on algebraic effects and their compilation, the official documentation of Koka, OCaml, and Flix, and cited community writing for sentiment; the CGP snippets are drawn from the knowledge base's [consumer/provider](../concepts/consumer-and-provider-traits.md), [implicit-arguments](../concepts/implicit-arguments.md), and [modular error handling](../concepts/modular-error-handling.md) documents and verified against current macro behavior.

- [Plotkin & Pretnar, *Handling Algebraic Effects* (LMCS 2013)](https://homepages.inf.ed.ac.uk/gdp/publications/handling-algebraic-effects.pdf) ([journal page](https://lmcs.episciences.org/705)) and [*Handlers of Algebraic Effects* (ESOP 2009)](https://homepages.inf.ed.ac.uk/gdp/publications/Effect_Handlers.pdf) — the foundational account of effects as operations with an equational theory and handlers as models interpreting them via the continuation.
- [Pretnar, *An Introduction to Algebraic Effects and Handlers*](https://www.eff-lang.org/handlers-tutorial.pdf) and [Bauer & Pretnar, *Programming with Algebraic Effects and Handlers*](https://www.researchgate.net/publication/221671686_Programming_with_Algebraic_Effects_and_Handlers) — an accessible tutorial and the Eff language, an untyped research realization.
- [Leijen, *Algebraic Effects for Functional Programming* (Koka)](https://www.microsoft.com/en-us/research/wp-content/uploads/2016/08/algeff-tr-2016-v2.pdf) and [The Koka Programming Language book](https://koka-lang.github.io/koka/doc/book.html) — Koka's row-typed effects and the `ctl`/`fun`/`val` operation distinction.
- [Xie et al., *Effect Handlers, Evidently* (ICFP 2020)](https://www.microsoft.com/en-us/research/wp-content/uploads/2020/07/evidently-5f0b7dbc1a998.pdf) ([ACM](https://dl.acm.org/doi/abs/10.1145/3408981)) and [Xie & Leijen, *Generalized Evidence Passing for Effect Handlers* (ICFP 2021)](https://xnning.github.io/papers/multip.pdf) — the tail-resumptive-handler-is-dynamic-binding result and the evidence-passing (dictionary-passing) compilation of the exactly-once fragment.
- [OCaml manual, *Effect handlers*](https://ocaml.org/manual/5.5/effects.html) and [Sivaramakrishnan et al., *Retrofitting Effect Handlers onto OCaml* (PLDI 2021)](https://kcsrk.info/slides/handlers_edinburgh.pdf) — OCaml 5's `perform`/`continue` syntax, its one-shot continuations, its lack of effect typing, and the runtime `Effect.Unhandled`.
- [Flix Effect System documentation](https://doc.flix.dev/effect-system.html) and [Effects and Handlers](https://doc.flix.dev/effects-and-handlers.html), with [Madsen et al., *Programming with Purity Reflection* (ECOOP 2023)](https://drops.dagstuhl.de/entities/document/10.4230/LIPIcs.ECOOP.2023.18) — Flix's set-based effect system, its `eff`/`run … with handler` syntax, and purity reflection.
- [Kammar, Lindley & Oury, *Handlers in Action* (ICFP 2013)](https://denotational.co.uk/publications/kammar-lindley-oury-handlers-in-action.pdf) — handler composition, the significance of handler order, and the range of effects handlers express.
- [Abramov, *Algebraic Effects for the Rest of Us*](https://overreacted.io/algebraic-effects-for-the-rest-of-us/) and [*Why Algebraic Effects?* (Ante)](https://antelang.org/blog/why_effects/) — community accounts of what users value (no function coloring, composability, interface/semantics separation) and dislike (novelty, control-flow opacity, performance).
- [`fused-effects`](https://hackage.haskell.org/package/fused-effects) and [`effet`](https://hackage.haskell.org/package/effet) — the type-class-based effect libraries and the `mtl` n²-instances problem, for the Haskell reader's point of comparison.
