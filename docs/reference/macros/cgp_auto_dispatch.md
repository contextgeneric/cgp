# `#[cgp_auto_dispatch]`

`#[cgp_auto_dispatch]` is an attribute macro that, given a trait implemented separately for each payload type, generates a blanket implementation of that trait for an enum by matching the enum's current variant and delegating to the implementation for that variant's payload.

## Purpose

`#[cgp_auto_dispatch]` solves the common case of "I have a trait with one impl per type, and I want it to work on an enum of those types too." Without it, a programmer would write a `match` arm per variant by hand, or wire up the [dispatch combinators](../providers/dispatch_combinators.md) — a matcher, a per-variant computer, and the field-handler machinery — manually. The macro removes that boilerplate: it inspects the trait's methods and emits both the per-variant handler each method needs and the enum-level blanket impl that runs the matcher, so the only code the programmer writes is the trait, its per-payload impls, and the derive that makes the enum extensible.

The macro is the highest-level entry point to the [dispatching](../../concepts/dispatching.md) pattern. It is meant for the frequent situation where the per-variant behavior is exactly "call the same trait method on the payload," and it generates the same matcher wiring described in [`dispatch_combinators`](../providers/dispatch_combinators.md) so that the generated impl behaves identically to a hand-written one. When the per-variant behavior is more elaborate, or the dispatch needs to be wired into a context's components rather than implemented directly on the enum, the underlying combinators are used directly instead.

## Syntax

`#[cgp_auto_dispatch]` is written above a trait definition and takes no arguments. The trait may have generic parameters and supertraits, and each method may take `self` by value, by shared reference, or by mutable reference, may take additional value or reference arguments, and may be `async`:

```rust
#[cgp_auto_dispatch]
pub trait HasArea {
    fn area(&self) -> f64;
}
```

Two restrictions apply, both enforced at expansion time. A trait method may not have non-lifetime generic parameters, because Rust lacks the quantified trait bounds the generated impl would need; lifetime parameters on a method are allowed. Every trait item must be a method — associated types and constants are rejected. Each method must have a `self` receiver, since the receiver is the enum value being matched.

## Expansion

The macro keeps the original trait unchanged and appends two kinds of generated code: one blanket impl of the trait for a fresh enum type parameter named `__Variants__`, and, for each method, one free function turned into a per-variant computer by [`#[cgp_computer]`](cgp_computer.md). Taking the `HasArea` trait above, the macro emits a per-variant computer for the `area` method:

```rust
#[cgp_computer(ComputeArea)]
fn area<'__a__, __Variants__: HasArea>(__Variants__: &'__a__ __Variants__) -> f64 {
    __Variants__.area()
}
```

The computer's name is `Compute` followed by the method name in PascalCase, so `area` yields `ComputeArea`. Its body simply calls the trait method on the payload, which means the per-variant handler is "invoke `HasArea::area` on whatever payload type this variant holds." The function is bound by `__Variants__: HasArea` so it applies to every payload type that implements the trait, and it borrows the payload by a fresh lifetime `'__a__` to mirror the `&self` receiver.

The macro then emits the enum-level blanket impl, which wires a matcher over `__Variants__`:

```rust
impl<__Variants__> HasArea for __Variants__
where
    MatchWithValueHandlersRef<ComputeArea>:
        for<'__a__> Computer<(), (), &'__a__ __Variants__, Output = f64>,
    __Variants__: HasExtractor,
{
    fn area(&self) -> f64 {
        MatchWithValueHandlersRef::<ComputeArea>::compute(
            &(),
            ::core::marker::PhantomData::<()>,
            self,
        )
    }
}
```

The matcher struct the impl picks depends on the method's receiver and arguments, and every choice is from the value-handler matcher family so that the per-variant computer receives the bare payload. A method whose receiver and argument list determine the selection as follows: a `&self` method with no extra arguments uses `MatchWithValueHandlersRef`, a `&mut self` method with no extra arguments uses `MatchWithValueHandlersMut`, and a by-value `self` method with no extra arguments uses `MatchWithValueHandlers`. When the method takes additional arguments, the matcher switches to the first-argument family — `MatchFirstWithValueHandlersRef`, `MatchFirstWithValueHandlersMut`, or `MatchFirstWithValueHandlers` respectively — and the arguments are bundled into the matcher input as a tuple. The matcher is invoked with a unit context `&()` and unit code `PhantomData::<()>`, since the per-variant logic depends only on the payload, not on any surrounding context.

A method that takes arguments shows the first-argument form. For a `contains(&self, x: f64, y: f64) -> bool` method, the generated impl bundles the receiver and the arguments into the input tuple and selects `MatchFirstWithValueHandlersRef`:

```rust
// where MatchFirstWithValueHandlersRef<ComputeContains>:
//     for<'__a__> Computer<(), (), (&'__a__ __Variants__, (f64, f64)), Output = bool>
fn contains(&self, arg_0: f64, arg_1: f64) -> bool {
    MatchFirstWithValueHandlersRef::<ComputeContains>::compute(
        &(),
        ::core::marker::PhantomData::<()>,
        (self, (arg_0, arg_1)),
    )
}
```

For an `async` method the macro selects the `AsyncComputer` form of the bound and the matcher call instead of `Computer`, appends the method's lifetime handling for any reference arguments or reference return type by introducing the `'__a__` lifetime and a `for<'__a__>` quantifier where needed, and awaits the matcher result. The enum-level impl always carries the `__Variants__: HasExtractor` bound, since matching requires the enum to be extensible.

## Examples

The macro turns a per-type trait into one that also works on an enum of those types, with no hand-written matching. The enum derives `CgpData` so it is extensible, the trait carries `#[cgp_auto_dispatch]`, and each payload type implements the trait on its own:

```rust
use cgp::prelude::*;

#[derive(CgpData)]
pub enum Shape {
    Circle(Circle),
    Rectangle(Rectangle),
}

pub struct Circle { pub radius: f64 }
pub struct Rectangle { pub width: f64, pub height: f64 }

#[cgp_auto_dispatch]
pub trait HasArea {
    fn area(&self) -> f64;
}

impl HasArea for Circle {
    fn area(&self) -> f64 { core::f64::consts::PI * self.radius * self.radius }
}

impl HasArea for Rectangle {
    fn area(&self) -> f64 { self.width * self.height }
}

// HasArea is now also implemented for Shape, dispatching to the variant's impl:
let shape = Shape::Rectangle(Rectangle { width: 2.0, height: 2.0 });
assert_eq!(shape.area(), 4.0);
```

Because the generated impl is bound by `MatchWithValueHandlersRef<ComputeArea>: …`, it requires every variant's payload to implement `HasArea`; forgetting an impl for one variant is a compile error at the point the enum's `area` is used. A `&mut self` method dispatches the same way through the `Mut` matcher, so a single `#[cgp_auto_dispatch]` trait can mix reading and mutating methods:

```rust
#[cgp_auto_dispatch]
pub trait CanScale {
    fn scale(&mut self, factor: f64);
}

impl CanScale for Circle {
    fn scale(&mut self, factor: f64) { self.radius *= factor; }
}

impl CanScale for Rectangle {
    fn scale(&mut self, factor: f64) { self.width *= factor; self.height *= factor; }
}

let mut shape = Shape::Rectangle(Rectangle { width: 2.0, height: 2.0 });
shape.scale(2.0);   // dispatches to Rectangle::scale through MatchFirstWithValueHandlersMut
```

## Known issues

The macro rejects trait methods with non-lifetime generic parameters, so a dispatch trait cannot have a generic method even though an ordinary trait can. This is a deliberate limitation rather than an oversight: the generated blanket impl would need a quantified trait bound over the method's type parameter to guarantee every variant's payload satisfies the bound for all instantiations, and Rust has no such bound. A method that needs to be generic must be handled with the dispatch combinators directly instead of through this macro.

## Related constructs

`#[cgp_auto_dispatch]` is the automated front end to the [dispatching](../../concepts/dispatching.md) pattern, and the providers it wires together are documented in [`dispatch_combinators`](../providers/dispatch_combinators.md) — specifically the value-handler matcher family (`MatchWithValueHandlers` and its `Ref`/`Mut` and `First` variants). It emits each per-variant handler with [`#[cgp_computer]`](cgp_computer.md), producing a [`Computer`](../components/computer.md) provider. The enum it dispatches over must be made extensible with a `CgpData`-style derive that supplies [`HasExtractor`](../traits/extract_field.md) and [`HasFields`](../traits/has_fields.md). For per-variant behavior that is not a direct method call, or for dispatch wired into a context's components rather than implemented on the enum, the combinators in [`dispatch_combinators`](../providers/dispatch_combinators.md) are used directly. The [extensible shapes](../../examples/extensible-shapes.md) example develops this macro end to end — dispatching an `area` reader, a mutating `scale`, and argument-taking methods over an enum of shapes — and shows how the generated wiring relates to using the combinators directly.

## Source

- Entry point: `cgp_auto_dispatch` in [crates/macros/cgp-extra-macro-lib/src/entrypoints/cgp_auto_dispatch.rs](../../../crates/macros/cgp-extra-macro-lib/src/entrypoints/cgp_auto_dispatch.rs), forwarded from the proc-macro shim in [crates/macros/cgp-extra-macro/src/lib.rs](../../../crates/macros/cgp-extra-macro/src/lib.rs) and re-exported through [crates/main/cgp-extra/src/prelude.rs](../../../crates/main/cgp-extra/src/prelude.rs).
- Matchers it generates: [crates/extra/cgp-dispatch/src/providers/matchers/](../../../crates/extra/cgp-dispatch/src/providers/matchers/).
- Internal walkthrough (the blanket-impl and per-variant-computer helpers, the matcher selection, the lifetime elaboration, and the index of behavioral tests): [implementation/entrypoints/cgp_auto_dispatch.md](../../implementation/entrypoints/cgp_auto_dispatch.md).
