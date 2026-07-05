# Adding capability supertraits

A CGP component often depends on another capability, and this guide is about declaring that dependency with `#[extend]`, which reads as importing a capability, rather than the native `:` supertrait syntax, which reads as inheritance.

This guide is the capability counterpart to [importing abstract types](importing-abstract-types.md), which handles a supertrait whose *type* the signature names.

## Add supertraits with `#[extend]`, not native `:` syntax

Add a non-type capability supertrait to a [`#[cgp_component]`](../reference/macros/cgp_component.md) trait with [`#[extend(...)]`](../reference/attributes/extend.md), rather than writing the native `pub trait CanDoX: Supertrait` form. Both produce the same trait with the same supertrait, but the attribute reads as an import — a capability the trait re-exports — which matches how CGP actually uses supertraits: as declared dependencies, not as a base class. Native `:` supertrait syntax tends to read as inheritance to programmers coming from object-oriented languages, suggesting an is-a relationship to a parent that a CGP component does not have. `#[extend(...)]` avoids that misreading and pairs symmetrically with [`#[uses(...)]`](declaring-dependencies.md): `#[uses]` imports a capability for the implementation's private use, `#[extend]` re-exports one as part of the trait's public contract. The native form:

```rust
#[cgp_component(Greeter)]
pub trait CanGreet: HasName {
    fn greet(&self) -> String;
}
```

becomes:

```rust
#[cgp_component(Greeter)]
#[extend(HasName)]
pub trait CanGreet {
    fn greet(&self) -> String;
}
```

`#[extend]` is the tool for a supertrait that contributes only a *capability* — like `HasName` here, which `CanGreet` depends on but whose value it reads through the getter rather than naming an abstract type in the signature. When the supertrait is instead an **abstract-type component** whose associated type the signature does name, use [`#[use_type]`](importing-abstract-types.md) instead: `#[use_type]` adds the supertrait *and* rewrites the bare type, which `#[extend]` does not, so it is the recommended form for abstract-type components. In [`#[cgp_fn]`](../reference/macros/cgp_fn.md), whose `where` clauses are impl-side dependencies rather than supertraits, `#[extend]` is the only way to declare a supertrait at all.

## Related guides

- [Importing abstract types](importing-abstract-types.md) — use `#[use_type]` instead when the supertrait carries an associated type the signature names.
- [Declaring dependencies](declaring-dependencies.md) — the `#[uses]` counterpart for a capability the implementation uses privately rather than re-exporting.
- [Guides summary](README.md#summary) — the cheat-sheet across all the guides.
