# `#[use_provider]`

`#[use_provider]` improves the ergonomics of higher-order providers by writing the inner provider's bound for you, hiding the extra `Self` generic that a provider trait inserts at its first position.

## Purpose

`#[use_provider]` exists to keep higher-order providers looking like ordinary providers. A higher-order provider is one that takes another provider as a generic parameter and delegates part of its work to it — for example a `ScaledArea` provider that multiplies whatever an `InnerCalculator` computes. The catch is that provider traits move the original `Self` into an explicit leading `Context` parameter, so the inner provider must be bound as `InnerCalculator: AreaCalculator<Self>`, not `InnerCalculator: AreaCalculator`. That stray `<Self>` is exactly the detail a reader does not expect, because the consumer trait it mirrors has no such parameter.

`#[use_provider]` lets the author write the bound without the `<Self>`. Annotating an impl with `#[use_provider(InnerCalculator: AreaCalculator)]` adds the `Self` argument back automatically and inserts the completed bound into the impl's `where` clause, so the source reads `InnerCalculator: AreaCalculator` while the generated code carries `InnerCalculator: AreaCalculator<Self>`. This preserves the illusion that a provider trait looks the same as the consumer trait it came from, which is why it is the idiomatic way to declare the inner dependency of a higher-order provider.

The body of such a provider still calls the inner provider as an associated function — `InnerCalculator::area(self)` rather than `self.area()` — because the inner provider is named explicitly rather than routed through the context's own wiring. `#[use_provider]` removes the surprise from the bound; the associated-function call at the use site is written out directly.

## Syntax

`#[use_provider]` is an attribute on a `#[cgp_impl]` or `#[cgp_fn]` definition, taking a provider type followed by a colon and the provider trait bounds it should satisfy. The shape is a provider, a colon, and one or more trait bounds joined by `+`:

```rust
#[use_provider(InnerCalculator: AreaCalculator)]
```

`InnerCalculator` is the provider type — usually a generic parameter of the impl — and `AreaCalculator` is the provider trait whose `Self`/context argument the macro fills in. The trait may carry its own further generic arguments after the context slot, and these are preserved in order behind the inserted `Self`. Several `#[use_provider]` bounds may be supplied — separated by commas inside one attribute or split across stacked attributes — to bind more than one inner provider.

## Expansion

`#[use_provider]` rewrites nothing in the body; it only completes and inserts the `where`-clause bound. Take this higher-order provider, where `ScaledArea` scales the area produced by an inner calculator:

```rust
#[cgp_component(AreaCalculator)]
pub trait CanCalculateArea {
    fn area(&self) -> f64;
}

#[cgp_impl(new ScaledArea<InnerCalculator>)]
#[use_provider(InnerCalculator: AreaCalculator)]
impl<InnerCalculator> AreaCalculator {
    fn area(&self, #[implicit] scale_factor: f64) -> f64 {
        InnerCalculator::area(self) * scale_factor * scale_factor
    }
}
```

The attribute takes the bound `InnerCalculator: AreaCalculator`, inserts the context type as the leading generic argument, and pushes the result onto the impl's `where` clause. After this step the impl is equivalent to writing the `<Self>` argument by hand:

```rust
#[cgp_impl(new ScaledArea<InnerCalculator>)]
impl<InnerCalculator> AreaCalculator
where
    InnerCalculator: AreaCalculator<Self>,
{
    fn area(&self, #[implicit] scale_factor: f64) -> f64 {
        InnerCalculator::area(self) * scale_factor * scale_factor
    }
}
```

The same applies to `#[cgp_fn]`. Here the inner provider is bound and then called as an associated function:

```rust
#[cgp_fn]
#[use_provider(RectangleAreaCalculator: AreaCalculator)]
fn rectangle_area(&self) -> f64 {
    RectangleAreaCalculator::area(self)
}
```

This desugars to the blanket impl with the completed bound; note the `<Self>` the macro supplied:

```rust
trait RectangleArea {
    fn rectangle_area(&self) -> f64;
}

impl<Context> RectangleArea for Context
where
    RectangleAreaCalculator: AreaCalculator<Self>,
{
    fn rectangle_area(&self) -> f64 {
        RectangleAreaCalculator::area(self)
    }
}
```

In both cases the body is left untouched, so it must invoke the inner provider directly as an associated function — `RectangleAreaCalculator::area(self)` — passing `self` as the explicit context argument. `#[use_provider]` supplies only the bound; it does not rewrite the call expression. Calling the inner provider as a method (`self.area()`) would instead route through whatever provider the context itself has wired for `AreaCalculator`, which is a different dispatch and usually not what a higher-order provider wants.

## Examples

A complete higher-order provider shows the outer form pulling its weight. The base component and a concrete provider come first:

```rust
use cgp::prelude::*;

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

The higher-order `ScaledArea` then wraps any inner calculator and scales its result, declaring the inner dependency with `#[use_provider]`:

```rust
#[cgp_impl(new ScaledArea<InnerCalculator>)]
#[use_provider(InnerCalculator: AreaCalculator)]
impl<InnerCalculator> AreaCalculator {
    fn area(&self, #[implicit] scale_factor: f64) -> f64 {
        let base_area = InnerCalculator::area(self);
        base_area * scale_factor * scale_factor
    }
}
```

A context can now wire `AreaCalculatorComponent` to `ScaledArea<RectangleArea>`, and `ScaledArea` will compute the rectangle area through `RectangleArea` and then scale it. The author never wrote `InnerCalculator: AreaCalculator<Self>`; `#[use_provider]` supplied the `<Self>`.

## Related constructs

`#[use_provider]` is written almost exclusively inside [`#[cgp_impl]`](../macros/cgp_impl.md) and [`#[cgp_fn]`](../macros/cgp_fn.md) implementations of components defined with [`#[cgp_component]`](../macros/cgp_component.md), and is the idiomatic tool for the higher-order provider pattern those macros support. It is the provider-bound counterpart to [`#[uses]`](uses.md), which imports consumer-trait dependencies on `Self`; where `#[uses]` adds a bound on the context, `#[use_provider]` adds a bound on a separate provider type and fills in that type's context argument. For dispatching to different providers based on a generic type rather than naming one statically, see [`UseDelegate`](../providers/use_delegate.md) and [`#[derive_delegate]`](derive_delegate.md).

## Known issues

`#[use_provider]` only completes and inserts a `where`-clause bound; there is no call-site form that rewrites a method call into a provider dispatch. The attribute's parser requires the `Provider: Trait` shape — a provider, a colon, and the trait bounds — so a bare `#[use_provider(InnerCalculator)]` applied to an expression is not accepted, and no pass rewrites `receiver.method(args)` into `Provider::method(receiver, args)`. A body that delegates to a named inner provider must therefore spell the associated-function call out itself, as `InnerCalculator::area(self)`.

## Source

- Parsing: the outer form is parsed by `UseProviderAttribute` in [crates/macros/cgp-macro-core/src/types/attributes/use_provider/attribute.rs](../../../crates/macros/cgp-macro-core/src/types/attributes/use_provider/attribute.rs); its `to_type_param_bounds` inserts the context type at index 0 of the trait's generic arguments, and `to_provider_bounds` builds the `where` predicate.
- Bound insertion: the bounds are appended to the impl by `add_type_param_bounds` in `attributes.rs`.
- Collection and application: the attribute is collected for `#[cgp_impl]` in `types/attributes/cgp_impl_attributes.rs` and for `#[cgp_fn]` in `types/attributes/function.rs`, and applied in `types/cgp_impl/item.rs` and `types/cgp_fn/preprocessed.rs`.
- Implementation document (the internal AST type, the bound completion, and the index of tests and snapshots): [implementation/asts/attributes.md](../../implementation/asts/attributes.md).
