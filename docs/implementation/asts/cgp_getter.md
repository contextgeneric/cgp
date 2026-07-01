# The `cgp_getter` and `cgp_auto_getter` AST stack

This stack covers the two getter macros together, because they share the getter-field parsing and the field-mode conversions and differ only in what they emit around a common getter-method body. `#[cgp_auto_getter]` parses a trait into `ItemCgpAutoGetter` and emits one blanket impl; `#[cgp_getter]` reuses the whole `#[cgp_component]` pipeline to produce an `EvaluatedCgpComponent`, wraps it in `ItemCgpGetter`, and appends three field-reading provider impls. Both feed on the shared `GetterField` parser and the `getter/` field-mode types. The two [entrypoint documents](../entrypoints/cgp_getter.md) cover what each stage produces; this document covers the types.

## Shared getter-field parsing: `GetterField` and `parse_getter_fields`

Both macros turn a getter trait's methods into `GetterField`s through the shared `parse_getter_fields` helper, which is the single source of truth for what a getter signature means. A `GetterField` records the field name (the method name), the field type to require, the return type, the receiver mutability, an optional `PhantomData` phantom-argument type, the field mode, and the receiver mode.

The parser enforces the getter-method contract and resolves the two shorthands a reader most needs to know. A getter method must be a plain (non-const, non-async, non-unsafe, non-generic) method whose first argument is a reference — either `&self` (`ReceiverMode::SelfReceiver`) or a typed `&SomeType` receiver (`ReceiverMode::Type`), the latter letting a getter read a field out of a type other than the context, with `Self` rewritten to the context. The return type then determines the field type and the *field mode* — the conversion the getter body applies — through the shared `parse_field_type`: a `&str` return reads a `String` field (`FieldMode::Str`, `.as_str()`), an `Option<&T>` reads `Option<T>` (`FieldMode::OptionRef`, `.as_ref()`), a `&[T]` reads an `AsRef<[T]>` field (`FieldMode::Slice`, `.as_ref()`), an `MRef<'_, T>` wraps the borrow (`FieldMode::MRef`, `MRef::Ref(...)`), a plain `&T` is a bare `FieldMode::Reference`, and any owned return is `FieldMode::Copy` (`.clone()`). `parse_getter_fields` also extracts an optional single associated return type and checks that, when present, the trait has exactly one method whose return type matches it.

The getter-method body itself is built by `derive_getter_method` from the `types/getter/` module, which emits `receiver.get_field(PhantomData::<Tag>) <conversion>` for the field's mode; the same `FieldMode` and `GetFieldWithModeExpr` types drive the `#[implicit]` bindings in the [`cgp_fn` stack](cgp_fn.md), which is why the two families convert fields identically.

## `ItemCgpAutoGetter`

`ItemCgpAutoGetter` is the whole AST for `#[cgp_auto_getter]` — a single struct holding the cleaned trait. Its `preprocess` associated function strips the CGP modifier attributes off the trait (discarding them, since the auto getter has no component to configure) and keeps the trait; there is no multi-stage pipeline because the macro emits no component.

Its `to_items` emits the trait unchanged plus one blanket impl, built by `to_blanket_impl` → `derive_blanket_impl`. That impl fixes the context type to `__Context__`, adds each getter method reading its like-named field, and requires the corresponding `HasField` bound; a trait supertrait becomes a `__Context__: Supertrait` predicate, a trait generic parameter is preserved onto the impl, and a single associated return type is added as an extra parameter set to itself with its bounds carried over. The shape of the emitted impl is shown in the [entrypoint document](../entrypoints/cgp_auto_getter.md).

## `EvaluatedCgpComponent` (reused) and `ItemCgpGetter`

`#[cgp_getter]` produces no getter-specific parse stage of its own; it drives the `#[cgp_component]` pipeline to an `EvaluatedCgpComponent` (documented in the [`cgp_component` AST stack](cgp_component.md)) and then wraps that in `ItemCgpGetter`. The `TryFrom<EvaluatedCgpComponent>` conversion is where the getter fields are parsed: it runs `parse_getter_fields` over the consumer trait and stores the resulting `GetterField`s and optional associated type alongside the evaluated component.

`ItemCgpGetter`'s `to_items` emits the component's own items first — the five core items plus the standard `UseContext`/`RedirectLookup` provider impls — and then appends the three getter-specific provider impls, each carrying its own `IsProviderFor` impl:

- `to_use_fields_impl` builds the `UseFields` impl, keyed by method name: for each field it emits the getter-method body reading `Symbol!("field_name")` and requires the matching `HasField` bound on the receiver type. Always emitted.
- `to_use_field_impl` builds the `UseField<__Tag__>` impl, where `__Tag__` is a *free* generic parameter added to the impl generics, so the getter reads whatever field the wiring supplies. Emitted only for a single-getter trait.
- `to_with_provider_impl` builds the `WithProvider<__Provider__>` impl, which delegates field access to an inner `FieldGetter`/`MutFieldGetter` provider (a slice field uses an `AsRef<[T]>`-valued bound). Emitted only for a single-getter trait.

Each of these threads the optional associated type through as an extra generic parameter, exactly as the auto-getter blanket impl does, keeping the three impls consistent with the consumer trait.

## Tests

- The stage transforms are exercised end-to-end by the expansion snapshots indexed in the two entrypoint documents' Snapshots sections — the [`#[cgp_getter]` snapshots](../entrypoints/cgp_getter.md) and the [`#[cgp_auto_getter]` snapshots](../entrypoints/cgp_auto_getter.md).
- There is no separate parser-rejection test file for either macro in `cgp-macro-tests`; the getter-method contract checks in `parse_getter_fields` (rejecting const/async/unsafe/generic getters, a bad receiver, or multiple associated types) are currently unpinned by dedicated failure cases.

## Source

- The auto-getter stack lives in [cgp-macro-core/src/types/cgp_auto_getter/](../../../crates/macros/cgp-macro-core/src/types/cgp_auto_getter/) (`item.rs`, `blanket.rs`), driven by [cgp-macro-lib/src/cgp_auto_getter.rs](../../../crates/macros/cgp-macro-lib/src/cgp_auto_getter.rs) and documented in [entrypoints/cgp_auto_getter.md](../entrypoints/cgp_auto_getter.md).
- The full-getter stack lives in [cgp-macro-core/src/types/cgp_getter/](../../../crates/macros/cgp-macro-core/src/types/cgp_getter/) (`item.rs`, `getter_field.rs`, `to_use_fields_impl.rs`, `use_field.rs`, `with_provider.rs`), driven by [cgp-macro-lib/src/cgp_getter.rs](../../../crates/macros/cgp-macro-lib/src/cgp_getter.rs) and documented in [entrypoints/cgp_getter.md](../entrypoints/cgp_getter.md); it reuses the [`cgp_component` stack](cgp_component.md).
- The shared getter-field parser is in [cgp-macro-core/src/functions/getter/parse.rs](../../../crates/macros/cgp-macro-core/src/functions/getter/parse.rs), the field-mode conversion in [cgp-macro-core/src/functions/field/parse.rs](../../../crates/macros/cgp-macro-core/src/functions/field/parse.rs), and the field-mode and getter-method emit types in [cgp-macro-core/src/types/getter/](../../../crates/macros/cgp-macro-core/src/types/getter/).
