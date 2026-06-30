# Extensible shapes

This example computes properties of geometric shapes — area, scaling — modeled as the variants of an enum, dispatching each operation to a per-variant implementation without a hand-written `match`. It progresses from a per-type operation that is lifted onto an enum automatically, through mutating and argument-taking operations, to converting between related shape enums and finally wiring the dispatch into a context. It is a template for any use case where one operation has a separate implementation per case of a sum type, and the set of cases should stay open to extension — the *extensible visitor* counterpart to the per-context dispatch in [area calculation](area-calculation.md).

The concepts each step demonstrates are documented in full in the reference; this example only notes which one is in play and links to it:

- shapes as the variants of an enum — [extensible variants](../concepts/extensible-variants.md) via [`#[derive(CgpData)]`](../reference/derives/derive_cgp_data.md)
- dispatching an operation to per-variant implementations — [`#[cgp_auto_dispatch]`](../reference/macros/cgp_auto_dispatch.md)
- per-variant handlers as computers — [`#[cgp_computer]`](../reference/macros/cgp_computer.md) producing [`Computer`](../reference/components/computer.md) providers
- widening and narrowing the variant set — [upcasting and downcasting](../reference/traits/cast.md)
- the dispatch machinery underneath — the [dispatch combinators](../reference/providers/dispatch_combinators.md) and [dispatching](../concepts/dispatching.md)
- wiring dispatch into a context — [`delegate_components!`](../reference/macros/delegate_components.md) with [`UseInputDelegate`](../reference/providers/use_delegate.md) and a [`check_components!`](../reference/macros/check_components.md) assertion

All snippets assume `use cgp::prelude::*;`; the dispatch combinators come from `cgp::extra::dispatch`, `UseInputDelegate` from `cgp::extra::handler`, and the cast helpers from `cgp::core::field::impls`. Each shape is its own payload struct:

```rust
#[derive(Debug, PartialEq)]
pub struct Circle {
    pub radius: f64,
}

#[derive(Debug, PartialEq)]
pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

#[derive(Debug, PartialEq)]
pub struct Triangle {
    pub base: f64,
    pub height: f64,
}
```

## A per-type operation, lifted onto enums

The smallest unit is an ordinary trait with one implementation per shape type. Marking the trait with [`#[cgp_auto_dispatch]`](../reference/macros/cgp_auto_dispatch.md) is what later lets any *enum* of these shapes gain the operation for free, dispatched to the matching variant — but the per-type impls themselves are plain Rust:

```rust
#[cgp_auto_dispatch]
pub trait HasArea {
    fn area(self) -> f64;
}

impl HasArea for Circle {
    fn area(self) -> f64 {
        core::f64::consts::PI * self.radius * self.radius
    }
}

impl HasArea for Rectangle {
    fn area(self) -> f64 {
        self.width * self.height
    }
}

impl HasArea for Triangle {
    fn area(self) -> f64 {
        self.base * self.height / 2.0
    }
}
```

`#[cgp_auto_dispatch]` leaves the trait and these impls untouched and generates, behind the scenes, the per-variant handler and the enum-level implementation that runs it. The operation reads like a normal method; the dispatch is what the macro supplies.

## An enum of shapes

A concrete shape is one enum over the payload types, made [extensible](../concepts/extensible-variants.md) with [`#[derive(CgpData)]`](../reference/derives/derive_cgp_data.md) so it can be taken apart by the variant name generically. Each variant holds exactly one payload — the single-field tuple form the derive requires:

```rust
#[derive(Debug, PartialEq, CgpData)]
pub enum Shape {
    Circle(Circle),
    Rectangle(Rectangle),
}
```

Because `HasArea` carries `#[cgp_auto_dispatch]` and `Shape` is extensible, `Shape` now implements `HasArea` too, matching its current variant and delegating to that payload's `area`:

```rust
let shape = Shape::Circle(Circle { radius: 5.0 });
assert_eq!(shape.area(), 25.0 * core::f64::consts::PI);
```

The dispatch is resolved at compile time, and forgetting an `impl HasArea` for one variant's payload is a compile error at the point `Shape::area` is used, not a runtime fallthrough.

## Operations that mutate or take arguments

The same dispatch covers every method shape, not just a by-value reader. A `&mut self` method that also takes an argument — scaling a shape by a factor — dispatches identically; `#[cgp_auto_dispatch]` selects the matcher form that matches the receiver and threads the extra argument through to each variant's implementation:

```rust
#[cgp_auto_dispatch]
pub trait CanScale {
    fn scale(&mut self, factor: f64);
}

impl CanScale for Circle {
    fn scale(&mut self, factor: f64) {
        self.radius *= factor;
    }
}

impl CanScale for Rectangle {
    fn scale(&mut self, factor: f64) {
        self.width *= factor;
        self.height *= factor;
    }
}

impl CanScale for Triangle {
    fn scale(&mut self, factor: f64) {
        self.base *= factor;
        self.height *= factor;
    }
}

let mut shape = Shape::Rectangle(Rectangle { width: 2.0, height: 2.0 });
shape.scale(2.0);
assert_eq!(shape.area(), 16.0);
```

A trait may mix reading and mutating methods, by-value and by-reference receivers, extra arguments, and `async` methods, all dispatched the same way; the [`#[cgp_auto_dispatch]`](../reference/macros/cgp_auto_dispatch.md) reference covers which matcher each shape selects.

## Widening and narrowing the shape set

Two enums that share variants interconvert with no hand-written conversion, through the [casting traits](../reference/traits/cast.md). A wider `ShapePlus` adds a `Triangle` case alongside the two `Shape` already has:

```rust
#[derive(Debug, PartialEq, CgpData)]
pub enum ShapePlus {
    Triangle(Triangle),
    Rectangle(Rectangle),
    Circle(Circle),
}
```

Upcasting a `Shape` into `ShapePlus` always succeeds, since every `Shape` variant has a home in the wider enum; downcasting back may fail, returning the value untouched when its variant has no home in the narrower target:

```rust
let shape = Shape::Circle(Circle { radius: 5.0 });
let wider: ShapePlus = shape.upcast(PhantomData::<ShapePlus>);
assert_eq!(wider, ShapePlus::Circle(Circle { radius: 5.0 }));

let narrowed = ShapePlus::Circle(Circle { radius: 5.0 }).downcast(PhantomData::<Shape>);
assert_eq!(narrowed.ok(), Some(Shape::Circle(Circle { radius: 5.0 })));
```

A `ShapePlus::Triangle` cannot narrow to `Shape`, so its `downcast` returns `Err(remainder)`. The remainder is the partial enum with the already-tried variants ruled out, and it can be narrowed again against another candidate with `downcast_fields` — narrowing a leftover `Triangle` into a `TriangleOnly` enum, which here cannot fail:

```rust
#[derive(Debug, PartialEq, CgpData)]
pub enum TriangleOnly {
    Triangle(Triangle),
}
```

## Under the hood: the dispatch combinators

What `#[cgp_auto_dispatch]` writes for `Shape::area` is a [dispatch combinator](../reference/providers/dispatch_combinators.md), and the same dispatch can be assembled by hand. The macro turns each method into a per-variant [`Computer`](../reference/components/computer.md) provider with [`#[cgp_computer]`](../reference/macros/cgp_computer.md) — `area` yields a provider named `ComputeArea` — and then runs a *matcher* over the enum's variants:

```rust
use cgp::extra::dispatch::{ExtractFieldAndHandle, HandleFieldValue, MatchWithHandlers};

let circle = Shape::Circle(Circle { radius: 5.0 });

let area = MatchWithHandlers::<Product![
    ExtractFieldAndHandle<Symbol!("Circle"), HandleFieldValue<ComputeArea>>,
    ExtractFieldAndHandle<Symbol!("Rectangle"), HandleFieldValue<ComputeArea>>,
]>::compute(&(), PhantomData::<()>, circle);
```

Each `ExtractFieldAndHandle` tries one variant by its [`Symbol!`](../reference/macros/symbol.md) name and, on a match, hands the payload to `ComputeArea`; the list runs first-match-wins, and once every variant has been tried the remainder is uninhabited, so the match is provably exhaustive with no wildcard arm. The `ComputeArea` provider is what [`#[cgp_computer]`](../reference/macros/cgp_computer.md) produces from a function that forwards to the trait method — equivalent to `fn compute_area<S: HasArea>(s: S) -> f64 { s.area() }` — and it is exactly the per-variant handler `#[cgp_auto_dispatch]` generates.

Spelling out one adapter per variant is mechanical, so `MatchWithValueHandlers` is the shorthand that derives the same list from the enum's own variant list; naming the per-variant provider once dispatches the whole enum:

```rust
let circle = Shape::Circle(Circle { radius: 5.0 });
let _area = MatchWithValueHandlers::<ComputeArea>::compute(&(), PhantomData::<()>, circle);
```

The matcher is a `Computer` provider invoked with a unit context `&()`, because the per-variant logic depends only on the payload — and this `MatchWithValueHandlers::<ComputeArea>` call is the very body `#[cgp_auto_dispatch]` writes into the enum's generated `area` method.

## Wiring dispatch into a context

Rather than implement the operation directly on the enum, the dispatch can be wired into a context's [`Computer`](../reference/components/computer.md) component, keyed on the *input* type with [`UseInputDelegate`](../reference/providers/use_delegate.md). A bare payload type routes straight to its computer, while an enum routes to the matcher, which dispatches each extracted payload back through the context's own wiring:

```rust
use cgp::extra::handler::UseInputDelegate;

pub struct App;

delegate_components! {
    App {
        ComputerComponent: UseInputDelegate<new AreaComputers {
            [
                Circle,
                Rectangle,
                Triangle,
            ]:
                ComputeArea,
            [
                Shape,
                ShapePlus,
            ]:
                MatchWithValueHandlers,
        }>,
    }
}
```

A `Circle` input resolves to `ComputeArea` directly. A `Shape` or `ShapePlus` input resolves to `MatchWithValueHandlers` — written here with no provider argument, so it defaults to [`UseContext`](../reference/providers/use_context.md) and dispatches each extracted payload *back through* `App`'s own `ComputerComponent` rather than to a fixed handler. That is why the payload types share the table: the matcher routes a `Circle` payload to whatever `App` wires for `Circle`, which here is `ComputeArea`.

Routing through the context is what makes individual variants overridable. Swapping the handler for one shape — wiring `Circle` to an optimized provider, say — is a one-line change to the table that leaves the matcher and every other variant untouched, precisely because the matcher never names `ComputeArea` itself. Pinning the matcher to a concrete provider as `MatchWithValueHandlers<ComputeArea>` is the other option, used when the dispatch should bypass the context entirely, as in the unit-context calls earlier.

Because shape handlers are leaves — `ComputeArea` never calls back into the dispatcher — the enums can wire `MatchWithValueHandlers` directly. A *recursive* visitor cannot: the [expression interpreter](expression-interpreter.md) routes its enum through a thin wrapper provider instead, to break the trait-resolution cycle that its self-recursive handlers would otherwise create. Because CGP wiring is [checked lazily](../concepts/check-traits.md), a [`check_components!`](../reference/macros/check_components.md) block asserts at compile time that both enums are fully dispatchable, listing each as a `(Code, Input)` pair for the generic `Computer` component:

```rust
check_components! {
    App {
        ComputerComponent: [
            ((), Shape),
            ((), ShapePlus),
        ],
    }
}
```
