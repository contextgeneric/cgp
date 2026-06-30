# `#[blanket_trait]`

`#[blanket_trait]` generates a blanket implementation for a trait whose methods, associated types, and constants carry default definitions, turning the trait into an extension trait that hides its supertrait constraints behind a clean interface.

## Purpose

`#[blanket_trait]` exists to remove the boilerplate of writing the blanket impl that makes an extension trait work. The blanket-trait pattern — the foundation CGP itself is built on — uses the `where` clause of a blanket impl to hide the constraints a piece of generic code needs from the interface that callers see. Written by hand, the pattern is repetitive: you state the supertrait bounds twice, once on the trait and once on the impl, and you copy each default method body from the trait into the impl. `#[blanket_trait]` lets you write the trait once, with its default bodies, and generates the matching impl for you.

The payoff is the impl-side-dependency, or dependency-injection, pattern in its purest form. A trait declared `pub trait FooBar: Foo + Bar` with a default `foo_bar` method exposes only `foo_bar` to its callers; the `Foo + Bar` requirements are carried by the generated blanket impl's `where` clause, so any context satisfying `Foo` and `Bar` automatically gains `FooBar` without naming those dependencies at the call site. This is the same constraint-hiding that the `/cgp` skill introduces as the core idea behind blanket traits, and it is preferred over a free generic function because transitive callers never have to repeat the `Foo + Bar` bounds.

`#[blanket_trait]` is not a CGP component. It produces an ordinary Rust trait and an ordinary blanket impl, with no consumer/provider split, no component name, and no wiring. It is the tool to reach for when a capability has exactly one definition and you want the extension-trait ergonomics without committing to the full CGP component machinery. When a capability later needs multiple alternative implementations, the trait can be promoted to a [`#[cgp_component]`](cgp_component.md).

## Syntax

`#[blanket_trait]` is applied as an attribute on a trait definition. Every method and constant in the trait must have a default body, and every associated type may declare bounds; these defaults are what the generated impl forwards. The supertraits of the trait become the dependencies that the blanket impl requires.

```rust
#[blanket_trait]
pub trait FooBar: Foo + Bar {
    fn foo_bar(&self) {
        self.foo();
        self.bar();
    }
}
```

The macro accepts an optional argument naming the generic context type used in the generated impl. When omitted, it defaults to `__Context__` — the same reserved identifier used across the CGP macros, chosen to avoid colliding with the user's own type parameters. Passing an identifier overrides this name, which is occasionally useful for readability in expanded output.

```rust
#[blanket_trait(Ctx)]
pub trait FooBar: Foo + Bar { /* ... */ }
```

The trait may carry generic parameters and associated types. Generic parameters on the trait are copied onto the impl. Associated types are turned into fresh generic parameters on the impl and bound through the supertrait's associated-type equality, which is how the pattern lifts an associated type out of a supertrait — covered in the expansion below.

## Syntax Grammar

The attribute argument of `#[blanket_trait]` is a single optional context name:

```ebnf
BlanketTraitArgs -> ContextName?

ContextName      -> IDENTIFIER
```

When the argument is omitted, the generic context type in the generated impl defaults to the reserved identifier `__Context__`. A given `IDENTIFIER` overrides that name.

## Expansion

`#[blanket_trait]` emits two items: the trait, unchanged from its definition, and a blanket impl for a generic context. The impl forwards each default method body, requires the trait's supertraits in its `where` clause, and strips the defaults from the trait so the trait declaration stays a pure interface. Starting from the method example:

```rust
#[blanket_trait]
pub trait FooBar: Foo + Bar {
    fn foo_bar(&self) {
        self.foo();
        self.bar();
    }
}
```

the macro produces the trait and a blanket impl whose `where` clause carries the `Foo + Bar` supertraits as the hidden dependency:

```rust
pub trait FooBar: Foo + Bar {
    fn foo_bar(&self);
}

impl<__Context__> FooBar for __Context__
where
    __Context__: Foo + Bar,
{
    fn foo_bar(&self) {
        self.foo();
        self.bar();
    }
}
```

The simplest case, a trait with no methods, generates an empty impl — a trait alias in everything but name:

```rust
#[blanket_trait]
pub trait FooBar: Foo + Bar {}
```

expands to:

```rust
pub trait FooBar: Foo + Bar {}

impl<__Context__> FooBar for __Context__
where
    __Context__: Foo + Bar,
{}
```

Associated types receive special handling, because lifting an associated type out of a supertrait is one of the pattern's main uses. Each associated type in the trait becomes a new generic parameter on the impl, the supertrait's associated-type equality is rewritten to reference that parameter, and the impl assigns the parameter back to the associated type. Given:

```rust
#[blanket_trait]
pub trait HasFooTypeAtBar: HasFooTypeAt<Bar, Foo = Self::FooBar> {
    type FooBar;
}
```

the macro emits:

```rust
pub trait HasFooTypeAtBar: HasFooTypeAt<Bar, Foo = Self::FooBar> {
    type FooBar;
}

impl<__Context__, FooBar> HasFooTypeAtBar for __Context__
where
    __Context__: HasFooTypeAt<Bar, Foo = FooBar>,
{
    type FooBar = FooBar;
}
```

When the associated type declares bounds, those bounds are moved into the impl's `where` clause as predicates on the introduced parameter. Declaring `type FooBar: Clone` adds `FooBar: Clone` to the generated `where` clause alongside the supertrait requirement. Associated constants are forwarded in the same way as methods: their default expressions become the impl's constant definitions, and the trait keeps only the declaration. A method or constant without a usable default is an error, since the macro has no body to forward; associated types need no default, because the macro supplies the assignment (`type FooBar = FooBar;`) itself from the introduced parameter.

## Examples

A self-contained extension trait that hides two dependencies behind one method illustrates the everyday use. The `FooBar` capability is defined once, with its body, and applies to any context implementing both `Foo` and `Bar`:

```rust
use cgp::prelude::*;

pub trait Foo {
    fn foo(&self);
}

pub trait Bar {
    fn bar(&self);
}

#[blanket_trait]
pub trait FooBar: Foo + Bar {
    fn foo_bar(&self) {
        self.foo();
        self.bar();
    }
}

pub struct Context;

impl Foo for Context {
    fn foo(&self) {}
}

impl Bar for Context {
    fn bar(&self) {}
}

fn run(ctx: &Context) {
    ctx.foo_bar(); // available because Context: Foo + Bar
}
```

`Context` never mentions `FooBar` directly; it implements only `Foo` and `Bar`, and the generated blanket impl supplies `foo_bar` automatically. A caller of `foo_bar` sees a single-method interface and is shielded from the `Foo + Bar` requirement entirely.

## Related constructs

`#[blanket_trait]` and [`#[cgp_fn]`](cgp_fn.md) both produce a blanket impl over a generic context, and both are single-implementation tools that need no wiring; the difference is the input. `#[cgp_fn]` derives the trait and its body from a plain function with `#[implicit]` field arguments, while `#[blanket_trait]` takes a trait with default bodies and supertrait constraints directly, making it the better fit when the dependencies are themselves traits rather than context fields. Both contrast with [`#[cgp_component]`](cgp_component.md), which produces a consumer/provider pair supporting multiple implementations selected through [`delegate_components!`](delegate_components.md); a `#[blanket_trait]` can be promoted to a `#[cgp_component]` when more than one implementation becomes necessary. The constraint-hiding it performs is the same impl-side-dependency mechanism that [`#[cgp_auto_getter]`](cgp_auto_getter.md) and [`HasField`](../traits/has_field.md) rely on for value injection.

## Source

The macro entry point is `blanket_trait` in [crates/macros/cgp-macro-lib/src/blanket_trait.rs](../../../crates/macros/cgp-macro-lib/src/blanket_trait.rs), which parses the optional context identifier (defaulting to `__Context__` when the attribute argument is empty) and the trait, then runs `item.to_items()?`. The logic lives in [crates/macros/cgp-macro-core/src/types/blanket_trait.rs](../../../crates/macros/cgp-macro-core/src/types/blanket_trait.rs): `to_item_impl` walks the trait items, forwards each default method/const body and associated-type assignment into the impl, lifts associated types into impl generics, moves associated-type bounds into the `where` clause, and appends the trait's supertraits as the `__Context__: ...` predicate. The `Self`-to-parameter rewriting for associated types is done by `RemoveSelfPathVisitor` in [crates/macros/cgp-macro-core/src/visitors/](../../../crates/macros/cgp-macro-core/src/visitors/). Behavioral and expansion-snapshot tests are in [crates/tests/cgp-tests/src/tests/blanket_trait.rs](../../../crates/tests/cgp-tests/src/tests/blanket_trait.rs), covering the empty, method, and associated-type-with-and-without-bounds cases.
