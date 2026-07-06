# `#[uses(...)]`

`#[uses(...)]` adds simple `Self: Trait<...>` bounds to a provider's `where` clause, written to read like a `use` import of the CGP capabilities the body depends on.

## Purpose

`#[uses(...)]` exists to make impl-side dependencies look like imports rather than trait bounds. A provider often calls capabilities defined elsewhere — another [`#[cgp_fn]`](../macros/cgp_fn.md) trait, or a [`#[cgp_component]`](../macros/cgp_component.md) consumer trait — and to do so it must require the context to implement them. Expressed in raw Rust, that is a `where Self: SomeTrait` clause, which is unfamiliar territory for many programmers: writing a bound on `Self` is uncommon in everyday Rust, and it reads as machinery rather than intent.

`#[uses(RectangleArea)]` instead reads as "this function uses the `RectangleArea` capability," which mirrors the mental model of a `use` statement bringing a name into scope. The attribute lists the capabilities the body relies on; the macro turns each into the corresponding `Self` bound on the generated impl. The body can then call those methods directly on `self`, exactly as if they had been imported.

This framing is the reason `#[uses(...)]` is recommended over hand-written `Self` bounds in basic CGP. It keeps the dependency declaration close in spirit to the `use` statements a reader already understands, and it keeps the code focused on *what* the provider needs rather than *how* the bound is spelled.

## Syntax

`#[uses(...)]` takes a comma-separated list of trait bounds, each of which becomes a `Self:` predicate on the generated impl:

```rust
#[uses(RectangleArea, CanCalculateArea)]
```

Each entry is the name of a capability, optionally with generic type arguments. A bare `RectangleArea` becomes `Self: RectangleArea`; a parameterized `CanCompute<Code, Input>` becomes `Self: CanCompute<Code, Input>`. When a provider depends on several capabilities, prefer listing them all in a single `#[uses(...)]` attribute — `#[uses(RectangleArea, CanCalculateArea)]` — since one combined import reads as a single dependency list. The entries may also be split across multiple `#[uses(...)]` attributes on the same item, and they accumulate, but stack a second attribute only when a genuine reason calls for it rather than as the default.

The idiomatic entry is the simple `Trait<Params>` form, since the attribute is meant to read like an import. An entry may nonetheless be any bound a Rust `where` clause accepts, including an associated-type-equality binding (`HasErrorType<Error = anyhow::Error>`), a higher-ranked bound, or a lifetime bound; each lands on the impl's `where` clause verbatim. Use that generality sparingly. To pin an abstract type to a concrete one, prefer the [`#[use_type]`](use_type.md) equality form `#[use_type(HasErrorType.{Error = anyhow::Error})]`, which adds the bound and also rewrites bare mentions of the type; and to place a bound on the generated *trait* rather than only its impl, use [`#[extend_where]`](extend_where.md) (on `#[cgp_fn]`).

`#[uses(...)]` is accepted in both [`#[cgp_fn]`](../macros/cgp_fn.md) and [`#[cgp_impl]`](../macros/cgp_impl.md). In either case it imports capabilities into the provider being defined, and those capabilities may themselves be defined with either [`#[cgp_fn]`](../macros/cgp_fn.md) or [`#[cgp_component]`](../macros/cgp_component.md) — the attribute does not care how the imported capability was produced, only that it is a trait the context can implement.

## Expansion

`#[uses(...)]` adds one `Self`-anchored predicate to the generated impl's `where` clause for the listed bounds, and changes nothing about the trait definition. Starting from two `#[cgp_fn]` capabilities where the second depends on the first:

```rust
#[cgp_fn]
fn rectangle_area(&self, #[implicit] width: f64, #[implicit] height: f64) -> f64 {
    width * height
}

#[cgp_fn]
#[uses(RectangleArea)]
fn scaled_rectangle_area(&self, #[implicit] scale_factor: f64) -> f64 {
    self.rectangle_area() * scale_factor * scale_factor
}
```

the second definition expands so that the trait is unchanged but the impl gains a `Self: RectangleArea` predicate, sitting alongside the `HasField` bound that the implicit `scale_factor` argument introduces:

```rust
pub trait ScaledRectangleArea {
    fn scaled_rectangle_area(&self) -> f64;
}

impl<Context> ScaledRectangleArea for Context
where
    Self: RectangleArea,
    Self: HasField<Symbol!("scale_factor"), Value = f64>,
{
    fn scaled_rectangle_area(&self) -> f64 {
        let scale_factor: f64 =
            self.get_field(PhantomData::<Symbol!("scale_factor")>).clone();

        self.rectangle_area() * scale_factor * scale_factor
    }
}
```

The imported bound lands on the impl only, never on the trait — it is an impl-side dependency, hidden from anyone who merely uses `ScaledRectangleArea`. Writing `#[uses(RectangleArea)]` is therefore exactly equivalent to writing `where Self: RectangleArea` in the function body; the two desugar identically. The generated context type is named `__Context__` in the real output, shown as `Context` here for readability.

Inside [`#[cgp_impl]`](../macros/cgp_impl.md) the behavior is the same. The listed bounds are appended to the impl's `where` clause as `Self` predicates, so a provider can import a `#[cgp_fn]` capability and call it directly:

```rust
#[cgp_impl(new RectangleAreaCalculator)]
#[uses(RectangleArea)]
impl AreaCalculator {
    fn area(&self) -> f64 {
        self.rectangle_area()
    }
}
```

This adds `Self: RectangleArea` to the `RectangleAreaCalculator` impl, letting `area` call `self.rectangle_area()`. The component remains usable by any context that implements `RectangleArea`, regardless of which provider supplies it.

## Examples

`#[uses(...)]` is the natural way to build a capability on top of a CGP component. Given an `AreaCalculator` component, a scaled-area function can import it and use it without knowing which provider is wired:

```rust
use cgp::prelude::*;

#[cgp_component(AreaCalculator)]
pub trait CanCalculateArea {
    fn area(&self) -> f64;
}

#[cgp_fn]
#[uses(CanCalculateArea)]
pub fn scaled_area(&self, #[implicit] scale_factor: f64) -> f64 {
    self.area() * scale_factor * scale_factor
}
```

The `#[uses(CanCalculateArea)]` attribute adds `Self: CanCalculateArea` to the generated `ScaledArea` impl, so the body may call `self.area()`. Any context that implements `CanCalculateArea` — through whatever `AreaCalculator` provider it has wired — automatically gains `scaled_area`, because the dependency is on the consumer trait rather than on a specific provider.

## Related constructs

`#[uses(...)]` is the impl-side counterpart to [`#[extend]`](extend.md): both add trait bounds to a generated construct, but `#[uses(...)]` adds hidden impl-side `where` bounds while `#[extend]` adds public supertraits — the `use` versus `pub use` distinction. It is used inside [`#[cgp_fn]`](../macros/cgp_fn.md) and [`#[cgp_impl]`](../macros/cgp_impl.md), and the capabilities it imports are typically defined with [`#[cgp_fn]`](../macros/cgp_fn.md) or [`#[cgp_component]`](../macros/cgp_component.md). To import an abstract associated type rather than a method capability, use [`#[use_type]`](use_type.md); to bring in a context field as a function argument, use [`#[implicit]`](implicit.md). To pin an abstract type, prefer the [`#[use_type]`](use_type.md) equality form over spelling the equality in `#[uses(...)]`; to make a bound part of the generated trait rather than only its impl, use [`#[extend_where]`](extend_where.md).

## Source

- Parsing: `#[uses(...)]` parsing lives in [crates/macros/cgp-macro-core/src/types/attributes/uses.rs](../../../crates/macros/cgp-macro-core/src/types/attributes/uses.rs) (the `UsesAttributes` type and its `to_type_param_bounds`).
- Dispatch: the attribute dispatch is in [crates/macros/cgp-macro-core/src/types/attributes/function.rs](../../../crates/macros/cgp-macro-core/src/types/attributes/function.rs) for `#[cgp_fn]` and [crates/macros/cgp-macro-core/src/types/attributes/cgp_impl_attributes.rs](../../../crates/macros/cgp-macro-core/src/types/attributes/cgp_impl_attributes.rs) for `#[cgp_impl]`.
- Injection: the bounds are added to the impl `where` clause in [crates/macros/cgp-macro-core/src/types/cgp_fn/preprocessed.rs](../../../crates/macros/cgp-macro-core/src/types/cgp_fn/preprocessed.rs).
- Implementation document (the internal AST type, what the attribute injects into each host, and the index of tests and snapshots): [implementation/asts/attributes/uses.md](../../implementation/asts/attributes/uses.md).
