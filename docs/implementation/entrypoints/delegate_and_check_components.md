# `delegate_and_check_components!` — implementation

`delegate_and_check_components!` wires a context and asserts the wiring in one step, by parsing the shared `DelegateTable`, evaluating its delegation half, and deriving a `CheckComponentsTable` from the delegation keys. This document covers how that fusion works internally; for the accepted syntax and the complete expansion a user sees, read the reference document [reference/macros/delegate_and_check_components.md](../../reference/macros/delegate_and_check_components.md).

## Entry point

The macro is driven by the thin `delegate_and_check_components` function in [cgp-macro-lib/src/delegate_and_check_components.rs](../../../crates/macros/cgp-macro-lib/src/delegate_and_check_components.rs). It parses the body into an [`ItemDelegateAndCheckComponents`](../asts/check_components.md) (a wrapper around a `DelegateTable`), derives the check table from it, evaluates the delegation half through the shared `DelegateTable`, and emits both halves:

```rust
let item: ItemDelegateAndCheckComponents = parse2(body)?;
let check_table = item.to_check_components()?;
let evaluated_table = item.table.eval()?;
let check_items = check_table.to_items()?;
Ok(quote! { #evaluated_table  #( #check_items )* })
```

All real logic lives in `cgp-macro-core`. The macro shares both stacks it fuses: the [`delegate_component` stack](../asts/delegate_component.md) for the wiring half and the [`check_components` stack](../asts/check_components.md) for the checking half.

## Pipeline

The macro reuses two existing pipelines rather than defining its own. The delegation half is exactly `DelegateTable::eval`, unchanged from [`delegate_components!`](delegate_components.md), so the wiring impls are identical to what that macro emits. The checking half is derived: `to_check_components` reads the delegation keys, converts each into a check entry, and packages them into a `CheckComponentsTable` whose `to_items` is exactly the [`check_components!`](check_components.md) pipeline. The only new work is the key-to-check-entry conversion and the per-entry `#[check_params]`/`#[skip_check]` handling.

## Generated items

The macro emits the delegation impls first — a `DelegateComponent` impl and an `IsProviderFor` forwarding impl per entry, exactly as `delegate_components!` produces — then the check trait and one impl per non-skipped entry, exactly as `check_components!` produces. The derived check trait defaults to `__CanUse{Context}` (not `__Check{Context}`), so a `delegate_and_check_components!` and a `check_components!` block can coexist once each in the same module:

```rust
// delegate_and_check_components! { MyContext { NameGetterComponent: UseField<Symbol!("name")> } }
impl DelegateComponent<NameGetterComponent> for MyContext {
    type Delegate = UseField<Symbol!("name")>;
}
impl<__Context__, __Params__>
    IsProviderFor<NameGetterComponent, __Context__, __Params__> for MyContext
where
    UseField<Symbol!("name")>: IsProviderFor<NameGetterComponent, __Context__, __Params__>,
{}
// … then the checking half:
trait __CanUseMyContext<__Component__, __Params__: ?Sized>:
    CanUseComponent<__Component__, __Params__>
{}
impl __CanUseMyContext<NameGetterComponent, ()> for MyContext {}
```

A table-level `#[check_trait(Name)]` overrides the derived name. A generic table threads its generics through both halves, since both reuse the same `impl_generics` from the parsed `DelegateTable`.

## Behavior and corner cases

**Every delegated key is checked by default.** The conversion walks the delegation entries and produces one bare check entry per key, so wiring an entry with no attribute both delegates and checks it.

**`#[check_params(...)]` supplies the parameters the check needs.** A component with generic parameters has a parameter-generic `DelegateComponent` impl but a check that needs concrete parameters, so `#[check_params(...)]` provides them: each listed parameter becomes its own check impl, while the single delegation impl stays generic. `#[skip_check]` contributes the delegation impls but no check impl.

**The two attributes are mutually exclusive and merge across bracket levels.** `#[check_params]` and `#[skip_check]` cannot both apply to one key, and at most one of each may appear. For an array key, a block-level attribute on the bracket merges with each inner key's own attribute — two `#[check_params]` sets union, while combining `#[skip_check]` with `#[check_params]` is an error.

**Statement forms and non-plain keys are not checked.** The `open`/`namespace`/`for` statements and redirect (`=>`) and `@`-path keys still produce delegation impls through `eval`, but the conversion generates no check entries for them; it validates that they carry no attributes rather than silently ignoring one. Path keys on the delegation side are likewise dropped from the check half. This means a per-value `open` dispatch is wired but not checked.

## Known issues

A **per-key generic list on a delegation key is not carried into the check half**. A key that introduces its own generic parameters (`<T> FooKey<T>: …`) would produce a check impl referencing those parameters unbound, since the derived check impl sees only the table-level generics. Such generic keys are therefore unsupported in the checking half; the workaround is `#[skip_check]` on that key and a separate `check_components!` block. The correct behavior would be to thread the per-key generics into the derived check entry as `check_components!` already does for its own generic values.

## Snapshots

Every `snapshot_delegate_and_check_components!` invocation across the suite is indexed here, since these snapshots belong to this entrypoint:

- [checking/delegate_and_check_basic.rs](../../../crates/tests/cgp-tests/tests/checking/delegate_and_check_basic.rs) — the basic form: two entries wired and checked, with the check trait renamed via `#[check_trait(...)]`.
- [checking/delegate_and_check_generic.rs](../../../crates/tests/cgp-tests/tests/checking/delegate_and_check_generic.rs) — a generic context (`<T> MyContext<T>`) threaded through both halves, with the check trait defaulting to `__CanUse{Context}`.
- [checking/delegate_and_check_params.rs](../../../crates/tests/cgp-tests/tests/checking/delegate_and_check_params.rs) — `#[check_params(...)]` supplying parameter tuples, an array key wiring several components to one provider, and a block-level `#[check_params(...)]` on the bracket merged with each entry's own.
- [dispatching/use_delegate_getter.rs](../../../crates/tests/cgp-tests/tests/dispatching/use_delegate_getter.rs) — a `UseDelegate`-table value wired and checked in one step, exercising the legacy nested-table form through this macro.

One variant has no snapshot: a `#[skip_check]` entry alongside checked entries, which the reference shows but no snapshot pins.

## Tests

- The snapshot files above are compile-only tests, so a successful build is the passing assertion for both the wiring and the derived check.
- There are no separate behavioral or `cgp-macro-tests` failure cases for this macro.

## Source

- Entry point: `delegate_and_check_components` in [cgp-macro-lib/src/delegate_and_check_components.rs](../../../crates/macros/cgp-macro-lib/src/delegate_and_check_components.rs).
- Wrapper item, key-to-check conversion, and attribute types: [cgp-macro-core/src/types/delegate_and_check_components/](../../../crates/macros/cgp-macro-core/src/types/delegate_and_check_components/), documented with the `check_components!` stack in [asts/check_components.md](../asts/check_components.md).
- The `__CanUse{Context}` default name and `#[check_trait]` handling are in `item.rs`, the `#[check_params]`/`#[skip_check]` parsing and mutual exclusion in `check_params.rs`, the per-key conversion in `key_with_check_params.rs`, and the walk over delegation entries in `to_keys_with_check_params.rs`.
- Reuses the [`DelegateTable`](../asts/delegate_component.md) for the wiring half and the [`CheckComponentsTable`](../asts/check_components.md) for the checking half.
