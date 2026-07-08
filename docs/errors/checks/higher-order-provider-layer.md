# Higher-order provider layer failure (surfaced)

A checked higher-order provider whose dependency is unmet surfaces the real leaf bound (`E0277`) like any [check-trait failure](check-trait-failure.md), but the diagnostic's shape also tells you *which layer* of the provider stack failed — and reading that shape is the difference between fixing the wrapper and fixing what it wraps.

## What triggers it

This class arises when a [higher-order provider](../../concepts/higher-order-providers.md) — a provider parameterized by another provider — is wired onto a context that cannot meet a dependency in one of its layers, and the wiring is checked. The outer provider carries its own impl-side dependency, and the inner provider it wraps carries a different one; the context may satisfy either, both, or neither. When exactly one layer's dependency is unmet, the check still fails, and the question a reader must answer is which layer to fix.

```rust
#[cgp_impl(new BaseArea)]
impl AreaCalculator
where
    Self: HasBaseArea, // the INNER layer's dependency
{ /* … */ }

#[cgp_impl(new ScaledArea<Inner>)]
#[use_provider(Inner: AreaCalculator)]
impl<Inner> AreaCalculator
where
    Self: HasScaleFactor, // the OUTER layer's dependency
{ /* … */ }

#[derive(HasField)]
pub struct Rectangle {
    pub scale_factor: f64, // present: the outer layer is satisfied
    // no `base_area`:      the inner layer is not
}

delegate_components! {
    Rectangle { AreaCalculatorComponent: ScaledArea<BaseArea> }
}

check_components! {
    Rectangle { AreaCalculatorComponent }
}
```

## The diagnostic

This is a **surfaced** class: the `help:` note names the concrete unmet leaf exactly as in a single-layer [check-trait failure](check-trait-failure.md), and the leaf tells you the offending field. What is specific to a higher-order provider is that the *rest* of the diagnostic differs by layer, so the two cases are told apart by shape rather than by the leaf alone.

When the **inner** layer fails, the compiler prints **two** `E0277` blocks. The first is an intermediate block reporting that the inner provider does not implement its provider trait for the concrete context — `BaseArea: AreaCalculator<Rectangle>` is not satisfied — carrying the [near-contradiction](../../guides/debugging.md) hint that `AreaCalculator<__Context__>` *is* implemented for `BaseArea` (the generic impl exists but is inapplicable here). The second block is the real one: `Rectangle: CanUseComponent<AreaCalculatorComponent>` is not satisfied, its `help:` naming the missing `HasField<Symbol!("base_area")>`, its `unsatisfied trait bound introduced here` caret landing on the *inner* provider's `Self: HasBaseArea` clause, and its `required for …` chain running through `BaseArea`'s `IsProviderFor` and then — after a `1 redundant requirement hidden` note — through `ScaledArea<BaseArea>`'s `IsProviderFor` to `CanUseComponent`. The chain passing through both providers' `IsProviderFor`, and the extra intermediate block, are the marks of an inner-layer failure.

When the **outer** layer fails, the compiler prints a **single**, shorter `E0277` block. It is the `Rectangle: CanUseComponent<AreaCalculatorComponent>` block, its `help:` naming the missing `HasField<Symbol!("scale_factor")>`, its caret landing on the *outer* provider's `Self: HasScaleFactor` clause, and its chain running from `ScaledArea<BaseArea>`'s `IsProviderFor` straight to `CanUseComponent` — with no `redundant requirement` note and no mention of the inner `BaseArea` at all. The outer layer fails before it ever delegates inward, so the inner provider never enters the proof.

## Where the root cause is

The root cause is **present** in the `help:` note of the `CanUseComponent` block in both cases, and the failing *layer* is present too, in two reinforcing places. The `unsatisfied trait bound introduced here` caret sits on the `where` clause of the provider whose dependency is unmet — the inner provider for an inner failure, the outer provider for an outer one — which is the most direct signal. The chain depth confirms it: an inner failure runs through *both* providers' `IsProviderFor` and carries the `1 redundant requirement hidden` note, while an outer failure stops at the outer provider's `IsProviderFor` and never names the inner one. The intermediate `Inner: ProviderTrait<Context>` block that appears only for the inner case is a third, weaker signal — its presence points inward, but it is also the noise a reader should skip past to reach the `CanUseComponent` block that carries the leaf.

## Resolving it

Fix the dependency at the layer the diagnostic points to. For the inner failure, supply the field the inner provider needs — add `base_area` to `Rectangle` so `BaseArea` implements `HasBaseArea`; for the outer failure, supply the field the outer provider needs — add `scale_factor` so `ScaledArea` implements `HasScaleFactor`. Wiring is unaffected: the stack `ScaledArea<BaseArea>` is correct in both cases, and the fix is always to satisfy the named leaf, never to change the delegation.

When a stack is deep enough that reading the chain is awkward, force the layer boundary explicitly with the `#[check_providers(...)]` form of [`check_components!`](../../reference/macros/check_components.md). It changes the assertion from `CanUseComponent` on the context to `IsProviderFor` on each named provider, so a dependency missing only from the outer wrapper errors on that provider's line alone while one missing from the inner provider errors on both — pinpointing the layer without decoding the chain. The [debugging guide](../../guides/debugging.md) prescribes this move; this document is the anatomy behind it.

## Notes for tooling

For a `cargo-cgp`-style post-processor, the headline is again the leaf in the `CanUseComponent` block's `help:` note, but this class adds a second fact worth reporting: **the provider layer at fault**, recoverable as the provider named in the innermost `IsProviderFor` note whose `where` clause the `introduced here` caret sits on. A tool should present the two together — "`Rectangle` is missing field `base_area`, needed by the inner provider `BaseArea` (wrapped by `ScaledArea`)" — reconstructing the wrapper relationship from the `IsProviderFor` chain rather than dumping it. The intermediate `BaseArea: AreaCalculator<Rectangle>` near-contradiction block should be suppressed as derived noise, exactly as in a [verbose cascade](verbose-cascade.md); it names only a provider trait and never reaches the field. Emulating `#[check_providers(...)]` — re-asserting `IsProviderFor` per layer — is the internal move a tool can make to localize the layer mechanically instead of parsing the chain.

## Backing fixtures

- [acceptable/check_components/higher_order_inner_dependency.rs](../../../crates/tests/cgp-compile-fail-tests/tests/acceptable/check_components/higher_order_inner_dependency.rs) — `ScaledArea<BaseArea>` wired onto a `Rectangle` with `scale_factor` but no `base_area`, so the inner layer fails; its `.stderr` pins the two-block shape, the `HasField<Symbol!("base_area")>` leaf, the caret on `BaseArea`'s `Self: HasBaseArea`, and the chain through both providers' `IsProviderFor` with the `1 redundant requirement hidden` note.
- [acceptable/check_components/higher_order_outer_dependency.rs](../../../crates/tests/cgp-compile-fail-tests/tests/acceptable/check_components/higher_order_outer_dependency.rs) — the mirror, with `base_area` but no `scale_factor`, so the outer layer fails; its `.stderr` pins the single, shorter block, the `HasField<Symbol!("scale_factor")>` leaf, the caret on `ScaledArea`'s `Self: HasScaleFactor`, and a chain that stops at the outer `IsProviderFor` without naming the inner provider.

## Related

- [Check-trait failure (surfaced)](check-trait-failure.md) — the single-layer form of the same surfaced diagnostic; this class is what it looks like when the failing provider wraps another.
- [Verbose dependency cascade](verbose-cascade.md) — the sibling class whose intermediate provider-trait blocks are the same noise this class also emits for an inner failure.
- [`check_components!`](../../reference/macros/check_components.md) and its `#[check_providers(...)]` form — the macro this class is expressed through and the tool that localizes the layer by hand.
- [`IsProviderFor`](../../reference/traits/is_provider_for.md) and [`#[use_provider]`](../../reference/attributes/use_provider.md) — the supertrait that carries each layer's `where` clause into the chain, and the attribute that completes the inner provider's bound.
- [Higher-order providers](../../concepts/higher-order-providers.md) and [Debugging CGP compile errors](../../guides/debugging.md) — the concept this class fails in, and the playbook that prescribes `#[check_providers]`.
