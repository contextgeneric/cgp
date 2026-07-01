# `#[derive(BuildField)]`

`#[derive(BuildField)]` derives just the incremental-builder machinery for a struct: a partial companion type plus the `HasBuilder`, `IntoBuilder`, `PartialData`, `FinalizeBuild`, and `UpdateField` impls that let the struct be assembled one field at a time.

## Purpose

`#[derive(BuildField)]` gives a struct a type-checked, field-by-field builder without the rest of the extensible-data machinery. It is the building block that supplies the *construction* half of a record: starting from an empty value, setting fields individually or copying them in bulk from other records, and finalizing only once every field is present. It exists as a standalone derive for cases where you want the builder but not the field representation traits — though in practice most code reaches for [`#[derive(CgpRecord)]`](derive_cgp_record.md) or [`#[derive(CgpData)]`](derive_cgp_data.md), which include this output.

The builder's distinguishing property is that field presence is tracked in the type, not at runtime. The derive generates a *partial* companion struct whose type parameters record, per field, whether that field is present, absent, or void. Generic builder providers advance those parameters as fields are added, and the finalize step is implemented only for the fully-present configuration. A premature finalize is therefore a compile error rather than a runtime failure.

The capability is exposed through `BuildField`, which is not generated per type but defined once in the field crate as a blanket impl over the generated `UpdateField` impls. `build_field` sets one field on a partial value; the surrounding `HasBuilder`/`IntoBuilder`/`FinalizeBuild` traits start and end the build.

## Syntax

The derive is applied to a struct and takes no arguments:

```rust
use cgp::prelude::*;

#[derive(BuildField)]
pub struct Person {
    pub first_name: String,
    pub last_name: String,
}
```

Each named field becomes a type-level string `Symbol!` used as the field's `Tag`, and its declared type becomes its value type. A tuple struct works equally well: an unnamed field at position `N` is keyed by [`Index<N>`](../types/index.md) instead of a `Symbol!`. Generic parameters on the struct are carried onto the generated impls. The derive emits the same builder impls that the record path of [`#[derive(CgpData)]`](derive_cgp_data.md) emits — it is that slice in isolation, with no `HasField` getters or `HasFields` representation traits.

## Expansion

`#[derive(BuildField)]` expands into a partial companion struct and the traits that drive it. The symbols below are abbreviated as `Symbol!("name")` in place of the full `Symbol<Len, Chars<...>>` form. Starting from:

```rust
#[derive(BuildField)]
pub struct Person {
    pub first_name: String,
    pub last_name: String,
}
```

it first emits the partial struct `__PartialPerson`, with one `MapType` parameter per field. The marker decides how each field is stored: `IsPresent` maps `T` to `T`, `IsNothing` maps `T` to `()`, and `IsVoid` maps `T` to the empty `Void` type:

```rust
pub struct __PartialPerson<__F0__: MapType, __F1__: MapType> {
    pub first_name: <__F0__ as MapType>::Map<String>,
    pub last_name: <__F1__ as MapType>::Map<String>,
}
```

It then emits the entry and exit points. `HasBuilder` starts an empty builder (`IsNothing, IsNothing`); `IntoBuilder` turns an existing value into a fully-present builder; `PartialData` records that any configuration of the partial type targets `Person`; and `FinalizeBuild` reconstructs the struct, implemented only for the all-`IsPresent` configuration:

```rust
impl HasBuilder for Person {
    type Builder = __PartialPerson<IsNothing, IsNothing>;
    fn builder() -> Self::Builder { __PartialPerson { first_name: (), last_name: () } }
}

impl IntoBuilder for Person {
    type Builder = __PartialPerson<IsPresent, IsPresent>;
    fn into_builder(self) -> Self::Builder { __PartialPerson { first_name: self.first_name, last_name: self.last_name } }
}

impl<__F0__: MapType, __F1__: MapType> PartialData for __PartialPerson<__F0__, __F1__> {
    type Target = Person;
}

impl FinalizeBuild for __PartialPerson<IsPresent, IsPresent> {
    fn finalize_build(self) -> Self::Target { Person { first_name: self.first_name, last_name: self.last_name } }
}
```

Next it emits, per field, an `UpdateField` impl. `UpdateField<Tag, M>` changes one field's marker from its current state to `M`, returning the old value alongside the rebuilt partial struct. This is the primitive that everything else builds on:

```rust
impl<__M1__: MapType, __M2__: MapType, __F1__: MapType>
    UpdateField<Symbol!("first_name"), __M2__> for __PartialPerson<__M1__, __F1__>
{
    type Value = String;
    type Mapper = __M1__;                                   // the field's prior marker
    type Output = __PartialPerson<__M2__, __F1__>;          // first_name now in state __M2__
    fn update_field(self, _: PhantomData<Symbol!("first_name")>, value: __M2__::Map<String>)
        -> (__M1__::Map<String>, Self::Output)
    { (self.first_name, __PartialPerson { first_name: value, last_name: self.last_name }) }
}
// plus the analogous UpdateField for "last_name"
```

Finally it emits, per field, a [`HasField`](../traits/has_field.md) impl on the partial type that is in scope only when that field's marker is `IsPresent`, so an already-set field can be read back out of a partially-built value:

```rust
impl<__F1__: MapType> HasField<Symbol!("first_name")> for __PartialPerson<IsPresent, __F1__> {
    type Value = String;
    fn get_field(&self, _: PhantomData<Symbol!("first_name")>) -> &String { &self.first_name }
}
// plus the analogous HasField for "last_name"
```

The `BuildField` trait itself is not emitted by the derive. It is defined once in the field crate as a blanket impl: any type implementing `UpdateField<Tag, IsPresent, Mapper = IsNothing>` — that is, setting a currently-absent field to present — automatically implements `BuildField<Tag>`. So `build_field` is sugar over the generated `update_field` that specifically transitions a field from `IsNothing` to `IsPresent`. `FinalizeBuild` is likewise defined in the field crate as a subtrait of `PartialData`; the derive only provides the all-present impl.

The key takeaway is that `builder()` yields `__PartialPerson<IsNothing, IsNothing>`, each `build_field` flips one marker to `IsPresent`, and `finalize_build` exists only at `__PartialPerson<IsPresent, IsPresent>` — so an incomplete build cannot be finalized.

## Examples

The builder is driven through `builder()`, `build_field`, and `finalize_build`, with `build_from` (from the field crate's `CanBuildFrom`) copying every shared field from another record at once:

```rust
use cgp::prelude::*;
use cgp::core::field::impls::CanBuildFrom;

#[derive(BuildField)]
pub struct FooBar { pub foo: u64, pub bar: String }

#[derive(BuildField)]
pub struct FooBarBaz { pub foo: u64, pub bar: String, pub baz: bool }

fn extend(foo_bar: FooBar) -> FooBarBaz {
    FooBarBaz::builder()
        .build_from(foo_bar)                              // sets foo + bar
        .build_field(PhantomData::<Symbol!("baz")>, true) // sets baz
        .finalize_build()
}
```

Each step changes the partial type, and only after the last field is set does the all-present `FinalizeBuild` impl apply.

## Related constructs

`#[derive(BuildField)]` is one slice of the record output of [`#[derive(CgpData)]`](derive_cgp_data.md) and [`#[derive(CgpRecord)]`](derive_cgp_record.md); those derives include it alongside the [`#[derive(HasField)]`](derive_has_field.md) getters and [`#[derive(HasFields)]`](derive_has_fields.md) representation traits. Its enum analogues are [`#[derive(ExtractField)]`](derive_extract_field.md) for incremental matching and [`#[derive(FromVariant)]`](derive_from_variant.md) for variant construction. The entry and exit points it generates target the [`HasBuilder`](../traits/has_builder.md) family of traits. The generated code reads back fields through [`HasField`](../traits/has_field.md), stores them in the [`product`](../macros/product.md)-shaped partial struct, and switches on the [`MapType`](../traits/map_type.md) markers `IsPresent`/`IsNothing`/`IsVoid`.

## Source

- Entry point: `derive_build_field` in [crates/macros/cgp-macro-lib/src/derive_build_field.rs](../../../crates/macros/cgp-macro-lib/src/derive_build_field.rs), which builds an `ItemCgpRecord` and calls `to_build_field_items()`.
- Codegen: that method, in [crates/macros/cgp-macro-core/src/types/cgp_data/record.rs](../../../crates/macros/cgp-macro-core/src/types/cgp_data/record.rs), composes the helpers in the [`derive_builder/`](../../../crates/macros/cgp-macro-core/src/types/cgp_data/derive_builder/) submodule (`builder_struct.rs`, `has_builder_impl.rs`, `into_builder_impl.rs`, `partial_data.rs`, `finalize_build_impl.rs`, `update_field_impls.rs`, `has_field_impls.rs`).
- Runtime traits: `BuildField` and `FinalizeBuild` in [crates/core/cgp-field/src/traits/build_field.rs](../../../crates/core/cgp-field/src/traits/build_field.rs), `UpdateField` in `update_field.rs`, `HasBuilder`/`IntoBuilder` in `has_builder.rs`, `PartialData` in `partial_data.rs`, and the `MapType` markers in `crates/core/cgp-field/src/impls/map_type.rs`.
- Internal walkthrough (the builder helpers, the corner-case handling, and the index of tests and expansion snapshots): [implementation/entrypoints/derive_build_field.md](../../implementation/entrypoints/derive_build_field.md).
