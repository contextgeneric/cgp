# Unsatisfied ordinary trait bound

A provider's impl-side dependency is an *ordinary* Rust trait bound — a standard-library, foreign, or plain user trait such as `Eq`, `Clone`, `Ord`, or `From<X>`, not a CGP capability — and the concrete type the context supplies for an abstract type or generic parameter does not implement it, so a check surfaces the failure as `E0277` naming that ordinary bound.

## What triggers it

This class is the ordinary-trait cousin of a [check-trait failure](check-trait-failure.md): the mechanism is the same lazy wiring, but the unmet leaf is a plain Rust trait rather than a `HasField` or a CGP component. A provider lists a `where`-clause bound that is an ordinary trait applied to an abstract type or an impl generic — `Scalar: Eq`, `Item: Ord`, `T: Clone` — and the context wires a concrete type that does not satisfy it. Because [wiring](../../reference/macros/delegate_components.md) is lazy, the entry is accepted without checking the bound; it fails only when the wiring is exercised.

The canonical case is an ordinary bound on an abstract type. `CompareScalars` needs the context's `Scalar` type to be `Eq`, and the context wires `Scalar` to `f64`, which is `PartialEq` but not `Eq`:

```rust
#[cgp_type]
pub trait HasScalarType {
    type Scalar;
}

#[cgp_component(ScalarEquality)]
#[use_type(HasScalarType.Scalar)]
pub trait CanCompareScalars {
    fn scalars_equal(&self, a: &Scalar, b: &Scalar) -> bool;
}

#[cgp_impl(new CompareScalars)]
#[use_type(HasScalarType.Scalar)]
impl ScalarEquality
where
    Scalar: Eq, // ordinary trait bound — rewritten to <Self as HasScalarType>::Scalar: Eq
{
    fn scalars_equal(&self, a: &Scalar, b: &Scalar) -> bool {
        a == b
    }
}

delegate_components! {
    App {
        ScalarTypeProviderComponent: UseType<f64>, // f64: !Eq
        ScalarEqualityComponent: CompareScalars,
    }
}
```

The same failure arises wherever CGP or Rust accepts a generic trait bound, which is what makes the class broad. An ordinary bound on an **impl generic** in `delegate_components!` behaves identically: a generic context `<T> Wrapper<T>` that wires its abstract type to `T` carries the provider's `Scalar: Eq` bound as `T: Eq`, unconditionally accepted, and checking `Wrapper<f64>` surfaces `f64: Eq` at that one instantiation. The bound may equally sit on an explicit `impl<T>` generic in `#[cgp_impl]`, on a `#[cgp_fn]` `#[extend_where]` clause, or on a `for … in … where` loop — anywhere an ordinary bound rides on a parameter the context ultimately fills. CGP lowers the bound faithfully and cannot see that the wired type violates it, so it defers to the compiler.

## The diagnostic

This is a **surfaced** class: forcing the wiring through a [`check_components!`](../../reference/macros/check_components.md) produces an `E0277` that names the ordinary bound on the concrete type, and its shape is the tell that distinguishes it from a `HasField` check-trait failure. The **primary** error is the leaf itself — "the trait bound `f64: Eq` is not satisfied," with the caret on the checked component entry and the label "the trait `Eq` is not implemented for `f64`." Beneath it, a `help:` note lists the standard types that *do* implement the trait (`i128`, `i16`, `i32`, …), which is `rustc`'s stock "these types implement the trait" hint for a missing ordinary impl — not the near-contradiction "but trait `HasField<other>` is implemented" hint a missing field produces.

The position of the cause is the sharp contrast with a [check-trait failure](check-trait-failure.md), and knowing it tells the two apart on sight. A missing `HasField` roots the *primary* error at `Ctx: CanUseComponent<Component>` and tucks the concrete leaf into a `help:` note below it; an unmet ordinary bound roots the primary error at the leaf (`f64: Eq`) directly, and the `CanUseComponent` obligation appears only as a `note` further down. Between them runs the same scaffolding: a `note: required for CompareScalars to implement IsProviderFor<ScalarEqualityComponent, App>` that points at the `Scalar: Eq` bound with "unsatisfied trait bound introduced here", then `required for App to implement CanUseComponent<…>`, then the bound in the generated `__CheckApp` trait. [`IsProviderFor`](../../reference/traits/is_provider_for.md) is again the supertrait that carries the provider's `where` clause into the proof, which is why the ordinary bound is named here and suppressed in the hidden form. The whole thing is ordinary `rustc` obligation resolution for [`E0277`](../error_codes/e0277.md); only the leaf's *kind* — a standard trait on a concrete type — changes what the message and its `help:` look like.

Exercised without a check — by calling the consumer method — the same broken wiring instead produces the [hidden `E0599`](../hidden/unsatisfied-dependency.md), which names only `App: CanCompareScalars` / `App: ScalarEquality<App>` and never mentions `f64: Eq`. The method-probe heuristic drops the nested `where`-clause bound regardless of whether it is a `HasField`, a capability, or an ordinary trait, so the ordinary-bound dependency hides exactly as any other does.

## Where the root cause is

The root cause is **present and sits at the very top** of the checked output: the primary `E0277` names the concrete type and the ordinary trait it fails (`f64: Eq`) outright. This is the most direct of the surfaced classes — there is no `help:`-note indirection as with a `HasField` leaf, and the `required for` notes below build *outward* from the leaf toward the check. The caret lands on the checked component entry, so the error also points at a site the user controls. The one thing the diagnostic does not state is which *wired type* is at fault when the bound is on an abstract type: the leaf names `f64`, and the `IsProviderFor` note names the `Scalar: Eq` bound, but connecting "`f64` is what `Scalar` resolves to" is left to the reader — trace it to the `ScalarTypeProviderComponent: UseType<f64>` wiring.

## Resolving it

The fix is to satisfy the ordinary trait, which is a different remedy from every capability-dependency class — not "wire a component" or "add a field," but make the concrete type implement the trait. Wire the abstract type (or instantiate the generic) to a type that *does* satisfy the bound — an integer type for `Eq`, say, instead of `f64` — or, when the offending type is one you own, derive or implement the trait for it (`#[derive(Eq)]`, a manual `impl Ord`). When the bound is stricter than the behavior needs — requiring `Eq` where `PartialEq` would do — relax the provider's `where` clause instead, so more concrete types qualify. The diagnostic already names both halves: the concrete type in the primary error and the bound in the `IsProviderFor` note.

## Notes for tooling

For a `cargo-cgp`-style post-processor, this class is nearly self-describing, so the value is small and mostly connective. The headline to extract is already the primary error — the concrete type and the ordinary trait (`f64: Eq`) — and the tool should **link it to the wiring that chose the type** by following the abstract-type component to its `UseType<…>` entry, reporting "`Scalar` is wired to `f64`, which the provider `CompareScalars` requires to be `Eq`." The `IsProviderFor` / `CanUseComponent` / `__Check…` frames are CGP scaffolding to collapse, and `rustc`'s list of conforming standard types is worth keeping as the actionable hint. Recognizing that the leaf is an ordinary trait rather than a `HasField` lets the tool phrase the fix as "satisfy or relax the trait bound" rather than "wire a component."

## Backing fixtures

- [acceptable/check_components/ordinary_bound_unsatisfied.rs](../../../crates/tests/cgp-compile-fail-tests/tests/acceptable/check_components/ordinary_bound_unsatisfied.rs) — a provider requiring `Scalar: Eq` on a context that wires `Scalar` to `f64`, checked; its `.stderr` pins the primary `E0277` on `f64: Eq`, the `help:` list of conforming standard types, and the `IsProviderFor` note pointing at the `Scalar: Eq` bound.
- [acceptable/check_components/generic_context_ordinary_bound.rs](../../../crates/tests/cgp-compile-fail-tests/tests/acceptable/check_components/generic_context_ordinary_bound.rs) — the same bound reached through impl generics: a generic `<T> Wrapper<T>` table checked at `Wrapper<f64>`, surfacing `f64: Eq` through `IsProviderFor<…, Wrapper<f64>>`.

The hidden (method-call) form of the same mistake is [acceptable/delegate_components/ordinary_bound_unsatisfied.rs](../../../crates/tests/cgp-compile-fail-tests/tests/acceptable/delegate_components/ordinary_bound_unsatisfied.rs), cataloged with the [hidden unsatisfied dependency](../hidden/unsatisfied-dependency.md) class it shares its shape with.

## Related

- [Check-trait failure (surfaced)](check-trait-failure.md) — the sibling surfaced class whose leaf is a CGP capability (`HasField`, a component); contrast the leaf's *kind* (an ordinary trait here) and its *position* (the primary error here, a `help:` note there).
- [Unsatisfied dependency (hidden)](../hidden/unsatisfied-dependency.md) — the `E0599` form this takes when reached by a method call, where the ordinary bound is suppressed just like any other leaf.
- [Verbose dependency cascade](verbose-cascade.md) — when several providers share the unmet ordinary bound, this diagnostic multiplies the same way.
- [`E0277`](../error_codes/e0277.md) — the Rust error code this class is reported under, and its `Sized` special case.
- [`#[use_type]`](../../reference/attributes/use_type.md), [`#[cgp_type]`](../../reference/macros/cgp_type.md), and [`check_components!`](../../reference/macros/check_components.md) — the abstract-type import, the abstract-type component, and the check that surfaces the bound.
- [Debugging CGP compile errors](../../guides/debugging.md) — the `E0277` entry in the decoder.
