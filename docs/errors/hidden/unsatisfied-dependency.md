# Unsatisfied dependency (hidden)

A provider is wired onto a context that cannot satisfy the provider's impl-side dependency, and the failure is triggered by calling the consumer-trait method directly — so the compiler reports only that the method's trait bounds are unsatisfied (`E0599`, or `E0277` when the consumer trait is used as a bound) and hides the dependency that actually failed.

## What triggers it

This class arises whenever [wiring](../../reference/macros/delegate_components.md) is lazy and the mistake is exercised through the consumer trait rather than a check. A provider carries an impl-side dependency in its `where` clause; a context is wired to that provider but does not meet the dependency; and because `delegate_components!` records the entry without verifying the provider's transitive requirements, the wiring is accepted. The failure appears only when the context's consumer-trait method is finally called.

```rust
#[cgp_component(Greeter)]
pub trait CanGreet {
    fn greet(&self);
}

#[cgp_auto_getter]
pub trait HasName {
    fn name(&self) -> &str;
}

#[cgp_impl(new GreetHello)]
impl Greeter
where
    Self: HasName, // impl-side dependency
{
    fn greet(&self) {
        let _ = self.name();
    }
}

#[derive(HasField)]
pub struct Person {
    pub age: u8, // no `name` field, so Person does not implement HasName
}

// Accepted even though Person cannot satisfy GreetHello's `Self: HasName`.
delegate_components! {
    Person {
        GreeterComponent: GreetHello,
    }
}

fn main() {
    Person { age: 0 }.greet(); // fails here, at the call site
}
```

The same hiding occurs, in a slightly different shape, when the consumer trait is used as a bound — `fn use_it<C: CanGreet>(c: &C)` called on a `Person` — rather than as a direct method call. Both exercise the broken wiring through the consumer trait, and both hide the cause; the difference is only the error code.

## The diagnostic

This is a **hidden** class: the compiler prints a short diagnostic that names the traits it could not satisfy but never descends to the dependency that failed. Calling the method directly produces an `E0599` — "the method `greet` exists for struct `Person`, but its trait bounds were not satisfied" — whose notes state that `Person` does not satisfy `Person: CanGreet` or `Person: Greeter<Person>` and that the trait `Greeter` must be implemented, and then stop. Using the consumer trait as a `where` bound instead produces an `E0277` reporting that `Person: CanGreet` is not satisfied, again without expanding why. In neither shape does the output mention `HasName` or the missing `HasField<Symbol!("name")>` — the real cause is absent, not merely buried.

The `E0599` form carries one more piece of misdirection worth naming so a reader discounts it. Alongside the unsatisfied-bound notes, the compiler adds "this is an associated function, not a method," reports `greet` under "found the following associated functions," and suggests rewriting the call as `Person::greet()`. This is an artifact of how `#[cgp_component]` lowers the method: the provider trait's `greet(context: &Context)` has no `self` receiver, so the method probe classifies it as an associated function. The suggestion is wrong — `Person::greet()` drops the required context argument — and it says nothing about the missing dependency; it is noise the class produces, not a clue.

The reason the cause is absent is a deliberate compiler heuristic, and it is the whole point of isolating this class. `#[cgp_component]` emits a blanket impl of the consumer trait for any context that is its own provider, so when the compiler checks `Person: CanGreet` it finds that blanket impl (alongside every other candidate impl in scope), determines it does not apply because `Person: Greeter<Person>` is unmet, and declines to expand the nested `where` bound that made it inapplicable. `rustc` does not, in general, report *which* `where`-clause bound of an inapplicable blanket impl failed — it reports only that the top-level trait is unimplemented and stops — because with a blanket impl and multiple candidates present, expanding every candidate's unmet sub-bound would usually be noise. This is a long-standing, acknowledged limitation of the `E0599` diagnostic, tracked upstream in [rust-lang/rust#61661](https://github.com/rust-lang/rust/issues/61661) and [#75222](https://github.com/rust-lang/rust/issues/75222), not a quirk of CGP. The suppression is exactly what CGP's [`IsProviderFor`](../../reference/traits/is_provider_for.md) supertrait exists to defeat — but only a check that requires `IsProviderFor` *directly* forces the compiler past the heuristic; the lazy consumer-trait path never does.

The same `E0599`/`E0277` shape, and the same absence of a cause, also arises when the wiring is not merely *unmet* but *missing* — a component never wired on the context, or an [unregistered namespace path](../checks/unregistered-namespace-path.md) whose redirect lands on an empty table slot. Whether a provider was found but its dependency failed, or no provider was found at all, the consumer-trait path hides it identically; the two are told apart only by promoting the error into a surfaced one.

## Where the root cause is

The root cause is **not present in the output.** A reader or tool must not scan this diagnostic for the failing dependency, because the compiler discarded it before printing. The most specific thing the output names is the unsatisfied provider trait (`Person: Greeter<Person>`) or consumer trait (`Person: CanGreet`); everything below that — the missing field, the unmet `HasName`, any deeper transitive bound — is gone. This absence is the defining property of the class and the reason it lives apart from the [surfaced classes](../checks/), whose diagnostics *do* carry the cause.

## Resolving it

The underlying fix is ordinary: give the context what the provider needs — add the `name` field to `Person`, or wire the getter component to an existing field — so `Person` satisfies `HasName` and `GreetHello` becomes a valid provider. The difficulty is not the fix but *finding* it, since the error points nowhere useful.

To make the cause visible, **promote the hidden error into a surfaced one** by forcing the check at the wiring site with [`check_components!`](../../reference/macros/check_components.md):

```rust
check_components! {
    Person {
        GreeterComponent,
    }
}
```

The check asserts `CanUseComponent<GreeterComponent>` for `Person`, which requires `GreetHello: IsProviderFor<GreeterComponent, Person>` as a *direct* bound. That defeats the suppression heuristic, and the compiler now reports the full `E0277` note chain ending at the missing `HasField<Symbol!("name")>`. That surfaced form is the [check-trait failure](../checks/check-trait-failure.md) class, and moving between the two is the standard technique the [debugging guide](../../guides/debugging.md) prescribes for a hidden dependency.

## Notes for tooling

A `cargo-cgp`-style post-processor **cannot extract the root cause from this diagnostic**, because the compiler never computed it into the printed output. Recognizing the class is straightforward: an `E0599` "method exists but its trait bounds were not satisfied" whose unmet bound is a provider trait of the form `Context: SomeProvider<Context>`, or an `E0277` on a consumer trait backed by a `…Component` blanket impl, with no note descending past that bound. Recovering the cause requires one of two moves. The tool can **promote and re-compile**: synthesize a `check_components!` (or an equivalent `CanUseComponent` assertion) for the failing component and compile that, then surface the resulting root cause in place of `rustc`'s unhelpful output. Or it can reach into compiler internals through `rustc_driver`, the way Clippy layers on the compiler, to inspect the fulfillment errors the trait solver produced but the diagnostic heuristic suppressed. Either way, the tool's job for this class is not to reformat the output but to *replace* it with the cause the output omits.

## Backing fixtures

- [acceptable/delegate_components/missing_dependency.rs](../../../crates/tests/cgp-compile-fail-tests/tests/acceptable/delegate_components/missing_dependency.rs) — `GreetHello` requires `Self: HasName`, `Person` lacks the `name` field, and the method is called directly; its `.stderr` pins the `E0599` shape that names `Person: Greeter<Person>` without descending to the missing field. Its checked counterpart, [acceptable/check_components/missing_dependency.rs](../../../crates/tests/cgp-compile-fail-tests/tests/acceptable/check_components/missing_dependency.rs), pins the surfaced `E0277` for the same mistake and belongs to the [check-trait failure](../checks/check-trait-failure.md) class.

## Related

- [Check-trait failure (surfaced)](../checks/check-trait-failure.md) — the same unmet dependency forced through a check, where the cause *is* reported; the two are the two halves of one phenomenon.
- [Debugging CGP compile errors](../../guides/debugging.md) — the playbook: read the error's shape, then move the error to the wiring site with a check to surface a hidden cause.
- [`IsProviderFor`](../../reference/traits/is_provider_for.md) and [`DelegateComponent`](../../reference/traits/delegate_component.md) — the traits every wiring error is ultimately about; `IsProviderFor` is the supertrait a check uses to defeat the suppression heuristic.
- [`check_components!`](../../reference/macros/check_components.md) and the [check-traits concept](../../concepts/check-traits.md) — why wiring is lazy and how a check forces a readable error at the wiring site.
