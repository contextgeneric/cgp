# Writing providers

A provider can be written at three levels of sugar over the same machinery, and this guide is about choosing the highest one — writing a provider that reads like an ordinary trait `impl` rather than the inside-out provider-trait form the macros desugar to.

This guide pairs with [declaring a provider's dependencies](declaring-dependencies.md) and [reading its context's fields](reading-context-fields.md), which cover what goes inside the provider once its header is written this way.

## Write providers with `#[cgp_impl]`, not the raw provider forms

Write a provider with [`#[cgp_impl]`](../reference/macros/cgp_impl.md), which keeps `self`, `Self`, and the consumer method signatures, rather than with the lower-level [`#[cgp_provider]`](../reference/macros/cgp_provider.md) or [`#[cgp_new_provider]`](../reference/macros/cgp_new_provider.md), which require the inside-out provider-trait shape. The lower forms move the context into an explicit leading type parameter and force the method to take `context: &Context` instead of `&self`; `#[cgp_impl]` restores the familiar shape and performs that rewrite for you. The legacy form:

```rust
#[cgp_new_provider]
impl<Context> AreaCalculator<Context> for RectangleArea
where
    Context: HasField<Symbol!("width"), Value = f64>,
    Context: HasField<Symbol!("height"), Value = f64>,
{
    fn area(context: &Context) -> f64 {
        *context.get_field(PhantomData::<Symbol!("width")>)
            * *context.get_field(PhantomData::<Symbol!("height")>)
    }
}
```

becomes, with [`#[implicit]`](../reference/attributes/implicit.md) arguments:

```rust
#[cgp_impl(new RectangleArea)]
impl AreaCalculator {
    fn area(&self, #[implicit] width: f64, #[implicit] height: f64) -> f64 {
        width * height
    }
}
```

`#[cgp_impl]` desugars back to `#[cgp_provider]`/`#[cgp_new_provider]`, so the raw forms are still what the reference documents show in their Expansion sections and what you read in generated code. Write the raw form yourself only when you specifically need the inside-out shape — for instance, to state a bound the sugar cannot express, or to implement a provider trait on a concrete context rather than a generic one.

## Omit the context parameter

Inside a `#[cgp_impl]` block, prefer the unqualified `impl AreaCalculator` and let the macro insert the context parameter, rather than naming it explicitly as `impl<Context> AreaCalculator for Context`. Omitting `for Context` is what makes the provider read like an ordinary trait `impl`; the macro supplies a reserved context parameter and treats `self`/`Self` as the context. Write the context out by hand:

```rust
#[cgp_impl(new RectangleArea)]
impl<Context> AreaCalculator for Context
where
    Context: HasDimensions,
{
    fn area(&self) -> f64 {
        self.width() * self.height()
    }
}
```

only when you must name it — to bound it with a lifetime or higher-ranked bound the sugar cannot spell, or to refer to it by a readable name. Otherwise write the shorter form and declare the bound with [`#[uses(...)]`](declaring-dependencies.md):

```rust
#[cgp_impl(new RectangleArea)]
#[uses(HasDimensions)]
impl AreaCalculator {
    fn area(&self) -> f64 {
        self.width() * self.height()
    }
}
```

## Related guides

- [Declaring a provider's dependencies](declaring-dependencies.md) — state the `where` bounds this idiom leaves off the header with `#[uses]` and `#[use_provider]`.
- [Reading context fields](reading-context-fields.md) — pull field values into a provider with `#[implicit]` arguments, as the first example does.
- [Guides summary](README.md#summary) — the cheat-sheet across all the guides, and the list of when an explicit form is still right.
