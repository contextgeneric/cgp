# Area calculation

This example computes the area of several shapes, progressing from a single field-driven function to a unified, wireable area-calculation component whose implementations compose through higher-order providers. It is a template for any use case where one operation has several interchangeable implementations chosen per context — the shapes here stand in for whatever set of variants an application needs to treat uniformly.

The concepts each step demonstrates are documented in full in the reference; this example only notes which one is in play and links to it:

- context-generic functions — [`#[cgp_fn]`](../reference/macros/cgp_fn.md) with [implicit arguments](../concepts/implicit-arguments.md)
- importing capabilities — [`#[uses]`](../reference/attributes/uses.md)
- field access on contexts — [`#[derive(HasField)]`](../reference/derives/derive_has_field.md)
- components and named providers — [`#[cgp_component]`](../reference/macros/cgp_component.md), [`#[cgp_impl]`](../reference/macros/cgp_impl.md), and the [consumer/provider trait duality](../concepts/consumer-and-provider-traits.md)
- wiring a context to providers — [`delegate_components!`](../reference/macros/delegate_components.md)
- composing providers — [higher-order providers](../concepts/higher-order-providers.md) with [`#[use_provider]`](../reference/attributes/use_provider.md)

All snippets assume `use cgp::prelude::*;`.

## A field-driven function

The smallest unit of reuse is a function that reads its inputs from the context's fields instead of from explicit arguments. Marking `rectangle_area` with [`#[cgp_fn]`](../reference/macros/cgp_fn.md) and tagging its parameters [`#[implicit]`](../reference/attributes/implicit.md) turns it into a method available on any context that carries a `width` and a `height` field:

```rust
#[cgp_fn]
pub fn rectangle_area(
    &self,
    #[implicit] width: f64,
    #[implicit] height: f64,
) -> f64 {
    width * height
}
```

A context becomes eligible by deriving [`HasField`](../reference/derives/derive_has_field.md), which generates the field accessors the implicit arguments resolve through. No per-context implementation is needed:

```rust
#[derive(HasField)]
pub struct PlainRectangle {
    pub width: f64,
    pub height: f64,
}

let area = PlainRectangle { width: 2.0, height: 3.0 }.rectangle_area();
assert_eq!(area, 6.0);
```

## Building on another function

A context-generic function can call another by importing it with [`#[uses]`](../reference/attributes/uses.md), which adds the imported capability as a hidden bound on the context rather than a visible parameter. Here `scaled_rectangle_area` reuses `rectangle_area` while contributing only its own `scale_factor` field:

```rust
#[cgp_fn]
#[uses(RectangleArea)]
pub fn scaled_rectangle_area(
    &self,
    #[implicit] scale_factor: f64,
) -> f64 {
    self.rectangle_area() * scale_factor * scale_factor
}
```

The imported name `RectangleArea` is the trait `#[cgp_fn]` derives from the `rectangle_area` function — a function `foo` generates a trait `Foo`. A context that adds the extra field can use both functions, while the original `PlainRectangle` keeps working with `rectangle_area` alone:

```rust
#[derive(HasField)]
pub struct ScaledRectangle {
    pub width: f64,
    pub height: f64,
    pub scale_factor: f64,
}

let r = ScaledRectangle { width: 3.0, height: 4.0, scale_factor: 2.0 };
assert_eq!(r.rectangle_area(), 12.0);
assert_eq!(r.scaled_rectangle_area(), 48.0);
```

## A unified interface across shapes

A single `#[cgp_fn]` defines exactly one implementation, so it cannot serve as a common interface that different shapes implement differently. For that, define a [component](../concepts/consumer-and-provider-traits.md) with [`#[cgp_component]`](../reference/macros/cgp_component.md). The annotated `CanCalculateArea` trait is the *consumer trait* callers use; the `AreaCalculator` argument names the generated *provider trait* that implementations target:

```rust
#[cgp_component(AreaCalculator)]
pub trait CanCalculateArea {
    fn area(&self) -> f64;
}
```

Each implementation is a *named provider* written with [`#[cgp_impl]`](../reference/macros/cgp_impl.md). Unlike a blanket `impl`, named providers may overlap freely — a rectangle calculator and a circle calculator can both exist even though a context could in principle have the fields for either. Implicit arguments work here just as in `#[cgp_fn]`, so the field-reading logic can be inlined directly:

```rust
#[cgp_impl(new RectangleAreaCalculator)]
impl AreaCalculator {
    fn area(&self, #[implicit] width: f64, #[implicit] height: f64) -> f64 {
        width * height
    }
}

#[cgp_impl(new CircleAreaCalculator)]
impl AreaCalculator {
    fn area(&self, #[implicit] radius: f64) -> f64 {
        core::f64::consts::PI * radius * radius
    }
}
```

## Wiring contexts to providers

Defining a provider does not attach it to any context; a context chooses its provider by wiring with [`delegate_components!`](../reference/macros/delegate_components.md). Each entry maps the component — keyed by its generated `…Component` name — to the provider that should implement it for that context:

```rust
#[derive(HasField)]
pub struct PlainCircle {
    pub radius: f64,
}

delegate_components! {
    PlainRectangle {
        AreaCalculatorComponent: RectangleAreaCalculator,
    }
}

delegate_components! {
    PlainCircle {
        AreaCalculatorComponent: CircleAreaCalculator,
    }
}
```

With the wiring in place, the consumer-trait method is available on each context, dispatched to its chosen provider at compile time with no runtime indirection:

```rust
assert_eq!(PlainRectangle { width: 2.0, height: 3.0 }.area(), 6.0);
assert_eq!(PlainCircle { radius: 4.0 }.area(), 16.0 * core::f64::consts::PI);
```

## Composing providers

Scaling applies to every shape the same way, so writing a separate scaled provider per shape would duplicate the same logic. A [higher-order provider](../concepts/higher-order-providers.md) captures the transformation once and takes the base calculation as a provider parameter — `InnerCalculator`, declared as an impl generic. The [`#[use_provider]`](../reference/attributes/use_provider.md) attribute supplies that inner provider's bound, filling in the leading context argument a provider trait carries, while the body invokes it as an associated function:

```rust
#[cgp_impl(new ScaledAreaCalculator<InnerCalculator>)]
#[use_provider(InnerCalculator: AreaCalculator)]
impl<InnerCalculator> AreaCalculator {
    fn area(&self, #[implicit] scale_factor: f64) -> f64 {
        InnerCalculator::area(self) * scale_factor * scale_factor
    }
}
```

A context now selects its base calculator and its scaling in one wiring entry by nesting the providers. The same `ScaledAreaCalculator` composes over any inner calculator, so a scaled circle reuses exactly the scaling logic a scaled rectangle uses:

```rust
#[derive(HasField)]
pub struct ScaledCircle {
    pub radius: f64,
    pub scale_factor: f64,
}

delegate_components! {
    ScaledRectangle {
        AreaCalculatorComponent: ScaledAreaCalculator<RectangleAreaCalculator>,
    }
}

delegate_components! {
    ScaledCircle {
        AreaCalculatorComponent: ScaledAreaCalculator<CircleAreaCalculator>,
    }
}

let r = ScaledRectangle { width: 3.0, height: 4.0, scale_factor: 2.0 };
assert_eq!(r.area(), 48.0);

let c = ScaledCircle { radius: 3.0, scale_factor: 2.0 };
assert_eq!(c.area(), 36.0 * core::f64::consts::PI);
```

Because providers are plain type-level markers, a long composition can be given a shorter name with an ordinary type alias — `pub type ScaledRectangleAreaCalculator = ScaledAreaCalculator<RectangleAreaCalculator>;` — and used anywhere the expanded form would be.
</content>
