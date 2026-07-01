# The `check_components` and `delegate_and_check_components` AST stacks

These two stacks generate the compile-time wiring checks. The `check_components` stack parses a check table into entries and lowers each into a check-trait impl; the `delegate_and_check_components` stack is a thin layer that reuses both this stack and the [`delegate_component` stack](delegate_component.md), deriving check entries from delegation keys. Evaluation flows from a `CheckComponentsTable` down to `EvaluatedCheckEntry` values, each rendering one empty impl of the table's check trait. The [`check_components!`](../entrypoints/check_components.md) and [`delegate_and_check_components!`](../entrypoints/delegate_and_check_components.md) entrypoint documents cover what the pipelines produce; this document covers the types.

## `CheckComponentsTables` and `CheckComponentsTable`

`CheckComponentsTables` is the whole `check_components!` body — a `Vec<CheckComponentsTable>`, parsed by looping until the input is empty, so one invocation can carry several context blocks. `to_items` concatenates each table's items.

`CheckComponentsTable` is one context block: an optional `check_providers` list, an optional leading generic list, the derived-or-overridden check-trait name, the context type, an optional `where` clause, and the `CheckEntries`. Its `Parse` reads the `#[check_trait]` and `#[check_providers]` attributes (rejecting any other), derives the trait name as `__Check{Context}` from the context type's leading identifier when not overridden, and parses the braced entries. Its `eval` builds the check trait once — supertraiting `CanUseComponent` normally, or `IsProviderFor<…, Context, …>` under `#[check_providers]` — then emits one impl per evaluated entry. For the context-checking form it overrides the impl's `Self`-type span with the component's span so an error points at the component; the `#[check_providers]` form instead emits one impl per listed provider.

## `CheckEntries` and `CheckEntry`

`CheckEntries` is the braced list, a `Punctuated<CheckEntry, Comma>`; its `eval` flattens every entry into `EvaluatedCheckEntry` values.

`CheckEntry` is one line: a `CheckKey` and an optional `CheckValue` after a colon. Its `eval` produces the cartesian product of keys and values — one evaluated entry per (key, value) pair — and, when there is no value, one entry with unit params. It also chooses the diagnostic span per entry, preferring the component or the parameter side depending on which list is longer, so the error lands on the token the user is most likely to have gotten wrong.

## A key: `CheckKey`

`CheckKey` is the left side, an enum of a single `Type` or a bracketed `Multi` list. `to_keys` returns one type for the single form and one per element for the array form, which is how a bracketed key checks several components at once.

## A value: `CheckValue`

`CheckValue` is the optional right side — the generic parameters to check the component with — an enum of a single `TypeWithGenerics` or a bracketed `Multi` list. `to_values` returns one or many, so a bracketed value checks one component against several parameter sets. `TypeWithGenerics` is a type plus an optional leading generic list, letting a parameter introduce its own generics (`<T> &'a T`) that merge with the table's when the impl is built. An omitted value defaults to the unit type.

## `EvaluatedCheckEntry`

`EvaluatedCheckEntry` is the lowered form: the component key, an optional `TypeWithGenerics` value, and the diagnostic span. It is a plain data struct; `CheckComponentsTable::eval` consumes it to build each impl, placing the key in the `__Component__` position and the value (or `()`) in the `__Params__` position.

## `ItemDelegateAndCheckComponents`

`ItemDelegateAndCheckComponents` is the entire `delegate_and_check_components!` body — just a wrapper around a parsed [`DelegateTable`](delegate_component.md). It adds the derivation from delegation to checking:

- `check_trait_ident` derives the check-trait name as `__CanUse{Context}` (distinct from `check_components!`'s `__Check{Context}`) or reads a single `#[check_trait]` attribute off the table.
- `to_check_entries` walks the delegation keys via `ToKeysWithCheckParams` and turns each into `CheckEntry` values.
- `to_check_components` packages those entries, the table's generics, the derived name, and the context type into a `CheckComponentsTable`, so the checking half is rendered by the ordinary `check_components` pipeline.

The wiring half is not re-derived here — the entrypoint calls `DelegateTable::eval` directly.

## `CheckParamsAttribute` and `KeyWithCheckParams`

`CheckParamsAttribute` is the per-entry check control parsed from the delegation key's attributes: `Default` (check with unit params), `Skip` (`#[skip_check]`, no check), or `Multi` (`#[check_params(...)]`, one check per listed parameter). Its `merge` combines a bracket-level attribute with an inner key's own — unioning two `Multi` sets, but erroring when `Skip` meets `Multi` — and `parse_attributes` enforces that at most one attribute appears and that `#[skip_check]` takes no arguments.

`KeyWithCheckParams` pairs a delegation key type with its resolved `CheckParamsAttribute`. Its `to_check_entries` turns the pairing into `CheckEntry` values: `Default` yields one bare entry, `Skip` yields none, and `Multi` yields one valued entry per parameter.

## `ToKeysWithCheckParams`

`ToKeysWithCheckParams` is the trait that walks a `DelegateEntries` and collects `KeyWithCheckParams`. It handles single and array keys (merging bracket-level and inner attributes), and it deliberately produces no check entries for redirect (`=>`) mappings, `@`-path keys, and the `open`/`namespace`/`for` statement forms — validating that each carries no attribute rather than silently dropping one. A per-key generic list on a `SingleDelegateKey` is intentionally not carried into the check entry, since the derived impl sees only the table-level generics; this limitation is recorded in the [entrypoint document's Known issues](../entrypoints/delegate_and_check_components.md).

## Tests

- Both stacks are exercised by the expansion snapshots indexed in the [`check_components!`](../entrypoints/check_components.md) and [`delegate_and_check_components!`](../entrypoints/delegate_and_check_components.md) entrypoint documents; each is a compile-only test, so a successful build is the passing assertion.
- There are no `cgp-macro-tests` failure cases for the check family.

## Source

- The `check_components` stack lives in [cgp-macro-core/src/types/check_components/](../../../crates/macros/cgp-macro-core/src/types/check_components/): the tables in `tables.rs` and `table.rs` (with the trait build, attribute parsing, name derivation, supertrait choice, and span override in `table.rs`), the entries in `entries.rs` and `entry.rs`, the key and value in `key.rs` and `value.rs`, `TypeWithGenerics` in `type_with_generics.rs`, and `EvaluatedCheckEntry` in `evaluated_check_entry.rs`.
- The `delegate_and_check_components` stack lives in [cgp-macro-core/src/types/delegate_and_check_components/](../../../crates/macros/cgp-macro-core/src/types/delegate_and_check_components/): the wrapper item in `item.rs`, the attributes in `check_params.rs`, the per-key conversion in `key_with_check_params.rs`, and the entry walk in `to_keys_with_check_params.rs`. It reuses the [`DelegateTable`](delegate_component.md).
- All impls are built with [parse_internal!](../macros/parse_internal.md).
