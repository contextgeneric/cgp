# `#[cgp_auto_getter]` — implementation

`#[cgp_auto_getter]` derives a single blanket getter impl over `HasField` from a trait's method names, so any context whose field names match the getters gains the trait with no wiring. This document covers how that works internally; for the accepted syntax and the complete expansion a user sees, read the reference document [reference/macros/cgp_auto_getter.md](../../reference/macros/cgp_auto_getter.md).

## Entry point

The macro is driven by the `cgp_auto_getter` function in [cgp-macro-lib/src/cgp_auto_getter.rs](../../../crates/macros/cgp-macro-lib/src/cgp_auto_getter.rs), which takes no attribute argument, parses the item into a `syn::ItemTrait`, then runs the two-step transform and emits the result.

```rust
let item = ItemCgpAutoGetter::preprocess(&item_trait)?;
let items = item.to_items()?;
```

The macro rejects any attribute argument up front with a spanned error — unlike `#[cgp_getter]`, it has no provider name, component, or keys to accept. Applying it to a non-trait item fails at `parse2::<ItemTrait>`. All real logic lives in `cgp-macro-core`.

## Pipeline

The macro moves through two short steps, both methods on `ItemCgpAutoGetter`, documented in the [`cgp_getter` AST stack](../asts/cgp_getter.md) alongside `#[cgp_getter]`.

- **preprocess** strips the CGP modifier attributes off the trait and keeps the cleaned trait; there is no component to derive, so this stage does little more than the attribute split.
- **to_items** parses the trait's methods into `GetterField`s and emits the trait unchanged followed by one blanket impl of that trait for `__Context__`.

## Generated items

`#[cgp_auto_getter]` emits exactly two items: the trait as written, and a single blanket impl over `__Context__` that implements every getter by reading the like-named field. Unlike `#[cgp_getter]`, there is no provider trait, no component marker, and no `UseField`/`UseFields`/`WithProvider` impls — the field each getter reads is fixed to the method name, so nothing is left to wire.

Each getter body reads its field with `get_field(PhantomData::<Symbol!("...")>)` and appends the field-mode conversion for its return type, and the field itself becomes a `HasField` bound on the impl:

```rust
// input
#[cgp_auto_getter]
pub trait HasFoo {
    fn foo(&self) -> &str;
}

// derived blanket impl
impl<__Context__> HasFoo for __Context__
where
    __Context__: HasField<Symbol!("foo"), Value = String>,
{
    fn foo(&self) -> &str {
        self.get_field(PhantomData::<Symbol!("foo")>).as_str()
    }
}
```

The conversions are exactly those `#[cgp_getter]` uses — `&str` reads a `String` field and appends `.as_str()`, `Option<&T>` reads `Option<T>` and appends `.as_ref()`, `&[T]` reads an `AsRef<[T]>` field, `MRef<'_, T>` wraps the borrow in `MRef::Ref`, an owned return appends `.clone()`, and a plain `&T` is taken by reference — because both macros share the [getter-field parsing](../asts/cgp_getter.md). The difference is only that `#[cgp_auto_getter]` emits a blanket impl on `__Context__` while `#[cgp_getter]` emits provider impls.

## Behavior and corner cases

**A supertrait on the getter trait becomes a context bound on the impl.** The trait's supertrait constraints are lowered to a `__Context__: Supertrait` predicate on the blanket impl, so a getter trait can require capabilities of any context that implements it without those constraints appearing on the impl's `Self` in trait position.

**A single associated return type is supported and inferred.** A getter trait may declare one `type Name;` used as its method's return type; the blanket impl adds `Name` as a generic parameter, sets `type Name = Name;`, and carries any bound on it (for example `Name: Display`, or a self-referential `Scalar: Mul<Output = Self::Scalar>`) onto the impl with `Self::Name` rewritten to the parameter. More than one associated type, or an associated type alongside more than one method, is rejected.

**A generic parameter on the trait is preserved onto the impl.** A trait generic over a type parameter keeps that parameter on the blanket impl and threads it through the field bound, so the same getter works for any field type inferred from the argument.

**A getter can read a field of another type.** A method with a typed receiver (`fn foo_bar(foo: &Self::Foo) -> &Self::Bar`) reads the field out of that receiver type; `Self` in the receiver is rewritten to the context, and the `HasField` bound lands on the receiver type rather than the context.

## Snapshots

Every `snapshot_cgp_auto_getter!` invocation across the suite is indexed here, since these snapshots all belong to this entrypoint:

- [getters/string_auto.rs](../../../crates/tests/cgp-tests/tests/getters/string_auto.rs) — the canonical plain case: a `&str` getter over a `String` field, `.as_str()` applied, no wiring.
- [getters/clone_auto.rs](../../../crates/tests/cgp-tests/tests/getters/clone_auto.rs) — an owned return `.clone()`d out by value.
- [getters/mref_auto.rs](../../../crates/tests/cgp-tests/tests/getters/mref_auto.rs) — an `MRef<'_, String>` return wrapping the borrow in `MRef::Ref`.
- [getters/option_auto.rs](../../../crates/tests/cgp-tests/tests/getters/option_auto.rs) — an `Option<&String>` return reading an `Option<String>` field via `.as_ref()`.
- [getters/slice_auto.rs](../../../crates/tests/cgp-tests/tests/getters/slice_auto.rs) — a `&[u8]` return reading an `AsRef<[u8]> + 'static` field via `.as_ref()`.
- [getters/non_self_auto.rs](../../../crates/tests/cgp-tests/tests/getters/non_self_auto.rs) — a non-`self` getter reading a field out of another type (`&Self::Foo`).
- [getters/auto_getter_generic.rs](../../../crates/tests/cgp-tests/tests/getters/auto_getter_generic.rs) — a trait generic over a type parameter, keyed by a `PhantomData<Foo>` tag.
- [getters/assoc_type_auto_getter.rs](../../../crates/tests/cgp-tests/tests/getters/assoc_type_auto_getter.rs) — a local associated return type inferred from the field, with a `Display` bound carried onto the impl.
- [getters/assoc_type_self_referential_auto.rs](../../../crates/tests/cgp-tests/tests/getters/assoc_type_self_referential_auto.rs) — a self-referential associated-type bound surviving onto the impl with `Self::Scalar` rewritten to the parameter.
- [getters/abstract_type_extend.rs](../../../crates/tests/cgp-tests/tests/getters/abstract_type_extend.rs), [getters/abstract_type_use_type.rs](../../../crates/tests/cgp-tests/tests/getters/abstract_type_use_type.rs) — getters whose return type is an abstract type imported via `#[extend]` and `#[use_type]`; each file pins both the auto and the full getter variant.

Coverage is broad; no distinct expansion variant is currently missing a snapshot.

## Tests

Each snapshot file also derives a concrete context and asserts the getter resolves, so the snapshot files carry the behavioral checks too:

- [getters/string_auto.rs](../../../crates/tests/cgp-tests/tests/getters/string_auto.rs) asserts `context.foo()` returns the `foo` field's contents as `&str`.
- [getters/assoc_type_auto_getter.rs](../../../crates/tests/cgp-tests/tests/getters/assoc_type_auto_getter.rs) confirms the associated type is inferred and the getter compiles under the `Display` bound.
- [getters/auto_getter_generic.rs](../../../crates/tests/cgp-tests/tests/getters/auto_getter_generic.rs) confirms the generic-parameter getter applies to any inferred field type.

## Source

- Entry point: `cgp_auto_getter` in [cgp-macro-lib/src/cgp_auto_getter.rs](../../../crates/macros/cgp-macro-lib/src/cgp_auto_getter.rs), which rejects any attribute argument and runs `ItemCgpAutoGetter::preprocess(...).to_items()`.
- The stack: [cgp-macro-core/src/types/cgp_auto_getter/](../../../crates/macros/cgp-macro-core/src/types/cgp_auto_getter/), documented in [asts/cgp_getter.md](../asts/cgp_getter.md): `item.rs` sets the `__Context__` identifier and drives field parsing, and `blanket.rs` builds the single blanket impl.
- Getter-method parsing and the return-type shorthands, shared with `#[cgp_getter]`: [cgp-macro-core/src/functions/getter/parse.rs](../../../crates/macros/cgp-macro-core/src/functions/getter/parse.rs), [cgp-macro-core/src/functions/field/parse.rs](../../../crates/macros/cgp-macro-core/src/functions/field/parse.rs), and [cgp-macro-core/src/types/getter/](../../../crates/macros/cgp-macro-core/src/types/getter/).
- Fragment construction: [parse_internal!](../macros/parse_internal.md).
