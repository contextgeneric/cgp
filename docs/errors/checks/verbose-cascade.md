# Verbose dependency cascade

One mistake deep in a dependency graph surfaces at every provider that transitively needs it, so the compiler prints far more error blocks than there are mistakes — and the reader's task is to find the one root cause among the repeats.

## What triggers it

This class is the *volume* form of a [surfaced check-trait failure](check-trait-failure.md): it arises whenever several components share a transitive dependency and that dependency is unmet. A chain of providers — `ProvideBaz` depends on `CanBar`, `ProvideBar` on `CanFoo`, `ProvideFoo` on a `name` field — is wired onto a context missing that field, and all three components are checked together.

```rust
#[cgp_impl(new ProvideFoo)]
impl Foo where Self: HasName { /* needs the `name` field */ }

#[cgp_impl(new ProvideBar)]
#[uses(CanFoo)]              // depends on Foo
impl Bar { /* … */ }

#[cgp_impl(new ProvideBaz)]
#[uses(CanBar)]             // depends on Bar
impl Baz { /* … */ }

#[derive(HasField)]
pub struct App { pub age: u8 } // no `name` field

delegate_components! {
    App { FooComponent: ProvideFoo, BarComponent: ProvideBar, BazComponent: ProvideBaz }
}

check_components! {
    App { BazComponent, BarComponent, FooComponent }
}
```

## The raw diagnostic

This section describes what plain `cargo check` prints — the fallback when `cargo-cgp` is not on hand; [How cargo-cgp presents it](#how-cargo-cgp-presents-it) below covers the reshaped form. The output is a cascade whose block count reflects the depth of the graph, not the number of mistakes. Checking three chained components over one missing field produces **six** `E0277` blocks, not three, and they are of two kinds. Each checked component yields a `CanUseComponent<…Component>` block that *does* reach the concrete root cause — its `help:` note names `HasField<Symbol!("name")>` as not implemented for `App` (and points at the nearest existing field, `HasField<Symbol!("age")>`, as a landmark). But the deeper components additionally emit **intermediate** blocks — `ProvideBar: Bar<App>` is not satisfied, `ProvideFoo: Foo<App>` is not satisfied — that name only an inner provider trait and never descend to the field. The intermediate blocks are the noise; the `CanUseComponent` blocks are the signal.

Two pieces of `rustc` machinery shape the cascade and are worth recognizing so they are not misread. Each intermediate block carries the [near-contradiction](../../guides/debugging.md) shape — `help: the trait Bar<App> is not implemented for ProvideBar` immediately followed by `help: the trait Bar<__Context__> is implemented for ProvideBar` — which is `rustc`'s "a similar impl exists" hint reporting that the *generic* provider impl (over `__Context__`) exists but does not hold for the concrete `App`, because a bound deeper in its chain fails. It does not mean the impl is both present and absent; it means the impl is present but inapplicable here. And the longer chains end with `= note: N redundant requirements hidden`, `rustc` collapsing repeated identical obligations it has already printed, so the elision is a sign the chain revisits the same bound, not that a distinct cause was omitted.

The hidden analogue exists too: exercising the same chain through direct consumer-trait *calls* rather than a check produces one `E0599` per call, none of which reaches the cause — the [hidden](../hidden/unsatisfied-dependency.md) failure, multiplied. This document covers the surfaced (checked) cascade, where the cause is present but buried in volume.

## Where the root cause is

The root cause is **present and repeated** — the same `HasField<Symbol!("name")>` appears in every `CanUseComponent` block's `help:` note. The reliable way to find it is not positional but structural: **look for a block whose `help:` names a concrete missing item — a `HasField`, an abstract type — rather than a provider trait like `ProvideFoo: Foo<App>`.** Those concrete-item blocks all point at the one thing to fix; the provider-trait blocks are intermediate consequences. When components are checked top-down (foundational last, as in the fixture), the final block is a clean concrete-item block, which is why the root cause is *often* found near the last or second-to-last message — but the concrete-item test is what makes it reliable regardless of order.

## How cargo-cgp presents it

`cargo-cgp` collapses the whole cascade to a single block. First it **drops every intermediate provider-trait block** as derived noise and recovers each *checked* component's failure to a `root cause:` tree that descends all the way to the leaf. Then it **coalesces** the checked components that share that leaf: rather than one block per component, it emits one `[CGP-E001]` headline naming every affected consumer trait — `` the consumer traits `CanBaz`, `CanBar`, and `CanFoo` are not implemented for context `App` `` — with a caret at each check entry and a single `root cause: [CGP-E106] missing field \`name\`` tree beneath. So a six-block raw cascade over three checked components becomes one block, and cargo's re-count reports one error. A component rustc happened to surface provider-side (a lone `[CGP-E002]` block) is worded uniformly as a consumer in the merged headline, since a `check_components!` entry failing *is* the consumer trait failing.

Listing every affected consumer in one headline is possible because the tool holds the compilation's diagnostics until they have all arrived (there is no end-of-compilation hook), then groups together the ones that *share* a root cause. Grouping on a shared cause rather than on a whole-failure key is what also reaches the harder shape, where one omission is reached at several instantiations and so surfaces as several root causes: each check entry stops at the first unmet leaf on its own branch while a use-site call reaches them all, so no two of those cause sets are equal and grouping on equality left one mistake reported as several blocks. The codes are defined in the [cargo-cgp error-code catalog](https://github.com/contextgeneric/cargo-cgp/blob/main/docs/error-code.md).

## Resolving it

Fix the single root cause and the entire cascade collapses at once — supply the `name` field, and all six blocks disappear together. The difficulty is isolation, not repair, so the [debugging guide](../../guides/debugging.md) prescribes two moves: **check one suspect component in isolation** in its own `check_components!` block, which strips the cascade down to that component's chain, or **bisect** a large table by commenting entries out until the failure vanishes. Both converge on the one broken link far faster than reading the cascade top to bottom.

## Notes for tooling

`cargo-cgp` reshapes this class in full, as [How cargo-cgp presents it](#how-cargo-cgp-presents-it) describes: it drops the intermediate provider-trait blocks, recovers each checked component's innermost concrete cause, and coalesces the components that share one leaf into a single headline that lists every affected consumer over one root-cause tree. The coalescing keys on the root causes themselves, consumer-independently and one cause at a time, so it generalizes to any set of checked components bottoming out on the same root cause — not just a field, and not only components whose causes match exactly, since failures whose cause sets merely overlap are grouped too.

## Backing fixtures

- [`acceptable/duplication/dependency_cascade.rs`](https://github.com/contextgeneric/cargo-cgp/blob/main/tests/ui/acceptable/duplication/dependency_cascade.rs) — a three-deep provider chain over one missing `name` field, checked across all three components; its `.rust.stderr` pins the six-block raw cascade (the concrete `HasField<Symbol!("name")>` in each `CanUseComponent` block's `help:` note, and the intermediate `ProvideFoo: Foo<App>` / `ProvideBar: Bar<App>` noise blocks), while its `.cgp.stderr` shows the single coalesced block — `` the consumer traits `CanBaz`, `CanBar`, and `CanFoo` `` over one `[CGP-E106] missing field \`name\`` tree, with a caret at each entry.
- [`acceptable/duplication/density_3.rs`](https://github.com/contextgeneric/cargo-cgp/blob/main/tests/ui/acceptable/duplication/density_3.rs) — a denser two-component check over one missing `height` field where the second component (`CanCalculateDensity`) depends transitively on the first (`CanCalculateArea`); its `.cgp.stderr` pins the two components coalesced into one `[CGP-E001]` headline over the shared `[CGP-E106]` leaf.

## Related

- [Check-trait failure (surfaced)](check-trait-failure.md) — the single-cause diagnostic this class multiplies.
- [Higher-order provider layer failure (surfaced)](higher-order-provider-layer.md) — a sibling class that emits the same intermediate provider-trait noise blocks, there to distinguish an inner-layer failure from an outer one.
- [Unsatisfied dependency (hidden)](../hidden/unsatisfied-dependency.md) — the same root cause seen through consumer-trait calls, where each block hides the cause instead of surfacing it.
- [Debugging CGP compile errors](../../guides/debugging.md) — why the error count tells you nothing about the number of mistakes, and how to isolate the one link.
- [`check_components!`](../../reference/macros/check_components.md) and [`IsProviderFor`](../../reference/traits/is_provider_for.md).
