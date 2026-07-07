# ML modules and modular implicits

ML modules are the signature/structure/functor system of OCaml and Standard ML — interfaces that specify types and values, implementations that satisfy them, and functors that turn one module into another — and *modular implicits* is the OCaml extension that adds type-directed, type-class-style resolution on top of that system. CGP's components, providers, and higher-order providers line up closely with signatures, structures, and functors, but CGP replaces the manual, ordered functor application that assembling a large ML program requires with a declarative wiring table that the compiler resolves — landing somewhere between raw functors and modular implicits, and adding per-context selection that neither of them has.

## Purpose

ML modules solve the problem of building large programs out of interchangeable, separately-specified parts with genuine type abstraction. A *signature* states what a component provides — some abstract types and some operations over them — without fixing their representation; a *structure* implements a signature; and a *functor* is a module parameterized by another module, so a component can be written once against an interface and instantiated many ways. This is the ML answer to modularity, dependency injection, and data abstraction all at once, and it is widely regarded as one of the most expressive module systems any language offers ([Dreyer, *Understanding and Evolving the ML Module System*](https://people.mpi-sws.org/~dreyer/thesis/main.pdf)).

This is the same territory CGP occupies, which is why the comparison is close rather than incidental. A CGP [component](../reference/macros/cgp_component.md) is an interface, a [provider](../reference/macros/cgp_impl.md) is an implementation, a [higher-order provider](../concepts/higher-order-providers.md) is a parameterized implementation, and [abstract-type components](../concepts/abstract-types.md) are exactly the abstract types a signature declares. The two paradigms diverge on *how a program is assembled from these parts* and *how the parts are selected*: ML makes you apply functors by hand in dependency order and select modules by name, whereas CGP resolves the dependency graph through the trait system from a declarative table keyed on the context. Modular implicits sits between them — it keeps ML's modules but adds automatic, type-directed selection — so the honest comparison runs across all three. It also connects to the [dependency-injection](dependency-injection.md), [implicit-parameters](implicit-parameters.md), and [algebraic-effects](algebraic-effects.md) documents, because ML modules, type classes, implicits, and CGP are one family of answers to the same question of how to choose an implementation.

## The concept in depth

ML modules layer three constructs — signatures, structures, and functors — on top of the core language, add abstract types and the sharing constraints that reconcile them, and distinguish applicative from generative functor application; modular implicits then adds implicit resolution over the whole thing. The subsections build up in that order, using OCaml syntax throughout, and close on the theoretical result that ties modules to type classes and thereby to CGP.

### Signatures, structures, and abstract types

A *signature* is an interface that specifies types and values, and a *structure* is an implementation that satisfies it, with a signature's *abstract types* giving true data abstraction. A signature that specifies an abstract type `t` and a `compare` over it, and a structure implementing it, read as:

```ocaml
module type OrderedType = sig
  type t
  val compare : t -> t -> int
end

module StringCompare = struct
  type t = string
  let compare = String.compare
end
```

The `type t` in the signature, left without a definition, is the crux of ML data abstraction: when a structure is *sealed* with a signature that keeps `t` abstract, clients cannot see the representation, so the module is free to change it, and the type system enforces representation independence ([*Modules and Data Abstraction in OCaml*](https://cs.wellesley.edu/~cs251/s12/handouts/modules.pdf)). This *sealing* is a stronger form of abstraction than mere genericity — it hides a *known* type's representation, not just abstracts over an unknown one.

### Functors: modules parameterized by modules

A *functor* is a function from modules to modules, and it is how ML expresses a reusable, parameterized component. A functor takes a module of some signature and produces a module built on top of it:

```ocaml
module Make (O : OrderedType) = struct
  (* ... a set implementation using O.compare ... *)
end

module StringSet = Make (StringCompare)
```

The application `Make (StringCompare)` is explicit and by name: the programmer chooses the argument module and writes the application. Functors are the ML mechanism for dependency injection at the module level — `Make` depends on *some* ordered type without naming which — and OCaml's standard library uses exactly this shape, `Set.Make(String)`, throughout ([OCaml, *Functors*](https://ocaml.org/docs/functors)).

### Sharing constraints and the applicative/generative distinction

Two subtleties make functors harder to use than the shape above suggests: sharing constraints and the applicative/generative choice. When a functor result's abstract type must be known to equal some other type, the programmer writes a *sharing constraint* with `with type`, as in `Make (O) : S with type t = O.t` — a construct widely found confusing and a recurring source of type errors. Separately, applying a functor twice raises the question of whether the two results' abstract types are equal: OCaml functors are *applicative* by default, so `Make(X).t` equals `Make(X).t` for the same `X`, while SML functors are *generative*, minting fresh incompatible types on each application; OCaml marks a functor generative by giving it a final `()` argument ([OCaml manual, *First-class modules*](https://ocaml.org/manual/5.4/firstclassmodules.html); [practical example thread](https://discuss.ocaml.org/t/practical-example-of-applicative-vs-generative-functors/13777)). These are the fine-grained type-equality controls that make ML modules powerful and also make them heavy.

### Assembling a program: manual functor plumbing

Building a whole application from functors means applying each one by hand, in dependency order, and this plumbing is a documented pain point at scale. Because a functor argument must be supplied explicitly, a large program becomes a sequence of functor applications — `module A = MakeA (...)`, `module B = MakeB (A)`, `module C = MakeC (A) (B)`, and so on — that the programmer writes out and orders correctly. The MirageOS project makes the cost concrete: a unikernel there can reach a *functor application depth of up to 10* across *more than 70 modules*, and the community built an entire DSL, **Functoria**, whose sole purpose is to *organize functor applications*, because OCaml's module language "is much less flexible than its expression language" and cannot express the conditional, dependency-driven wiring such assembly needs ([*Programming Unikernels in the Large via Functor Driven Development*](https://arxiv.org/pdf/1905.02529)). That a mature ecosystem wrote a configuration DSL to escape manual functor application is the sharpest evidence of the limitation CGP's wiring addresses.

### Modular implicits: type-directed resolution over modules

*Modular implicits* extends OCaml so that module arguments can be marked implicit and resolved automatically by type, bringing type-class-style ad-hoc polymorphism to the module system. Introduced by Leo White, Frédéric Bour, and Jeremy Yallop, it lets a function take an implicit module parameter, written in curly braces, and lets the caller omit it; the compiler searches the implicit modules in scope for one of the required signature ([White, Bour & Yallop, *Modular implicits*](https://arxiv.org/abs/1512.01895)):

```ocaml
module type SERIALIZABLE = sig
  type t
  val to_string : t -> string
end

let to_string (type a) {S : SERIALIZABLE with type t = a} = S.to_string

implicit module SInt : SERIALIZABLE with type t = int = struct
  type t = int
  let to_string = string_of_int
end

implicit module SList {A : SERIALIZABLE} : SERIALIZABLE with type t = A.t list = struct
  type t = A.t list
  let to_string l = "[" ^ String.concat ";" (List.map A.to_string l) ^ "]"
end
```

At a call site, `to_string 2` resolves the implicit to `SInt`, and `to_string [2; 3]` resolves it to the *implicit functor* `SList` applied to `SInt` — recursive resolution that constructs the needed module by applying implicit functors, the module-system counterpart of Haskell resolving `Show [Int]` from `Show Int` ([tycon, *First-Class Modules and Modular Implicits*](https://tycon.github.io/modular-implicits.html)). It is motivated by OCaml's lack of ad-hoc polymorphism — the notorious separate `+` and `+.`, `print_int` versus `print_string` — and elaborates into OCaml's existing first-class functors, so it adds no new runtime mechanism. It remains an [experimental fork](https://github.com/ocamllabs/ocaml-modular-implicits), never merged into mainline OCaml.

### Classes as signatures: the modular-type-classes result

The mapping this comparison rests on — component to signature, provider to structure, higher-order provider to functor — is a known theoretical result, not an improvisation. Dreyer, Harper, and Chakravarty's *Modular Type Classes* treats **classes as signatures and instances as structures and functors**, which is exactly the correspondence CGP reuses ([Dreyer, Harper & Chakravarty, *Modular Type Classes*](https://people.mpi-sws.org/~dreyer/papers/mtc/main-long.pdf)). That result belongs as much to the [type classes](type-classes.md) comparison as to this one, and is developed there in full — including the *canonicity*-versus-modularity tension it exposes and how CGP resolves it by abandoning canonicity for per-context selection, as [bypassing coherence](../concepts/coherence.md) describes. Here it is enough that the module side of the mapping has a firm footing: a CGP component really can be read as a signature and a provider as a structure or functor.

## How CGP expresses it

CGP reproduces the signature/structure/functor triad with its consumer/provider machinery, then replaces manual functor application with a declarative table. The correspondence is close enough to translate construct by construct, and the two genuine differences — declarative wiring in place of ordered functor application, and per-context selection in place of by-name linking or implicit search — are exactly where CGP earns its keep.

### Components are signatures; providers are structures

A CGP component is a signature and a provider is a structure implementing it, matching the modular-type-classes mapping directly. Declaring a component states the interface, and a provider supplies the implementation:

```rust
#[cgp_component(AreaCalculator)]
pub trait CanCalculateArea {
    fn area(&self) -> f64;
}

#[cgp_impl(new RectangleArea)]
impl AreaCalculator {
    fn area(&self, #[implicit] width: f64, #[implicit] height: f64) -> f64 {
        width * height
    }
}
```

`CanCalculateArea` is the signature — the operations a caller may invoke — and `RectangleArea` is a structure ascribing to it. The one piece with no ML counterpart is CGP's [consumer/provider duality](../concepts/consumer-and-provider-traits.md): CGP splits the trait into a consumer side callers use and a provider side implementers target, whereas an ML signature is a single interface used from both sides. That split exists because Rust has coherence and ML does not — ML lets a program hold many structures of one signature simply by *naming them*, so it never needs the maneuver CGP uses to escape the one-implementation-per-type rule. A CGP component is also typically a *small* signature — one capability — where an ML signature bundles many values and types; a rich ML signature corresponds to a bundle of CGP components combined through supertraits and [`#[uses]`](../reference/attributes/uses.md).

### Abstract-type components are a signature's abstract types

An [abstract-type component](../concepts/abstract-types.md) is CGP's version of a signature's abstract `type t`, defined with [`#[cgp_type]`](../reference/macros/cgp_type.md):

```rust
#[cgp_type]
pub trait HasErrorType {
    type Error: Debug;
}
```

`HasErrorType` declares an abstract `Error` the way `OrderedType` declares an abstract `t`, and generic code refers to it as `Self::Error` without naming a concrete type, just as functor bodies refer to `O.t`. The abstraction here is the *parametric* kind — generic code is polymorphic over the type it is given — which matches how a functor body abstracts over its argument's types. What CGP does *not* reproduce is ML's *sealing*: once a context wires `Error` to `anyhow::Error`, there is no type-system boundary hiding that representation from other code, so CGP leaves representation-hiding to Rust's ordinary module privacy rather than to the abstract-type mechanism. CGP abstract types defer and configure a type; ML abstract types can additionally seal one.

### Higher-order providers are functors

A [higher-order provider](../concepts/higher-order-providers.md) is a functor: a provider parameterized by another provider, exactly as a functor is a module parameterized by another module. A `ScaledArea` provider takes an inner calculator and builds on it:

```rust
#[cgp_impl(new ScaledArea<InnerCalculator>)]
#[use_provider(InnerCalculator: AreaCalculator)]
impl<InnerCalculator> AreaCalculator {
    fn area(&self, #[implicit] scale_factor: f64) -> f64 {
        InnerCalculator::area(self) * scale_factor * scale_factor
    }
}
```

`ScaledArea<RectangleArea>` is `Make (StringCompare)` in CGP form — a parameterized implementation applied to a specific argument. Two differences distinguish it from a plain functor, and both favor CGP's ergonomics. First, a higher-order provider can default its inner parameter to [`UseContext`](../reference/providers/use_context.md), so an unparameterized `ScaledArea` falls back to whatever the context itself wires for that component — an *open, recursive* application that a plain functor, which always demands its argument explicitly, cannot express without recursive modules. Second, CGP sidesteps the sharing-constraint problem entirely: where an ML functor needs `with type t = O.t` to make its result's abstract types line up with its argument's, CGP's abstract types are supplied by the *shared context* through `HasType`, so every provider in a context automatically agrees on `Self::Error` and `Self::Scalar` with no sharing annotation to write.

### `delegate_components!` replaces manual functor application

The decisive difference is that CGP wires a program declaratively where ML applies functors by hand in order. A context lists its component-to-provider choices in a [`delegate_components!`](../reference/macros/delegate_components.md) table, in any order, and the trait system resolves each provider's dependencies through the context:

```rust
delegate_components! {
    App {
        AreaCalculatorComponent: ScaledArea<RectangleArea>,
        ErrorTypeProviderComponent: UseType<anyhow::Error>,
        // ... further components, in any order
    }
}
```

This is CGP's built-in Functoria. Where MirageOS needed a separate DSL to organize functor applications up to ten deep, CGP resolves that dependency graph as ordinary trait resolution: a provider states what it needs as [impl-side dependencies](../concepts/impl-side-dependencies.md), and the compiler threads each requirement to whatever provider the context wires for it, with no application order to get right. The counterpart to ML's type-checking that a functor argument satisfies its parameter signature is [`check_components!`](../reference/macros/check_components.md), which verifies at compile time that a context supplies every capability its providers transitively need. Grouping a reusable bundle of wirings is an [aggregate provider](../concepts/aggregate-providers.md) or a [namespace](../concepts/namespaces.md) — CGP's way of packaging a sub-assembly the way a functor packages a parameterized structure.

### Modular implicits and CGP wiring: the same problem, opposite selection

Modular implicits and `delegate_components!` are two answers to the same limitation of raw ML modules — that assembling a program from functors is manual — but they select implementations by opposite means. Modular implicits resolves by *implicit search*: the compiler looks through the implicit modules and functors in scope for one matching the required signature, constructing it recursively, which is the type-class route and shares type classes' coherence-leaning character (a search can be ambiguous, and canonicity fights abstraction). CGP resolves by *explicit table*: a context names one provider per component, so there is no search, no ambiguity, and each context may choose differently — the per-context selection [coherence](../concepts/coherence.md) is built to allow. The deep mechanism the two share is *recursive composition*: an implicit functor like `SList {A : SERIALIZABLE}` builds `Show` of a list from `Show` of its element, exactly as a higher-order provider builds its behavior from an inner provider resolved through the context. So the honest statement is not that modular implicits *is* `delegate_components!` but that both automate the functor plumbing ML leaves manual, one by searching and one by tabulating, over the same recursive-composition core.

## What users like and dislike

ML modules are admired for exactly the abstraction and parameterization they were designed to provide, and the praise is long-standing. Practitioners value *genuine data abstraction* — sealing hides a representation and the type system enforces it — *functors* for writing reusable components against an interface, *separate compilation*, and *first-class modules* for the cases where an implementation must be chosen at runtime ([OCaml manual, *First-class modules*](https://ocaml.org/manual/5.4/firstclassmodules.html)). The module system is routinely described as one of the most expressive and principled in any language, and the theoretical work around it — applicative functors, 1ML, modular type classes — is a rich, respected literature.

The complaints cluster around weight, plumbing, and the absence of implicit resolution. The most concrete is *functor boilerplate at scale*: assembling a large application means applying functors by hand in dependency order, painful enough that MirageOS built Functoria to manage it, and rooted in the *stratification* of ML — the module language is a second, weaker language layered above the expression language, without the conditionals and flexible dependency handling ordinary code enjoys ([*Functor Driven Development*](https://arxiv.org/pdf/1905.02529); [Rossberg, *1ML — core and modules united*](https://www.cambridge.org/core/journals/journal-of-functional-programming/article/1ml-core-and-modules-united/47B10882829E4B32F98FBA93B28CEF30)). *Sharing constraints* (`with type`) are a recurring source of confusion, and the *applicative/generative* distinction is a subtlety that trips people. Above all, base ML has *no ad-hoc polymorphism*: everything must be named and applied explicitly, the very gap that motivated modular implicits, whose paper opens on OCaml's separate `+` and `+.` and its family of `print_*` functions ([White, Bour & Yallop 2015](https://arxiv.org/abs/1512.01895)). Modular implicits itself is liked for bringing type-class ergonomics to OCaml by reusing the module system rather than bolting on a separate class construct, and for naturally supporting inheritance, constructor classes, and associated types — but it is disliked, or at least unavailable, for the plainest of reasons: it remains an experimental fork and has never landed in mainline OCaml, and its own authors note that canonicity cannot be fully reconciled with modular abstraction.

## How CGP compares

CGP keeps the ML module correspondence — component is signature, provider is structure, higher-order provider is functor — and changes the two things ML users complain about, at the cost of the one thing ML does best. On *assembly*, CGP is declarative where ML is manual: a `delegate_components!` table lists choices in any order and the compiler resolves the dependency graph, so there is no functor-application chain to write or order, and no need for a Functoria-style DSL because CGP's wiring already is one. On *selection*, CGP is per-context and coherence-free where ML is by-name-explicit and modular implicits is implicit-search: two contexts wire the same component to different providers with no conflict, no ambiguity, and no canonicity constraint. And on *stratification*, CGP has none — providers and contexts are ordinary Rust types and traits, wired in ordinary Rust, so there is no separate, weaker module language above the expression language. What CGP gives up is ML's *sealing*: CGP's abstract types defer and configure a type but do not hide a representation behind a type-system boundary, so representation hiding falls to Rust's module privacy rather than to the abstraction mechanism itself.

The costs beyond sealing are worth naming plainly. ML modules are separately compiled and their functor applications are simple, legible module expressions; CGP's wiring is trait resolution, which compiles to zero-overhead direct calls but produces the verbose, generated-type errors the [check traits](../concepts/check-traits.md) exist to tame. ML's first-class modules let an implementation be chosen at runtime from a value; CGP resolves at compile time and monomorphizes, so a genuinely runtime-chosen implementation needs a different tool. And ML's explicit, by-name linking gives a control and predictability that some programs want, where CGP's table-driven resolution is more automatic but less locally obvious.

Neither is uniformly better, and the honest positioning names where each wins. When a program needs true representation hiding enforced by the type system, separately-compiled modules, runtime selection via first-class modules, or the explicit control of by-name linking, ML modules are the better tool, and reaching for CGP would forgo abstraction guarantees ML provides natively. When a program needs many interchangeable implementations chosen per deployment, a dependency graph the compiler wires rather than the programmer, abstract types unified into the same selection mechanism, and all of it in one language with no module stratum — and can accept Rust as the platform — CGP delivers the functor-and-signature style of modularity without the manual plumbing, and does so as a working library where modular implicits, the ML feature aimed at the same ergonomics, is still an unmerged fork.

## Presenting CGP to someone who knows this

A reader fluent in ML modules holds most of CGP's structure already, and the fastest way in is to translate the vocabulary and then flag the two changes. A **component is a signature**, a **provider is a structure**, a **higher-order provider is a functor**, an **abstract-type component is a signature's abstract `type t`**, and **`delegate_components!` is the functor-application configuration** — the thing they would otherwise write by hand or reach for Functoria to manage. **`check_components!`** is the check that a module satisfies the signature its consumer requires, moved to the wiring site. Framed this way, CGP is not a foreign paradigm but the signature/structure/functor style they know, with the assembly step turned from a manual functor chain into a declarative table.

Two expectations to correct up front, because an ML reader will carry both. The first is *application order*: this reader expects to apply functors themselves, in dependency order, and CGP does not work that way — the table is unordered and the compiler resolves each provider's dependencies through the context, so the "wire A before B before C" discipline simply does not exist. Present that as the payoff, not a loss of control: it is Functoria's job done by the type system. The second is *abstraction*: this reader expects an abstract type to be *sealed*, its representation hidden by the signature, and CGP's abstract types are not sealed — they defer and configure a type but do not hide a known one, so representation hiding is Rust's module privacy, separate from the abstract-type mechanism. Say this plainly, because a reader who assumes sealing will look for a guarantee CGP does not make through this feature.

The analogy to reach for, especially with an OCaml reader who has felt the pain, is *functors with the plumbing automated*. If they have wired a MirageOS unikernel or wished for modular implicits, the pitch lands: CGP is the functor-and-signature discipline with declarative, compiler-resolved wiring, per-context selection instead of a canonical-instance search, and no separate module language — and unlike modular implicits, it is not an experimental fork but a library on stable Rust. For the reader who knows *Modular Type Classes*, the sharpest framing is that CGP takes that paper's mapping — classes as signatures, instances as structures and functors — and makes the *explicit-linking* default the whole design, dropping canonicity in favor of a per-context table, which is precisely the choice that lets it host the overlapping implementations coherence would forbid.

## Sources

The account of the related work draws on the OCaml documentation, the primary literature on ML modules and their relationship to type classes, and the modular-implicits proposal and its implementation; the CGP snippets are drawn from the knowledge base's [consumer/provider](../concepts/consumer-and-provider-traits.md), [abstract types](../concepts/abstract-types.md), and [higher-order providers](../concepts/higher-order-providers.md) documents and verified against current macro behavior.

- [OCaml — *Functors*](https://ocaml.org/docs/functors) and [OCaml manual — *First-class modules*](https://ocaml.org/manual/5.4/firstclassmodules.html) — the concrete syntax of signatures, structures, functors, functor application, `with type` sharing constraints, and the applicative/generative distinction.
- [*Modules and Data Abstraction in OCaml* (Wellesley CS251)](https://cs.wellesley.edu/~cs251/s12/handouts/modules.pdf) and [*A Crash Course on ML Modules*](https://jozefg.bitbucket.io/posts/2015-01-08-modules.html) — signatures as interfaces, structures as implementations, and sealing for data abstraction.
- [Dreyer, *Understanding and Evolving the ML Module System* (PhD thesis)](https://people.mpi-sws.org/~dreyer/thesis/main.pdf) and [Rossberg, *1ML — core and modules united*](https://www.cambridge.org/core/journals/journal-of-functional-programming/article/1ml-core-and-modules-united/47B10882829E4B32F98FBA93B28CEF30) — the depth of the ML module system and the critique of its stratification from the expression language.
- [Dreyer, Harper & Chakravarty, *Modular Type Classes* (POPL 2007)](https://people.mpi-sws.org/~dreyer/papers/mtc/main-long.pdf) — classes as signatures, instances as structures and functors, canonical instances for implicit resolution, and the canonicity-versus-abstraction tension.
- [White, Bour & Yallop, *Modular implicits* (ML Workshop 2014)](https://arxiv.org/abs/1512.01895) ([Cambridge PDF](https://www.cl.cam.ac.uk/~jdy22/papers/modular-implicits.pdf)), the [experimental fork](https://github.com/ocamllabs/ocaml-modular-implicits), and [tycon, *First-Class Modules and Modular Implicits in OCaml*](https://tycon.github.io/modular-implicits.html) — implicit module parameters, implicit functors, recursive resolution, and the ad-hoc-polymorphism motivation.
- [*Programming Unikernels in the Large via Functor Driven Development* (MirageOS / Functoria)](https://arxiv.org/pdf/1905.02529) — the functor-application boilerplate at scale (depth up to 10, 70+ modules) and the DSL built to manage it.
- [Applicative vs. generative functors (OCaml discussion)](https://discuss.ocaml.org/t/practical-example-of-applicative-vs-generative-functors/13777) — the practical distinction and its type-equality consequences, tracing to Leroy's module design.
