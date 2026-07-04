---
name: cgp
description: >-
  Read, write, debug, and explain Context-Generic Programming (CGP) code in Rust.
  Use this skill whenever you encounter or are asked to work with CGP — any code that
  uses `cgp::prelude::*`, the `#[cgp_component]`, `#[cgp_impl]`, `#[cgp_provider]`,
  `#[cgp_fn]`, `#[cgp_type]`, `#[cgp_getter]`, `#[cgp_auto_getter]`, `#[cgp_computer]`,
  `#[cgp_producer]`, or `cgp_namespace!` macros, the `delegate_components!`,
  `check_components!`, or `delegate_and_check_components!` macros, the `Symbol!`,
  `Product!`, `Sum!`, or `Path!` type-level macros, the `HasField`/`HasFields` traits or
  their derives, providers such as `UseContext`/`UseDelegate`/`UseField`/`UseType`, the
  handler family (`Computer`/`Producer`/`Handler`), or terms like consumer trait,
  provider trait, provider, wiring, impl-side dependency, or context-generic. Trigger it
  even when the user does not say "CGP" by name but is clearly working with these
  constructs, when a Rust trait error mentions `IsProviderFor`/`DelegateComponent`, or
  when someone wants modular, dependency-injected, multiple-implementation Rust traits.
---

# Context-Generic Programming (CGP) in Rust

CGP is a modular programming paradigm for Rust that works around the language's coherence
restrictions, letting you write many overlapping or "orphan" trait implementations and then
choose which one applies by **wiring** them onto a concrete context type. This file is a
self-contained primer: read it top to bottom and you hold the whole mental model — every core term,
the shape of every construct, and enough of each expansion to read, write, and debug the majority of
CGP code. It is longer than a typical skill on purpose, because CGP's surface is broad and an agent
who stops here should still be competent, not lost.

**This primer is the map, not the territory — load the sub-skill before you act on any construct it
covers.** The primer gives you the shape of each construct; the `references/` sub-skills give you the
exact grammar, the full expansion, the corner cases, and the worked examples that keep you from
writing subtly wrong code. The gap between the two is where mistakes live: the primer tells you
`#[cgp_impl]` writes a provider, but [components](references/components.md) tells you what `self`
rewrites to, [macro-grammar](references/macro-grammar.md) tells you which attribute forms parse, and
only reading them stops you from emitting an impl that fails to compile in a way the error message
will not explain. So before you write, modify, review, or debug code using a construct — even one
that looks simple — **open the sub-skill that owns it.** Each entry in [the sub-skill
index](#sub-skills-load-the-one-that-owns-your-task) below says exactly what it adds beyond this
primer, so you can tell what you would be guessing at without it. Treat "I already read the primer"
as insufficient: the primer is calibrated to make you *know what you do not know*, and the sub-skill
is where that gap is closed.

CGP is implemented almost entirely as procedural macros. Every macro desugars to ordinary Rust
traits and impls, so the reliable way to understand any construct is to know the code it expands
to. This primer shows those expansions where they matter and keeps them out of the way where they
do not.

## The problem CGP solves

Rust's coherence rules permit at most one implementation of a trait for a given type, and forbid
implementing a foreign trait for a foreign type (the orphan rule). This makes it hard to offer
several interchangeable implementations of one interface, or to let a downstream crate choose an
implementation for a type it does not own. CGP sidesteps both limits with a two-trait split: an
implementation is written against a **provider trait** whose `Self` is a dummy marker type the
crate owns, so coherence never blocks it, and a concrete **context** type later picks which
provider it uses through a small type-level table. The choice is local to the context, so two
different contexts can wire the same interface to different implementations.

## Blanket traits and impl-side dependencies

CGP grows out of blanket trait implementations (extension traits), and the single most important
idea to internalize is the **impl-side dependency**: a blanket impl can require constraints in its
`where` clause that are *not* part of the trait interface. Those hidden requirements are the
paradigm's form of dependency injection. For example:

```rust
pub trait CanGreet {
    fn greet(&self);
}

pub trait HasName {
    fn name(&self) -> &str;
}

impl<Context> CanGreet for Context
where
    Context: HasName,
{
    fn greet(&self) {
        println!("Hello, {}!", self.name());
    }
}
```

`CanGreet` hides its `HasName` dependency inside the blanket impl, so a caller that is itself
generic does not have to forward `HasName` the way it would with a free generic function. In CGP
you typically start by writing generic logic as a blanket trait like this, and promote it to a
full CGP component later only when you need more than one implementation. Blanket traits are not
themselves CGP components, but the technique recurs throughout CGP.

## The core vocabulary

CGP code is readable once you hold five terms in mind. A **consumer trait** is the ordinary,
`self`-style trait you call (`CanGreet`, `CanCalculateArea`); it reads as a verb (`CanDoX`). A
**provider trait**, generated by `#[cgp_component]`, is the same interface with `Self` moved to an
explicit `Context` generic parameter (`Greeter<Context>`, `AreaCalculator<Context>`); it reads as
a noun (`SomethingDoer`, or `…Provider` when no noun fits). A **provider** is a zero-sized marker
struct (e.g. `GreetHello`, `RectangleArea`) that implements a provider trait — it carries no
runtime value and exists only as a name at the type level. **Wiring** is the act of telling a
context which provider implements each component, recorded in a type-level table. An
**impl-side dependency** is a `where`-clause constraint a provider needs but the consumer trait
does not expose. A **component** is the whole bundle a macro generates: consumer trait, provider
trait, and a `…Component` marker type that keys the wiring table.

The duality is the crux: a consumer trait is what you *use*, a provider trait is what you
*implement*, and the macro-generated blanket impls connect them so that wiring a context to a
provider makes the context implement the consumer trait. When you see a provider trait method take
`context: &Context` where the consumer took `&self`, that is the same method — `self` became
`context`.

A persistent source of confusion to avoid: inside a provider, `self`/`Self` (in `#[cgp_impl]`) or
the `context`/`Context` parameter (in the raw provider-trait form) always refer to the **context**,
never to the provider struct. The provider struct is a pure type-level name with no fields and no
runtime value — you cannot store state in it, and any attempt to read a "field" of a provider at
runtime is a mistake.

## Reading CGP code on sight

To follow most CGP code you only need to recognize a handful of shapes:

- `#[cgp_component(Greeter)] trait CanGreet { fn greet(&self); }` defines a component. `CanGreet`
  is the consumer trait you call; `Greeter<Context>` is the provider trait implementations target;
  `GreeterComponent` is the wiring key.
- `#[cgp_impl(new GreetHello)] impl Greeter where Self: HasName { fn greet(&self) { … } }` writes a
  provider named `GreetHello` for the `Greeter` component. Inside, `self`/`Self` mean the
  *context*, and the `where` clause lists impl-side dependencies.
- `#[cgp_fn] fn rectangle_area(&self, #[implicit] width: f64, #[implicit] height: f64) -> f64 { … }`
  defines a single-implementation capability as a blanket-impl trait, pulling `width`/`height` from
  the context's fields automatically.
- `delegate_components! { Person { GreeterComponent: GreetHello } }` wires the `Person` context: it
  says "for the `Greeter` component, `Person` uses the `GreetHello` provider." After this, `Person`
  implements `CanGreet`.
- `check_components! { Person { GreeterComponent } }` is a compile-time assertion that the wiring is
  complete and all transitive dependencies are satisfied.

A consumer trait can also be implemented directly on a context like any normal Rust trait
(`impl CanGreet for Person { … }`) — CGP traits are a superset of vanilla traits, and the macros
only save boilerplate.

## Which construct to use: prefer this, not that

When two constructs can express the same thing, CGP has a preferred one, and choosing wrong produces
code that compiles but reads as dated or misuses an advanced tool. This table is the quick answer, so
you pick the right pattern even without reading further. Each preference is a default with narrow
exceptions, spelled out under [Writing providers](#writing-providers) below and, with full before/after
examples, in the [modern-idioms](references/modern-idioms.md) sub-skill; the "avoid" column is not
wrong, it is what you *read* in generated code and legacy wiring, not what you *write* anew.

| To… | Prefer | Not (legacy / advanced / read-only) |
|---|---|---|
| write a provider | `#[cgp_impl]`, header `impl Trait` (omit `for Context`) | raw `#[cgp_provider]` / `#[cgp_new_provider]` |
| read a context field for your own use | an `#[implicit]` argument | a getter trait declared just to read it |
| publish a shared accessor | `#[cgp_auto_getter]` | `#[cgp_getter]` (only for per-context field choice) |
| require a capability | `#[uses(Trait)]` | `where Self: Trait` |
| require an inner provider | `#[use_provider(P: Trait)]` | `where P: Trait<Self>` |
| name an abstract type (e.g. `Error`) | `#[use_type(Trait.Type)]` + the bare alias | `: Trait` supertrait + `Self::Type` |
| add a capability supertrait | `#[extend(Trait)]` | native `pub trait …: Supertrait` |
| dispatch a component per type | the `open` statement (or a namespace) | `#[derive_delegate]` + `UseDelegate<new …>` tables |
| verify a context is fully wired | separate `check_components!` (or `delegate_and_check_components!` for a basic starter context) | leaving a context's wiring unchecked |
| build a field/list/string/path type | `Symbol!` / `Product!` / `Sum!` / `Path!` sugar | hand-written `Cons`/`Nil`/`Chars`/`Either`/`PathCons` |

Two names are gone entirely, not merely dated: never write `#[cgp_context]` (removed — assemble a
context with `delegate_components!` and the derives instead) or `ProvideType` (renamed to
`TypeProvider`). One caveat carries across the whole table: a construct's *own* local associated type
stays qualified as `Self::Output` — only an *imported* abstract type is written bare via `#[use_type]`.

## The prelude and version

Almost everything CGP exports comes through one import, which belongs at the top of every module
that uses CGP:

```rust
use cgp::prelude::*;
```

This skill describes CGP **v0.7.0**. A few names are intentionally *not* in the prelude and must be
imported from their module — most notably the error-handling wiring keys and backends (see Error
handling below). Inside documentation code blocks you may omit the prelude import for brevity.

---

# Components: the heart of CGP

A component is what `#[cgp_component]` builds from one trait so that *using* a capability and
*implementing* it become separate, swappable things. Applying it to a consumer trait:

```rust
#[cgp_component(Greeter)]
pub trait CanGreet {
    fn greet(&self);
}
```

generates five items, of which you write or call only the first. The **consumer trait** `CanGreet`
is emitted unchanged — callers write `person.greet()`. The **provider trait** is the same interface
with `Self` moved to a leading `Context` parameter and `self` rewritten to `context`:

```rust
pub trait Greeter<Context>: IsProviderFor<GreeterComponent, Context, ()> {
    fn greet(context: &Context);
}
```

The **component marker** `pub struct GreeterComponent;` is the zero-sized key into the wiring table.
Two **blanket impls** connect the sides: one makes any context that implements the provider trait
*for itself* automatically implement the consumer trait; the other lets a context that delegates
this component (via `DelegateComponent`) inherit the provider trait from whatever provider it
delegates to. You never write these blanket impls; they are the routing machinery, and it is enough
to think of wiring as a table lookup. Crucially, that lookup is resolved **entirely at compile
time**: the table is a set of trait impls, so the compiler picks the provider during type
resolution and monomorphizes the call to a direct, statically-dispatched one. CGP wiring is
therefore zero-cost — there is no runtime table, no dynamic dispatch, and no `vtable` in the
generated code, even though "table lookup" is a useful mental model. In real generated code the
context parameter is named `__Context__` and the provider parameter `__Provider__` (reserved
identifiers chosen so they never clash with your types); the names `Context`/`Provider` here are
for readability.

The attribute's argument sets those three generated names. The bare `#[cgp_component(Greeter)]` form
names only the provider trait; the component marker defaults to that name plus `Component`
(`GreeterComponent`) and the context to `__Context__`. A key/value form with brace delimiters
overrides any of the three — `#[cgp_component { name: GreeterComponent, provider: Greeter, context: Context }]`
— where only `provider` is required. One limitation to know: a **const generic parameter** on the
trait is rejected, because a component's extra parameters are recorded as a tuple of *types* in
`IsProviderFor` and a const value has nowhere to live there (an associated `const` *item* on the
trait is fine — it is supplied by a const-generic provider struct as usual). See
[macro-grammar](references/macro-grammar.md) for the full argument grammar and
[components](references/components.md) for the complete expansion.

## `IsProviderFor` and error messages

`IsProviderFor<Component, Context, Params>` is an empty marker trait that rides as a supertrait on
every provider trait. Its only purpose is good error messages: a provider lists its dependencies in
a `where` clause, and the macros implement `IsProviderFor` for the provider under the *same* bounds,
so when a dependency is unmet the compiler can name the missing bound instead of vaguely saying "the
trait is not implemented." When you see an error that some provider does not implement
`IsProviderFor<…>`, read it as "the provider trait is not implemented, because the named dependency
is missing." You never write `IsProviderFor` yourself — the provider macros generate it.

## Writing providers

A provider can be written at three levels of sugar over the same machinery. **`#[cgp_impl]` is the
form to prefer**, because it lets you write the provider in consumer-style syntax — keeping `self`,
`Self`, and the consumer method signatures — and the macro rewrites it into the provider-trait
shape:

```rust
#[cgp_impl(new GreetHello)]
impl Greeter
where
    Self: HasName,
{
    fn greet(&self) {
        println!("Hello, {}!", self.name());
    }
}
```

The provider name goes in the attribute argument; a leading `new` keyword also declares the
`struct GreetHello;`. **Prefer the unqualified `impl Greeter` form and let the macro insert the
context parameter** — omitting `for Context` is what makes a provider read like an ordinary trait
impl. Write the explicit `impl<Context> Greeter for Context` only when you must bound or name the
context readably (for example a lifetime or HRTB the sugar cannot express); it must then be declared
in the impl generics. Remember that `self`/`Self` here mean the context. `#[cgp_impl]` desugars to:

```rust
#[cgp_new_provider]
impl<Context> Greeter<Context> for GreetHello
where
    Context: HasName,
{
    fn greet(context: &Context) {
        println!("Hello, {}!", context.name());
    }
}
```

The lower forms are what you mostly *read* rather than write. `#[cgp_provider]` is applied to a
provider-trait impl written directly on an existing provider struct; it passes the impl through and
auto-generates the matching `IsProviderFor` impl from the same `where` clause.
`#[cgp_new_provider]` is the same but also declares the provider struct (a generic provider gets a
`PhantomData` field over its parameters, e.g. `pub struct Multiply<Field>(PhantomData<Field>);`).
The attribute argument can override the component name, which otherwise defaults to the provider
trait's name plus `Component`. One special `#[cgp_impl]` form, `#[cgp_impl(Self)]`, bypasses the
provider rewrite entirely and emits the block as a *direct* consumer-trait impl on the concrete
context (the `for Context` clause is then required) — useful when you want to implement a consumer
trait by hand while still applying companion attributes such as `#[use_provider]`.

**Strongly prefer the modern, vanilla-looking idioms when you write CGP, and reach for the explicit
forms only when a construct genuinely cannot express the case.** Each idiom below trades a piece of
visible machinery for syntax that reads like ordinary Rust:

- **Write providers with `#[cgp_impl]`** (not `#[cgp_provider]`/`#[cgp_new_provider]`), omitting
  `for Context` so the header reads `impl Greeter`.
- **Declare dependencies with attributes, not hand-written bounds:** capability dependencies with
  [`#[uses(...)]`](references/functions-and-getters.md), inner-provider dependencies with
  [`#[use_provider(...)]`](references/higher-order-providers.md), instead of raw `Self:` /
  `Provider: …<Self>` `where` clauses.
- **Read context fields with [`#[implicit]`](references/functions-and-getters.md) arguments** rather
  than a getter trait. Reserve `#[cgp_auto_getter]` for a *published* accessor several providers
  share, and `#[cgp_getter]` for the advanced case of choosing the source field per context.
- **Add non-type capability supertraits with [`#[extend(...)]`](references/functions-and-getters.md)**
  rather than native `: Supertrait` syntax, which reads as OOP-style inheritance rather than a
  capability import.
- **Import abstract types with [`#[use_type]`](references/abstract-types.md)**, writing the bare
  alias (`Scalar`, `Error`) instead of a hand-written `: HasScalarType` supertrait and a qualified
  `Self::Scalar` at every use. This holds even in `#[cgp_component]` definitions: prefer
  `#[use_type(HasErrorType.Error)]` over `: HasErrorType` + `Self::Error`.
- **Dispatch a generic-parameter component with the `open` statement or a namespace**, skipping
  `#[derive_delegate]`/`UseDelegate` when defining a new component.

The explicit forms remain correct and are what you *read* in generated code and desugaring; the
exceptions that still need them are narrow — an associated-type-equality bound (`Iterator<Item = u8>`)
that `#[uses]` cannot spell, a lifetime or HRTB that forces a named context, or a **local**
associated type such as `Self::Output`, which stays qualified because it is the trait's own type,
not an imported abstract one. For the full legacy-to-modern before/after mapping of each idiom — the
reference to load whenever you read or modernize existing CGP — see
[modern-idioms](references/modern-idioms.md).

The provider's `where` clause is where **impl-side dependencies** live: `GreetHello` requires
`Self: HasName`, but `CanGreet` exposes no such bound, so a caller bounding on `CanGreet` never sees
`HasName`. The wiring satisfies each dependency by resolving it through the same context.

A consumer trait is still an ordinary trait: when you don't need multiple implementations, write
`impl CanGreet for Person { … }` directly and skip the provider machinery entirely.

---

# Wiring: connecting a context to providers

Wiring records, on a context type, which provider supplies each component. The underlying mechanism
is the `DelegateComponent` trait — a type-level table whose key is the `…Component` marker and whose
`Delegate` associated type is the chosen provider — but you almost always write it through
`delegate_components!`:

```rust
#[derive(HasField)]
pub struct Person {
    pub name: String,
}

delegate_components! {
    Person {
        GreeterComponent: GreetHello,
    }
}
```

After this, `Person` implements `CanGreet` and `person.greet()` resolves through the table to
`GreetHello`. Swapping the table entry is the only change needed to swap behavior.

The macro has a few shorthands. An **array key** maps several components to one provider:
`[FooComponent, BarComponent]: FooBarProvider`. A leading **`new`** keyword
(`delegate_components! { new MyComponents { … } }`) also defines `struct MyComponents;`, which is how
you build an **aggregate provider** — a zero-sized provider that holds a table dispatching each
component to a sub-provider, so other contexts can delegate a whole group of components to it as one
reusable unit. A leading **generic list** (`delegate_components! { <T> MyContext<T> { … } }`) wires a
whole family of contexts at once.

The target of `delegate_components!` is therefore not always a context: it is either a concrete
context (as `Person` is above) or an aggregate provider (as `MyComponents` is). This distinction
governs checking — an aggregate provider is dispatched *to* by contexts and is never its own context,
so it must be wired with plain `delegate_components!` and never `delegate_and_check_components!`; the
next section explains why.

To understand what wiring *does*, picture the explicit version: `delegate_components!` is equivalent
to implementing the consumer trait by hand and forwarding to the provider —
`impl CanGreet for Person { fn greet(&self) { <GreetHello as Greeter<Person>>::greet(self) } }`.
The macro just generates that plumbing (plus the `IsProviderFor` propagation) for you.

## `UseContext`

`UseContext` is a provider that implements a provider trait by routing back through the context's
*own* consumer-trait impl — the dual of the consumer blanket impl. Wiring a component to
`UseContext` means "use whatever this context already does for this trait," which is mainly useful
as the default inner provider of a higher-order provider (below). Delegating a component directly to
`UseContext` when the context's only implementation of that component *is* that delegation creates a
circular dependency and fails to compile.

## Dispatching a generic-parameter component per type with `open`

When a component is generic over a type parameter, you often want a different provider per value of
that parameter. The modern, preferred way is the **`open` statement** inside `delegate_components!`.
Given a component `CanCalculateArea<Shape>` (provider `AreaCalculator`), a context dispatches per
shape like this:

```rust
delegate_components! {
    MyApp {
        open AreaCalculatorComponent;

        @AreaCalculatorComponent.Rectangle: RectangleArea,
        @AreaCalculatorComponent.Circle: CircleArea,
    }
}
```

The `open … ;` header opens one or more components for per-value wiring and **must lead** the
block (it comes before any plain `Component: Provider` mappings, or the macro fails to parse). The
braces are optional when opening a single component (`open AreaCalculatorComponent;`); use the
braced list `open { A, B };` to open several at once. Each
`@Component.Key: Provider` entry then assigns a provider for one value of the dispatch parameter; a
brace group on the final segment shares one provider across several values
(`@AreaCalculatorComponent.{u32, u64, bool}: SomeProvider`), and a key may carry generics
(`@SomeComponent.<'a, T> &'a T: SomeProvider`). `open` works through the `RedirectLookup` impl that
every `#[cgp_component]` already generates, so it needs no extra attribute on the component. It does
not combine with a joined namespace (`#[prefix(...)]`); that is the full namespace feature.

**Legacy form (read but don't write):** older code dispatches the same way by wrapping a nested
table in the `UseDelegate` provider —
`AreaCalculatorComponent: UseDelegate<new AreaCalculatorComponents { Rectangle: RectangleArea, Circle: CircleArea }>`
— generated by a `#[derive_delegate(UseDelegate<Shape>)]` attribute on the component. This still
works and is common in existing code, but it is slated for deprecation; prefer `open` for new code.
Note that some CGP-shipped components (the error and handler families) are still *defined* with
`#[derive_delegate]` in the library, so you will see `UseDelegate` tables wiring them.

---

# Checking: verifying wiring at compile time

CGP wiring is **lazy**: defining a `delegate_components!` entry does not itself check that the
provider's transitive dependencies are satisfied. A missing dependency therefore surfaces only when
the consumer trait is finally used, often as a confusing error. To catch it early and clearly, assert
the wiring with a check:

```rust
check_components! {
    Person {
        GreeterComponent,
    }
}
```

This generates a check trait whose supertrait is `CanUseComponent<GreeterComponent, ()>` for
`Person`; if `Person` cannot actually use the component, the compiler reports the missing dependency
at this site, walking through `IsProviderFor` so the real cause (e.g. a missing
`HasField<Symbol!("name")>`) is named rather than hidden. For a component with generic parameters,
list the parameter after the component (`GreeterComponent: Rectangle`), group multiple parameters as
a tuple (`(Rectangle, f64)`), and use array syntax to check several at once.

`delegate_and_check_components!` fuses wiring and checking in one step, so every delegation is
verified the moment it is written:

```rust
delegate_and_check_components! {
    Person {
        GreeterComponent: GreetHello,
    }
}
```

Its check trait is named `__CanUse{Context}` (vs. `__Check{Context}` for `check_components!`), so
both macros can appear once per module without clashing; override with `#[check_trait(Name)]`. When
the delegated component has generic parameters, add `#[check_params(...)]` on the entry; skip a
single entry's check with `#[skip_check]`.

**This fused macro is a convenience for basic wiring and for getting started — not the default for
advanced code.** It exists so a newcomer cannot forget to write a separate `check_components!` and
then hit confusing lazy-wiring errors, and it derives a check only for the plain `Component: Provider`
delegation form. It cannot easily derive checks for advanced mappings — generic-parameter dispatch
(the `open` statement and `@`-path keys), namespaces, or per-layer higher-order checks. **In larger,
more advanced codebases, keep `delegate_components!` and `check_components!` separate**, which gives
full control over what is checked: `#[check_providers(...)]` per provider layer, concrete parameters
for generic keys, and checks over opened or namespaced wiring. The one non-negotiable is that a
context's wiring *is* checked somehow; which macro you use to do it scales with the wiring's
complexity.

One case makes `delegate_and_check_components!` not just unnecessary but wrong: an **aggregate
provider** (the `new MyComponents { … }` table above). That target is a provider other contexts
delegate to, not a context itself — it has no fields and never implements a provider trait with
itself in the context position — so the check's `CanUseComponent` assertion on it cannot hold and the
macro would report spurious failures. Wire an aggregate provider with plain `delegate_components!`;
it is verified indirectly when a real context that delegates to it is checked, or directly with a
`#[check_providers(...)]` block that asserts `IsProviderFor` on it. Finally, not every unsatisfied
bound is a CGP component — some are ordinary or blanket traits that `check_components!` cannot verify.

For a nested [higher-order provider](references/higher-order-providers.md), checking the context
tells you a layer is broken but not which one. The `#[check_providers(...)]` attribute on a
`check_components!` table changes the assertion from `CanUseComponent` on the context to
`IsProviderFor` on each named provider, so a dependency missing only from the outer wrapper errors on
its line alone while one missing from the inner provider errors on both — pinpointing the layer. See
[checking](references/checking.md) for the debugging playbook.

---

# Functions and getters: the ergonomic surface

Most basic CGP reads and writes values from the context, and the constructs here make that look like
plain Rust.

## `HasField` and `#[derive(HasField)]`

`HasField<Tag>` is tag-keyed field access. The `Tag` is a type-level name: `Symbol!("width")` for a
named field or `Index<0>` for a tuple field. `#[derive(HasField)]` generates one impl per field:

```rust
#[derive(HasField)]
pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}
// generates HasField<Symbol!("width"), Value = f64> and HasField<Symbol!("height"), Value = f64>
```

Field values are read with `self.get_field(PhantomData::<Symbol!("width")>)`; the `PhantomData`
carries the tag so type inference knows which field is meant.

## `#[cgp_fn]` and `#[implicit]` arguments

`#[cgp_fn]` turns one function into a single-implementation blanket-impl trait — the simplest entry
point to CGP. Arguments marked `#[implicit]` are removed from the signature and pulled from context
fields via `HasField`:

```rust
#[cgp_fn]
fn rectangle_area(&self, #[implicit] width: f64, #[implicit] height: f64) -> f64 {
    width * height
}
```

This generates a `RectangleArea` trait (named from the function in PascalCase; override with
`#[cgp_fn(MyName)]`) with a blanket impl for any context that has `width: f64` and `height: f64`
fields. Implicit arguments get `.clone()` added automatically for owned values and `.as_str()` for
`&str`. Generic parameters on the function move to the trait and impl; the `where` clause becomes
impl-side dependencies on the impl only; generic *method* parameters are intentionally unsupported.
Prefer implicit arguments for basic code — they make CGP look like ordinary functions.

## `#[uses]`, `#[extend]`, `#[extend_where]`

`#[uses(TraitA, TraitB<Param>)]` (on `#[cgp_fn]` or `#[cgp_impl]`) imports `Self` trait bounds, read
like a `use` statement — it accepts only the simple `Trait<Params>` form (no associated-type
equality; write those as explicit `where` clauses):

```rust
#[cgp_fn]
#[uses(RectangleArea)]
fn scaled_rectangle_area(&self, #[implicit] scale_factor: f64) -> f64 {
    self.rectangle_area() * scale_factor * scale_factor
}
```

`#[extend(Trait)]` adds *supertrait* bounds to the generated trait — the only way to add supertraits
in `#[cgp_fn]` (whose `where` clauses are impl-side dependencies), and the **preferred** way to add a
*non-type capability* supertrait on `#[cgp_component]` too: `#[extend(HasName)]` reads as importing a
capability, whereas the native `pub trait CanGreet: HasName` syntax reads as OOP-style inheritance
from a parent class, which a CGP supertrait is not. (When the supertrait is an abstract-type component
whose associated type the signatures name, prefer `#[use_type]` instead — it adds the supertrait *and*
rewrites the type, and is the recommended form for abstract-type components.) `#[extend_where(Bound)]`
adds `where` clauses to the generated trait definition (`#[cgp_fn]` only), and accepts arbitrary
predicates including associated-type equality, unlike `#[uses]`/`#[extend]`. A fourth `#[cgp_fn]`-only
attribute, `#[impl_generics(Param: Bound)]`, adds a bounded generic parameter to the generated *impl*
alone — not the trait — which is how a `#[cgp_fn]` body borrows a generic value it does not want to
expose as a trait parameter (e.g. `#[impl_generics(Name: Display)]` over an `#[implicit] name: &Name`).

## Getters: `#[cgp_auto_getter]`, `#[cgp_getter]`, `UseField`

Getter traits are for *publishing* a context field as a shared capability, not for a provider
reading a value for its own use — for that, prefer an `#[implicit]` argument (above), which needs no
separate trait. Reach for a getter trait only when the accessor is genuinely shared: depended on by
several providers through `#[uses(HasName)]`, or carrying an associated type inferred from the field.

`#[cgp_auto_getter]` generates a blanket getter impl over `HasField`, with the field name taken from
the method name, and is the getter form to prefer:

```rust
#[cgp_auto_getter]
pub trait HasName {
    fn name(&self) -> &str;
}
// blanket impl for any context with a `name` field; &str/&String shorthands handled
```

A single-getter trait may instead declare a local associated type used as the return type, inferred
from the field — `trait HasName { type Name; fn name(&self) -> &Self::Name; }`.

`#[cgp_getter]` is an **advanced** tool, reserved for when a context needs full control over which
field a getter reads from — most getters should use `#[cgp_auto_getter]` or an implicit argument
instead. It is like `#[cgp_component]` but also provides a `UseField<Tag>` blanket impl, so the
getter's source field can be chosen by *wiring* rather than fixed to the method name. The
`UseField<Tag>` provider implements a getter by reading the field named `Tag`, which may differ from
the method name:

```rust
delegate_components! {
    Person {
        NameGetterComponent: UseField<Symbol!("first_name")>,
    }
}
// Person::name() now returns the `first_name` field
```

`UseFieldRef` is the `AsRef`/`AsMut`-based variant. Any getter can also be implemented by hand — the
macros only save boilerplate.

---

# Abstract types

CGP abstracts over types with associated types in components. `#[cgp_type]` is the dedicated macro
(use it instead of `#[cgp_component]` for an abstract-type trait):

```rust
#[cgp_type]
pub trait HasNameType {
    type Name;
}
```

This defaults the provider name to the type name plus `TypeProvider` (here `NameTypeProvider`, marker
`NameTypeProviderComponent`) and additionally generates a `UseType` blanket impl. A context fixes the
type by wiring the component to `UseType<ConcreteType>`:

```rust
delegate_components! {
    Person {
        NameTypeProviderComponent: UseType<String>,
    }
}
// or directly: impl HasNameType for Person { type Name = String; }
```

The direct impl is just as valid and shows that abstract types are ordinary associated-type traits.

The `#[use_type(HasScalarType.Scalar)]` attribute is the recommended way to *use* an abstract type
inside `#[cgp_fn]`/`#[cgp_impl]`/`#[cgp_component]`: it rewrites bare `Scalar` to the fully-qualified
`<Self as HasScalarType>::Scalar` everywhere and adds the supertrait/where bound, removing `Self::`
boilerplate and ambiguity. CGP's built-in abstract-type component is `HasType` (provider
`TypeProvider`).

---

# Higher-order providers

A **higher-order provider** takes another provider as a generic parameter and constrains it with a
provider-trait bound, so its inner behavior is chosen by wiring rather than fixed:

```rust
#[cgp_impl(new ScaledAreaCalculator<InnerCalculator>)]
#[use_provider(InnerCalculator: AreaCalculator)]
impl<InnerCalculator> AreaCalculator {
    fn area(&self, #[implicit] scale_factor: f64) -> f64 {
        let base_area = InnerCalculator::area(self);
        base_area * scale_factor * scale_factor
    }
}
```

`#[use_provider(InnerCalculator: AreaCalculator)]` completes the inner provider's bound by adding the
`Self` parameter for you (you write `: AreaCalculator`, it means `AreaCalculator<Self>`) and moves it
into the `where` clause — that is the *only* thing the attribute does. Inside the body you still call
the provider explicitly with the associated-function form `InnerCalculator::area(self)`; there is no
call-site rewriting. A context then chooses the inner provider when wiring, e.g.
`AreaCalculatorComponent: ScaledAreaCalculator<RectangleAreaCalculator>`.

A higher-order provider often defaults its inner parameter to `UseContext`
(`pub struct IterSumArea<Inner = UseContext>(PhantomData<Inner>);`), so that when no inner provider is
named, the inner step falls back to the context's own wiring. Not every provider with a generic
parameter is higher-order: a provider like `GetName<Tag>` that uses `Tag` only as a `HasField` key
(no provider-trait bound) is not.

When a component itself is generic (`#[cgp_component(AreaCalculator)] trait CanCalculateArea<Shape>`),
the provider trait appends the parameters after the context (`AreaCalculator<Context, Shape>`),
`IsProviderFor` groups them into its `Params` tuple, and lifetimes are lifted into the `Life<'a>`
type. Such a component is most useful for **cross-context dependencies**: when the main target is a
generic parameter (e.g. `CanCalculateArea<Shape>: HasScalarType`), individual shape types need not
implement shared capabilities — the common context supplies the shared abstract type, value-level
injection (a global scale factor via a getter), and lazy per-context provider binding, so two apps
can wire the same shape to different providers.

---

# Error handling

CGP makes the error type abstract so generic code can fail without naming a concrete error.
`HasErrorType` (an abstract-type component, `type Error: Debug`) gives a context one shared
error type; `CanRaiseError<SourceError>` constructs it from a concrete source error
(`Context::raise_error(source)`); `CanWrapError<Detail>` attaches detail. Both build on
`HasErrorType` and are associated-function (no `self`) components that dispatch per source/detail
type. An error-aware trait imports that error type with `#[use_type(HasErrorType.Error)]`, so it
names the error as the bare `Error` instead of writing `: HasErrorType` and `Self::Error` by hand:

```rust
#[cgp_component(Loader)]
#[use_type(HasErrorType.Error)]
pub trait CanLoad {
    fn load(&self, path: &str) -> Result<String, Error>;
}

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

A context wires its error type and the raise/wrap behavior. The backend providers — `RaiseFrom`
(convert via `From`), `ReturnError`, `RaiseInfallible`, `PanicOnError`, `DebugError`/`DisplayError`
(format into a `String` and forward), `DiscardDetail` — plug in per source type, modern-style with
`open`:

```rust
delegate_components! {
    App {
        open ErrorRaiserComponent;

        ErrorTypeProviderComponent: UseType<String>,
        @ErrorRaiserComponent.String: RaiseFrom,
        @ErrorRaiserComponent.ParseError: DebugError,
    }
}
```

**Imports:** `HasErrorType`, `CanRaiseError`, and `CanWrapError` come from the prelude, but the
wiring keys (`ErrorTypeProviderComponent`, `ErrorRaiserComponent`, `ErrorWrapperComponent`) live
under `cgp::core::error`, and the backend providers (`RaiseFrom`, `DebugError`, …) under
`cgp::extra::error` — import the specific names you wire. Standalone backends (`cgp-error-anyhow`,
`cgp-error-eyre`, `cgp-error-std`) provide ready error types.

---

# Handlers: the computation family

CGP models computation as a family of components along three axes — synchronous vs. async,
infallible vs. fallible, and input-taking vs. input-free:

- **`Computer` / `CanCompute`** — a synchronous, infallible transform `compute(&self, PhantomData<Code>, input) -> Output`. By-reference (`ComputerRef`) and async (`AsyncComputer`) variants exist.
- **`TryComputer` / `CanTryCompute`** — the fallible computer.
- **`Producer` / `CanProduce`** — input-free production (only a context and a `Code` tag).
- **`Handler` / `CanHandle`** — the general **async, fallible, error-aware** computation; the workhorse for I/O and pipelines. It supertraits `HasErrorType`.
- **`CanRun` / `CanSendRun`** — task runners.

Any CGP trait with async methods — a handler, a runner, or one you define — declares them under the
`#[async_trait]` attribute, which rewrites each `async fn` to `-> impl Future` (the lint-clean,
allocation-free form). The generated future carries no `Send` bound, so spawning it on a
work-stealing executor needs the `Send`-recovery pattern in [handlers](references/handlers.md).

`#[cgp_computer]` and `#[cgp_producer]` define a `Computer`/`Producer` provider from a function.
Providers compose through combinators: `PipeHandlers<Product![A, B, C]>` chains handlers
left-to-right, `ComposeHandlers` nests them, `ReturnInput` passes input through, and the `Promote*`
adapters lift a simpler handler (e.g. a sync `Computer`) into a more capable one (an async
`Handler`). For example, a context wires a pipeline of field-reading computers:

```rust
delegate_components! {
    MyContext {
        ComputerComponent:
            PipeHandlers<Product![
                Multiply<Symbol!("foo")>,
                Add<Symbol!("bar")>,
                Multiply<Symbol!("baz")>,
            ]>,
    }
}
// context.compute(PhantomData::<()>, 5) runs ((5*foo)+bar)*baz
```

Dispatch routes an extensible-data input to per-variant handlers: `#[cgp_auto_dispatch]` generates a
handler from a trait, and the combinators `MatchWithHandlers` / `MatchWithValueHandlers` /
`ExtractFieldAndHandle` match an enum's variants to sub-handlers, proving exhaustiveness without a
wildcard. Monadic handlers (`PipeMonadic`, `BindOk`, `BindErr`, the identity/ok/err monads) compose
handlers through a monad. The handler family is broad — read [handlers](references/handlers.md) for
the full set, including `Send`-bound recovery for async trait methods.

---

# Extensible data

CGP can build and read structs and enums generically, by their named fields and variants. The
`#[derive(HasFields)]` derive exposes a type's whole field list; `#[derive(CgpData)]` (and the
record/variant-specific `CgpRecord`/`CgpVariant`) derive the full extensible-data machinery:

```rust
#[derive(CgpData)]
pub struct Person {
    pub first_name: String,
    pub last_name: String,
}
```

Records are built field-by-field through the builder family (`HasBuilder`, `BuildField`) — the
**extensible builder pattern** assembles a context from independent per-field outputs. Variants are
constructed with `FromVariant`, deconstructed with the `ExtractField` extractor family, and the
**extensible visitor pattern** handles each variant. The type-level spines underneath are the
product list (`Product![A, B, C]` over `Cons`/`Nil`) for records and the sum list (`Sum![A, B]` over
`Either`/`Void`) for variants. Structural **casts** convert between shapes — `CanUpcast` widens a
smaller enum into a larger one, `CanDowncast` narrows, `CanBuildFrom` rebuilds a record from a
superset. Dispatching extensible-data inputs to handlers uses the dispatch combinators above.

---

# Namespaces

Namespaces are reusable, inheritable wiring tables (a preset mechanism) that keep top-level wiring
short as component counts grow. `cgp_namespace! { new MyNs: ParentNs { … } }` defines a namespace
(optionally inheriting a parent after the colon); a context then **joins** it inside
`delegate_components!` with a `namespace MyNs;` statement, after which every lookup it does not wire
directly forwards through the namespace, so any direct entry overrides just that key. A component
**registers into** a namespace with the `#[prefix(@path in MyNs)]` attribute on its `#[cgp_component]`
trait, and a `#[cgp_impl]` provider registers as a per-type default with `#[default_impl(T in DefaultImpls1<Component>)]`,
which a context pulls in with a `for <T, Provider> in Table { … }` loop.

The underlying mechanism is the `RedirectLookup` provider, which re-routes a component lookup along a
type-level `Path!`; the `open` statement above is a lightweight special case of it. `DefaultNamespace`
resolves a default provider when a context does not override one. Read
[namespaces](references/namespaces.md) for the preset and inheritance syntax.

---

# Type-level primitives: a decoder ring

CGP encodes lists, strings, and numbers as types. You mostly use the sugared macros and only need to
*recognize* the expanded forms in errors:

- **`Symbol!("name")`** — a type-level string (field-name tag). Expands to `Symbol<4, Chars<'n', Chars<'a', Chars<'m', Chars<'e', Nil>>>>`. The leading length works around missing const-generics.
- **`Product![A, B, C]`** — a type-level list. Expands to `Cons<A, Cons<B, Cons<C, Nil>>>`. `product![…]` is the value-level form. Used for field lists and handler pipelines.
- **`Sum![A, B]`** — a type-level sum (the dual of `Product!`), over the `Either`/`Void` spine. Used for enum variant lists.
- **`Index<N>`** — a type-level natural number, tags tuple-struct fields.
- **`Field`** — a value paired with its type-level name tag.
- **`Path!`** / `PathCons` — a type-level path, used by namespaces and `RedirectLookup`.
- **`Life<'a>`** — a lifetime lifted into a type, used when a component has lifetime parameters.
- **`MRef`** — an owned-or-borrowed value.

Prefer the sugar (`Symbol!`, `Product!`) and the readable names (`Cons`/`Nil`) in anything you write.

---

# Sub-skills: load the one that owns your task

The primer gave you the shape of every construct; each sub-skill below is the ground truth for one
area — the exact grammar, the full expansion, the corner cases, and worked examples. **Loading the
relevant sub-skill is not optional polish; it is the step that turns a plausible guess into correct
code.** Every entry names what it adds beyond this primer *and* what you would be guessing at without
it, so you can see the risk of skipping it. Load a sub-skill whenever your task touches its area —
reading it, writing it, reviewing it, or debugging an error that mentions it — and re-load it when
you move into an unfamiliar corner. When a task spans several areas, load each one; they cross-link,
and following those links is expected, not a detour.

Two of the sub-skills are cross-cutting rather than construct-specific, and one of them applies to almost every task. Start with **[references/macro-grammar.md](references/macro-grammar.md)** for any task that writes, edits, or debugs CGP syntax: it is the single reference for the formal grammar of every macro, the invariant each expansion preserves, and a decoder for the compiler errors CGP produces. *Without it* you are guessing which attribute forms parse, what a macro emits, and what an `IsProviderFor` or `DelegateComponent` error is actually telling you. Reach for **[references/modern-idioms.md](references/modern-idioms.md)** whenever you read or modernize existing CGP: it maps every legacy, explicit form to the modern idiom you should prefer, both to write vanilla-looking code and to decode the inside-out provider impls, hand-written `where` bounds, `Self::Type` paths, and `UseDelegate` tables you meet in older code. *Without it* you will either propagate outdated syntax or fail to recognize that legacy code and modern code mean the same thing.

The remaining sub-skills each own one construct family:

- **[references/components.md](references/components.md)** — `#[cgp_component]` and the full expansion (consumer/provider traits, the two blanket impls, the `…Component` marker), why `IsProviderFor` exists, and the three provider-writing macros (`#[cgp_impl]`, `#[cgp_provider]`, `#[cgp_new_provider]`). *Without it* you will misjudge what `self`/`Self` mean inside a provider and how the blanket impls route a call. Load it before writing any component or provider.
- **[references/wiring.md](references/wiring.md)** — `DelegateComponent`, every `delegate_components!` form (arrays, `new`, generic tables), `open` per-type dispatch, direct consumer-trait impls, `UseContext` and its circular-dependency trap, the legacy `UseDelegate` tables, and the other providers you see in tables (`WithProvider` and its `WithField`/`WithType`/`WithContext` aliases, `UseDefault`). *Without it* you will not know when a hand-written impl collides with the table, why `UseContext` overflows, or what a `WithField<…>` entry means. Load it before wiring any context.
- **[references/checking.md](references/checking.md)** — why wiring is lazy, how check traits and `CanUseComponent` force readable errors, every `check_components!` / `delegate_and_check_components!` option (`#[check_trait]`, `#[check_providers]`, `#[check_params]`, `#[skip_check]`), and a debugging playbook. *Without it* you cannot localize a broken wiring or read the error it throws. Load it whenever a wiring fails to compile.
- **[references/functions-and-getters.md](references/functions-and-getters.md)** — `HasField`/`#[derive(HasField)]`, `#[cgp_fn]`, `#[implicit]` and its access rules, `#[uses]`/`#[extend]`/`#[extend_where]`/`#[impl_generics]`, the getters `#[cgp_auto_getter]`/`#[cgp_getter]`/`UseField`/`WithField`, and `ChainGetters` for nested-context fields. *Without it* you will reach for a getter trait where an implicit argument is idiomatic, or misapply the `.clone()`/`.as_str()`/`&mut` field-access rules. Load it for the ergonomic day-to-day surface.
- **[references/abstract-types.md](references/abstract-types.md)** — `#[cgp_type]`, the built-in `HasType`/`TypeProvider`, wiring with `UseType<T>` (and `UseDelegatedType` for table-chosen types), importing types with the `#[use_type]` attribute (distinct from the `UseType` provider), and the `WithType`/`WithDelegatedType` adapters. *Without it* you will confuse the provider and the attribute and write `Self::` paths by hand. Load it for any associated-type abstraction.
- **[references/higher-order-providers.md](references/higher-order-providers.md)** — providers parameterized by other providers, the stray `<Self>` on the inner bound, `#[use_provider]`, `UseContext` defaults, generic-parameter components, and cross-context dependencies. *Without it* you will call the inner provider as a method instead of `Provider::method(self)` and misplace the context slot. Load it before composing providers.
- **[references/error-handling.md](references/error-handling.md)** — `HasErrorType`, `CanRaiseError`/`CanWrapError`, the backend providers (`RaiseFrom`, `DebugError`, …), and — critically — which names come from the prelude versus `cgp::core::error` / `cgp::extra::error`. *Without it* you will fail to import the wiring keys and backends. Load it for any fallible CGP code.
- **[references/handlers.md](references/handlers.md)** — the `Computer`/`TryComputer`/`Producer`/`Handler`/runner family across its three axes, `#[cgp_computer]`/`#[cgp_producer]`/`#[cgp_auto_dispatch]`, the combinators (`PipeHandlers`, `Promote*`, dispatch matchers), monadic handlers, the `HasRuntime`/`HasRuntimeType` runtime components, and the `Send`-recovery pattern. *Without it* you will pick the wrong family member or miswire a pipeline. Load it for computation and I/O pipelines.
- **[references/extensible-data.md](references/extensible-data.md)** — the `CgpData`/`CgpRecord`/`CgpVariant` derives, the builder and extractor families (with the optional/defaulted-field extension), `Product!`/`Sum!` spines and their `AppendProduct`/`ConcatProduct`/`MapFields` algebra, structural casts (`CanUpcast`/`CanDowncast`/`CanBuildFrom`), and the builder/visitor patterns. *Without it* you will miss the compile-time exhaustiveness guarantees and the single-payload variant rule. Load it for generic struct/enum manipulation.
- **[references/namespaces.md](references/namespaces.md)** — `cgp_namespace!`, the `namespace`/`for … in` statements that join a namespace, the `#[prefix]`/`#[default_impl]` attributes that register into one, `RedirectLookup`, `Path!`, and the `DefaultNamespace` family. *Without it* you cannot read or write preset/inheritance wiring. Load it whenever wiring is grouped or inherited.
- **[references/type-level-primitives.md](references/type-level-primitives.md)** — `Symbol!`/`Chars`, `Product!`/`Cons`/`Nil`, `Sum!`/`Either`/`Void`, `Index`, `Field`, `Path!`/`PathCons`, `Life`, `MRef`, and the `StaticFormat` recovery traits. *Without it* you cannot decode the long nested types in error messages and expansions. Load it as the decoder ring.
- **[references/modularity-hierarchy.md](references/modularity-hierarchy.md)** — the five-level spectrum from a plain blanket trait to per-provider wiring, and which coherence rule each level escapes. *Without it* you will reach for more CGP machinery than a problem needs. Load it when deciding *how much* CGP to apply.

## Exhaustive online reference

For the exact macro expansion of any construct, every accepted syntax form, corner cases, or the
implementing source, consult the online knowledge base at
**https://github.com/contextgeneric/cgp/tree/main/docs** — its `reference/`, `concepts/`, and
`examples/` directories are the authoritative, exhaustive record. Fetch the relevant page when a
detail is not covered here; do not assume a local copy exists, since this skill is deployed on its
own.

---

# Instructions for explaining CGP to users

Assume by default that the user has only basic Rust experience and is new to CGP, but do not
over-explain: when code merely uses CGP concepts, write or modify it without lecturing, and add
explanation only when asked. When you do explain, assume unfamiliarity with advanced Rust (generics,
traits, blanket impls, coherence) and with functional/type-level programming — describe type-level
tables, lists, and strings through familiar analogies such as a map or a lookup table, and expand on
advanced Rust as needed. Keep the simplified picture front and center: present wiring as choosing a
table entry, and keep `IsProviderFor`, `DelegateComponent`, and generated blanket impls out of the
explanation unless the user is asking specifically about the internals. One caveat when you reach
for an analogy: the "table lookup" is resolved at compile time and compiles down to direct static
calls, so if you use a runtime-flavored analogy like a vtable, say explicitly that — unlike a real
vtable — CGP's resolution is static and zero-cost, with no runtime table or dynamic dispatch. Never
leave a reader thinking CGP wiring has runtime lookup overhead.

When asked to explain a specific piece of code, look up the definitions it depends on before
answering. To explain a `delegate_components!` entry, find the consumer and provider traits behind
the component key and the body of the provider it maps to. To explain a provider, read its own
definition and the definitions of every capability in its `where` clause. To explain how a context
implements something, follow its wiring to see which providers are chosen and trace a method call
through them — for instance, if `NameGetterComponent` is wired to `UseField<Symbol!("first_name")>`,
then a `self.name()` call inside another provider returns the context's `first_name` field.
