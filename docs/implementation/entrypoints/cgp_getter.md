# `#[cgp_getter]` — implementation

`#[cgp_getter]` builds a full getter component — everything `#[cgp_component]` produces — and then appends three field-reading provider impls (`UseFields`, `UseField<Tag>`, `WithProvider<Provider>`) so a context can bind each getter to a source field by wiring rather than by method name. This document covers how that works internally; for the accepted syntax and the complete expansion a user sees, read the reference document [reference/macros/cgp_getter.md](../../reference/macros/cgp_getter.md).

## Entry point

The macro is driven by the `cgp_getter` function in [cgp-macro-lib/src/cgp_getter.rs](../../../crates/macros/cgp-macro-lib/src/cgp_getter.rs), which parses the attribute into `CgpComponentRawArgs` and the item into a `syn::ItemTrait`, then reuses the `#[cgp_component]` pipeline before layering the getter-specific impls on top.

```rust
let evaluated = ItemCgpComponent { args, item_trait }.preprocess()?.eval()?;
let item_getter = ItemCgpGetter::try_from(evaluated)?;
let items = item_getter.to_items()?;
```

Before building the component, the entry function derives the default provider name specific to getters: if the user gave no provider identifier and the trait name begins with `Has`, the provider defaults to the remainder plus `Getter` (so `HasName` yields `NameGetter`, component `NameGetterComponent`). Only after that does it hand off to the shared component args conversion. Applying the macro to a non-trait item fails at `parse2::<ItemTrait>`, and a malformed attribute is rejected by the `CgpComponentRawArgs` parser.

## Pipeline

The macro runs the entire `#[cgp_component]` pipeline and then a getter-specific emit stage; the getter-side AST types are documented in the [`cgp_getter` AST stack](../asts/cgp_getter.md) and the component stages in the [`cgp_component` AST stack](../asts/cgp_component.md).

- **preprocess → eval** are the `#[cgp_component]` stages unchanged: they strip the CGP modifier attributes off the trait, then derive the provider trait, the two routing blanket impls, and the component marker, producing an `EvaluatedCgpComponent`.
- **parse getter fields** happens in `ItemCgpGetter::try_from`, which parses each method of the consumer trait into a `GetterField` (its field name, field type, return type, receiver mode, and field mode) and captures an optional single associated return type.
- **to_items** emits the component's own items first, then appends the three getter provider impls.

## Generated items

`#[cgp_getter]` emits the five core `#[cgp_component]` items plus the standard `UseContext`/`RedirectLookup` provider impls, and then adds three more provider impls that all read from `HasField`. The reference document shows the full expansion; the point worth understanding here is what distinguishes the three added impls.

- **`UseFields`** implements the getter by reading the field named after the method — the same behavior a `#[cgp_auto_getter]` blanket impl gives, but expressed as a provider a context can wire.
- **`UseField<__Tag__>`** implements the getter by reading the field named `__Tag__`, a *free* generic parameter, so wiring `UseField<Symbol!("first_name")>` makes the getter read `first_name` regardless of the method name. This is the whole reason `#[cgp_getter]` exists rather than `#[cgp_auto_getter]`.
- **`WithProvider<__Provider__>`** implements the getter by delegating to an inner `FieldGetter`/`MutFieldGetter` provider, so the field access itself can be supplied by another provider.

Each of these reads the field through the same getter-method body the auto-getter uses — a `get_field(PhantomData::<Tag>)` call with the field-mode conversion appended. The `UseField` impl keys on `__Tag__`:

```rust
// for `fn foo(&self) -> &str` reading a String field
impl<__Context__, __Tag__> FooGetter<__Context__> for UseField<__Tag__>
where
    __Context__: HasField<__Tag__, Value = String>,
{
    fn foo(__context__: &__Context__) -> &str {
        __context__.get_field(PhantomData::<__Tag__>).as_str()
    }
}
```

Every provider-trait impl is paired with a matching `IsProviderFor` impl carrying the same bounds, exactly as the component macro pairs its own impls, so wiring failures still name the missing `HasField` bound.

## Behavior and corner cases

**The `UseField` and `WithProvider` impls are only emitted for a single-getter trait.** Both `to_use_field_impl` and `to_with_provider_impl` return `None` when the trait declares more than one getter method, because a per-field tag or per-field inner provider is meaningless once several fields are in play; the `UseFields` impl, keyed by method name, is always emitted.

**A getter can read a field of a type other than the context.** When a method takes a typed receiver rather than `&self` — `fn foo_bar(foo: &Self::Foo) -> &Self::Bar` — the receiver's `Self` is rewritten to the context and the generated impls read the field out of that receiver type instead of the context. The provider impls then bound that receiver type, not the context, with the `HasField` requirement.

**A single associated return type is supported and inferred from the field.** A getter trait may declare one `type Name;` used as the return type; the associated type is added as an extra generic parameter to each provider impl, set to itself via `type Name = Name;`, and any bound on it (for example `Name: Display`) is carried onto the impl with `Self::Name` rewritten to the parameter. More than one associated type, or an associated type alongside more than one method, is rejected during field parsing.

**The return-type shorthands are shared with the auto-getter.** The `&str`-reads-`String`, `Option<&T>`-reads-`Option<T>`, `&[T]`-reads-`AsRef<[T]>`, `MRef<'_, T>`, and owned-`.clone()` conversions all come from the shared field-mode parsing, so `#[cgp_getter]` and [`#[cgp_auto_getter]`](cgp_auto_getter.md) treat a given signature identically — they differ only in the items they emit around the shared getter-method body.

## Snapshots

Every `snapshot_cgp_getter!` invocation across the suite is indexed here, since these snapshots all belong to this entrypoint:

- [getters/string.rs](../../../crates/tests/cgp-tests/tests/getters/string.rs) — the canonical full expansion: a `&str` getter over a `String` field, showing all five component items plus the `UseContext`, `RedirectLookup`, `UseFields`, `UseField`, and `WithProvider` impls.
- [getters/clone.rs](../../../crates/tests/cgp-tests/tests/getters/clone.rs) — an owned return `.clone()`d out by value.
- [getters/mref.rs](../../../crates/tests/cgp-tests/tests/getters/mref.rs) — an `MRef<'_, String>` return wrapping the borrow in `MRef::Ref`.
- [getters/option.rs](../../../crates/tests/cgp-tests/tests/getters/option.rs) — an `Option<&String>` return reading an `Option<String>` field via `.as_ref()`.
- [getters/slice.rs](../../../crates/tests/cgp-tests/tests/getters/slice.rs) — a `&[u8]` return reading any `AsRef<[u8]> + 'static` field via `.as_ref()`.
- [getters/non_self.rs](../../../crates/tests/cgp-tests/tests/getters/non_self.rs) — a non-`self` getter reading a field out of another type (`&Self::Foo`).
- [getters/string_custom_name.rs](../../../crates/tests/cgp-tests/tests/getters/string_custom_name.rs) — `#[cgp_getter(GetString)]` overriding the provider name (and component).
- [getters/string_custom_spec.rs](../../../crates/tests/cgp-tests/tests/getters/string_custom_spec.rs) — the brace-spec form overriding provider and component names independently.
- [getters/assoc_type_getter.rs](../../../crates/tests/cgp-tests/tests/getters/assoc_type_getter.rs) — a local associated return type, showing the associated-type parameter threaded through every provider impl.
- [getters/assoc_type_self_referential.rs](../../../crates/tests/cgp-tests/tests/getters/assoc_type_self_referential.rs) — a self-referential associated-type bound with the source field bound by wiring to a differently-named field.
- [getters/abstract_type_extend.rs](../../../crates/tests/cgp-tests/tests/getters/abstract_type_extend.rs), [getters/abstract_type_use_type.rs](../../../crates/tests/cgp-tests/tests/getters/abstract_type_use_type.rs) — getters whose return type is an abstract type imported from another component via `#[extend]` and `#[use_type]`; each file pins both the auto and the full getter variant.

The `UseDelegate`-table dispatch form of a getter is snapshotted separately in [dispatching/use_delegate_getter.rs](../../../crates/tests/cgp-tests/tests/dispatching/use_delegate_getter.rs), which the dispatching concept owns rather than this entrypoint. One variant has no snapshot yet: a getter component carrying its own generic parameter (distinct from the auto-getter's `auto_getter_generic.rs`).

## Tests

The getter snapshot files also wire concrete contexts and assert the getters resolve, so the same files carry the behavioral checks:

- [getters/string.rs](../../../crates/tests/cgp-tests/tests/getters/string.rs) and [getters/assoc_type_self_referential.rs](../../../crates/tests/cgp-tests/tests/getters/assoc_type_self_referential.rs) confirm that wiring `UseField<Symbol!("first_name")>` makes the getter read a field whose name differs from the method.
- [getters/clone.rs](../../../crates/tests/cgp-tests/tests/getters/clone.rs), [getters/mref.rs](../../../crates/tests/cgp-tests/tests/getters/mref.rs), [getters/option.rs](../../../crates/tests/cgp-tests/tests/getters/option.rs), and [getters/slice.rs](../../../crates/tests/cgp-tests/tests/getters/slice.rs) verify each return-type shorthand at run time.

## Source

- Entry point: `cgp_getter` in [cgp-macro-lib/src/cgp_getter.rs](../../../crates/macros/cgp-macro-lib/src/cgp_getter.rs), which derives the `Has…` → `…Getter` default name and reuses the component pipeline.
- Getter-specific stack: [cgp-macro-core/src/types/cgp_getter/](../../../crates/macros/cgp-macro-core/src/types/cgp_getter/), documented in [asts/cgp_getter.md](../asts/cgp_getter.md): `item.rs` assembles the items, `to_use_fields_impl.rs`/`use_field.rs`/`with_provider.rs` build the three added provider impls.
- Component stages it reuses: [cgp-macro-core/src/types/cgp_component/](../../../crates/macros/cgp-macro-core/src/types/cgp_component/), documented in [asts/cgp_component.md](../asts/cgp_component.md).
- Getter-method parsing and the return-type shorthands, shared with `#[cgp_auto_getter]`: [cgp-macro-core/src/functions/getter/parse.rs](../../../crates/macros/cgp-macro-core/src/functions/getter/parse.rs) and [cgp-macro-core/src/functions/field/parse.rs](../../../crates/macros/cgp-macro-core/src/functions/field/parse.rs).
- Fragment construction: [parse_internal!](../macros/parse_internal.md).
