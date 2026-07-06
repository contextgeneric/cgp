# Unresolved imported abstract type

A `#[use_type]` import names an associated type the owning trait does not declare, so the macro rewrites the bare alias into a `<Self as Trait>::WrongName` path that resolves to nothing and the compiler rejects it with `E0576`.

## What triggers it

This class is a *lowering* failure, not a wiring one: the mistake is a misnamed associated type in the `#[use_type]` attribute itself, which the macro cannot validate at expansion time. `#[use_type]` works by textual substitution — it rewrites every bare use of an imported alias into `<Context as Trait>::Assoc` — and it has no knowledge of which associated types the trait actually declares. So an alias that names a nonexistent associated type is lowered faithfully into a qualified path, and only the compiler, resolving that path, discovers the name is not there.

The canonical case is a typo in the imported type name:

```rust
#[cgp_type]
pub trait HasScalarType {
    type Scalar;
}

#[cgp_fn]
#[use_type(HasScalarType.Scalr)] // typo: the trait declares `Scalar`, not `Scalr`
pub fn get_scalar(&self) -> Scalr {
    todo!()
}
```

The import declares the alias `Scalr`, so the bare `Scalr` in the return type is rewritten to `<Self as HasScalarType>::Scalr`, which names an associated type `HasScalarType` never declares. The same failure arises from a stale name after a trait rename, or from importing an item that lives on a different trait than the one named. CGP is working as designed here: it cannot see the trait's item list during expansion, so it lowers the name literally and defers to the compiler — an [acceptable failure](../../implementation/AGENTS.md), not a defect.

## The diagnostic

This is a **surfaced** class with the simplest diagnostic in the catalog: a single [`E0576`](../error_codes/e0576.md), "cannot find associated type `Scalr` in trait `HasScalarType`", with the caret on the offending name. Because the substitution copies the *span* of the identifier the user wrote onto the rewritten path, the caret lands on the `Scalr` in the user's signature — the token they actually typed — rather than on the `#[use_type]` attribute or the whole macro block. When a similarly named item exists, `rustc` adds a "similarly named associated type `Scalar` defined here" note pointing at the trait's real declaration and a `help:` that suggests the correct spelling inline. There is no note chain, no CGP scaffolding, and no `IsProviderFor`/`DelegateComponent` frame, because the failure is caught during name resolution, before any bound is evaluated.

## Where the root cause is

The root cause is **present and is the entire diagnostic** — the primary `E0576` names both the missing associated type and the trait it was sought in, and the caret points at the exact token to change. This is the opposite of the hidden and cascading classes: nothing is suppressed, nothing is buried, and there is no verbosity to wade through. The `help:` note, when present, even supplies the fix. The only CGP-specific fact the message does not state is that the name came from a `#[use_type]` import — but the caret sits on the alias in the signature, which is the same name written in the attribute, so following it to the attribute is immediate.

## Resolving it

Correct the imported name so it matches an associated type the trait declares — change `#[use_type(HasScalarType.Scalr)]` and every use of the alias to `Scalar`, or use an `as` clause (`#[use_type(HasScalarType.{Scalar as Scalr})]`) if the short local name was intended. The `help:` note usually names the correct spelling outright. If the type genuinely lives on a different trait, point the import at that trait instead. Because the diagnostic is a plain resolution error at the user's own token, no CGP-specific tracing is needed.

## Notes for tooling

This class needs no special tool handling beyond recognizing its origin. A `cargo-cgp`-style post-processor should pass the `E0576` through essentially unchanged — the caret and the `help:` already point at the fix — and at most annotate that the name was introduced by a `#[use_type]` import so a user who does not see the connection between the signature alias and the attribute can find it. There is nothing to suppress and no hidden cause to recover.

## A sibling: the unresolved *context*

The same textual-substitution mechanism produces a related failure when the part that cannot be resolved is the `in Context`, not the associated-type name. Two nested imports whose `in Context` clauses reference each other — `#[use_type(HasA.A in B, HasB.B in A)]` — form a cycle with no valid grounding order. Grounding iterates to a fixpoint and deliberately stops rather than loops, so the context aliases are never resolved and the rewrite leaves the bare `A` and `B` in type position. The compiler reports this as [`E0425`](https://doc.rust-lang.org/error_codes/E0425.html) "cannot find type" — a *type*, not an associated type, so a different code from the misnamed-name case above — with the caret on the unresolved alias the user wrote in the attribute. The fix is the same in spirit: make the imports form an acyclic chain (any acyclic order grounds fine). CGP could detect the cycle locally and reject it at macro time, but currently lowers it faithfully and defers to the compiler.

## Backing fixtures

- [acceptable/cgp_fn/use_type_unknown_assoc.rs](../../../crates/tests/cgp-compile-fail-tests/tests/acceptable/cgp_fn/use_type_unknown_assoc.rs) — a `#[cgp_fn]` importing `HasScalarType.Scalr` where the trait declares `Scalar`; its `.stderr` pins the `E0576` with the caret on the `Scalr` in the signature and the "similarly named associated type" `help:`, doubling as a guard that the substitution preserves the user's identifier span.
- [acceptable/cgp_fn/use_type_cyclic_context.rs](../../../crates/tests/cgp-compile-fail-tests/tests/acceptable/cgp_fn/use_type_cyclic_context.rs) — the unresolved-*context* sibling: two `in Context` clauses referencing each other, whose `.stderr` pins the `E0425` "cannot find type" with the caret on the unresolved `in` alias.

## Related

- [`#[use_type]`](../../reference/attributes/use_type.md) and [`#[cgp_type]`](../../reference/macros/cgp_type.md) — the import attribute whose textual substitution produces the unresolved path, and the macro defining the abstract-type trait it imports from.
- [Ill-formed generated type](ill-formed-generated-type.md) — the sibling lowering class, where the generated type is *resolvable* but not well-formed (an unsized type) rather than unresolvable by name.
- [`E0576`](../error_codes/e0576.md) — the Rust error code this class is reported under.
- [Debugging CGP compile errors](../../guides/debugging.md) — the prescriptive playbook this catalog supports.
