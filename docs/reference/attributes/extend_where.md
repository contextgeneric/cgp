# `#[extend_where(...)]`

`#[extend_where(...)]` adds `where` clauses to the generated trait definition in [`#[cgp_fn]`](../macros/cgp_fn.md), so a bound becomes part of the trait's interface rather than only its implementation.

## Purpose

`#[extend_where(...)]` exists to put a `where` clause on the *trait* a `#[cgp_fn]` generates, not just on its impl. By default `#[cgp_fn]` treats every `where` clause written in the function body as an impl-side dependency: the bound goes onto the generated impl and is hidden from the trait definition. That is usually what you want, but sometimes a bound must be visible on the trait itself — for example, a constraint on a generic type parameter that callers of the trait need to know about. `#[extend_where(...)]` is how you promote such a bound onto the trait.

It complements [`#[extend]`](extend.md). Where `#[extend(...)]` adds *supertrait* bounds (`pub trait Foo: Bar`) to the generated trait, `#[extend_where(...)]` adds *`where`-clause* bounds (`pub trait Foo where T: Bar`) to it. Both make a requirement part of the trait's public interface; they differ only in which syntactic position the bound occupies.

## Syntax

`#[extend_where(...)]` takes a comma-separated list of full `where`-clause predicates:

```rust
#[extend_where(Scalar: Clone)]
```

Unlike [`#[uses]`](uses.md) and [`#[extend]`](extend.md), which accept only the simplified trait-path form, `#[extend_where(...)]` accepts arbitrary predicates — the same things a Rust `where` clause allows, including associated-type-equality bounds. Each predicate is added verbatim to the generated trait's `where` clause.

`#[extend_where(...)]` is supported only in [`#[cgp_fn]`](../macros/cgp_fn.md). It has no meaning in [`#[cgp_impl]`](../macros/cgp_impl.md) or [`#[cgp_component]`](../macros/cgp_component.md), because in those macros the `where` clause you write is already part of the trait definition — there is nothing to promote, so write the bound as a normal `where` clause directly.

## Expansion

`#[extend_where(...)]` adds its predicates to the `where` clause of the generated trait, and the same predicates also remain on the impl. Starting from a generic `#[cgp_fn]` definition:

```rust
#[cgp_fn]
#[extend_where(Scalar: Clone)]
fn rectangle_area<Scalar>(
    &self,
    #[implicit] width: Scalar,
    #[implicit] height: Scalar,
) -> Scalar
where
    Scalar: Mul<Output = Scalar>,
{
    width * height
}
```

the macro emits a trait whose definition carries the `Scalar: Clone` bound in its own `where` clause:

```rust
pub trait RectangleArea<Scalar>
where
    Scalar: Clone,
{
    fn rectangle_area(&self) -> Scalar;
}
```

The `Scalar: Mul<Output = Scalar>` bound, written in the function body, stays as an impl-side dependency on the generated impl and does not appear on the trait. The `Scalar: Clone` bound from `#[extend_where(...)]` is what gets promoted onto the trait definition.

## Examples

`#[extend_where(...)]` is the right tool when a generic parameter of a `#[cgp_fn]` trait needs a publicly visible bound. The example above already shows the realistic shape: a `Scalar`-generic area function whose trait advertises `Scalar: Clone` while keeping the multiplication bound private to the impl. The promoted bound means any code naming `RectangleArea<Scalar>` can rely on `Scalar: Clone` without restating it.

## Related constructs

`#[extend_where(...)]` is the `where`-clause sibling of [`#[extend]`](extend.md): both promote a requirement onto the generated trait, with `#[extend]` adding supertraits and `#[extend_where(...)]` adding `where` predicates. It is specific to [`#[cgp_fn]`](../macros/cgp_fn.md), since only there are body `where` clauses hidden from the trait. To add hidden impl-side bounds instead of trait-visible ones, use [`#[uses]`](uses.md).

## Source

`#[extend_where(...)]` is parsed in [crates/macros/cgp-macro-core/src/types/attributes/function.rs](../../../crates/macros/cgp-macro-core/src/types/attributes/function.rs) (the `extend_where` field of `FunctionAttributes`), and its predicates are added to both the trait and impl `where` clauses in [crates/macros/cgp-macro-core/src/types/cgp_fn/preprocessed.rs](../../../crates/macros/cgp-macro-core/src/types/cgp_fn/preprocessed.rs). An expansion snapshot that exercises `#[extend_where(...)]` alongside `#[use_type]` lives in [crates/tests/cgp-tests/tests/cgp_fn_tests/nested_foreign_type.rs](../../../crates/tests/cgp-tests/tests/cgp_fn_tests/nested_foreign_type.rs), among the other `#[cgp_fn]` tests in [crates/tests/cgp-tests/tests/cgp_fn_tests/](../../../crates/tests/cgp-tests/tests/cgp_fn_tests/).
