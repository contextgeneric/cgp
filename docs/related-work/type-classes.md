# Type classes

Type classes are the mechanism for *principled ad-hoc polymorphism* — a class declares an interface, an instance implements it for a type, and the compiler resolves which instance a constrained function needs by translating the class into a hidden *dictionary* argument. Rust traits are Rust's type classes, so CGP already lives inside a type-class system; its defining move is to escape that system's *coherence* — the one-instance-per-type rule — by making instances first-class values selected explicitly per context, which is exactly the freedom that overlapping and incoherent instances chase in Haskell, Agda, and Lean without ever making it safe.

## Purpose

Type classes solve overloading without giving up type inference or static safety. Before them, a name like `show` or `+` either meant one fixed thing or was resolved by unprincipled special-casing in the compiler; type classes, introduced by Philip Wadler and Stephen Blott to "make ad-hoc polymorphism less ad hoc," let a single name be overloaded across many types in a way the type system tracks and the programmer extends ([Wadler & Blott, *How to make ad-hoc polymorphism less ad hoc*](https://dl.acm.org/doi/10.1145/75277.75283)). A function constrained by `Show a =>` works for every type with a `Show` instance, the right instance is found by the compiler, and no instance is chosen by accident.

This is the comparison closest to CGP's core, because CGP is built on Rust traits and Rust traits *are* type classes. Where the [dependency-injection](dependency-injection.md) and [implicit-parameters](implicit-parameters.md) documents approach CGP from the runtime-container and implicit-value angles, this one meets it on its own ground: a CGP [component](../reference/macros/cgp_component.md) is a class, a [provider](../reference/macros/cgp_impl.md) is an instance, [wiring](../reference/macros/delegate_components.md) is instance resolution, and [impl-side dependencies](../concepts/impl-side-dependencies.md) are class constraints. The single thing CGP changes is coherence, and the whole knowledge base's account of [bypassing coherence](../concepts/coherence.md) is, read against this document, an account of how to have the overlapping and incoherent instances every type-class language struggles to offer safely.

## The concept in depth

Type classes have a stable core — classes, instances, constraints, and the dictionary-passing translation — and a large periphery of coherence rules and the extensions that relax them, realized differently across Haskell, Agda, and Lean. The subsections build from the core translation through the coherence property that governs it, the overlapping and incoherent extensions that bend it, the two dependently-typed treatments that abandon it, and the *modular type classes* result that reframes the whole thing in terms of ML modules.

### Classes, instances, and dictionary passing

A class declares an interface, an instance implements it for a type, and a constraint requires it — and underneath, the class is a record of functions passed invisibly. In Haskell a class and an instance read as:

```haskell
class Show a where
  show :: a -> String

instance Show Bool where
  show True  = "True"
  show False = "False"

describe :: Show a => a -> String
describe x = "value: " ++ show x
```

The `Show a =>` constraint on `describe` is the crux. Wadler and Blott's translation compiles a class into a *dictionary* — a record whose fields are the class methods — an instance into a dictionary value, and a constrained function into one that takes the dictionary as an extra hidden argument, so `describe` elaborates to `describe dict x = "value: " ++ show dict x` and the compiler supplies the `Show Bool` dictionary at each call ([Wadler & Blott 1989](https://dl.acm.org/doi/10.1145/75277.75283)). Dictionary passing is the shared implementation model behind every system in this document, and behind CGP: it is the same idea as the evidence passing in the [algebraic effects](algebraic-effects.md) comparison and the qualified-types constraints in the [row polymorphism](row-polymorphism.md) one. Classes also compose through *superclasses* — `class Eq a => Ord a` requires every `Ord` type to be `Eq` — and modern class systems add *associated types* (a type member of a class), which correspond to CGP's [abstract-type components](../concepts/abstract-types.md).

### Coherence: one instance per type, globally

The property that governs instance resolution, and the one CGP discards, is *coherence*: for a given class and type there is one instance, and every resolution anywhere in the program finds the same one. Coherence is what makes silent, automatic resolution safe — because the compiler always finds the same `Show Bool`, it can insert the dictionary without the programmer worrying that a different `Show Bool` is chosen elsewhere. The property is usually decomposed into *confluence* (any way of resolving a constraint gives the same dictionary), *coherence* (the program behaves as if there were a canonical instance), and *global uniqueness* (there really is only one instance per type program-wide), which GHC guarantees for the instances in scope during a compilation but does not enforce across a whole program ([Yang, *Type classes: confluence, coherence and global uniqueness*](https://blog.ezyang.com/2014/07/type-classes-confluence-coherence-global-uniqueness/)). Two rules protect it: at most one instance per (class, type), and the *orphan rule* discouraging an instance defined in neither the class's nor the type's module. The canonical benefit is a `Set` of an ordered element type: because there is one `Ord` for that type, values inserted under one ordering and read under another can never disagree, so the set cannot silently corrupt.

Rust makes the same choice but *enforces* it where Haskell only advises. Rust traits are type classes with a hard orphan rule — an `impl Trait for Type` is allowed only when the crate owns the trait or the type — so coherence is a compile error to violate, not a discouraged convention ([Rust RFC 2451, *re-rebalancing coherence*](https://rust-lang.github.io/rfcs/2451-re-rebalancing-coherence.html); [*Type class*, Wikipedia](https://en.wikipedia.org/wiki/Type_class)). This strictness is precisely the wall CGP is built to get around: the [coherence](../concepts/coherence.md) concept opens on exactly these overlap and orphan rules.

### Overlapping instances

The first extension relaxes the one-instance rule to allow instances where one is strictly more specific, letting the compiler pick the most specific match. Haskell exposes this through per-instance pragmas — `{-# OVERLAPPING #-}`, `{-# OVERLAPPABLE #-}`, `{-# OVERLAPS #-}` — that replaced the blunt module-wide `OverlappingInstances` flag in GHC 7.10, so that a general instance and a special case can coexist:

```haskell
instance {-# OVERLAPPABLE #-} Show a => Show [a] where   -- lists in general
  show xs = "[" ++ intercalate "," (map show xs) ++ "]"

instance {-# OVERLAPPING #-} Show [Char] where           -- but strings specially
  show s = s
```

Resolution commits to an instance only when it is strictly more specific than every other match; otherwise the program is rejected as ambiguous ([GHC User's Guide, *Instance declarations and resolution*](https://ghc.gitlab.haskell.org/ghc/doc/users_guide/exts/instances.html)). The cost is fragility: the most-specific heuristic is subtle, and — the point that matters for CGP — GHC's own manual warns that "overlapping instances must be used with care as they can give rise to incoherence (different instance choices are made in different parts of the program) even without `-XIncoherentInstances`."

### Incoherent instances

The second extension goes further and lets the compiler commit to an instance even when the choice is not unique, breaking coherence outright. An instance marked `{-# INCOHERENT #-}` may be selected in a context where a more specific instance could later apply, so different parts of a program can resolve the same constraint to different dictionaries. GHC documents the danger plainly: "GHC's optimiser assumes that type-classes are coherent, and hence it may replace any type-class dictionary argument with another dictionary of the same type," which "may cause unexpected results if incoherence occurs," and notes that `INCOHERENT` "still leads to indeterministic behavior and thus should be used with caution" ([GHC User's Guide](https://ghc.gitlab.haskell.org/ghc/doc/users_guide/exts/instances.html)). Incoherent instances are widely regarded as a last resort, because the very automation that makes type classes pleasant — the compiler silently choosing a dictionary — becomes a footgun once the choice is not guaranteed unique. This is the exact hazard CGP confronts head-on, and the reason its embrace of many instances has to be paired with explicit selection.

### Instance arguments in Agda: coherence dropped, resolution scoped

Agda offers type-class-style overloading without a class construct and without coherence, through *instance arguments*. Devriese and Piessens's design reuses Agda's ordinary dependently-typed records as classes and marks a resolved argument with double braces, so resolution is "a new type of function argument resolved from call-site scope in a type-directed way" ([Devriese & Piessens, *On the Bright Side of Type Classes: Instance Arguments in Agda*](https://dl.acm.org/doi/10.1145/2034574.2034796)):

```agda
record Show (A : Set) : Set where
  field show : A → String

instance
  showBool : Show Bool
  showBool = record { show = λ b → if b then "true" else "false" }

print : {A : Set} → {{Show A}} → A → String
print {{s}} x = Show.show s x
```

Agda does not enforce global uniqueness: instance search succeeds when exactly one instance in the call-site scope matches, and a genuine ambiguity is a *local* error at that use site rather than a program-wide coherence guarantee. This is a scoped, search-based resolution — closer to Scala's implicits than to Haskell's global coherence — and it is the first hint of the design CGP takes further: many instances allowed, the conflict resolved where the choice is made rather than by a global uniqueness rule.

### Type classes in Lean: search, priorities, and the diamond problem

Lean uses type classes pervasively for its mathematical library, resolves them by a backtracking tabled search with per-instance priorities, and pays for the absence of coherence with the *diamond problem*. A class is a structure, an instance is registered for search, and resolution finds a value of the required class:

```lean
class Show (α : Type) where
  show : α → String

instance : Show Bool where
  show b := if b then "true" else "false"

#eval Show.show true
```

Because Lean allows multiple instances and orders them by priority, the same constraint can be satisfiable in more than one way, and when two resolution paths to the same class instance disagree — a *diamond*, "the existence of multiple conflicting terms of a class found within the typeclass instance graph" — inference can pick the wrong one or diverge; tabled resolution was introduced partly to tame the exponential blowup diamonds cause ([Selsam, Ullrich & de Moura, *Tabled Typeclass Resolution*](https://arxiv.org/pdf/2001.04301)). In a proof assistant the stakes are higher than a wrong value: two instances that are not *definitionally equal* break proofs that assume they coincide, so the Mathlib community must ensure that any overlapping instances are definitionally equal for every instantiation in the overlap — a standing, laborious discipline of incoherence management ([Baanen, *Use and abuse of instance parameters in the Lean mathematical library*](https://arxiv.org/pdf/2202.01629)). Lean is the clearest cautionary example of what unmanaged instance choice costs, and the sharpest backdrop for CGP's explicit per-context selection, which admits no diamond because a context names exactly one provider per component.

### Modular type classes: classes as signatures, instances as structures

The result that unifies this document with the [ML modules](ml-modules.md) comparison is that type classes *are* ML modules plus a resolution layer, established by Dreyer, Harper, and Chakravarty. Their *Modular Type Classes* treats **classes as signatures and instances as structures and functors**, adding a notion of designating certain instance modules as *canonical* within a scope so the compiler can resolve them implicitly, while keeping explicit module linking as the default ([Dreyer, Harper & Chakravarty, *Modular Type Classes*](https://people.mpi-sws.org/~dreyer/papers/mtc/main-long.pdf)). The reframing is illuminating because it separates two things a plain type-class system fuses: the *interface/implementation* structure (which is just modules) and the *canonical implicit resolution* (which is the extra ingredient type classes add on top).

Separating them also names the tension every design in this document negotiates: **canonicity fights modularity.** Canonicity — one designated instance the compiler may assume everywhere — is what makes implicit resolution safe, but it is fundamentally a *non-modular* property, since a module cannot in general guarantee its instance is the canonical one program-wide, and full modular abstraction lets two modules each supply a different one. Dreyer, Harper, and Chakravarty confine canonicity to a scope to get some of both; OCaml's [modular implicits](ml-modules.md) inherit the same tension and, as their authors concede, cannot fully reconcile canonicity with modular abstraction. The design space is therefore a spectrum. At one end sits Haskell: fully implicit resolution, global coherence, one instance per type, no modularity of choice. In the middle sit modular type classes, modular implicits, Agda instance arguments, and Scala implicits: implicit resolution with canonicity scoped or dropped, paying in ambiguity and search subtleties. At the far end sits CGP: no canonicity at all, no search, explicit per-context selection — the fully modular extreme, where the price of dropping canonicity is that selection must be written down, and the reward is that overlapping and orphan instances become ordinary rather than exceptional. CGP is what the modular-type-classes line of work looks like when canonicity is abandoned entirely rather than merely scoped.

## How CGP expresses it

CGP is a type-class system with the coherence rule removed and replaced by explicit per-context instance selection, built by splitting each class into a consumer and a provider side. The consumer trait is an ordinary Rust type class; the provider trait and the wiring are the machinery that make instances first-class values, so many overlapping instances can coexist and a context picks one. Every construct below maps onto a type-class concept, and the extension features that type-class languages bolt on with pragmas are, in CGP, the default.

### A component is a class; a provider is a first-class instance

A CGP component is a type class, and a provider is an instance made into a named, selectable value. Declaring a component is declaring a class:

```rust
#[cgp_component(AreaCalculator)]
pub trait CanCalculateArea {
    fn area(&self) -> f64;
}
```

`CanCalculateArea` is the class interface, exactly as `class Show a` is. The difference is what an instance may be. A Haskell instance is anonymous and canonical — there is one `Show Bool`, chosen by the compiler — whereas a CGP provider is a named marker type carrying a provider-trait impl, which is precisely a *first-class dictionary*: a value you can name, pass, and choose among. Moving the implementation off the context and onto a provider marker, through the [consumer/provider split](../concepts/consumer-and-provider-traits.md), is the single device that lifts the one-instance-per-type limit — because a provider implements the provider trait for *its own* type, coherence never forbids a second one. Wiring then plays the role of resolution, but explicitly: a [`delegate_components!`](../reference/macros/delegate_components.md) entry names which instance a context uses, where Haskell's compiler would search for the canonical one. A provider's `where` bounds and [`#[uses]`](../reference/attributes/uses.md) imports are the class constraints threaded by dictionary passing, and the context is the dictionary that carries them.

### Overlapping instances are the default, with no heuristic

Where Haskell needs pragmas and a most-specific heuristic to permit overlap, CGP permits unlimited overlapping instances by construction and resolves them by naming, not guessing. The [modular serialization](../examples/modular-serialization.md) example defines several providers that all serialize the same types and overlap freely:

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

A type like `String` matches both, so as vanilla type-class instances one would be rejected and even with `OVERLAPPING` the compiler would need one to be strictly more specific — but as CGP providers both compile, because each implements the provider trait for its own marker. There is no most-specific rule and therefore no ambiguity: a context simply names the provider it wants for a given type, `ValueSerializerComponent: SerializeBytes` or a per-type [`open`](../reference/macros/delegate_components.md) dispatch. The fragility GHC warns about — overlap silently producing incoherence — cannot arise, because the choice is never inferred.

### Incoherent instances made deterministic and local

The deepest correspondence is that CGP embraces the very incoherence type-class languages fear, and makes it safe by moving the choice from a global search to an explicit, per-context table. GHC warns that incoherent or overlapping instances let "different instance choices be made in different parts of the program," silently, and that the optimiser may swap one dictionary for another; Lean pays for the same freedom with the diamond problem. CGP allows the same many-instances-for-one-type situation — that is the whole point of [bypassing coherence](../concepts/coherence.md) — but the resolution is neither global nor implicit: each context selects one provider per component in its wiring, so two contexts may resolve the same type differently *on purpose*, and within a single context the choice is unambiguous and fixed. The `AppA` that serializes a `Vec<u8>` as hexadecimal and the `AppB` that serializes it as base64 are two coherent local scopes, not a global incoherence — the different-choice-in-different-places that is a footgun in Haskell is a feature in CGP because it is written down rather than inferred. CGP is, in one line, *incoherent instances with the incoherence made deterministic and the selection made explicit.*

### No orphan rule, no newtype dance

Two everyday type-class frustrations disappear because a provider's `Self` is always a type its crate owns. The orphan rule — Rust's enforced version of Haskell's convention — never bites, so a downstream crate can supply a provider for a component and a type it did not define, the exact case the [coherence](../concepts/coherence.md) concept shows type classes forbidding. And the newtype trick that Haskell forces when a type needs a second instance (a `Sum` and a `Product` monoid wrapping `Int`) is unnecessary: a second behavior for a type is just a second provider, named directly, with no wrapper type to introduce and unwrap. Where a type-class programmer restructures modules to place an instance or wraps a type to duplicate one, a CGP programmer writes another provider and a wiring line.

## What users like and dislike

Type classes are among the most loved features of the languages that have them, and the praise is specific. Programmers value that overloading is *principled* — the compiler tracks and checks it — and *inferred*, so the dictionary is supplied automatically and generic code reads as if monomorphic. They value coherence's payoff of *global uniqueness*: one `Ord` per type means a `Set` or a `Map` cannot be corrupted by mixing orderings, a guarantee that holds without the programmer thinking about it ([Yang 2014](https://blog.ezyang.com/2014/07/type-classes-confluence-coherence-global-uniqueness/)). And they value the *lawful abstractions* type classes make idiomatic — `Functor`, `Monoid`, `Monad` — and the way superclass constraints compose. In dependently-typed settings the same machinery organizes enormous algebraic hierarchies, which is why Lean's Mathlib rests on it.

The complaints track the coherence rules and the extensions that strain against them. The one-instance-per-type limit forces the *newtype workaround*, which is boilerplate and "breaks down when the type is embedded in another type" ([Yang 2014](https://blog.ezyang.com/2014/07/type-classes-confluence-coherence-global-uniqueness/)); the *orphan rule* forces awkward module structure ([Queensland FP Lab, *Orphan Rules*](https://qfpl.io/posts/orphans-and-fundeps/)); *overlapping instances* are subtle and *incoherent instances* are, by GHC's own account, indeterministic and dangerous ([GHC User's Guide](https://ghc.gitlab.haskell.org/ghc/doc/users_guide/exts/instances.html)). Haskell has no easy *local* or *scoped* instance because local instances reintroduce a coherence problem, which is a long-running community sore point and a place where Scala, Agda, and Lean chose differently. Some practitioners argue the whole coherence bargain is wrong and prefer explicit dictionaries so the choice is visible and local, as Paul Chiusano does in *The trouble with typeclasses* ([Chiusano 2018](https://pchiusano.github.io/2018-02-13/typeclasses.html)); the coherence debate is active enough to have its own recent survey ([Racordon, *On the State of Coherence in the Land of Type Classes*](https://arxiv.org/pdf/2502.20546)). And in Agda and Lean the absence of enforced coherence surfaces as the *diamond problem* and the burden of keeping overlapping instances definitionally equal ([Baanen 2022](https://arxiv.org/pdf/2202.01629)), plus resolution-performance and error-message costs.

## How CGP compares

CGP makes the opposite coherence bargain from Haskell and a more disciplined one than Agda or Lean, and the trade is cleanest stated across three axes. On *resolution*, type classes are implicit and CGP is explicit: Haskell finds the dictionary, CGP asks a context to name the provider. On *coherence*, type classes are coherent — globally unique instances, enforced in Rust, conventional in Haskell — while CGP is deliberately incoherent at the definition level, hosting unlimited overlapping and orphan providers. On *safety of the incoherence*, CGP is the disciplined one: where Haskell's `INCOHERENT` and Lean's diamonds let a wrong instance be chosen silently or a proof break, CGP's per-context table makes every choice explicit and local, so incoherence never means indeterminism. Each side pays for what the other gets. Type classes get zero-boilerplate resolution and global uniqueness and pay with the newtype dance, the orphan rule, and the fragility of the overlap extensions; CGP gets many local instances, per-context choice, and freedom from orphans and diamonds, and pays with the wiring — there is no search, so the selection must be written down.

Neither is better in the abstract, and the honest positioning names where each wins. When a program genuinely wants one canonical instance per type program-wide — one `Ord`, one `Show`, one serialization, so that a `Set` cannot be corrupted and generic code needs no wiring — coherent type classes are the right tool, and simulating that with CGP's per-context wiring would be over-engineering that reintroduces by hand the uniqueness the compiler would give for free. When a program needs several interchangeable instances, per-deployment or per-context choice, instances for types and traits it does not own, or must escape the diamond and orphan problems, CGP's explicit selection is the better tool, and the coherence it discards was the very thing in the way. CGP's explicitness is, for its audience, a feature rather than a cost, in the spirit of the "explicit dictionaries" critics of coherence advocate: the choice is a greppable line in a wiring table, not the outcome of a resolution search that overlap or incoherence could quietly derail.

## Presenting CGP to someone who knows this

A reader who knows type classes holds essentially all of CGP's conceptual furniture, and the way in is to state the correspondence and then the one change. A **component is a class**, a **provider is an instance** — but a first-class, named one — **wiring is instance resolution made explicit**, an **impl-side dependency is a class constraint**, the **context is the dictionary** that carries them, and a component's **associated type is a class's associated type**. The one change is coherence: CGP removes the one-instance-per-type rule and, with it, automatic resolution, and puts explicit per-context selection in their place. Framed this way CGP is not a new paradigm to this reader but their own type-class system with coherence swapped for modular, per-context choice — the fully-modular end of the [modular type classes](#modular-type-classes-classes-as-signatures-instances-as-structures) spectrum.

The expectations to correct are the two coherence buys. First, *automatic resolution*: this reader expects the compiler to find the instance by type, and CGP does not — the provider is named in a table. Present that as the deliberate consequence of the capability they will find most striking, which is that CGP hosts the overlapping and orphan instances their language forbids or makes fragile. Lead with the pains coherence causes them: the newtype wrapper to get a second `Monoid`, the orphan-rule module contortions, the `OVERLAPPING` pragmas that can still go incoherent, the `INCOHERENT` footgun, and — for the Agda or Lean reader — the diamond problem and the definitional-equality drudgery of keeping overlapping instances aligned. Each of these disappears when instances are distinct marker types selected per context, and the pitch that lands is *overlapping and incoherent instances, made safe*: the freedom those extensions reach for, with the indeterminism designed out because the choice is explicit and local rather than searched and global.

Second, *global uniqueness*: this reader may expect that once a type has an instance, it is the instance everywhere, so a `Set` is safe. CGP deliberately does not promise this program-wide — it promises it *per context*. Say so plainly, because a reader who assumes global uniqueness will look for a guarantee CGP scopes rather than globalizes; then frame the scoping as the point, since it is what lets two contexts serialize the same type two ways without conflict. For the reader who has read *The trouble with typeclasses* or fought a coherence bug, the framing is that CGP is the explicit-dictionary design they wished for, with the ergonomics of a wiring table and the compile-time verification of [`check_components!`](../reference/macros/check_components.md) standing in for the resolution they gave up.

## Sources

The account of the related work draws on the primary literature on type classes and their coherence, the official documentation of GHC, Agda, and Lean, and cited community writing for sentiment; the CGP snippets are drawn from the knowledge base's [bypassing coherence](../concepts/coherence.md), [consumer and provider traits](../concepts/consumer-and-provider-traits.md), and [modular serialization](../examples/modular-serialization.md) material and verified against current macro behavior.

- [Wadler & Blott, *How to make ad-hoc polymorphism less ad hoc* (POPL 1989)](https://dl.acm.org/doi/10.1145/75277.75283) ([PDF](http://users.csc.calpoly.edu/~akeen/courses/csc530/references/wadler.pdf)) — the origin of type classes and the dictionary-passing translation that compiles a class into a record of methods passed as a hidden argument.
- [GHC User's Guide — Instance declarations and resolution](https://ghc.gitlab.haskell.org/ghc/doc/users_guide/exts/instances.html) — the one-instance rule, the orphan-instance rule, the `OVERLAPPING`/`OVERLAPPABLE`/`OVERLAPS`/`INCOHERENT` pragmas, and GHC's own warnings that overlap can give rise to incoherence and that incoherent instances are indeterministic.
- [Yang, *Type classes: confluence, coherence and global uniqueness*](https://blog.ezyang.com/2014/07/type-classes-confluence-coherence-global-uniqueness/) — the decomposition of coherence into confluence, coherence, and global uniqueness, the `Set`/`Ord` safety argument, and the newtype workaround and its breakdown.
- [Rust RFC 2451 — re-rebalancing coherence](https://rust-lang.github.io/rfcs/2451-re-rebalancing-coherence.html) and [*Type class* (Wikipedia)](https://en.wikipedia.org/wiki/Type_class) — Rust traits as type classes with an *enforced* orphan rule, versus Haskell's discouraged-by-convention orphans.
- [Devriese & Piessens, *On the Bright Side of Type Classes: Instance Arguments in Agda* (ICFP 2011)](https://dl.acm.org/doi/10.1145/2034574.2034796) — Agda's instance arguments as call-site-scoped, type-directed resolution over dependently-typed records, without a coherence guarantee.
- [Selsam, Ullrich & de Moura, *Tabled Typeclass Resolution* (Lean)](https://arxiv.org/pdf/2001.04301) and [Baanen, *Use and abuse of instance parameters in the Lean mathematical library*](https://arxiv.org/pdf/2202.01629) — Lean's priority-based backtracking resolution, the diamond problem, and Mathlib's discipline of keeping overlapping instances definitionally equal.
- [Dreyer, Harper & Chakravarty, *Modular Type Classes* (POPL 2007)](https://people.mpi-sws.org/~dreyer/papers/mtc/main-long.pdf) — classes as signatures and instances as structures and functors, scoped canonicity for implicit resolution, and the canonicity-versus-modularity tension that places CGP at the fully-modular extreme.
- [Chiusano, *The trouble with typeclasses*](https://pchiusano.github.io/2018-02-13/typeclasses.html), [Racordon, *On the State of Coherence in the Land of Type Classes*](https://arxiv.org/pdf/2502.20546), and [Queensland FP Lab, *Multi-Parameter Type Classes and their Orphan Rules*](https://qfpl.io/posts/orphans-and-fundeps/) — community sentiment on coherence, the case for explicit dictionaries, and the orphan-instance pain.
