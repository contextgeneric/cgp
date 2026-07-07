# Implicit parameters

Implicit parameters are function arguments the compiler supplies automatically from the surrounding context instead of the caller passing them by hand — Scala's `given`/`using` and Haskell's `ImplicitParams` are the direct forms, and the type-class resolution both languages build on is the same idea generalized. CGP shares the goal of threading context through code without explicit plumbing, but supplies the values from a context's fields and wiring rather than from a compiler-driven search, which trades away automatic resolution to gain the freedom to have many overlapping implementations.

## Purpose

Some values are needed everywhere and interesting nowhere: a configuration object, a logger, a comparison strategy, an error type, an execution environment. Passing them explicitly through every function that transitively needs them clutters signatures and forces intermediate functions to accept parameters they only forward. Implicit parameters remove that clutter by letting the caller omit such an argument and having the compiler fill it in from what is in scope, so the value propagates through a call chain without appearing at every call. The declaration still records the dependency in the type, but the *passing* becomes invisible.

This is the same tension CGP resolves with [impl-side dependencies](../concepts/impl-side-dependencies.md) and [implicit arguments](../concepts/implicit-arguments.md), which is why the comparison is illuminating rather than incidental. Both CGP and the implicit-parameter languages want a piece of code deep in a call graph to obtain what it needs from its surroundings without every caller in between having to know. They differ in *what the surroundings are* and *how the value is found*: Scala and Haskell search an implicit scope by type, while CGP reads a field or resolves a wiring entry from the context that is already threaded through every provider. Showing a reader that CGP's context *is* their implicit environment — made a first-class, explicitly-wired type — is the heart of the comparison.

## The concept in depth

Implicit parameters appear in several forms that a reader may or may not distinguish, and the comparison to CGP is clearest when they are kept apart. There is the direct implicit *value* parameter (Scala's `using`, Haskell's `ImplicitParams`), and there is the type-class mechanism (Scala's `given` type classes, Haskell's `class`/`instance`) that is implicit resolution specialized to finding an implementation for a type. Both rest on the compiler searching a scope by type, and both are governed by *coherence*, the property that makes CGP's approach fundamentally different.

### Context parameters in Scala (`using` and `given`)

Scala 3 lets a parameter list be marked `using`, which makes its arguments contextual: at the call site the programmer may omit them, and the compiler supplies a matching `given` value from scope. A function that needs a `Config` everywhere in a rendering pipeline declares it once with `using` and never threads it again:

```scala
def renderWebsite(path: String)(using config: Config): String =
    "<html>" + renderWidget(List("cart")) + "</html>"    // config passed implicitly

def renderWidget(items: List[String])(using config: Config): String = ???
```

A `given` value in scope is what the compiler injects for a `using` parameter, and the caller writes nothing:

```scala
given Config = Config(8080, "docs.scala-lang.org")

renderWebsite("/home")     // the given Config is supplied automatically
```

The mechanism generalizes to type classes, which is its most common use. A `given` can be defined *for a type*, and a method that takes a `using` parameter of that type resolves the right instance by the types at the call site:

```scala
trait Comparator[A]:
  def compare(x: A, y: A): Int

given Comparator[Int] with
  def compare(x: Int, y: Int): Int = x - y

def max[A](x: A, y: A)(using c: Comparator[A]): A =
  if c.compare(x, y) > 0 then x else y

max(1, 2)     // Comparator[Int] resolved and passed implicitly
```

Scala uses this one mechanism for both dependency injection and type classes — a `using Config` is DI, a `using Comparator[A]` is a type class — which is why the language's implicits sit at the intersection of the two related-work topics; see also [dependency injection](dependency-injection.md). Historically this was all written with the `implicit` keyword in Scala 2; Scala 3 renamed it to `given`/`using` precisely because the old keyword was overloaded and hard to teach.

### Implicit parameters in Haskell (`ImplicitParams`)

Haskell's `ImplicitParams` extension is the more literal form of the same idea: a function can name a dynamically-bound variable `?x` of some type, which shows up as a constraint `(?x :: t)` on its signature and is filled from the binding in scope. A sort that needs a comparison declares it as an implicit parameter and calls it as `?cmp`:

```haskell
sort :: (?cmp :: a -> a -> Bool) => [a] -> [a]
sort = sortBy ?cmp
```

The constraint propagates automatically to any caller that does not bind it, so a function built on `sort` inherits its `?cmp` without restating the intent:

```haskell
least :: (?cmp :: a -> a -> Bool) => [a] -> a
least xs = head (sort xs)
```

The value is supplied with a `let` binding, at which point the constraint is discharged and the function below it becomes ordinary:

```haskell
min :: Ord a => [a] -> a
min = let ?cmp = (<=) in least
```

`ImplicitParams` is the honest baseline for the comparison but a cautionary one: it is barely used in modern Haskell. The constraints leak into every signature, so the parameters are not very "implicit"; there is no way to declare a default; and the feature interacts awkwardly with the monomorphism restriction. Haskell programmers reach instead for the `Reader` monad to thread an environment, or, far more often, for type classes.

### Type classes as implicit dictionary passing

Type classes are the mechanism Haskell and Scala actually use for the implicit-resolution job, and understanding them as *implicit dictionary passing* is what connects them to CGP. A `class` declares an interface, an `instance` provides it for a type, and when a function constrained by the class is called, the compiler finds the instance for the concrete type and passes it as a hidden argument — a *dictionary* of the instance's methods. The constraint `Ord a =>` is elaborated into an extra parameter carrying the `Ord` dictionary:

```haskell
class Ord a where
  compare :: a -> a -> Ordering

instance Ord Int where
  compare x y = ...   -- the compiler builds and passes an Ord Int dictionary
```

The resolution is *type-directed and automatic*: the programmer names the constraint, and the compiler searches for the matching instance by type, with no explicit selection. This is exactly the convenience CGP gives up and the reason it can do something type classes cannot, so the next point is the pivot of the whole comparison.

### Coherence: one instance, globally

The property that governs type-class and implicit resolution — and that CGP deliberately abandons — is *coherence*: for a given type there is exactly one instance, and every resolution anywhere in the program finds the same one. Coherence is what makes automatic resolution safe. Because the compiler will always find the same `Ord Int`, it can inject it silently without the programmer worrying that a different `Ord Int` might be chosen elsewhere and make two pieces of code disagree. Haskell enforces this with the orphan-instance rule and by forbidding overlapping instances; Scala's implicits are coherent in the common case and rely on scoping and priority rules at the edges.

The price of coherence is the price CGP was built to escape. Because there can be only one `Ord Int`, a program cannot have two legitimate orderings of `Int` as first-class instances — the standard workaround is to wrap the type in a `newtype` so a second instance attaches to a distinct type. And a crate cannot add an instance for a type and class it does not own. These are the same overlap and orphan restrictions Rust's own coherence imposes, described from the CGP side in [bypassing coherence](../concepts/coherence.md). The comparison to CGP therefore comes down to a single trade: implicit resolution buys automatic, type-directed selection at the cost of one global instance per type, while CGP buys many overlapping per-context implementations at the cost of selecting them explicitly.

## How CGP expresses it

CGP threads context through code and lets deep code read what it needs, but it does so through the context that is already the `Self` of every provider, not through a scope search. The value a Scala `using` parameter or a Haskell implicit parameter carries is, in CGP, a field of the context or an entry in its wiring — and the context is passed to every provider automatically as the receiver, so nothing has to be threaded by hand. Two CGP constructs map onto the two forms of implicit parameter: [`#[implicit]`](../reference/attributes/implicit.md) arguments correspond to implicit *value* parameters, and the [consumer/provider component](../concepts/consumer-and-provider-traits.md) with its wiring corresponds to type classes.

### Implicit arguments are implicit value parameters

An `#[implicit]` argument is written as an ordinary function parameter but is supplied from the context's fields rather than by the caller, which is precisely the shape of a Scala `using` parameter — except the source is a struct field, not a `given` in scope. A provider that needs width and height reads them as implicit arguments, and any context carrying those fields supplies them:

```rust
#[cgp_fn]
pub fn rectangle_area(&self, #[implicit] width: f64, #[implicit] height: f64) -> f64 {
    width * height
}
```

Where Scala's `renderWebsite` obtains its `Config` from an implicit `given`, a CGP provider obtains `width` and `height` from the context threaded through it as `self`. The parallel is close enough that CGP's own name for the feature — *implicit arguments* — is the same word Scala and Haskell use, and the desugaring is the same idea: the parameter disappears from the public signature and is bound from the surroundings before the body runs. The difference is where "the surroundings" live. A `using` parameter searches the implicit scope by type; an `#[implicit]` argument reads the context field of the matching name, so resolution is by *field*, decided when the context is defined, not by a compiler search at the call site.

### A shared context value is a threaded environment

When several pieces of code must agree on one value — the case Scala handles by a single `given Config` in scope and Haskell by the `Reader` monad — CGP has them all read the same field or the same [abstract type](../concepts/abstract-types.md) from the shared context. A configuration value read by many providers is injected once and every provider that names it receives the same one, with no coordination between them, exactly as one `given Config` serves an entire Scala call graph. CGP's shared error type is the canonical instance: every fallible provider names the context's `Self::Error`, so they all agree on one error type the context supplies once — the type-level echo of a threaded `Reader` environment, resolved at compile time and paid for at runtime with nothing.

### Components and wiring are type classes without coherence

A CGP [component](../concepts/consumer-and-provider-traits.md) is a type class, a provider is an instance, and wiring is the resolution step — but because a provider's `Self` is its own marker type, many providers for the same component coexist without violating coherence, which is what type-class instances cannot do. The `Comparator[Int]` that Scala can define only once, or the `Ord Int` that Haskell pins globally, becomes in CGP any number of interchangeable providers, each selected per context. The [modular serialization](../examples/modular-serialization.md) example makes the contrast concrete: `UseSerde`, `SerializeBytes`, and `SerializeWithDisplay` all serialize a `String` and overlap freely, where the equivalent overlapping type-class instances would be rejected:

```rust
#[cgp_impl(UseSerde)]
impl<Value> ValueSerializer<Value>
where
    Value: serde::Serialize,
{ /* ... */ }

#[cgp_impl(SerializeBytes)]
impl<Value> ValueSerializer<Value>
where
    Value: AsRef<[u8]>,
{ /* ... */ }
```

The cost of this freedom is the flip side of the type-class trade: CGP will not *find* the provider for you by type. A context names its choice in a [`delegate_components!`](../reference/macros/delegate_components.md) table, where Scala and Haskell would resolve the instance automatically from the type. What CGP loses in automatic resolution it gains in the ability to have `AppA` serialize a `Vec<u8>` as hexadecimal and `AppB` as base64 — two coherent local choices where a global instance would force one answer on both.

## What users like and dislike

Implicit parameters and the type classes built on them are among the most loved and most criticized features of the languages that have them, and both reactions are instructive. What users value is the erasure of boilerplate: a `Config`, a comparator, or an execution context threads through a deep call graph without appearing at every call, and type classes let one generic function work over any type that has an instance, resolved automatically. Scala programmers build whole architectures on implicits — type-class derivation, context propagation, dependency injection — precisely because the mechanism is so general, and Haskell's type classes are widely regarded as one of the cleanest solutions to ad-hoc polymorphism in any language.

The dislike is the mirror image of the same generality: implicit resolution is hard to follow. The recurring Scala complaint is that a value appears "from nowhere," and tracking down *which* `given` was selected — or why an expected one was not — is a notorious time sink, made worse in Scala 2 by the single overloaded `implicit` keyword that Scala 3 split apart to address. Error messages when resolution fails are often opaque. Haskell's `ImplicitParams` is disliked enough to be effectively abandoned, for the concrete reasons that the constraints leak into every signature, no default can be given, and the monomorphism restriction interferes. And coherence itself, though it is what makes resolution safe, is a persistent source of friction: the orphan rule forces awkward module structure, and the one-instance-per-type limit forces `newtype` wrappers whenever a second interpretation of a type is wanted.

## How CGP compares

CGP makes the opposite trade from implicit resolution on the two axes that matter, and the comparison is cleanest stated as that trade. On *resolution*, implicit parameters are automatic and type-directed while CGP wiring is explicit: Scala and Haskell find the instance for you, and CGP asks you to name it in a table. On *coherence*, implicit parameters are coherent while CGP is not: the languages guarantee one instance per type and forbid overlap and orphans, and CGP permits unlimited overlapping providers and per-context choice by moving `Self` to a provider marker. Each side pays for what the other gets. The implicit-parameter languages get zero-boilerplate resolution and pay with the one-instance restriction and the "where did this come from" opacity; CGP gets many local implementations and explicit, greppable wiring and pays with the wiring itself — there is no automatic search, so the choice must be written down.

Neither trade is strictly better, and the honest positioning names where each wins. When a program genuinely wants one canonical instance per type — one `Ord`, one `Show`, one serialization — and values automatic resolution above all, type classes are the better tool, and fighting their coherence with CGP-style machinery would be over-engineering. When a program needs several interchangeable implementations, per-deployment or per-context choice, or must implement a behavior for types and traits it does not own, CGP's explicit wiring is the better tool, and the coherence it discards was the very thing in the way. CGP's resolution being explicit is also, for its intended audience, a feature rather than a cost: the wiring table is the one place selection is decided, so the "spooky" resolution that dogs implicits is replaced by a lookup a reader can point at.

## Presenting CGP to someone who knows this

A reader fluent in implicits or type classes holds most of CGP's conceptual furniture already, and the move that unlocks it is to name the correspondence outright: a **component is a type class**, a **provider is an instance**, **wiring is instance resolution**, and an **`#[implicit]` argument is a `using` parameter** whose value comes from a context field. The context itself is the implicit environment they already reason about — the `Reader` they thread, the set of `given`s in scope — reified as a single explicit type that every provider receives automatically. Framed this way, CGP is not a foreign paradigm but their own implicit machinery with the resolution step made visible.

The one thing to correct up front is the expectation of automatic resolution. This reader will assume CGP finds the provider by type the way their compiler finds an instance, and it does not — wiring is written by hand in a table. Present that not as a missing feature but as the deliberate consequence of the capability they will find most striking: because CGP does not resolve by type, it is free of coherence, so it can host the overlapping instances their language forbids. Lead with the pain coherence causes them — the `newtype` dance to get a second `Ord`, the orphan-rule contortions to add an instance for a foreign type, the single global choice forced on unrelated code — and show that each disappears when providers are distinct marker types selected per context. For the Scala reader specifically, the resonant pitch is "implicits without the mystery": the value still arrives without being threaded by hand, but *which* implementation was chosen is a line in a wiring table rather than the outcome of a scope search, so the debugging nightmare they know is designed out. For the Haskell reader, the pitch is "type classes without the orphan rule, and overlapping instances made legal."

The analogy to avoid is promising that CGP "just finds the right implementation." It does not, and a reader sold on automatic resolution will feel misled the first time they have to write a `delegate_components!` entry. Set the expectation honestly — you name the provider, once, per context — and pair it immediately with what that explicitness buys: no ambiguity, no hidden priority rules, no coherence straitjacket, and the same implementation-hiding decoupling delivered through a table they can read. A reader who has spent an afternoon debugging an implicit resolution will hear the trade as a good one.

## Sources

The account of the related work draws on the official language documentation, primary references on type-class semantics, and community writing on the ergonomics of implicits, cited where a specific claim rests on one.

- [Scala 3 Book — Context Parameters](https://docs.scala-lang.org/scala3/book/ca-context-parameters.html) and [Contextual Parameters (Tour of Scala)](https://docs.scala-lang.org/tour/implicit-parameters.html) — the authoritative description of `using` clauses, `given` instances, and type-directed resolution.
- [Scala 3 Implicit Redesign (Baeldung)](https://www.baeldung.com/scala/scala-3-implicit-redesign) — the rename from Scala 2's overloaded `implicit` keyword to `given`/`using` and the reasons for it.
- [GHC User's Guide — Implicit Parameters](https://ghc.gitlab.haskell.org/ghc/doc/users_guide/exts/implicit_parameters.html) and [Implicit parameters (HaskellWiki)](https://wiki.haskell.org/Implicit_parameters) — the `?x` constraint syntax, `let`-binding, propagation semantics, and the noted limitations that keep the feature little-used.
- [Type class (Wikipedia)](https://en.wikipedia.org/wiki/Type_class) and [Implementing, and Understanding Type Classes (okmij.org)](https://okmij.org/ftp/Computation/typeclass.html) — type classes as dictionary-passing elaboration and how instances are resolved.
- [Type classes: confluence, coherence and global uniqueness (ezyang's blog)](https://blog.ezyang.com/2014/07/type-classes-confluence-coherence-global-uniqueness/) and [Coherence of Type Class Resolution (Bottu et al.)](https://xnning.github.io/papers/coherence-class.pdf) — the definition of coherence and why it constrains instances to one per type.
