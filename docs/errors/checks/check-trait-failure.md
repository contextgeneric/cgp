# Check-trait failure (surfaced)

A check forces an unsatisfied impl-side dependency through `IsProviderFor`, so the compiler names the real missing bound (`E0277`) at the wiring site — the surfaced counterpart of the [hidden unsatisfied dependency](../hidden/unsatisfied-dependency.md), produced from the very same mistake.

## What triggers it

This class arises from exactly the mistake behind the [hidden class](../hidden/unsatisfied-dependency.md) — a provider wired onto a context that cannot meet the provider's impl-side dependency — but exercised through a [`check_components!`](../../reference/macros/check_components.md) assertion (or the fused `delegate_and_check_components!`) instead of a direct method call. The check is what changes the outcome: it asserts `CanUseComponent` for each listed component, and that assertion requires `IsProviderFor` as a *direct* bound, which forces the compiler to evaluate the provider's `where` clause rather than suppressing it.

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
    pub age: u8, // no `name` field
}

delegate_components! {
    Person {
        GreeterComponent: GreetHello,
    }
}

// Forces the failure here, at the wiring, instead of at a later call site.
check_components! {
    Person {
        GreeterComponent,
    }
}
```

A `check_components!` is the canonical way to force this diagnostic, but it is not the only one: any *direct* obligation on a capability bound produces the same shape. A [`#[use_type]`](../../reference/attributes/use_type.md) foreign import (`HasScalarType.Scalar in Types`) puts the capability bound `Types: HasScalarType` — grounded to `<Self as HasTypes>::Types: HasScalarType` for a nested import — onto the *generated trait itself*, so asserting or using that trait for a context whose supplied type does not implement the capability surfaces the identical `E0277` with a capability leaf, without any `check_components!`. Here the `required for …` chain runs through the trait's own `where` bound (`required by a bound in CanCalculateArea` / `GetScalar`) rather than through `CanUseComponent`, but the leaf, the `DelegateComponent`/`IsProviderFor` `help:`, and the position of the cause are the same.

## The diagnostic

This is a **surfaced** class: the compiler prints an `E0277` that names the concrete missing bound, unlike the hidden class that omits it. The primary error reports that `Person: CanUseComponent<GreeterComponent>` is not satisfied, and its caret lands on the `GreeterComponent` entry *inside the `check_components!` block* — not on the `Person` context type — because the check re-spans the shared context token onto each listed component in turn. Immediately below, a `help:` note gives the actual unmet leaf bound: that `HasField<Symbol!("name")>` is not implemented for `Person`, "but trait `HasField<Symbol!("age")>` is implemented for it." That second half is a useful landmark — the compiler is pointing at the *nearest existing* field impl, which tells you the context has a field, just not the one the provider expects.

Below the `help:` note, a `required for …` chain traces the dependency path outward from the leaf: `Person` to implement `HasName`, then `GreetHello` to implement `IsProviderFor<GreeterComponent, Person>`, then `Person` to implement `CanUseComponent<GreeterComponent>`, and finally the bound in the generated `__CheckPerson` trait that the `check_components!` block emitted. The chain is the scaffolding; the leaf in the `help:` note is the cause.

What makes the cause visible is that the check produces a *direct* trait obligation, and this is where the class differs mechanically from its hidden twin. A `check_components!` asserts `Person: CanUseComponent<GreeterComponent>` as a bound on the generated `__CheckPerson` trait, so the solver must prove that bound outright — and proving it means discharging the whole `where`-clause chain down to the leaf and reporting the first bound that cannot be met. The `required for …` notes are simply `rustc`'s ordinary `E0277` obligation-tracing output for that proof. It is exactly the path the [hidden class](../hidden/unsatisfied-dependency.md) never takes: a method call lets the solver abandon an inapplicable blanket impl at the top instead of proving a direct bound, so it never descends to the leaf. [`IsProviderFor`](../../reference/traits/is_provider_for.md) is the supertrait that carries the provider's own `where` clause into this chain, which is why the leaf is named here and suppressed there. The `help:` note's "but trait `HasField<Symbol!("age")>` is implemented for it" is a second piece of standard machinery — `rustc`'s "a similar impl exists" hint, pointing at the nearest impl of the same trait to show the context has *a* field, just not the expected one.

## Where the root cause is

The root cause is **present**, and it is near the top — in the compiler's `help:` note, not at the end of the output. This is the opposite of what a reader might expect from a long note chain: the concrete unmet bound (`HasField<Symbol!("name")>`) is stated early, and the `required for …` notes that follow build *outward* from it toward the check trait, rather than drilling down to it. The caret's position is the other half of the value — it sits on the wiring entry the user controls, so the error points at the fix site rather than at a distant call. When many providers depend transitively on one leaf, the output multiplies into a [verbose cascade](verbose-cascade.md), and the position guidance shifts to *which block* carries the actionable cause; for a single checked component it is simply the `help:` note.

## When the derive is missing entirely

A distinct sub-case of this class is worth recognizing because its fix differs and its diagnostic drops the usual landmark: the mistake is not a missing *field* but a missing `#[derive(HasField)]` on the context altogether. When the struct has the field the getter names but no derive, it has *no* `HasField` impls at all, so the getter trait is still unsatisfiable and the check still fails with the same `CanUseComponent` / `IsProviderFor` / `HasField` shape. The tell is what is *absent*: the `help:` note names the missing `HasField<Symbol!("name")>` and points at the `struct` definition, but there is **no** "but trait `HasField<…>` is implemented for it" line, because the context implements the trait for no field at all. The near-impl hint that a single missing field always produces cannot appear when every field is missing.

Read the absence as its own signal: a checked context that implements `HasField` for nothing behind a `#[derive(HasField)]`-shaped requirement has most likely forgotten the derive, and the fix is to add `#[derive(HasField)]` to the struct, not to add fields one at a time. A tool handling this class should special-case it — zero `HasField` impls plus an unmet `HasField` leaf means the derive, and the headline should say so rather than sending the user after a single field.

## Resolving it

The fix is what the diagnostic already points to: satisfy the named leaf bound. Add the `name` field to `Person`, or wire the getter component to the existing field, so `Person` implements `HasName` and `GreetHello` becomes a valid provider for the component. Because the check surfaced both the concrete bound *and* the wiring entry, no further tracing is usually needed — which is exactly why the standard remedy for a [hidden](../hidden/unsatisfied-dependency.md) failure is to add a check and read this class instead.

## Notes for tooling

For a `cargo-cgp`-style post-processor, the fact to extract as the headline is the **leaf bound in the `help:` note** — the `HasField`, abstract type, or capability that is genuinely unimplemented — together with the caret's component entry. The `CanUseComponent` / `IsProviderFor` / `__Check…` frames in the `required for` chain are CGP scaffolding a user did not write and should be suppressed or collapsed into a short dependency path. This class is the *target* a tool aims for when handling the hidden class: promoting a hidden failure by synthesizing a check produces precisely this diagnostic, so a tool can normalize both to the same headline.

## Backing fixtures

- [acceptable/check_components/missing_dependency.rs](../../../crates/tests/cgp-compile-fail-tests/tests/acceptable/check_components/missing_dependency.rs) — the surfaced `E0277` for `GreetHello`'s unmet `Self: HasName`, whose `.stderr` pins the `help:` note naming `HasField<Symbol!("name")>` and the caret landing on `GreeterComponent` inside the block. Its unchecked counterpart, [acceptable/delegate_components/missing_dependency.rs](../../../crates/tests/cgp-compile-fail-tests/tests/acceptable/delegate_components/missing_dependency.rs), pins the [hidden](../hidden/unsatisfied-dependency.md) `E0599` for the same mistake. The fused `delegate_and_check_components!` form produces the same surfaced shape.
- [acceptable/cgp_component/use_type_foreign_unsatisfied.rs](../../../crates/tests/cgp-compile-fail-tests/tests/acceptable/cgp_component/use_type_foreign_unsatisfied.rs) — the capability bound reached through a `#[use_type]` foreign import instead of a check: naming a component for a `Types` that does not implement the imported `HasScalarType` surfaces `E0277` on `NoScalar: HasScalarType`, pinning that the foreign bound is *enforced on the generated trait* rather than silently dropped.
- [acceptable/cgp_fn/use_type_nested_unsatisfied.rs](../../../crates/tests/cgp-compile-fail-tests/tests/acceptable/cgp_fn/use_type_nested_unsatisfied.rs) — the same, but through a *nested* two-hop import, so the grounded bound `<Self as HasTypes>::Types: HasScalarType` is the one enforced; its `.stderr` pins the `required by this bound in GetScalar` note at the `HasScalarType.Scalar in Types` attribute, confirming the transitively-grounded foreign bound is checked at depth.
- [acceptable/check_components/missing_has_field_derive.rs](../../../crates/tests/cgp-compile-fail-tests/tests/acceptable/check_components/missing_has_field_derive.rs) — the [derive-missing variant](#when-the-derive-is-missing-entirely): a `Person` that has the `name` field but no `#[derive(HasField)]`, so the check fails with the same shape but *without* the "but trait `HasField<…>` is implemented" landmark, since `Person` implements the trait for no field. Its `.stderr` pins that absent landmark as the signal the whole derive is missing.

## Related

- [Unsatisfied dependency (hidden)](../hidden/unsatisfied-dependency.md) — the hidden counterpart; the two are the two halves of one phenomenon, and promoting the hidden one yields this class.
- [Unsatisfied ordinary trait bound (surfaced)](ordinary-trait-bound.md) — the sibling surfaced class whose leaf is an ordinary Rust trait (`Eq`, `Clone`) on a concrete type rather than a CGP capability like `HasField`; the leaf's kind changes the `help:` note and the position of the cause.
- [Verbose dependency cascade](verbose-cascade.md) — this diagnostic multiplied when many providers depend transitively on one leaf.
- [`check_components!`](../../reference/macros/check_components.md), [`CanUseComponent`](../../reference/traits/can_use_component.md), and [`IsProviderFor`](../../reference/traits/is_provider_for.md) — the macro and traits this class is expressed through.
- [Debugging CGP compile errors](../../guides/debugging.md) and the [check-traits concept](../../concepts/check-traits.md) — why the check moves the error here and how to read it.
