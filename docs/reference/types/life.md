# `Life`

`Life<'a>` is a zero-sized type that lifts a lifetime into a type, so that a lifetime parameter from a CGP trait can travel through machinery that only accepts types.

## Purpose

`Life` exists because CGP's wiring is parameterized by *types*, not lifetimes, yet a CGP trait may carry a lifetime generic of its own. The marker that surfaces a provider's dependencies, [`IsProviderFor`](../traits/is_provider_for.md), takes a tuple of the trait's generic parameters as a single type argument so that the compiler can match a provider against the exact instantiation it is asked for. A lifetime cannot appear directly in that tuple — a tuple is a type and its members must be types — so any lifetime parameter on the trait must first be turned into a type. `Life<'a>` is that conversion: it packages the lifetime `'a` as a concrete type that can sit alongside the trait's other type parameters.

Without this lift, a consumer or provider trait that borrows — one declaring `fn get_reference(&self) -> &'a T` — could not record its `'a` in the dependency marker, and the wiring would be unable to distinguish one lifetime instantiation from another. `Life` lets the lifetime ride through `IsProviderFor` as `(Life<'a>, T)`, preserving it as part of the provider's identity while keeping the marker's argument a plain type. The macros insert `Life` automatically when generating provider traits for lifetime-carrying components; a user rarely writes it by hand but will see it in generated code and in compiler errors about provider resolution.

## Definition

`Life` is a tuple struct wrapping a single `PhantomData` over a raw pointer to a borrowed unit:

```rust
pub struct Life<'a>(pub PhantomData<*mut &'a ()>);
```

The struct holds no runtime data — it is a zero-sized marker whose only job is to carry the lifetime `'a` in the type system. The choice of `PhantomData<*mut &'a ()>` for the phantom type is deliberate and controls how `Life<'a>` relates to other lifetimes under subtyping. A `*mut T` is *invariant* in `T`, so wrapping `&'a ()` behind a `*mut` makes `Life<'a>` invariant in `'a`: a `Life<'long>` is neither a subtype nor a supertype of a `Life<'short>`. Invariance is the correct choice here because the lifetime is being used as an exact identity in the dependency marker — two providers wired for different lifetimes must be treated as wired for genuinely different things, and a variant `Life` would let the compiler silently coerce one instantiation into another and pick the wrong provider. The raw pointer also keeps `Life<'a>` from carrying any auto-trait obligations tied to an actual borrow, since it does not own or reference a real value.

## Behavior

`Life` has no methods and implements no CGP traits of its own; its entire behavior is to occupy a type position. In a generated provider trait for a component with a lifetime, the lifetime is collected into the `IsProviderFor` argument tuple as `Life<'a>` so that the provider's dependency obligation reads the same way it would for any type parameter. The provider trait, its blanket forwarding impl, and the impls that satisfy it all agree on the same `(Life<'a>, T)` shape, which is what lets a borrowing component be wired and checked exactly like a non-borrowing one.

## Examples

`Life` appears in the wiring generated for a component whose consumer trait carries a lifetime. Given a borrowing getter component:

```rust
use cgp::prelude::*;

#[cgp_component(ReferenceGetter)]
pub trait HasReference<'a, T: 'a + ?Sized> {
    fn get_reference(&self) -> &'a T;
}
```

the generated provider trait records the lifetime in its dependency marker through `Life`, so its `IsProviderFor` bound names the lifetime as the type `Life<'a>` rather than as a bare `'a`:

```rust
// generated, in readable form:
// pub trait ReferenceGetter<'a, __Context__, T: 'a + ?Sized>:
//     IsProviderFor<ReferenceGetterComponent, __Context__, (Life<'a>, T)>
// {
//     fn get_reference(__context__: &__Context__) -> &'a T;
// }
```

Every impl that wires this component — whether through `UseContext`, a `UseField` getter, or a hand-written provider — carries the same `(Life<'a>, T)` tuple, so the lifetime is preserved end to end through the resolution machinery.

## Related constructs

`Life` is consumed by [`IsProviderFor`](../traits/is_provider_for.md), whose final type argument is the tuple of a component's generic parameters and into which a lifetime parameter is lifted as `Life<'a>`. It is emitted by the component-defining macros, principally [`#[cgp_component]`](../macros/cgp_component.md), when a consumer trait declares a lifetime. Conceptually it sits alongside the other type-level markers CGP uses to make non-type things addressable in trait resolution — [`Index`](index.md) lifts a `usize` and [`Symbol`](chars.md) lifts a string the way `Life` lifts a lifetime.

## Source

The type is defined in [crates/core/cgp-field/src/types/life.rs](../../../crates/core/cgp-field/src/types/life.rs). The macro logic that wraps a trait's lifetime parameters in `Life` when building the `IsProviderFor` argument tuple is in [crates/macros/cgp-macro-core/src/functions/is_provider_params.rs](../../../crates/macros/cgp-macro-core/src/functions/is_provider_params.rs), with related placement in [crates/macros/cgp-macro-core/src/types/empty_struct.rs](../../../crates/macros/cgp-macro-core/src/types/empty_struct.rs) and [crates/macros/cgp-macro-core/src/types/cgp_provider/provider_impl_args.rs](../../../crates/macros/cgp-macro-core/src/types/cgp_provider/provider_impl_args.rs). The generated wiring for a lifetime-carrying component is exercised by the snapshot tests in [crates/tests/cgp-tests/tests/generic_components/](../../../crates/tests/cgp-tests/tests/generic_components/).
