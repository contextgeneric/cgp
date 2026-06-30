# Check traits

Check traits are compile-time-only assertions that a context's CGP wiring is complete, written to turn the vague errors that lazy wiring produces into readable ones that name the actual missing dependency.

## Why CGP wiring is lazy

CGP wiring is lazy: recording that a context delegates a component to a provider does not verify that the provider can actually satisfy that component. When a context's delegation table maps `AreaCalculatorComponent` to some provider, the type system accepts the [`DelegateComponent`](../reference/traits/delegate_component.md) entry on its own terms — it stores "this key points to this provider" as an associated type and asks no further questions. Whether that provider's own `where` bounds hold for this particular context is simply not checked at the point of delegation. The check is deferred until something downstream tries to *use* the component, which is the first moment the compiler is forced to evaluate the provider's bounds against the concrete context.

This laziness is what makes CGP composable, but it has a cost: a context can look fully wired and still be broken. Every entry compiles, the struct compiles, the whole module compiles — and then the first call to a consumer trait method fails, often far from the wiring that caused it. A missing `name` field, a missing abstract type, an unsatisfied transitive bound three providers deep: none of these surface where the mistake was made.

## Why the resulting errors are poor

When a lazily-wired context is finally used and a dependency is missing, the compiler reports the outermost unmet conclusion and hides the reasoning behind it. Asking the plain question "does this context implement the consumer trait?" makes Rust answer with the last link in the chain — typically that the provider does not implement the provider trait for this context — without explaining *why* the provider's bounds were not met. The root cause, often a single missing getter or type, is buried beneath a conclusion that points at the provider rather than at the gap. Worse, the relevant type names are expanded into their full type-level forms (the verbose `Symbol<…, Chars<…>>` spelling of a field name, for instance), so even the surface error is hard to read.

The fix is to force the compiler to evaluate the provider's bounds *and report them in detail*, at a location the author controls — the wiring site — rather than wherever the component happens to be used first.

## How check traits force readable errors

A check trait is a dummy trait whose supertrait is the requirement being asserted; implementing it for a context with an empty body compiles only if that requirement holds. The hand-written form is plain Rust:

```rust
trait CanUsePerson: CanGreet {}
impl CanUsePerson for Person {}
```

The `impl` block has nothing to prove on its own, so it succeeds exactly when `Person: CanGreet` holds and fails to compile otherwise. Placed next to the wiring, it converts a latent gap into an immediate compile error at a known line. But asserting the consumer trait directly is not enough, because it reproduces the same vague error described above — it tells you the provider trait is not implemented without saying why. The remedy is to route the assertion through [`CanUseComponent`](../reference/traits/can_use_component.md) instead of the consumer trait.

`CanUseComponent` is satisfied only when the context both delegates the component and the delegated provider satisfies [`IsProviderFor`](../reference/traits/is_provider_for.md) for that context. The crucial property is that `IsProviderFor` is generated to carry the provider's *real* `where` bounds — the same bounds the provider needs to implement its provider trait. Routing a check through `CanUseComponent` therefore forces the compiler to evaluate those bounds and, because they are stated explicitly on the marker, to report the specific one that failed. A missing `name` field surfaces as an unsatisfied `HasField` bound pointing at the context, not as a bare "provider not implemented." `IsProviderFor` is otherwise a near-invisible marker; its whole purpose is to make this error-surfacing work, and check traits are where it earns its keep.

## Writing checks with the macros

Rather than spell out check traits by hand, [`check_components!`](../reference/macros/check_components.md) generates them from a short table of components to verify. Given a context and a list of components, it emits a marker trait aliasing `CanUseComponent` and one empty impl per component:

```rust
check_components! {
    Person {
        GreeterComponent,
    }
}
```

The generated impl compiles only if `Person: CanUseComponent<GreeterComponent, ()>`, which in turn drags in the delegated provider's `IsProviderFor` bounds and reports the first that fails. A successful build *is* the passing assertion — these checks have no runtime existence. For components with generic parameters, the parameters to test are listed after a colon, since the check must name them explicitly to have anything concrete to verify.

Because keeping a standalone check block in sync with the wiring is manual bookkeeping, [`delegate_and_check_components!`](../reference/macros/delegate_and_check_components.md) fuses the two: it wires each entry exactly as [`delegate_components!`](../reference/macros/delegate_components.md) would and derives a check for each delegated key, so every wiring is proven the moment it is written. This is the recommended form for a main context. Its check trait is named `__CanUse{Context}` while `check_components!` names its trait `__Check{Context}`, deliberately distinct so both can appear once in the same module. Plain `delegate_components!` remains right for intermediary provider tables that are not contexts in their own right.

## Checking higher-order providers

For higher-order providers, the `#[check_providers(...)]` form of [`check_components!`](../reference/macros/check_components.md) checks each provider layer independently rather than checking the context as a whole. Instead of asserting `CanUseComponent` on the context, it asserts [`IsProviderFor`](../reference/traits/is_provider_for.md) directly on each named provider — so `RectangleAreaCalculator` and `ScaledAreaCalculator<RectangleAreaCalculator>` are each verified on their own impl. This localizes failures: a dependency missing from the inner `RectangleAreaCalculator` errors on both lines, while one missing only from the outer wrapper errors on the wrapper alone, which narrows down where in a nested stack the gap lives. Provider-level checks are the practical tool for debugging the composition described in [higher-order providers](higher-order-providers.md).

## Related constructs

Check traits verify the wiring produced by [`delegate_components!`](../reference/macros/delegate_components.md), and the two macros that write them are [`check_components!`](../reference/macros/check_components.md) for a standalone check and [`delegate_and_check_components!`](../reference/macros/delegate_and_check_components.md) for wiring-and-checking in one step. The assertion they generate is built on [`CanUseComponent`](../reference/traits/can_use_component.md), which combines [`DelegateComponent`](../reference/traits/delegate_component.md) (the lazy table entry whose acceptance is the source of the problem) with [`IsProviderFor`](../reference/traits/is_provider_for.md) (the marker that carries a provider's real bounds so the compiler reports them). The `#[check_providers]` variant of `check_components!` asserts `IsProviderFor` directly on providers, which is the right tool for the [higher-order providers](higher-order-providers.md) it most often checks.

## Source

The runtime traits live in [crates/core/cgp-component/src/traits/](../../crates/core/cgp-component/src/traits/) (`CanUseComponent` in `can_use_component.rs`, `IsProviderFor` in `is_provider.rs`, `DelegateComponent` in `delegate_component.rs`); the check-table macros are in [crates/macros/cgp-macro-core/src/types/check_components/](../../crates/macros/cgp-macro-core/src/types/check_components/) and [delegate_and_check_components/](../../crates/macros/cgp-macro-core/src/types/delegate_and_check_components/), with expansion snapshots in [crates/tests/cgp-tests/src/tests/check_components.rs](../../crates/tests/cgp-tests/src/tests/check_components.rs).
