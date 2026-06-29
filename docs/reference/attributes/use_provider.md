# `#[use_provider]`

`#[use_provider]` improves the ergonomics of higher-order providers by hiding the extra `Self` generic that a provider trait inserts at its first position, in two complementary forms: an outer form that writes the inner provider's bound for you, and an inner form that dispatches a method call to a named provider.

## Purpose

`#[use_provider]` exists to keep higher-order providers looking like ordinary providers. A higher-order provider is one that takes another provider as a generic parameter and delegates part of its work to it — for example a `ScaledArea` provider that multiplies whatever an `InnerCalculator` computes. The catch is that provider traits move the original `Self` into an explicit leading `Context` parameter, so the inner provider must be bound as `InnerCalculator: AreaCalculator<Self>`, not `InnerCalculator: AreaCalculator`. That stray `<Self>` is exactly the detail a reader does not expect, because the consumer trait it mirrors has no such parameter.

The outer form lets the author write the bound without the `<Self>`. Annotating an impl with `#[use_provider(InnerCalculator: AreaCalculator)]` adds the `Self` argument back automatically and inserts the completed bound into the impl's `where` clause, so the source reads `InnerCalculator: AreaCalculator` while the generated code carries `InnerCalculator: AreaCalculator<Self>`. This preserves the illusion that a provider trait looks the same as the consumer trait it came from.

The inner form, as described by the `/cgp` skill, addresses the matching problem on the call side: dispatching a method to the inner provider rather than to the context. Because the inner provider is invoked as an associated function — `InnerCalculator::area(self)` — rather than as a method, the call site stops reading like a method chain. The inner `#[use_provider]` is meant to let the author keep the `receiver.method(args)` form and have the macro rewrite it into `Provider::method(receiver, args)`. The `/cgp` skill recommends `#[use_provider]` whenever higher-order providers are written, because without it the extra `Self` parameter and the associated-function calls confuse readers who expect provider code to mirror consumer code.

## Syntax

The outer form is an attribute on a `#[cgp_impl]` or `#[cgp_fn]` definition and takes a provider type followed by a colon and the provider trait bounds it should satisfy. The shape is a provider, a colon, and one or more trait bounds joined by `+`:

```rust
#[use_provider(InnerCalculator: AreaCalculator)]
```

`InnerCalculator` is the provider type — usually a generic parameter of the impl — and `AreaCalculator` is the provider trait whose `Self`/context argument the macro fills in. The trait may carry its own further generic arguments after the context slot, and several `#[use_provider]` attributes may be stacked to bind more than one inner provider.

The inner form, per the `/cgp` skill, is an attribute applied to a method-call expression inside a function body and takes only the provider type. Written as `#[use_provider(InnerCalculator)] self.area()`, it names the provider that the immediately following method call should be dispatched to. Unlike the outer form it has no colon and no trait bounds — it carries just the provider identifier.

## Expansion

The outer form rewrites nothing in the body; it only completes and inserts the `where`-clause bound. Take this higher-order provider, where `ScaledArea` scales the area produced by an inner calculator:

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

The inner form, as the `/cgp` skill describes it, rewrites the annotated call site rather than the `where` clause. It turns the method-call form back into an associated-function call against the named provider, so the following two expressions are equivalent:

```rust
#[use_provider(InnerCalculator)] self.area()
```

```rust
InnerCalculator::area(self)
```

More generally `#[use_provider(Provider)] receiver.method(args)` becomes `Provider::method(receiver, args)`, dispatching the call to the specified provider instead of routing it through the context's own wiring. This lets the author keep reading `self.area()` while the call is statically directed to `InnerCalculator`. In current code the inner provider is invoked directly in the associated-function form (`InnerCalculator::area(self)`), as the examples above show.

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

`#[use_provider]` is written almost exclusively inside [`#[cgp_impl]`](../macros/cgp_impl.md) and [`#[cgp_fn]`](../macros/cgp_fn.md) implementations of components defined with [`#[cgp_component]`](../macros/cgp_component.md), and is the idiomatic tool for the higher-order provider pattern those macros support. It is the provider-bound counterpart to [`#[uses]`](uses.md), which imports consumer-trait dependencies on `Self`; where `#[uses]` adds a bound on the context, `#[use_provider]` adds a bound on a separate provider type and fills in that type's context argument. For dispatching to different providers based on a generic type rather than naming one statically, see [`UseDelegate`](../types/use_delegate.md) and [`#[derive_delegate]`](derive_delegate.md).

## Source

The outer form is parsed by `UseProviderAttribute` in [crates/macros/cgp-macro-core/src/types/attributes/use_provider/attribute.rs](../../../crates/macros/cgp-macro-core/src/types/attributes/use_provider/attribute.rs); its `to_type_param_bounds` inserts the context type at index 0 of the trait's generic arguments, and `to_provider_bounds` builds the `where` predicate. The bounds are appended to the impl by `add_type_param_bounds` in `attributes.rs`. The attribute is collected for `#[cgp_impl]` in `types/attributes/cgp_impl_attributes.rs` and for `#[cgp_fn]` in `types/attributes/function.rs`, and applied in `types/cgp_impl/item.rs` and `types/cgp_fn/preprocessed.rs`. The expansion snapshot for the `#[cgp_fn]` outer form is in [crates/tests/cgp-tests/tests/cgp_fn_tests/use_provider.rs](../../../crates/tests/cgp-tests/tests/cgp_fn_tests/use_provider.rs); `#[cgp_impl]` usage is exercised in [crates/tests/cgp-tests/tests/component_tests/cgp_impl/use_provider.rs](../../../crates/tests/cgp-tests/tests/component_tests/cgp_impl/use_provider.rs) and `shape.rs`.
