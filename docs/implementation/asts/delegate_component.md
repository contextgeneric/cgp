# The `delegate_component` AST stack

The `delegate_component` stack is the set of AST types that `delegate_components!` (and, for its wiring half, [`delegate_and_check_components!`](../entrypoints/delegate_and_check_components.md)) parses the macro body into and lowers to impls. The top is `DelegateTable`; below it the body is a `DelegateEntries` of leading statements plus comma-separated mappings, and each mapping pairs a key with a value under one of three operators. Evaluation flows in one direction: the tree lowers to a flat list of `EvaluatedDelegateEntry` values, each of which renders a `DelegateComponent` impl and an `IsProviderFor` impl. The [entrypoint document](../entrypoints/delegate_components.md) covers what the whole pipeline produces; this document covers the types, one per role in the grammar.

## `DelegateTable` and `EvaluatedDelegateTable`

`DelegateTable` is the whole parsed body: outer attributes, an optional leading generic list (`ImplGenerics`), an optional `new` keyword, the target type, and the braced `DelegateEntries`. Its `eval` produces an `EvaluatedDelegateTable` — a bag of `ItemImpl`s and `EmptyStruct`s — whose `ToTokens` emits the structs first, then the impls.

`eval` does three things: if `new` is present it parses the target into an identifier-plus-generics and pushes an `EmptyStruct` for it; it builds the entry impls from `DelegateEntries::build_impls`; and it collects every nested `UseDelegate` inner table (via `ExtractInnerDelegateTables`) and emits each inner table's own struct and impls. The target type and the outer generics are threaded down into every entry so all impls carry the table's generics.

## `DelegateEntries`

`DelegateEntries` is the table body: a `Vec<DelegateStatement>` followed by a `Punctuated<DelegateMapping, Comma>`. Its `Parse` impl peeks for a statement keyword (`namespace`, `open`, or `for`) and consumes all leading statements first, then parses the remaining comma-separated mappings — which is why a statement written after a mapping fails to parse. Its `eval_entries` lowers the statements first, then the mappings, concatenating the evaluated entries; `build_impls` then renders each into its `DelegateComponent`/`IsProviderFor` pair.

## `InnerDelegateTable`

`InnerDelegateTable` is a nested table lifted out of a `UseDelegate<new Inner { … }>` value: an identifier, its generics, and its own `DelegateEntries`. It builds its own `EmptyStruct` and, treating its own identifier-plus-generics as the target type, its own entry impls — so a nested table is evaluated exactly like a top-level one. `ExtractInnerDelegateTables` recurses so that nesting to any depth is flattened into the table's struct-and-impl list.

## `DelegateStatement`

`DelegateStatement` is the leading statement form, an enum over three variants that all lower through `eval_entries` into the same flat entry list:

- **`OpenDelegateStatement`** (`open { A, B };`) — opens each listed component for per-value wiring. For each component it emits an entry whose key is the component and whose value is `RedirectLookup<TableType, PathCons<Component, Nil>>`, rooting a redirect at the component name in the context's own table. The `@Component.Key` mappings that follow store providers under the extended path.
- **`NamespaceDelegateStatement`** (`namespace SomeNamespace;`) — forwards every lookup through a namespace trait. It lowers via the shared "for-entry" path to a blanket `DelegateComponent<__Key__>` impl bounded on `__Key__: SomeNamespace<TableType, Delegate = __Value__>`, so any key the namespace defines is inherited.
- **`ForDelegateStatement`** (`for <Key, Provider> in SomeTable where … { … }`) — a loop that pulls mappings out of another lookup table. Each inner mapping produces a for-entry whose namespace-trait bound is reconstructed with the table type and a `Delegate = value` binding appended to the namespace path's arguments.

The namespace and `for` forms share `EvaluatedForEntry` and `eval_delegate_entries_via_for`, which build the `Namespace<…, Delegate = …>` bound and the `__Key__`/`__Value__` generics before producing the final `EvaluatedDelegateEntry`.

## `DelegateMapping`

`DelegateMapping` is one comma-separated entry, an enum whose variant is chosen by the operator after the key. `DelegateMode` peeks the operator and drives the split:

- **`NormalDelegateMapping`** (`:`) — maps the key straight to the value type; the evaluated entry's `Delegate` is the value.
- **`DirectDelegateMapping`** (`->`) — forwards to the value's own entry for the key. It sets `Delegate` to `<Value as DelegateComponent<Key>>::Delegate` and adds a `Value: DelegateComponent<Key>` bound to the entry's generics.
- **`RedirectDelegateMapping`** (`=>`) — redirects the lookup along an `@`-path value. It sets `Delegate` to `RedirectLookup<TableType, Path>`, using the path directly for a plain key or a wildcard-terminated prefix for a path key.

Only Normal and Direct mappings can carry a nested inner table (their values are `DelegateValue`); a Redirect value is a bare path and contributes no inner table.

## A key: `DelegateKey`

`DelegateKey` is the left side of a mapping, an enum over three shapes selected by a lookahead on a fork (after skipping any attributes and generics): a leading `@` gives a `PathDelegateKey`, a leading bracket gives a `MultiDelegateKey`, otherwise a `SingleDelegateKey`. `EvalDelegateKey::eval` returns a `Vec<EvaluatedDelegateKey>` (a type plus its generics), so a single mapping can expand to several keyed impls.

- **`SingleDelegateKey`** — attributes, an optional generic list, and a type; evaluates to one key carrying its generics.
- **`MultiDelegateKey`** — a bracketed, comma-separated list of `SingleDelegateKey`; the array-key form, evaluating to one key per element so `[A, B]: P` becomes two impls.
- **`PathDelegateKey`** — the `@`-prefixed open key. It parses a `PathHead` after the `@` and lowers each expanded path to a prefix type terminated by a `__Wildcard__` generic, appending that wildcard to the key's generics. A brace group in the path (`@C.{u32, u64}`) expands to several paths, each becoming its own key. This is what lets a dispatch parameter slot into the redirect path at lookup time.

## A value: `DelegateValue`

`DelegateValue` is the right side of a Normal or Direct mapping, an enum of either a plain `Type` or a `DelegateValueWithInnerTable`. Its `Parse` speculatively tries the inner-table form first and falls back to a bare type.

`DelegateValueWithInnerTable` parses the legacy nested-dispatch shape `Wrapper<new Inner { … }>`: a wrapper identifier, `<`, the `new` keyword, an `InnerDelegateTable`, `>`. Its `eval` produces the value type `Wrapper<Inner…>` (dropping the `new`, which only signals that the inner struct must be declared), and `ExtractInnerDelegateTables` yields the inner table itself so `DelegateTable::eval` emits its struct and impls alongside the outer entry.

## `EvaluatedDelegateEntry`

`EvaluatedDelegateEntry` is the lowered form every key/value/statement collapses to: the target type, the merged generics, the key type, and the value (`Delegate`) type. It owns the rendering: `build_delegate_component_impl` emits the `DelegateComponent<Key> for TableType { type Delegate = Value; }` impl, and `build_is_provider_for_impl` emits the forwarding `IsProviderFor<Key, __Context__, __Params__>` impl bounded on `Value: IsProviderFor<Key, __Context__, __Params__>`, appending the reserved `__Context__` and `__Params__` generics. (`build_namespace_impl`, on the same type, is used by the namespace preset machinery to emit a `Namespace for Key` impl instead.)

## Tests

- The stack is exercised end-to-end by the expansion snapshots and behavioral tests indexed in the [entrypoint document](../entrypoints/delegate_components.md); there are no `cgp-macro-tests` failure cases for the delegate family.

## Source

- The stack lives in [cgp-macro-core/src/types/delegate_component/](../../../crates/macros/cgp-macro-core/src/types/delegate_component/): the table and `new` keyword in `table/main.rs` and inner tables in `table/inner.rs`; the entries in `entries.rs`; the keys in `key/` (`single.rs`, `multi.rs`, `path.rs`, dispatched in `key/mod.rs`); the values in `value/` (`inner_table.rs`, dispatched in `value/mod.rs`); the mappings and their operator in `mapping/` (`normal.rs`, `direct.rs`, `redirect.rs`, `mode.rs`, and the `EvaluatedDelegateEntry` renderer in `mapping/eval.rs`); and the statements in `statement/` (`open.rs`, `namespace.rs`, `for_loop.rs`, and the shared for-entry evaluator in `statement/eval.rs`).
- Attribute rejection is in `validate_attributes.rs`, path parsing (`PathHead`, `UniPath`) in [types/path/](../../../crates/macros/cgp-macro-core/src/types/path/), and all impls are built with [parse_internal!](../macros/parse_internal.md).
- The `RedirectLookup` value the `open` and `=>` forms emit is the impl generated by [`#[cgp_component]`](../entrypoints/cgp_component.md).
