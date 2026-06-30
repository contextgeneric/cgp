# `check_components!`

`check_components!` asserts at compile time that a context's wiring is complete, generating `CanUseComponent`-based checks that force the compiler to report exactly which dependency is missing.

## Purpose

`check_components!` exists because CGP wiring is lazy. When [`delegate_components!`](delegate_components.md) records that a context delegates a component to some provider, the type system does not eagerly verify that the provider can actually satisfy that component for that context with all of its transitive dependencies met. The [`DelegateComponent`](../traits/delegate_component.md) impl is accepted on its own terms; whether the provider's `where` bounds hold is only tested when something downstream tries to *use* the component. A context can therefore look fully wired and still fail the moment a consumer trait is invoked.

When that failure happens far from the wiring, the error is hard to read. Asking only "does this context implement the consumer trait?" makes the compiler report the outermost unmet bound — typically that the provider does not implement the provider trait — without explaining why, because Rust hides the indirect reasoning behind that one conclusion. The root cause, often a single missing getter or type, is buried.

`check_components!` solves this by turning the question into a `CanUseComponent` check. [`CanUseComponent`](../traits/can_use_component.md) is satisfied only when the context both delegates the component and the delegated provider satisfies [`IsProviderFor`](../traits/is_provider_for.md) for that context. Because `IsProviderFor` carries the provider's real `where` bounds, routing the check through it forces the compiler to evaluate and report those bounds, so an unsatisfied transitive requirement surfaces as a detailed error pointing at the actual missing dependency. The macro writes these checks for you, as a compile-time-only test: a successful build *is* the passing assertion.

## Syntax

The macro takes one or more check tables, each a context type followed by a brace-delimited list of the components to check on it. The simplest table lists bare component names:

```rust
check_components! {
    Person {
        GreeterComponent,
    }
}
```

Each entry names a component the macro should confirm `Person` can use. Multiple tables may appear in a single invocation, each beginning with its own context type and optional attributes.

For a component with generic parameters, the parameters to check are given after a colon. A single parameter is written bare; multiple parameters are grouped into a tuple, mirroring how the provider trait groups them in its `IsProviderFor` `Params` position:

```rust
check_components! {
    MyApp {
        AreaOfShapeCalculatorComponent: Rectangle,                 // one parameter
        TransformCalculatorComponent: (Rectangle, f64),            // two parameters, as a tuple
    }
}
```

Array syntax on either side expands to the cartesian product of the bracketed entries, so a set of components can be checked against a set of parameters in one line. A bracketed value checks one component against several parameter sets; a bracketed key checks several components against one parameter set; bracketing both checks every combination:

```rust
check_components! {
    MyApp {
        [AreaCalculatorComponent, RotatorComponent]: [Rectangle, Circle],
    }
}
```

The check trait's name can be set with `#[check_trait(Name)]` on the table. The macro otherwise derives a name of the form `__Check{Context}` (for example `__CheckPerson`), so the override is needed when two `check_components!` tables in the same module would otherwise collide. A leading `<...>` generic list and a trailing `where` clause may also be attached to a table to introduce and constrain generics used by the checked parameters.

A `#[check_providers(...)]` attribute changes what is checked: instead of verifying the context, it verifies that each listed provider is a provider for the context. This is the form to reach for when a higher-order provider needs each layer checked separately, since each provider in the list is asserted independently.

## Syntax Grammar

The input to `check_components!` is one or more check tables, each an optional attribute set and generic list, a context type, an optional `where` clause, and a brace-delimited list of check entries:

```ebnf
CheckComponents -> CheckTable+

CheckTable      -> TableAttr* Generics? ContextType WhereClause? `{` CheckEntries `}`

TableAttr       -> `#` `[` `check_trait` `(` IDENTIFIER `)` `]`
                 | `#` `[` `check_providers` `(` Type ( `,` Type )* `,`? `)` `]`

ContextType     -> Type

CheckEntries    -> ( CheckEntry ( `,` CheckEntry )* `,`? )?

CheckEntry      -> CheckKey ( `:` CheckValue )?

CheckKey        -> Type
                 | `[` Type ( `,` Type )* `,`? `]`

CheckValue      -> CheckParam
                 | `[` CheckParam ( `,` CheckParam )* `,`? `]`

CheckParam      -> Generics? Type
```

A single invocation may carry several `CheckTable`s, each with its own context type. The optional `#[check_trait(...)]` overrides the derived `__Check{Context}` trait name, and `#[check_providers(...)]` switches the check to verify the listed providers instead of the context. The `where` clause and a leading `Generics` list introduce and constrain generics used by the checked parameters. A `CheckEntry`'s value is omitted for a component with no generic parameters; when present, a bracketed `CheckKey` or `CheckValue` expands to the cartesian product, so a set of components is checked against a set of parameters. `WhereClause`, `Generics`, and `Type` are Rust grammar productions.

## Expansion

A check table expands to one marker trait plus one impl per checked entry. The marker trait is an alias whose supertrait is the check being asserted; each impl is an empty body that compiles only if that supertrait holds for the entry. Starting from:

```rust
check_components! {
    Person {
        GreeterComponent,
    }
}
```

the macro emits a check trait carrying `CanUseComponent` as its supertrait, followed by an impl of that trait for `Person` at the listed component and a unit params tuple:

```rust
trait __CheckPerson<__Component__, __Params__: ?Sized>:
    CanUseComponent<__Component__, __Params__>
{}

impl __CheckPerson<GreeterComponent, ()> for Person {}
```

The impl compiles only if `Person: CanUseComponent<GreeterComponent, ()>`, which in turn requires that `Person` delegates `GreeterComponent` and that its delegate satisfies `IsProviderFor<GreeterComponent, Person, ()>`. If the provider's dependencies are unmet — say it needs a `name` field the context lacks — the compiler reports the unsatisfied `HasField` bound rather than a bare "provider trait not implemented", which is the whole point of the indirection through `CanUseComponent`. The check trait name follows the `__Check{Context}` pattern, and the generic parameters are literally `__Component__` and `__Params__` in the emitted code.

Generic parameters appear in the `__Params__` slot of each impl. A single parameter is placed there directly, and multiple parameters as a tuple:

```rust
// AreaOfShapeCalculatorComponent: Rectangle
impl __CheckMyApp<AreaOfShapeCalculatorComponent, Rectangle> for MyApp {}

// TransformCalculatorComponent: (Rectangle, f64)
impl __CheckMyApp<TransformCalculatorComponent, (Rectangle, f64)> for MyApp {}
```

Array syntax expands to the cartesian product before emitting impls, so `[AreaCalculatorComponent, RotatorComponent]: [Rectangle, Circle]` produces four impls — each component paired with each parameter — and the table-level generics and `where` clause are merged into every impl. A `<'a, I> Context where I: Clone { FooComponent: &'a I }` table, for instance, expands to `impl<'a, I> __CheckContext<FooComponent, &'a I> for Context where I: Clone {}`.

The `#[check_providers(...)]` form changes both the supertrait and the implementing type. The check trait gains `IsProviderFor` as its supertrait instead of `CanUseComponent`, and the impls are written for each listed provider rather than for the context:

```rust
check_components! {
    #[check_trait(CheckScaledRectangleProviders)]
    #[check_providers(
        RectangleAreaCalculator,
        ScaledAreaCalculator<RectangleAreaCalculator>,
    )]
    ScaledRectangle {
        AreaCalculatorComponent,
    }
}
```

expands to:

```rust
trait CheckScaledRectangleProviders<__Component__, __Params__: ?Sized>:
    IsProviderFor<__Component__, ScaledRectangle, __Params__>
{}

impl CheckScaledRectangleProviders<AreaCalculatorComponent, ()> for RectangleAreaCalculator {}
impl CheckScaledRectangleProviders<AreaCalculatorComponent, ()>
    for ScaledAreaCalculator<RectangleAreaCalculator> {}
```

Because each provider is checked on its own impl, a missing dependency affecting only the outer `ScaledAreaCalculator<RectangleAreaCalculator>` produces an error on that line alone, while a dependency affecting the inner `RectangleAreaCalculator` errors on both — letting the failures localize the root cause.

## Examples

A check that catches a wiring mistake makes the value concrete. Given a greeter that depends on a `name` field, wired onto a context that has the wrong field name:

```rust
use cgp::prelude::*;

#[cgp_auto_getter]
pub trait HasName {
    fn name(&self) -> &str;
}

#[cgp_component(Greeter)]
pub trait CanGreet {
    fn greet(&self);
}

#[cgp_impl(new GreetHello)]
impl Greeter
where
    Self: HasName,
{
    fn greet(&self) {
        println!("Hello, {}!", self.name());
    }
}

#[derive(HasField)]
pub struct Person {
    pub first_name: String, // mismatch: GreetHello needs `name`
}

delegate_components! {
    Person {
        GreeterComponent: GreetHello,
    }
}

check_components! {
    Person {
        GreeterComponent,
    }
}
```

The `delegate_components!` block compiles on its own because wiring is lazy, but `check_components!` forces the assertion `Person: CanUseComponent<GreeterComponent, ()>`, which fails to compile and reports that `Person` is missing the `name` field — pinpointing the mismatch at the wiring site instead of at some distant call to `person.greet()`.

A generic-parameter check supplies the parameters explicitly:

```rust
#[cgp_component(AreaOfShapeCalculator)]
pub trait CanCalculateAreaOfShape<Shape> {
    fn area(&self, shape: &Shape) -> f64;
}

check_components! {
    MyApp {
        AreaOfShapeCalculatorComponent: [Rectangle, Circle],
    }
}
```

This verifies `MyApp: CanCalculateAreaOfShape<Rectangle>` and `MyApp: CanCalculateAreaOfShape<Circle>` in one table.

## Related constructs

`check_components!` verifies the wiring produced by [`delegate_components!`](delegate_components.md) for components defined with [`#[cgp_component]`](cgp_component.md). Its checks are built on [`CanUseComponent`](../traits/can_use_component.md), which itself relies on [`DelegateComponent`](../traits/delegate_component.md) and [`IsProviderFor`](../traits/is_provider_for.md); the `#[check_providers(...)]` form checks `IsProviderFor` on named providers directly, which is especially useful for higher-order providers wired through [`use_delegate.md`](../providers/use_delegate.md). When you want wiring and checking in a single step — the recommended approach for a main context — use [`delegate_and_check_components!`](delegate_and_check_components.md) instead; note its default check trait name (`__CanUse{Context}`) differs from this macro's (`__Check{Context}`) so the two can coexist in one module.

## Source

The macro entry point is `check_components` in [crates/macros/cgp-macro-lib/src/check_components.rs](../../../crates/macros/cgp-macro-lib/src/check_components.rs), which parses `CheckComponentsTables` and emits their items. The logic lives in [crates/macros/cgp-macro-core/src/types/check_components/](../../../crates/macros/cgp-macro-core/src/types/check_components/): table parsing, the `#[check_trait]`/`#[check_providers]` attributes, the `__Check{Context}` name derivation, and the choice between `CanUseComponent` and `IsProviderFor` supertraits are all in `table.rs`; key and value parsing (including array syntax) in `key.rs` and `value.rs`; and the cartesian-product expansion of entries in `entry.rs`. Expansion snapshots are in [crates/tests/cgp-tests/src/tests/check_components.rs](../../../crates/tests/cgp-tests/src/tests/check_components.rs).
