# Wiring cycle

A delegation chases its own tail — a component wired to a provider whose resolution routes back to that same component — so when the wiring is forced through a check the trait solver overflows with `E0275`.

## What triggers it

The classic cycle is delegating a component to [`UseContext`](../../reference/providers/use_context.md) when the context's only implementation of that component *is* that delegation. `UseContext` implements the provider trait by routing back through the context's own consumer-trait impl, but that consumer impl exists only via this delegation to `UseContext`, so resolving the provider trait requires resolving the consumer trait, which requires the provider trait again.

```rust
#[cgp_component(Greeter)]
pub trait CanGreet {
    fn greet(&self);
}

#[derive(HasField)]
pub struct Person { pub name: String }

// Person's only source of CanGreet is this delegation to UseContext, which
// resolves back to CanGreet — a cycle with no terminating provider.
delegate_components! {
    Person {
        GreeterComponent: UseContext,
    }
}
```

CGP lowers the wiring faithfully and cannot see that the delegation is self-referential without a whole-program view, so it accepts the entry and defers the failure to the compiler. A mutual cycle between two components that each delegate through the other fails the same way.

## The diagnostic

How the cycle surfaces depends on how it is exercised, and the two shapes are very different. Forcing the wiring through a [`check_components!`](../../reference/macros/check_components.md) drives the solver directly into the loop, and it overflows with **`E0275`** — "overflow evaluating the requirement `Person: IsProviderFor<GreeterComponent, Person>`" — followed by a note chain that *names the cycle*: `Person` to implement `Greeter<Person>`, then `CanGreet`, then `UseContext` to implement `IsProviderFor<GreeterComponent, Person>`, then back into `CanUseComponent<GreeterComponent>`. The loop is visible in the chain.

Exercised instead by a plain method call on the context, the same cycle does *not* overflow: the method probe treats the unresolvable requirement as simply unsatisfied and reports the [hidden `E0599`](../hidden/unsatisfied-dependency.md) — "method exists but its trait bounds were not satisfied" — with no hint that a cycle is the reason. So a wiring cycle is `E0275` when checked and a hidden `E0599` when called, and only the checked form actually reveals the cause.

The [`E0275`](../error_codes/e0275.md) itself is ordinary `rustc` behavior. The trait solver follows a requirement's supporting bounds only to a bounded depth — the [default `recursion_limit` is 128](https://doc.rust-lang.org/reference/attributes/limits.html) — and raises `E0275` when it exceeds that depth without resolving, appending its standard `help: consider increasing the recursion limit`. That advice suits a merely *deep* obligation, but a true cycle never terminates at any depth, so the limit here is only ever hit, never cleared; the suggestion does not apply and should be read past. This is why the cycle must be exercised through a check to surface at all: the overflow is computed only when the solver is driven into the loop, and `#[cgp_component]`'s blanket impl otherwise lets the method probe abandon the requirement as unsatisfied without ever entering it.

## Where the root cause is

The cause — the cycle — is **present in the checked (`E0275`) form** and readable from the note chain: the requirement that overflows and the intervening `Greeter<Person>` / `CanGreet` / `UseContext` notes together trace the loop, so the participants (the component, the consumer trait, and `UseContext`) are all named. What the diagnostic does *not* state is the remedy. In the hidden (`E0599`) form the cause is absent, exactly as for any [hidden unsatisfied dependency](../hidden/unsatisfied-dependency.md) — which is itself a reason to reach for a check when a `UseContext` wiring is suspect.

## Resolving it

Break the cycle by wiring the component to a concrete provider that terminates the lookup rather than routing back to the context. `UseContext` is only safe as the *inner* provider of a higher-order provider or where the context supplies the component through some other impl; delegating a component's sole implementation to `UseContext` is always a cycle. When the overflow is genuinely a depth problem rather than a true cycle, raising `#![recursion_limit = "…"]` is the wrong fix here — the requirement never terminates, so no limit is high enough.

## Notes for tooling

A `cargo-cgp`-style post-processor should **detect the repeating requirement** — the same component/provider pair recurring in the `required for` chain — and report it as a *cycle* ("`GreeterComponent` on `Person` delegates to `UseContext`, which routes back to `GreeterComponent`") rather than surfacing the raw overflow depth, which tells the user nothing. Because the same mistake hides as `E0599` when called, a tool that already promotes hidden errors by synthesizing a check (see the [hidden class](../hidden/unsatisfied-dependency.md)) will convert this cycle from its hidden `E0599` form into the `E0275` form whose chain it can then analyze for the loop.

## Backing fixtures

- [acceptable/delegate_components/use_context_cycle.rs](../../../crates/tests/cgp-compile-fail-tests/tests/acceptable/delegate_components/use_context_cycle.rs) — a component wired to `UseContext` with no terminating provider, forced through a `check_components!`; its `.stderr` pins the `E0275` overflow and the note chain that names the `Greeter<Person>` → `CanGreet` → `UseContext` loop.

## Related

- [Unsatisfied dependency (hidden)](../hidden/unsatisfied-dependency.md) — the `E0599` shape the same cycle takes when exercised by a method call instead of a check.
- [Namespace inheritance cycle](namespace-inheritance-cycle.md) — the sibling `E0275` cycle through circular namespace inheritance rather than `UseContext` delegation; that one is caught *eagerly* at the namespace definitions, where this one is lazy.
- [Conflicting wiring](conflicting-wiring.md), [Orphan-rule violation](orphan-rule.md), [Unconstrained generic](unconstrained-generic.md) — the sibling structural classes.
- [`UseContext`](../../reference/providers/use_context.md) — the provider whose misuse most often causes the cycle.
- [Debugging CGP compile errors](../../guides/debugging.md) — the `E0275` entry in the decoder.
