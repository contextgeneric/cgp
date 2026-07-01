# `#[use_type]`

`#[use_type]` imports an abstract associated type into a `#[cgp_fn]`, `#[cgp_impl]`, or `#[cgp_component]` definition and rewrites every bare mention of that type into the fully-qualified `<Self as Trait>::AssocType` form, adding the trait as a supertrait or bound at the same time.

## Purpose

`#[use_type]` removes the boilerplate of referring to an abstract type that lives on another CGP trait. A CGP trait often needs a type that is defined elsewhere — a `Scalar` from `HasScalarType`, an `Error` from `HasErrorType` — and Rust requires every reference to that type to be written in fully-qualified form, `<Self as HasScalarType>::Scalar`, because a bare `Scalar` is not a type the compiler knows about. Writing that prefix on every occurrence, in the return type, in each implicit argument, and in the body, is verbose and easy to get wrong.

The attribute lets you write the bare identifier `Scalar` everywhere and have the macro expand it for you. You declare the type once in the attribute — `#[use_type(HasScalarType::Scalar)]` — and the macro replaces each standalone `Scalar` type with `<Self as HasScalarType>::Scalar`, while also adding `HasScalarType` as a supertrait of the generated trait (for `#[cgp_component]`) or as a `where`-clause bound on the impl (for `#[cgp_impl]` and `#[cgp_fn]`). The bare identifier reads like a normal generic, but resolves to the qualified associated type.

Beyond saving keystrokes, the fully-qualified rewrite removes ambiguity that the bare form cannot express. Because the macro always emits the `<Self as Trait>::Type` path, nested associated types compose without the author ever spelling out the path, foreign abstract types can be pulled from a type parameter rather than `Self`, and type-equality constraints between two imported types can be stated declaratively. These capabilities are why the `/cgp` skill recommends `#[use_type]` as the default way to import abstract types in all three macros.

## Syntax

`#[use_type]` is applied as an outer attribute alongside the `#[cgp_fn]`, `#[cgp_impl]`, or `#[cgp_component]` attribute, and its argument names a trait and one or more of its associated types. The simplest form imports a single type from a trait by path:

```rust
#[use_type(HasScalarType::Scalar)]
```

The path before the final segment is the trait, and the final segment is the associated type to import. The rewrite target — the type the bare identifier expands into — defaults to `Self`, so the example above rewrites `Scalar` to `<Self as HasScalarType>::Scalar`.

A leading `@` changes the rewrite target from `Self` to a named type, which is how foreign abstract types are imported. The form `#[use_type(@Types::HasScalarType::Scalar)]` treats the first segment, `Types`, as the context type and rewrites `Scalar` to `<Types as HasScalarType>::Scalar`. `Types` is typically a generic parameter of the function or impl rather than `Self`, which lets a trait pull an abstract type from a parameter instead of from the implementing context.

Several types from the same trait can be imported in one attribute using a braced list, and each entry may be renamed with `as` or constrained with `=`. The braced form `#[use_type(HasFooType::{Foo, Bar as Baz})]` imports `Foo` under its own name and `Bar` under the local alias `Baz`. The equality form `#[use_type(HasScalarType::{Scalar = f64})]` imports `Scalar` and additionally constrains it, emitting `Self: HasScalarType<Scalar = f64>` in the `where` clause. Multiple `#[use_type]` attributes may also be stacked, and several trait paths may be separated by commas inside one attribute, as in `#[use_type(HasBarType::{Bar as Baz = Foo}, HasFooType::Foo)]`.

One restriction applies to `#[cgp_component]` specifically: the `= ...` type-equality form is rejected there, because a trait definition cannot carry the impl-side equality constraint that the equality form produces. Equality constraints belong on `#[cgp_fn]` and `#[cgp_impl]`, where they become `where` bounds.

## Expansion

`#[use_type]` runs before the rest of the macro and does two things: it substitutes every matching bare type identifier with the qualified associated type, and it adds the trait as a supertrait or bound. Consider this `#[cgp_fn]` using the single-import form:

```rust
pub trait HasScalarType {
    type Scalar: Clone + Mul<Output = Self::Scalar>;
}

#[cgp_fn]
#[use_type(HasScalarType::Scalar)]
fn rectangle_area(
    &self,
    #[implicit] width: Scalar,
    #[implicit] height: Scalar,
) -> Scalar {
    width * height
}
```

The macro first rewrites every standalone `Scalar` to `<Self as HasScalarType>::Scalar` and appends `HasScalarType` to the bounds, then desugars the resulting `#[cgp_fn]` as usual. The effective expansion is:

```rust
pub trait RectangleArea: HasScalarType {
    fn rectangle_area(&self) -> <Self as HasScalarType>::Scalar;
}

impl<Context> RectangleArea for Context
where
    Self: HasField<Symbol!("width"), Value = <Self as HasScalarType>::Scalar>
        + HasField<Symbol!("height"), Value = <Self as HasScalarType>::Scalar>,
    Self: HasScalarType,
{
    fn rectangle_area(&self) -> <Self as HasScalarType>::Scalar {
        let width: <Self as HasScalarType>::Scalar =
            self.get_field(PhantomData::<Symbol!("width")>).clone();
        let height: <Self as HasScalarType>::Scalar =
            self.get_field(PhantomData::<Symbol!("height")>).clone();
        width * height
    }
}
```

The substitution is purely textual at the type level: it matches single-segment type paths with no arguments whose identifier equals the imported name (or its alias), and replaces them with `<Self as HasScalarType>::Scalar`. A bare `Scalar` anywhere — return type, implicit-argument annotation, or a `let` binding inside the body — is rewritten the same way, which is what makes nested uses work without the author writing any path.

Because the rewrite fires only on the bare identifier of an *imported* type, a construct's own **local associated types must always stay qualified as `Self::Assoc`** and are left untouched. A `#[cgp_component]` trait or a `#[cgp_impl]` provider that declares its own `type Output` refers to it as `Self::Output`, never as a bare `Output`, precisely because `Output` is the construct's own type rather than one imported from another trait — `#[use_type]` neither imports it nor rewrites it, and it should not be listed in a `#[use_type]` attribute. This is why a mixed signature such as `Result<Self::Output, Error>` is correct and idiomatic: the local `Self::Output` stays qualified while the imported foreign type `Error` (from `#[use_type(HasErrorType::Error)]`) is written bare. Attempting to write the local type bare would leave a `Output` identifier that resolves to nothing, since the substitution pass has no entry for it.

For `#[cgp_component]`, the trait is added as a supertrait rather than a `where` bound, and the rewrite touches the trait's own signatures. Starting from:

```rust
#[cgp_component(AreaCalculator)]
#[use_type(HasScalarType::Scalar)]
pub trait CanCalculateArea {
    fn area(&self) -> Scalar;
}
```

the `#[use_type]` phase rewrites the trait into the following before `#[cgp_component]` proceeds:

```rust
#[cgp_component(AreaCalculator)]
pub trait CanCalculateArea: HasScalarType {
    fn area(&self) -> <Self as HasScalarType>::Scalar;
}
```

The supertrait is added only when the rewrite target is `Self`. With the foreign-type `@` form the target is a named type, so no supertrait is added; instead the bound lands in the impl's `where` clause. This `#[cgp_fn]` imports `Scalar` from a generic parameter `Types`:

```rust
#[cgp_fn]
#[use_type(@Types::HasScalarType::Scalar)]
pub fn rectangle_area<Types: HasScalarType>(
    &self,
    #[implicit] width: Scalar,
    #[implicit] height: Scalar,
) -> Scalar
where
    Scalar: Mul<Output = Scalar> + Copy,
{
    let res: Scalar = width * height;
    res
}
```

Every `Scalar`, including the ones in the explicit `where` clause, expands to `<Types as HasScalarType>::Scalar`, and the bound `Types: HasScalarType` is added to the impl's `where` clause rather than as a supertrait:

```rust
pub trait RectangleArea<Types: HasScalarType> {
    fn rectangle_area(&self) -> <Types as HasScalarType>::Scalar;
}

impl<Context, Types: HasScalarType> RectangleArea<Types> for Context
where
    <Types as HasScalarType>::Scalar:
        Mul<Output = <Types as HasScalarType>::Scalar> + Copy,
    Self: HasField<Symbol!("width"), Value = <Types as HasScalarType>::Scalar>
        + HasField<Symbol!("height"), Value = <Types as HasScalarType>::Scalar>,
    Types: HasScalarType,
{
    fn rectangle_area(&self) -> <Types as HasScalarType>::Scalar {
        let width: <Types as HasScalarType>::Scalar =
            self.get_field(PhantomData::<Symbol!("width")>).clone();
        let height: <Types as HasScalarType>::Scalar =
            self.get_field(PhantomData::<Symbol!("height")>).clone();
        let res: <Types as HasScalarType>::Scalar = width * height;
        res
    }
}
```

The type-equality form adds a constrained bound on top of the substitution. Writing `#[use_type(HasScalarType::{Scalar = f64})]` substitutes `Scalar` to `<Self as HasScalarType>::Scalar` exactly as before, but emits `Self: HasScalarType<Scalar = f64>` in the `where` clause in place of the plain `Self: HasScalarType`, pinning the abstract type to `f64`. When one import's equality target names another import's alias — as in `#[use_type(HasBarType::{Bar as Baz = Foo}, HasFooType::Foo)]` — the macro resolves the target across specs and emits `Self: HasBarType<Bar = <Self as HasFooType>::Foo>`, tying the two abstract types together. Two imports may not share the same identifier or alias; doing so is a compile error.

## Examples

A realistic use threads one abstract `Scalar` type through a component and a provider, with neither writing `Self::` by hand. The component and the type trait come first:

```rust
use cgp::prelude::*;
use core::ops::Mul;

#[cgp_type]
pub trait HasScalarType {
    type Scalar: Clone + Mul<Output = Self::Scalar>;
}

#[cgp_component(AreaCalculator)]
#[use_type(HasScalarType::Scalar)]
pub trait CanCalculateArea {
    fn area(&self) -> Scalar;
}
```

`CanCalculateArea` ends up with `HasScalarType` as a supertrait and `area` returning `<Self as HasScalarType>::Scalar`. A provider for it imports the same type and writes its body in terms of the bare name:

```rust
#[cgp_impl(new RectangleArea)]
#[use_type(HasScalarType::Scalar)]
impl AreaCalculator {
    fn area(&self, #[implicit] width: Scalar, #[implicit] height: Scalar) -> Scalar {
        width * height
    }
}
```

The provider's `#[use_type]` adds `Self: HasScalarType` to its `where` clause and rewrites each `Scalar` to the qualified path, so the implicit `width` and `height` fields and the return value all agree on the context's chosen scalar type. A concrete context then wires `HasScalarType` to a concrete type with `UseType<f64>` and supplies the fields, and `area()` works without any reference to associated-type syntax in the user's own code.

## Related constructs

`#[use_type]` is most often paired with [`#[cgp_type]`](../macros/cgp_type.md), which defines the abstract type trait it imports, and with the [`UseType` provider](../providers/use_type.md), which a context uses to bind that abstract type to a concrete one. It applies to all three implementation macros — [`#[cgp_fn]`](../macros/cgp_fn.md), [`#[cgp_impl]`](../macros/cgp_impl.md), and [`#[cgp_component]`](../macros/cgp_component.md) — adjusting whether it emits a supertrait or a `where` bound based on which it annotates. It overlaps in role with [`#[extend]`](extend.md), which adds a supertrait bound without rewriting type identifiers; `#[use_type]` is preferred when the imported type is actually mentioned in signatures, since it also performs the substitution. The abstract type itself is read through [`HasType`](../components/has_type.md) at the provider level.

## Source

- Parsing: the attribute is parsed by `UseTypeAttribute` in [crates/macros/cgp-macro-core/src/types/attributes/use_type/attribute.rs](../../../crates/macros/cgp-macro-core/src/types/attributes/use_type/attribute.rs), with per-type entries (`as` alias and `=` equality) in `ident.rs`.
- Two-phase transform (substitute then add bounds): lives in `attributes.rs` as `transform_item_trait` (supertrait for `#[cgp_component]`) and `transform_item_impl` (`where` bound for impls); the type-equality and foreign-context resolution are in `type_predicates.rs`.
- Identifier substitution: the `SubstituteAbstractType` `VisitMut` pass in [crates/macros/cgp-macro-core/src/visitors/substitute_abstract_type.rs](../../../crates/macros/cgp-macro-core/src/visitors/substitute_abstract_type.rs), which matches single-segment, argument-free type paths.
- `= ...` rejection for component traits: enforced in `types/attributes/cgp_component_attributes.rs`.
- Implementation document (the internal AST types, the two-phase transform, and the index of tests and snapshots): [implementation/asts/attributes.md](../../implementation/asts/attributes.md).
