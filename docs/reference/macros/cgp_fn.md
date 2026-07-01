# `#[cgp_fn]`

`#[cgp_fn]` turns a plain Rust function into a single-implementation CGP capability — it generates a trait and a blanket impl for every context from one function body, so a context gains the method with no separate wiring step.

## Purpose

`#[cgp_fn]` exists to make the simplest, most common form of CGP reachable with nothing more than a function. Writing a capability by hand means defining a trait, writing a blanket impl over a generic context, and threading the dependencies the body needs through that impl's `where` clause. `#[cgp_fn]` collapses all of that into a single function: you write the body as if `self` were a concrete value, mark the values you want pulled from the context with `#[implicit]`, and the macro produces the trait and the blanket impl that wires it up.

The result is a capability that any context implements automatically, as long as the context can satisfy the impl-side dependencies. Because the generated impl is a blanket impl over a generic context, there is no `delegate_components!` call, no provider type, and no component name — the method simply becomes available on every type that has the fields the body reads. This is what makes `#[cgp_fn]` the recommended entry point for basic CGP: a reader only needs to understand plain Rust functions to use it, and the trait machinery stays hidden.

The trade-off against [`#[cgp_component]`](cgp_component.md) is single versus multiple implementations. A `#[cgp_component]` trait can have many alternative providers, one chosen per context through wiring; that flexibility is exactly what costs the extra ceremony. `#[cgp_fn]` permits only one implementation — the function body — and in exchange removes the wiring entirely. Reach for `#[cgp_fn]` when a capability has a single natural definition, and graduate to `#[cgp_component]` only when a context genuinely needs to swap in a different implementation. The two interoperate: a `#[cgp_fn]` capability can depend on a `#[cgp_component]` one and vice versa through [`#[uses]`](../attributes/uses.md).

## Syntax

`#[cgp_fn]` is applied as an attribute on a free function whose first parameter is `&self` (or `&mut self`). The function name, in snake case, becomes the generated method name; the trait name defaults to that function name converted to PascalCase.

```rust
#[cgp_fn]
pub fn rectangle_area(&self, #[implicit] width: f64, #[implicit] height: f64) -> f64 {
    width * height
}
```

Any parameter marked `#[implicit]` is removed from the method signature and instead fetched from a field of the context whose name matches the parameter. The function above therefore generates a `RectangleArea` trait with a `rectangle_area(&self) -> f64` method, and the body reads `width` and `height` from the context rather than from arguments.

The trait name can be set explicitly by passing an identifier as the attribute argument, which overrides the PascalCase default. This is useful when the verb-style trait name reads better than the function name:

```rust
#[cgp_fn(CanCalculateRectangleArea)]
pub fn rectangle_area(&self, #[implicit] width: f64, #[implicit] height: f64) -> f64 {
    width * height
}
```

Generic parameters and a `where` clause on the function are handled with a deliberate split. Every generic parameter declared in the function's `<...>` list goes onto both the generated trait and the impl. The `where` clause, by contrast, is treated as an impl-side dependency: it lands only on the impl, hidden from the trait interface, exactly as the constraints in a hand-written blanket impl would be.

```rust
#[cgp_fn]
pub fn rectangle_area<Scalar>(
    &self,
    #[implicit] width: Scalar,
    #[implicit] height: Scalar,
) -> Scalar
where
    Scalar: Mul<Output = Scalar> + Copy,
{
    width * height
}
```

Here `Scalar` appears on both `RectangleArea<Scalar>` (the trait) and its impl, while `Scalar: Mul<Output = Scalar> + Copy` appears only on the impl. A bound that should apply to the impl but not be a trait parameter can instead be written with the `#[impl_generics(...)]` attribute, which adds generic parameters to the impl block alone:

```rust
#[cgp_fn]
#[impl_generics(Name: Display)]
pub fn greet(&self, #[implicit] name: &Name) -> String {
    format!("Hello, {}!", name)
}
```

One restriction is intentional: `#[cgp_fn]` does not support generics on the desugared *method* itself. Generics belong to the trait and impl, not to the generated method signature. Method-level generics are uncommon in CGP and, where genuinely needed, are considered an advanced case better written as an explicit blanket impl or a [`#[cgp_component]`](cgp_component.md) provider.

Several companion attributes refine the generated code and are documented separately. [`#[uses(...)]`](../attributes/uses.md) adds trait bounds on `Self` as impl-side dependencies; [`#[use_type(...)]`](../attributes/use_type.md) imports an abstract type and rewrites its occurrences to fully-qualified form; [`#[use_provider(...)]`](../attributes/use_provider.md) supports higher-order providers; [`#[extend(...)]`](../attributes/extend.md) adds supertrait bounds to the generated trait; and [`#[extend_where(...)]`](../attributes/extend_where.md) adds `where` predicates to the generated trait definition.

## Syntax Grammar

The attribute argument of `#[cgp_fn]` is a single optional trait name:

```ebnf
CgpFnArgs -> TraitName?

TraitName -> IDENTIFIER
```

When the argument is omitted, the trait name defaults to the function name converted to PascalCase. The `#[implicit]` markers on parameters and the companion attributes (`#[uses]`, `#[use_type]`, `#[extend]`, and the rest) are separate attributes with their own grammars, documented on their own pages.

## Expansion

`#[cgp_fn]` emits exactly two items: the trait carrying the method, and a blanket impl of that trait for a generic context. Starting from the basic form:

```rust
#[cgp_fn]
pub fn rectangle_area(&self, #[implicit] width: f64, #[implicit] height: f64) -> f64 {
    width * height
}
```

the macro produces the trait — the `#[implicit]` parameters stripped from the signature — followed by the blanket impl over the reserved context type `__Context__`. The implicit parameters become `HasField` bounds on the impl and `get_field` bindings at the top of the body:

```rust
pub trait RectangleArea {
    fn rectangle_area(&self) -> f64;
}

impl<__Context__> RectangleArea for __Context__
where
    Self: HasField<Symbol!("width"), Value = f64>
        + HasField<Symbol!("height"), Value = f64>,
{
    fn rectangle_area(&self) -> f64 {
        let width: f64 = self.get_field(PhantomData::<Symbol!("width")>).clone();
        let height: f64 = self.get_field(PhantomData::<Symbol!("height")>).clone();

        width * height
    }
}
```

The generated context type parameter is literally `__Context__`, not `Context` — the same reserved name `#[cgp_component]` uses — and references to it inside the impl appear as `Self`. The `Symbol!("...")` shorthand stands for the type-level string the macro actually emits (for `width`, `Symbol<5, Chars<'w', Chars<'i', Chars<'d', Chars<'t', Chars<'h', Nil>>>>>>`). Each implicit binding follows the same conversion rules as [`#[cgp_auto_getter]`](cgp_auto_getter.md): an owned value gets a trailing `.clone()`, and a `&str` return gets `.as_str()`, while a borrowed `&Name` is taken by reference with no conversion.

Generics and the `where` clause expand according to the split described above. Given the `Scalar` example, the generic goes on both trait and impl while the function's `where` bound stays on the impl, ordered before the implicit `HasField` bounds — the implicit bounds are always appended last:

```rust
pub trait RectangleArea<Scalar> {
    fn rectangle_area(&self) -> Scalar;
}

impl<__Context__, Scalar> RectangleArea<Scalar> for __Context__
where
    Scalar: Mul<Output = Scalar> + Copy,
    Self: HasField<Symbol!("width"), Value = Scalar>
        + HasField<Symbol!("height"), Value = Scalar>,
{
    fn rectangle_area(&self) -> Scalar { /* ... */ }
}
```

The bounds contributed by the companion attributes are layered into this same impl. `#[uses(Trait)]` and `#[extend(Trait)]` push a `Self: Trait` predicate onto the impl's `where` clause; `#[extend(Trait)]` additionally adds `Trait` as a supertrait of the generated trait, and `#[extend_where(...)]` adds its predicates to the trait's own `where` clause. `#[impl_generics(...)]` inserts its parameters into the impl generics only. The implicit-argument bounds are always appended last, after the attribute-contributed predicates.

## Examples

A two-layer capability shows `#[cgp_fn]` composing with itself through `#[uses]`. The first function defines the base area calculation; the second builds a scaled version on top of it, declaring its dependency on the first with `#[uses(RectangleArea)]`:

```rust
use cgp::prelude::*;

#[cgp_fn]
pub fn rectangle_area(&self, #[implicit] width: f64, #[implicit] height: f64) -> f64 {
    width * height
}

#[cgp_fn]
#[uses(RectangleArea)]
pub fn scaled_rectangle_area(&self, #[implicit] scale_factor: f64) -> f64 {
    self.rectangle_area() * scale_factor * scale_factor
}
```

A concrete context only needs the right fields; no wiring is required. Deriving `HasField` is enough for both capabilities to apply automatically:

```rust
#[derive(HasField)]
pub struct Rectangle {
    pub width: f64,
    pub height: f64,
    pub scale_factor: f64,
}

fn report(rect: &Rectangle) {
    println!("base area  = {}", rect.rectangle_area());
    println!("scaled area = {}", rect.scaled_rectangle_area());
}
```

Because `Rectangle` derives `HasField` and carries `width`, `height`, and `scale_factor`, it satisfies the `HasField` bounds on both generated impls, so `rect.rectangle_area()` and `rect.scaled_rectangle_area()` both resolve directly through the blanket impls — there is no `delegate_components!` block anywhere in this example.

## Related constructs

`#[cgp_fn]` is the lightweight counterpart to [`#[cgp_component]`](cgp_component.md): both produce a capability usable through a method call, but `#[cgp_fn]` allows a single implementation with no wiring, whereas `#[cgp_component]` allows many providers selected per context through [`delegate_components!`](delegate_components.md). It shares its `#[implicit]` argument mechanism with [`#[cgp_impl]`](cgp_impl.md), and the field-access semantics with [`#[cgp_auto_getter]`](cgp_auto_getter.md) and the underlying [`HasField`](../traits/has_field.md) trait. The companion attributes [`#[uses]`](../attributes/uses.md), [`#[use_type]`](../attributes/use_type.md), [`#[use_provider]`](../attributes/use_provider.md), [`#[extend]`](../attributes/extend.md), and [`#[extend_where]`](../attributes/extend_where.md) shape what the macro generates. Like a hand-written extension trait, the impl `#[cgp_fn]` emits is a blanket impl in the style of [`#[blanket_trait]`](blanket_trait.md); the difference is that `#[cgp_fn]` derives the trait and its body from a function rather than from a trait with default methods.

## Source

- Entry point: `cgp_fn` in [crates/macros/cgp-macro-lib/src/cgp_fn.rs](../../../crates/macros/cgp-macro-lib/src/cgp_fn.rs), which parses the optional trait-name identifier and the function, then runs `item.preprocess()?.to_items()?`.
- Logic: [crates/macros/cgp-macro-core/src/types/cgp_fn/](../../../crates/macros/cgp-macro-core/src/types/cgp_fn/) — `item.rs` performs the PascalCase default-name derivation (`to_camel_case_str`), implicit-argument extraction, and attribute parsing; `preprocessed.rs` builds the trait in `to_item_trait` and the blanket impl in `to_item_impl`, including the generics/`where`-clause split and the insertion of the leading `__Context__` parameter.
- Implicit-argument handling: [crates/macros/cgp-macro-core/src/types/implicits/](../../../crates/macros/cgp-macro-core/src/types/implicits/); the companion-attribute parsing in [crates/macros/cgp-macro-core/src/types/attributes/function.rs](../../../crates/macros/cgp-macro-core/src/types/attributes/function.rs).
- Internal walkthrough (the pipeline stages, the function that synthesizes each generated item, the corner-case handling, and the index of tests and expansion snapshots): [implementation/entrypoints/cgp_fn.md](../../implementation/entrypoints/cgp_fn.md).
