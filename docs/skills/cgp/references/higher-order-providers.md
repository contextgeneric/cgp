# Higher-order providers

How one provider takes another provider as a generic parameter and builds on whatever that inner provider computes — the type-level counterpart to passing a function to a function.

A higher-order provider is a [provider](components.md) parameterized by another provider, so part of its behavior is supplied by an inner provider it delegates to rather than fixed in its own code. The motivating shape is a wrapper that transforms the output of an existing implementation: a `ScaledArea<InnerCalculator>` that multiplies whatever area an inner calculator produces, an `IterSumArea<InnerCalculator>` that sums the areas an inner calculator returns over a collection, a `GloballyScaledArea` that applies a context-wide factor on top of a base calculation. In each case the outer provider knows the *transformation* but not the *base case* — the base case is a parameter. Because a provider carries no runtime value (it is a zero-sized marker naming an implementation), nesting providers costs nothing: the composition happens entirely in types. Assume `use cgp::prelude::*;` throughout; the CGP version is v0.7.0.

## The shape, and the stray `<Self>`

A higher-order provider is written like any other provider with `#[cgp_impl]`, with the inner provider declared as a generic parameter in the provider's `Self` position and bound to a *provider trait* in the `where` clause. The detail that makes it look stranger than it is the extra `<Self>` on that inner bound. Recall that a [provider trait](components.md) moves the consumer trait's `Self` into an explicit leading `Context` parameter — `AreaCalculator<Context>`, not the consumer trait's `CanCalculateArea`. So when the outer provider depends on its inner provider, the bound must name that context argument explicitly:

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

The `<Self>` in `InnerCalculator: AreaCalculator<Self>` is exactly that leading context slot, filled with the context the outer provider is implementing for. Inside `#[cgp_impl]`, `self`/`Self` mean the *context*, so `<Self>` is the context type. The same asymmetry shows at the call site: the inner provider is invoked as the associated function `InnerCalculator::area(self)`, not as a method `self.area()`, because the provider trait's method takes the context as an explicit first argument. A reader who thinks of a provider as "just the consumer trait implemented elsewhere" is caught off guard by both the bound and the call.

## Hiding the friction with `#[use_provider]`

The `#[use_provider]` attribute exists to erase the bound surprise, and is the idiomatic way to write higher-order providers — always reach for it, since it preserves the illusion that a provider trait has the same shape as the consumer trait it came from. Written alongside `#[cgp_impl]`, it takes the inner bound *without* the context argument and fills the `<Self>` back in for you:

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

The author writes `InnerCalculator: AreaCalculator` and the macro inserts the context type at index 0 of the trait's generic arguments, emitting `InnerCalculator: AreaCalculator<Self>` into the `where` clause — the two snippets above are equivalent. The shape it parses is `Provider: Trait`: a provider type, a colon, and one or more provider-trait bounds joined by `+`; the trait may carry further generic arguments after the context slot, preserved in order behind the inserted `Self`. Bind several inner providers by separating bounds with commas in one attribute or by stacking multiple attributes. The same attribute works on `#[cgp_fn]`.

What `#[use_provider]` does *not* do is rewrite the body. It completes the bound only; the inner provider is still invoked as the associated function `InnerCalculator::area(self)`, with `self` passed as the explicit context. There is no call-site form — a bare `#[use_provider(InnerCalculator)]` on an expression is not accepted, and no pass turns `receiver.method(args)` into `Provider::method(receiver, args)`. The body must spell the associated-function call out itself. Calling the inner provider as a method (`self.area()`) would instead route through whatever provider the context has wired for the component, a different dispatch and usually not what a higher-order provider wants.

## `UseContext` as a default inner provider

A higher-order provider can default its inner provider to [`UseContext`](wiring.md) so that, absent an explicit choice, it falls back to whatever the context itself is already wired to. This requires giving the provider an explicit struct definition with a default generic parameter:

```rust
pub struct IterSumArea<InnerCalculator = UseContext>(pub PhantomData<InnerCalculator>);
```

`UseContext` is the provider that implements any provider trait by deferring to the context's own consumer-trait implementation — the dual of the consumer blanket impl. With `IterSumArea<UseContext>` as the default, an unparameterized `IterSumArea` computes each inner element through the context's existing area wiring, so the wrapper drops in without restating the base case. Overriding the parameter — `IterSumArea<RectangleArea>` — instead binds the inner behavior statically, bypassing the context's wiring, which is useful for overriding or short-circuiting what the context would otherwise resolve. This default exists *only* when the provider has an explicit struct carrying the default parameter; a provider defined purely through `#[cgp_impl(new ...)]` has no default and must always have its inner provider named.

## Not every generic provider is higher-order

A provider that merely has generic parameters is not a higher-order provider; it becomes one only when a parameter is constrained to implement a provider trait. Many providers are generic for unrelated reasons. The `UseField`-style getter provider is generic over a field tag:

```rust
#[cgp_impl(new GetName<Tag>)]
impl<Tag> NameGetter
where
    Self: HasField<Tag, Value = String>,
{
    fn name(&self) -> &str {
        self.get_field(PhantomData)
    }
}
```

Here `Tag` is a type-level field name used only as a `HasField` key, with no provider-trait bound — so `GetName<Tag>` is an ordinary parameterized provider, not a higher-order one. The defining trait of a higher-order provider is that a generic parameter appears in a *provider-trait* bound and is invoked to do part of the work, as `InnerCalculator: AreaCalculator<Self>` does. When that bound is absent, `#[use_provider]` has nothing to fill in and none of the machinery applies.

## Generic-parameter CGP traits

A component's trait may itself carry generic parameters, and `#[cgp_component]` appends them after the context slot. Declaring the area component to dispatch on a `Shape`:

```rust
#[cgp_component(AreaCalculator)]
pub trait CanCalculateArea<Shape> {
    fn area(&self, shape: &Shape) -> f64;
}
```

generates the provider trait `AreaCalculator<Context, Shape>` — the original trait's parameters land *after* the leading `Context`, in declaration order. The `IsProviderFor` supertrait that every provider trait carries groups all those extra parameters into a single tuple in its `Params` slot (`()` when there are none), and any lifetime parameters are lifted into the `Life<'a>` type rather than appearing as bare lifetimes. A provider then writes `impl<Context> AreaCalculator<Context, Rectangle> for RectangleArea`, naming the concrete `Shape` it handles.

## Dispatching on a generic parameter with `open`

When the right provider depends on *which* concrete type a generic parameter is — `Rectangle` handled by one provider, `Circle` by another — the choice is made by a second lookup keyed on that parameter rather than on the component marker. The modern way a context writes this per-type dispatch is the `open` statement of `delegate_components!`, which folds the per-value entries directly into the context's own table. Given a `CanCalculateArea<Shape>` consumer trait whose `Shape` parameter selects the area formula, a context opens the component and then assigns a provider per shape:

```rust
delegate_components! {
    MyApp {
        open { AreaCalculatorComponent };

        @AreaCalculatorComponent.Rectangle: RectangleArea,
        @AreaCalculatorComponent.Circle: CircleArea,
    }
}
```

The leading `open { AreaCalculatorComponent };` header opens the component for per-value wiring, and each `@AreaCalculatorComponent.Key: Provider` entry assigns a provider for one value of the dispatch parameter: when `Shape` is `Rectangle`, `MyApp` calculates area through `RectangleArea`, and `Circle` resolves to `CircleArea`. After this wiring, `MyApp` implements `CanCalculateArea<Rectangle>` through `RectangleArea` and `CanCalculateArea<Circle>` through `CircleArea`, with the dispatch parameter selecting between them. Adding a shape is one more entry; the providers stay untouched. The `open` form needs no extra macro on the component — it rides the `RedirectLookup` impl that every `#[cgp_component]` already generates, so dispatching a component per type requires no `#[derive_delegate]` on the trait. Two shorthands keep the entries compact: an array on the final path segment shares one provider across several values (`@AreaCalculatorComponent.[Rectangle, Circle]: SomeProvider`), and generic parameters precede the dispatch value when it needs them (`@AreaCalculatorComponent.<'a, T> &'a T: SomeProvider`). The [wiring](wiring.md) reference is the canonical home of the `open` statement and its grammar.

### Legacy: `derive_delegate` and `UseDelegate` nested tables

An older form generates a `UseDelegate<Components>` provider and wires the per-type entries into a separate table it points at. The `derive_delegate` option asks `#[cgp_component]` to generate that provider:

```rust
#[cgp_component {
    provider: AreaCalculator,
    derive_delegate: UseDelegate<Shape>,
}]
pub trait CanCalculateArea<Shape> {
    fn area(&self, shape: &Shape) -> f64;
}
```

`UseDelegate<Components>` is a zero-sized provider that treats its `Components` type as a type-level table — a `DelegateComponent` table keyed on the named parameter (`Shape`) — and forwards each method to the delegate that table maps the concrete type to. A context wires it through a nested table inside `delegate_components!`, building the outer entry and the inner lookup in one place:

```rust
delegate_components! {
    MyApp {
        AreaCalculatorComponent:
            UseDelegate<new AreaCalculatorComponents {
                Rectangle: RectangleArea,
                Circle: CircleArea,
            }>,
    }
}
```

This reads in two layers: `MyApp` delegates `AreaCalculatorComponent` to `UseDelegate<AreaCalculatorComponents>`, and the inner table — defined inline by `new` — maps `Rectangle` to `RectangleArea` and `Circle` to `CircleArea`. The end effect matches the `open` example above, but the dispatch values live in a named side table reached through `UseDelegate` instead of in `MyApp`'s table directly. This is a legacy dispatch mechanism, retained for compatibility and expected to be deprecated; prefer `open` for new code, but expect to still encounter the `derive_delegate`/`UseDelegate` form when reading existing wiring (see [wiring](wiring.md) for both forms side by side).

A component may dispatch on more than one parameter by listing several dispatchers. `derive_delegate: [ UseDelegate<Code>, UseInputDelegate<Input> ]` generates one impl per dispatcher — the default `UseDelegate` keying on `Code`, plus a user-defined `UseInputDelegate<Components>` (an ordinary struct of the same single-parameter shape) keying on `Input` — so each parameter is looked up through its own table. Only the parameter named in a dispatcher's angle brackets is used as the key; the others flow through. Per-type dispatch and higher-order providers compose either way: an `open` entry or a nested delegation table can map each shape to a different `ScaledArea<...>`.

## Cross-context dependencies through a shared context

The real leverage of generic-parameter dispatch appears when the main target of a capability is itself a generic parameter, and a supertrait constrains the *context* rather than each parameter type. Consider an area component whose result type is an abstract scalar the context supplies:

```rust
#[cgp_component(AreaCalculator)]
#[use_type(HasScalarType::Scalar)]
pub trait CanCalculateAreaOfShape<Shape> {
    fn area_of_shape(&self, shape: &Shape) -> Scalar;
}
```

Because the shared capability — `HasScalarType` and any value-level injection — lives on the common context, the individual shape types (`Rectangle`, `Circle`) need not implement it themselves; they only know their own geometry. The context provides the shared abstract type ([abstract types](abstract-types.md)): one app might wire `Scalar` to `f64`, another to a fixed-point type, and every shape provider produces that scalar. The context also provides value-level injection — a `GloballyScaledArea` provider can read a global scale factor through a getter on the context and multiply every shape's area by it, so the scale is configured once per context rather than per shape. Most importantly, provider binding is *lazy and per-context*: a `BaseApp` and a `ScaledApp` can wire the very same `Shape` to different providers — `BaseApp` opening `AreaOfShapeCalculatorComponent` and mapping `@AreaOfShapeCalculatorComponent.Rectangle: RectangleArea` to a plain `RectangleArea`, `ScaledApp` mapping `@AreaOfShapeCalculatorComponent.Rectangle: GloballyScaledArea<RectangleArea>` instead — so the same generic capability resolves to different behavior in each context without either app's shapes knowing about the other. Each context writes its per-shape choices with the `open` statement detailed in [wiring](wiring.md); the legacy `derive_delegate`/`UseDelegate` nested-table form wires the same dispatch and still appears in existing code. Each layer of such nesting can be verified independently with the `#[check_providers]` form of `check_components!`, which is what makes higher-order wiring debuggable; see [checking](checking.md).

## Related constructs

Higher-order providers build on the consumer/provider split and the `IsProviderFor` propagation described in [components](components.md), and they are connected to contexts through [wiring](wiring.md), where `UseContext`, the `open` statement, and the legacy nested-table `UseDelegate` forms all live. `UseContext` lets the inner provider fall back to the context's own consumer-trait impl; the `open` statement dispatches a component on a generic parameter by folding per-value entries into the context's own table, with the legacy `UseDelegate` provider (generated by the `derive_delegate` option) doing the same through a separate side table; the abstract result types that make cross-context dependencies work are covered in [abstract types](abstract-types.md) and the type-level keys in [type-level primitives](type-level-primitives.md). Each layer of a nested provider is checked independently through [checking](checking.md).

Further reference (online docs): [concepts/higher-order-providers.md](https://github.com/contextgeneric/cgp/blob/main/docs/concepts/higher-order-providers.md), [reference/attributes/use_provider.md](https://github.com/contextgeneric/cgp/blob/main/docs/reference/attributes/use_provider.md), [reference/providers/use_delegate.md](https://github.com/contextgeneric/cgp/blob/main/docs/reference/providers/use_delegate.md), [reference/attributes/derive_delegate.md](https://github.com/contextgeneric/cgp/blob/main/docs/reference/attributes/derive_delegate.md).
