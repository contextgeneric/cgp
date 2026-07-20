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

## The raw diagnostic

This section describes what plain `cargo check` prints — the fallback when `cargo-cgp` is not on hand; [How cargo-cgp presents it](#how-cargo-cgp-presents-it) below covers the readable form. This is a **surfaced** class: the `help:` note names the concrete unmet leaf exactly as in a single-layer [check-trait failure](check-trait-failure.md), and the leaf tells you the offending field. What is specific to a higher-order provider is that the *rest* of the diagnostic differs by layer, so the two cases are told apart by shape rather than by the leaf alone.

When the **inner** layer fails, the compiler prints **two** `E0277` blocks. The first is an intermediate block reporting that the inner provider does not implement its provider trait for the concrete context — `BaseArea: AreaCalculator<Rectangle>` is not satisfied — carrying the [near-contradiction](../../guides/debugging.md) hint that `AreaCalculator<__Context__>` *is* implemented for `BaseArea` (the generic impl exists but is inapplicable here). The second block is the real one: `Rectangle: CanUseComponent<AreaCalculatorComponent>` is not satisfied, its `help:` naming the missing `HasField<Symbol!("base_area")>`, its `unsatisfied trait bound introduced here` caret landing on the *inner* provider's `Self: HasBaseArea` clause, and its `required for …` chain running through `BaseArea`'s `IsProviderFor` and then — after a `1 redundant requirement hidden` note — through `ScaledArea<BaseArea>`'s `IsProviderFor` to `CanUseComponent`. The chain passing through both providers' `IsProviderFor`, and the extra intermediate block, are the marks of an inner-layer failure.

When the **outer** layer fails, the compiler prints a **single**, shorter `E0277` block. It is the `Rectangle: CanUseComponent<AreaCalculatorComponent>` block, its `help:` naming the missing `HasField<Symbol!("scale_factor")>`, its caret landing on the *outer* provider's `Self: HasScaleFactor` clause, and its chain running from `ScaledArea<BaseArea>`'s `IsProviderFor` straight to `CanUseComponent` — with no `redundant requirement` note and no mention of the inner `BaseArea` at all. The outer layer fails before it ever delegates inward, so the inner provider never enters the proof.

## Where the root cause is

The root cause is **present** in the `help:` note of the `CanUseComponent` block in both cases, and the failing *layer* is present too, in two reinforcing places. The `unsatisfied trait bound introduced here` caret sits on the `where` clause of the provider whose dependency is unmet — the inner provider for an inner failure, the outer provider for an outer one — which is the most direct signal. The chain depth confirms it: an inner failure runs through *both* providers' `IsProviderFor` and carries the `1 redundant requirement hidden` note, while an outer failure stops at the outer provider's `IsProviderFor` and never names the inner one. The intermediate `Inner: ProviderTrait<Context>` block that appears only for the inner case is a third, weaker signal — its presence points inward, but it is also the noise a reader should skip past to reach the `CanUseComponent` block that carries the leaf.

## How cargo-cgp presents it

`cargo-cgp` reshapes both cases into a single `[CGP-E001]` headline — `[CGP-E001] the consumer trait \`CanCalculateArea\` is not implemented for context \`Rectangle\`` — over one `root cause:` tree, and **encodes the failing layer in the shape of that tree** rather than leaving it to be read from chain depth and caret position. For the inner failure the tree bottoms out at `[CGP-E106] missing field \`base_area\`` beneath *two* stacked `[CGP-E102]` provider-trait hops — first `ScaledArea<BaseArea>`, then the `BaseArea` it wraps — before reaching `[CGP-E105] HasBaseArea` and the field; the two provider hops are the wrapper relationship made explicit. For the outer failure the tree bottoms out at `[CGP-E106] missing field \`scale_factor\`` beneath a *single* `[CGP-E102]` hop (`ScaledArea<BaseArea>`) with the inner provider never named, exactly mirroring the shorter raw block. The intermediate near-contradiction block that clutters the raw inner-layer output is dropped entirely, so the recovered form reads the same in both cases save for that one-hop-versus-two-hop depth. The codes are defined in the [cargo-cgp error-code catalog](https://github.com/contextgeneric/cargo-cgp/blob/main/docs/error-code.md).

## Resolving it

Fix the dependency at the layer the diagnostic points to. For the inner failure, supply the field the inner provider needs — add `base_area` to `Rectangle` so `BaseArea` implements `HasBaseArea`; for the outer failure, supply the field the outer provider needs — add `scale_factor` so `ScaledArea` implements `HasScaleFactor`. Wiring is unaffected: the stack `ScaledArea<BaseArea>` is correct in both cases, and the fix is always to satisfy the named leaf, never to change the delegation.

When a stack is deep enough that reading the chain is awkward, force the layer boundary explicitly with the `#[check_providers(...)]` form of [`check_components!`](../../reference/macros/check_components.md). It changes the assertion from `CanUseComponent` on the context to `IsProviderFor` on each named provider, so a dependency missing only from the outer wrapper errors on that provider's line alone while one missing from the inner provider errors on both — pinpointing the layer without decoding the chain. The [debugging guide](../../guides/debugging.md) prescribes this move; this document is the anatomy behind it.

## Notes for tooling

`cargo-cgp` already does the work this class used to ask of a post-processor — it suppresses the intermediate near-contradiction block and reconstructs the wrapper relationship as the stacked `[CGP-E102]` hops of the tree. What it does *not* yet do is state the layer in words: the tree shows the wrapper structurally (two hops for an inner failure, one for an outer), but a reader still infers "inner versus outer" from the depth rather than reading a phrase like "needed by the inner provider `BaseArea`, wrapped by `ScaledArea`." A tool wanting to name the layer explicitly can emulate `#[check_providers(...)]`, re-asserting `IsProviderFor` per layer to localize it mechanically.

## Backing fixtures

- [`acceptable/providers/higher_order_inner_dependency.rs`](https://github.com/contextgeneric/cargo-cgp/blob/main/tests/ui/acceptable/providers/higher_order_inner_dependency.rs) — `ScaledArea<BaseArea>` wired onto a `Rectangle` with `scale_factor` but no `base_area`, so the inner layer fails; its `.rust.stderr` pins the two-block shape, the `HasField<Symbol!("base_area")>` leaf, the caret on `BaseArea`'s `Self: HasBaseArea`, and the chain through both providers' `IsProviderFor` with the `1 redundant requirement hidden` note, while its `.cgp.stderr` collapses that to one `[CGP-E001]` block whose `root cause: [CGP-E106] missing field \`base_area\`` tree carries *two* `[CGP-E102]` provider hops (`ScaledArea<BaseArea>` then `BaseArea`).
- [`acceptable/providers/higher_order_outer_dependency.rs`](https://github.com/contextgeneric/cargo-cgp/blob/main/tests/ui/acceptable/providers/higher_order_outer_dependency.rs) — the mirror, with `base_area` but no `scale_factor`, so the outer layer fails; its `.rust.stderr` pins the single, shorter block, the `HasField<Symbol!("scale_factor")>` leaf, the caret on `ScaledArea`'s `Self: HasScaleFactor`, and a chain that stops at the outer `IsProviderFor` without naming the inner provider, and its `.cgp.stderr` a `[CGP-E001]` block whose `root cause: [CGP-E106] missing field \`scale_factor\`` tree carries a *single* `[CGP-E102]` hop with the inner provider absent.

## Related

- [Check-trait failure (surfaced)](check-trait-failure.md) — the single-layer form of the same surfaced diagnostic; this class is what it looks like when the failing provider wraps another.
- [Verbose dependency cascade](verbose-cascade.md) — the sibling class whose intermediate provider-trait blocks are the same noise this class also emits for an inner failure.
- [`check_components!`](../../reference/macros/check_components.md) and its `#[check_providers(...)]` form — the macro this class is expressed through and the tool that localizes the layer by hand.
- [`IsProviderFor`](../../reference/traits/is_provider_for.md) and [`#[use_provider]`](../../reference/attributes/use_provider.md) — the supertrait that carries each layer's `where` clause into the chain, and the attribute that completes the inner provider's bound.
- [Higher-order providers](../../concepts/higher-order-providers.md) and [Debugging CGP compile errors](../../guides/debugging.md) — the concept this class fails in, and the playbook that prescribes `#[check_providers]`.
