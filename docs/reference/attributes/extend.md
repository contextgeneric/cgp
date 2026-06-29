# `#[extend(...)]`

`#[extend(...)]` adds the given trait bounds as supertraits of the generated trait, making them a public part of the trait's interface rather than a hidden impl-side dependency.

## Purpose

`#[extend(...)]` exists to add supertraits to a CGP trait through an import-like attribute, and in [`#[cgp_fn]`](../macros/cgp_fn.md) it is the only way to do so. A supertrait is a bound that every implementor of a trait must also satisfy, and that every user of the trait may rely on. In `#[cgp_fn]`, the `where` clauses written in the function body are treated as impl-side dependencies and deliberately kept out of the generated trait definition — so there is no place to write a supertrait by hand. `#[extend(...)]` fills that gap: the bounds it lists are promoted onto the trait itself.

The distinction from [`#[uses]`](uses.md) is the whole point. Both attributes accept the same simplified trait-path syntax and both feel like imports, but they import into different places. `#[uses(...)]` adds a hidden `Self` bound to the impl only — a private dependency that callers never see. `#[extend(...)]` adds a supertrait to the trait — a public requirement that becomes part of the contract. The natural way to describe the pair is that `#[extend(...)]` is the `pub use` equivalent of `#[uses(...)]`: where `#[uses(...)]` imports a capability for the implementation's own use, `#[extend(...)]` re-exports it as part of what the trait guarantees.

This framing also explains when to reach for it. `#[extend(...)]` exists to make supertraits approachable for programmers who are not yet comfortable with Rust's supertrait syntax, by presenting them as imports. Authors who are comfortable writing `pub trait Foo: Bar` directly can do so in [`#[cgp_component]`](../macros/cgp_component.md); the choice there is stylistic. In `#[cgp_fn]`, where direct supertrait syntax is unavailable, `#[extend(...)]` is the mechanism.

## Syntax

`#[extend(...)]` takes a comma-separated list of trait bounds in the simplified form `TraitIdent<ParamA, ParamB, ...>`:

```rust
#[extend(HasScalarType)]
```

Each entry names a trait that becomes a supertrait of the generated trait, optionally with generic type arguments. A bare `HasScalarType` becomes a `: HasScalarType` supertrait; a parameterized form carries its arguments through. Multiple bounds may be listed in one attribute or spread across several `#[extend(...)]` attributes, and they accumulate.

`#[extend(...)]` is accepted in [`#[cgp_fn]`](../macros/cgp_fn.md) and in [`#[cgp_component]`](../macros/cgp_component.md). It is not available in [`#[cgp_impl]`](../macros/cgp_impl.md), because a provider impl has no trait definition of its own to attach supertraits to — the supertraits belong to the component's trait, defined by `#[cgp_component]`.

## Expansion

`#[extend(...)]` adds each listed bound as a supertrait of the generated trait, and the same bound also appears in the impl's `where` clause so the implementation may rely on it. Starting from a `#[cgp_fn]` definition that depends on an abstract `Scalar` type:

```rust
pub trait HasScalarType {
    type Scalar: Clone + Mul<Output = Self::Scalar>;
}

#[cgp_fn]
#[extend(HasScalarType)]
fn rectangle_area(
    &self,
    #[implicit] width: Self::Scalar,
    #[implicit] height: Self::Scalar,
) -> Self::Scalar {
    width * height
}
```

the macro emits a trait carrying `HasScalarType` as a supertrait, and an impl that carries both `Self: HasScalarType` and the `HasField` bounds from the implicit arguments:

```rust
pub trait RectangleArea: HasScalarType {
    fn rectangle_area(&self) -> Self::Scalar;
}

impl<Context> RectangleArea for Context
where
    Self: HasScalarType,
    Self: HasField<Symbol!("width"), Value = Self::Scalar>
        + HasField<Symbol!("height"), Value = Self::Scalar>,
{
    fn rectangle_area(&self) -> Self::Scalar {
        let width: Self::Scalar =
            self.get_field(PhantomData::<Symbol!("width")>).clone();
        let height: Self::Scalar =
            self.get_field(PhantomData::<Symbol!("height")>).clone();

        width * height
    }
}
```

The bound appears in two places for a reason. On the trait it is a supertrait, so `Self::Scalar` resolves and callers know any `RectangleArea` type is also a `HasScalarType`. In the impl `where` clause it lets the implementation actually use the associated type. This double placement is the difference from [`#[uses]`](uses.md), which adds the bound to the impl only. The generated context type is named `__Context__` in the real output; `Context` is used here for readability.

In [`#[cgp_component]`](../macros/cgp_component.md) the effect is purely on the consumer trait, and it is exactly equivalent to writing the supertrait directly. The definition

```rust
#[cgp_component(AreaCalculator)]
#[extend(HasScalarType)]
pub trait CanCalculateArea {
    fn area(&self) -> Self::Scalar;
}
```

is the same as `pub trait CanCalculateArea: HasScalarType`. Here `#[extend(...)]` buys nothing the language does not already offer — it is available only so that the `use`/`pub use` pairing with `#[uses(...)]` reads consistently across both macros.

## Examples

`#[extend(...)]` shines when a `#[cgp_fn]` capability depends on an abstract type that the context must provide. The function below works for any context that defines a `Scalar` type and supplies `width` and `height` fields of that type:

```rust
use cgp::prelude::*;
use core::ops::Mul;

pub trait HasScalarType {
    type Scalar: Clone + Mul<Output = Self::Scalar>;
}

#[cgp_fn]
#[extend(HasScalarType)]
pub fn rectangle_area(
    &self,
    #[implicit] width: Self::Scalar,
    #[implicit] height: Self::Scalar,
) -> Self::Scalar {
    width * height
}

#[derive(HasField)]
pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

impl HasScalarType for Rectangle {
    type Scalar = f64;
}
```

Because `HasScalarType` is a supertrait of `RectangleArea`, the abstract `Self::Scalar` is usable in the signature and body, and `Rectangle` — which implements `HasScalarType` with `Scalar = f64` and derives `HasField` for its two fields — satisfies every bound, gaining `rectangle_area`.

## Related constructs

`#[extend(...)]` is the `pub use` counterpart to [`#[uses]`](uses.md): the two share syntax but differ in placement, with `#[extend(...)]` adding public supertraits and `#[uses(...)]` adding hidden impl-side bounds. It is used in [`#[cgp_fn]`](../macros/cgp_fn.md), where it is the only way to declare supertraits, and in [`#[cgp_component]`](../macros/cgp_component.md), where it duplicates the native supertrait syntax for stylistic consistency. When the supertrait is an abstract-type trait whose associated type is referenced throughout the signature, prefer [`#[use_type]`](use_type.md), which adds the supertrait *and* rewrites bare type names into fully-qualified form. To add `where` clauses (not supertraits) to a `#[cgp_fn]` trait definition, use [`#[extend_where]`](extend_where.md).

## Source

`#[extend(...)]` is parsed in [crates/macros/cgp-macro-core/src/types/attributes/function.rs](../../../crates/macros/cgp-macro-core/src/types/attributes/function.rs) (the `extend` field of `FunctionAttributes`). For `#[cgp_fn]`, the bounds are added to the trait's supertraits and to the impl `where` clause in [crates/macros/cgp-macro-core/src/types/cgp_fn/preprocessed.rs](../../../crates/macros/cgp-macro-core/src/types/cgp_fn/preprocessed.rs). For `#[cgp_component]`, the attribute is parsed by `CgpComponentAttributes::parse` and its bounds appended to the consumer trait's supertraits in [crates/macros/cgp-macro-core/src/types/attributes/cgp_component_attributes.rs](../../../crates/macros/cgp-macro-core/src/types/attributes/cgp_component_attributes.rs). Expansion snapshots are in [crates/tests/cgp-tests/tests/cgp_fn_tests/extend.rs](../../../crates/tests/cgp-tests/tests/cgp_fn_tests/extend.rs) and the component abstract-type tests in [crates/tests/cgp-tests/tests/component_tests/abstract_types/](../../../crates/tests/cgp-tests/tests/component_tests/abstract_types/).
