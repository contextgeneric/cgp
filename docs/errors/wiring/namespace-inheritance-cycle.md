# Namespace inheritance cycle

Two namespaces that inherit from each other — or one that inherits itself — make resolving any key chase the inheritance chain forever, so the trait solver overflows with `E0275`, reported *at the `cgp_namespace!` definitions themselves*.

## What triggers it

The mistake is a circular parent chain among namespaces: `A` names `B` as its parent while `B` names `A`, or a namespace names itself. A [namespace](../../reference/macros/cgp_namespace.md) inherits its parent through a blanket impl whose `where` clause requires the parent's lookup trait, so a parent chain that loops back on itself produces a `where` clause that can never be discharged.

```rust
cgp_namespace! {
    new NamespaceA: NamespaceB {} // A inherits B
}

cgp_namespace! {
    new NamespaceB: NamespaceA {} // B inherits A — the chain loops
}
```

`new NamespaceA: NamespaceB` emits the inheritance blanket impl `impl<Table, Key, Value> NamespaceA<Table> for Key where Key: NamespaceB<…>, Key: NamespaceB<Table, Delegate = Value>`, and `new NamespaceB: NamespaceA` emits the mirror. A self-inheriting `new A: A {}` collapses the two into one impl whose bound requires the trait it defines. CGP cannot see the parent chain is circular from one macro invocation — each `cgp_namespace!` knows only its own parent — so it lowers each namespace faithfully and defers the contradiction to the compiler.

## The diagnostic

The compiler reports **`E0275` "overflow evaluating the requirement"**, and the defining property of this class is *where* it lands: on the `cgp_namespace!` blocks that define the namespaces, with no context or use site required. Evaluating the well-formedness of `NamespaceA`'s inheritance impl means evaluating its `where` bound `__Key__: NamespaceB<…>`, which pulls in `NamespaceB`'s impl and its bound `__Key__: NamespaceA<…>`, which pulls in the first again — an infinite chain. The overflow message names the requirement that recurses (`__Key__: NamespaceA<__NamespaceBComponents>`), a `note:` chain that walks `NamespaceB<…>` → `NamespaceA<…>` and names the loop, a `= note: N redundant requirements hidden` marking the collapsed repetition, and the standard `help: consider increasing the recursion limit`. Each of the two definitions carries its own `E0275`.

This eager, definition-site failure is the sharp contrast with the sibling [wiring cycle](wiring-cycle.md). A [`UseContext`](../../reference/providers/use_context.md) delegation cycle is *lazy*: the wiring is accepted, and the overflow appears only when the wiring is forced through a check (and hides as an [`E0599`](../hidden/unsatisfied-dependency.md) when reached by a plain call). A namespace inheritance cycle is *eager*: the cycle lives in the `where` clause of a generated blanket impl, and the compiler evaluates that clause when it checks the impl, so the overflow fires at the definition before anything uses the namespace. A context that *does* join the cycle and is then checked simply adds more `E0275` blocks — an `App: DelegateComponent<GreeterComponent>` overflow at the check — on top of the definition-site ones; it does not change the cause.

The overflow itself is ordinary `rustc` behavior. The trait solver walks a requirement's supporting bounds to a bounded depth — the [default `recursion_limit` is 128](https://doc.rust-lang.org/reference/attributes/limits.html) — and reports [`E0275`](../error_codes/e0275.md) when it exceeds that depth without terminating. A genuine cycle never terminates, so the limit is only ever reached, never cleared; the `help:` suggestion to raise it is generic advice the compiler prints for every overflow and does not apply here.

## Where the root cause is

The cause is **present and the diagnostic points straight at it**: the two `E0275` carets land on the two `cgp_namespace!` definitions, and the `note:` chain names both namespaces and the loop between them. Unlike a hidden failure there is nothing to promote and nothing suppressed — the participants are all named. What the diagnostic does not state is the remedy, and it actively misleads on it: the `help: increase the recursion limit` line suggests a fix that cannot work, because the requirement does not terminate at any depth. Read the overflow as "these namespaces' parent chain is circular," not as "the chain is merely deep."

## Resolving it

Break the cycle so the parent chain is acyclic — a namespace's inheritance must form a tree, not a loop. Remove one direction of the mutual inheritance so one namespace is unambiguously the parent, or, when both namespaces genuinely need a shared set of entries, factor those entries into a third base namespace that both inherit from, rather than having them inherit each other. Raising `#![recursion_limit]` is the wrong move: the chain has no terminating step, so no limit is high enough.

## Notes for tooling

A `cargo-cgp`-style post-processor should **recognize the repeating namespace requirement** — the same `Namespace<…>` pair recurring in the overflow's `note:` chain — and report it as an inheritance cycle ("`NamespaceA` inherits `NamespaceB` inherits `NamespaceA`"), naming the two `cgp_namespace!` definitions rather than echoing the raw overflow depth. Crucially, it should **suppress the compiler's `increase the recursion limit` suggestion for this class**, since that advice is actively wrong for a true cycle; the value the tool adds is replacing misleading generic advice with the actual structural fix.

## Backing fixtures

- [acceptable/cgp_namespace/inheritance_cycle.rs](../../../crates/tests/cgp-compile-fail-tests/tests/acceptable/cgp_namespace/inheritance_cycle.rs) — two namespaces that inherit each other, with no context at all; its `.stderr` pins the two `E0275` overflow blocks landing on the two `cgp_namespace!` definitions and the `note:` chain that names the `NamespaceB<…>` → `NamespaceA<…>` loop, confirming the failure is caught eagerly at definition.

## Related

- [Wiring cycle](wiring-cycle.md) — the sibling cycle class through [`UseContext`](../../reference/providers/use_context.md); both overflow with `E0275`, but that one is lazy (surfaces only when checked, hides as `E0599` when called) while this one is eager (caught at the namespace definitions).
- [Conflicting wiring](conflicting-wiring.md), [Orphan-rule violation](orphan-rule.md), [Unconstrained generic](unconstrained-generic.md) — the sibling structural classes.
- [`#[cgp_namespace]`](../../reference/macros/cgp_namespace.md) and [`DefaultNamespace`](../../reference/traits/default_namespace.md) — the inheritance blanket impl whose `where` clause the cycle poisons.
- [Debugging CGP compile errors](../../guides/debugging.md) — the `E0275`/overflow entry in the decoder.
