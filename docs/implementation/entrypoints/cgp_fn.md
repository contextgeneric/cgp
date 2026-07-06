# `#[cgp_fn]` — implementation

`#[cgp_fn]` turns one plain Rust function into a single-implementation capability by deriving a trait carrying the method and a blanket impl of that trait over a generic context, pulling every `#[implicit]` argument out of the signature and reading it from the context's fields. This document covers how that works internally; for the accepted syntax and the complete expansion a user sees, read the reference document [reference/macros/cgp_fn.md](../../reference/macros/cgp_fn.md).

## Entry point

The macro is driven by the thin `cgp_fn` function in [cgp-macro-lib/src/cgp_fn.rs](../../../crates/macros/cgp-macro-lib/src/cgp_fn.rs), which parses the attribute into an optional trait-name `Ident` and the item into a `syn::ItemFn`, then runs the two-stage pipeline and emits the result.

```rust
let item_cgp_fn = ItemCgpFn { ident, item_fn };
let items = item_cgp_fn.preprocess()?.to_items()?;
```

Two failures surface at the boundary: applying the macro to a non-function item fails at `parse2::<ItemFn>`, and a malformed attribute (anything other than a single optional identifier) fails at `parse2::<Option<Ident>>`. All real logic lives in `cgp-macro-core`.

## Pipeline

The macro moves through two stages, each a method on the AST type the previous one produced; the [`cgp_fn` AST stack](../asts/cgp_fn.md) documents those types in full.

- **preprocess** normalizes the function into the pieces the emit stage needs: it derives the trait name (the attribute identifier, or the function name converted to PascalCase), extracts the `#[implicit]` arguments out of the signature and prepends their field-reading bindings to the body, splits the companion attributes (`#[uses]`, `#[extend]`, `#[use_type]`, and the rest) out of the raw attribute list, and moves the function's generics aside.
- **to_items** renders the two output items — the trait carrying the method signature, then the blanket impl over `__Context__` carrying the body.

## Generated items

`#[cgp_fn]` emits exactly two items in a fixed order: the trait with the method, and the blanket impl of that trait for the reserved context type `__Context__`. There is no provider struct, no component marker, and no wiring — the method becomes available on every context that satisfies the impl's `where` clause.

The interesting transform is what happens to an `#[implicit]` argument. The argument is removed from the method signature entirely, its field is required as a `HasField` bound on the impl, and a `let` binding that reads the field is spliced onto the front of the body so the original body sees the name unchanged:

```rust
// input
#[cgp_fn]
pub fn greet(&self, #[implicit] name: &str) {
    println!("Hello, {}!", name);
}

// derived trait — the implicit argument is gone from the signature
pub trait Greet {
    fn greet(&self);
}

// derived blanket impl — the field is a bound, the binding is prepended
impl<__Context__> Greet for __Context__
where
    Self: HasField<Symbol!("name"), Value = String>,
{
    fn greet(&self) {
        let name: &str = self.get_field(PhantomData::<Symbol!("name")>).as_str();
        println!("Hello, {}!", name);
    }
}
```

The conversion applied to each binding is chosen by the argument's type, following the same field-mode rules the getter macros use: a `&str` argument reads a `String` field and appends `.as_str()`, an owned value — a path type, tuple, or array — appends `.clone()`, an `Option<&T>` reads an `Option<T>` field and appends `.as_ref()`, an `Option<&str>` reads an `Option<String>` field and appends `.as_deref()`, an `&[T]` reads an `AsRef<[T]>` field and appends `.as_ref()`, and a plain `&T` is taken by reference with no conversion. Each reference mode has a mutable mirror selected by a `&mut` in the argument's type: a `&mut T` reads through `HasFieldMut`/`get_field_mut`; a `&mut [T]` reads an `AsMut<[T]>` field and appends `.as_mut()`; an `Option<&mut T>` reads an `Option<T>` field and appends `.as_mut()`; an `Option<&mut str>` appends `.as_deref_mut()`. Every mutable read requires a `&mut self` receiver, while every immutable argument reads through `HasField`/`get_field` regardless of the receiver. The slice and option modes are keyed off the *argument's own* reference, not the receiver's, so an immutable `&[T]` keeps its shared `AsRef<[T]>` bound even under a `&mut self` receiver rather than being forced mutable. These modes are shared with `#[cgp_auto_getter]` and `#[cgp_getter]` through the [field-parsing helpers](../asts/cgp_getter.md); the difference is only where the read lands — a prepended `let` in the body here, a getter-method body there.

## Behavior and corner cases

**Generic parameters and the `where` clause split deliberately.** Every generic parameter in the function's `<...>` list is copied onto both the generated trait and the impl, while the function's own `where` clause is treated as an impl-side dependency and lands only on the impl, hidden from the trait interface. The implicit `HasField` bounds are always appended last, after any attribute-contributed predicates, so the impl's `where` clause reads user-declared bounds first and field bounds last.

**The companion attributes are layered into the same two items.** `#[uses(Trait)]` and `#[extend(Trait)]` each push a `Self: Trait` predicate onto the impl; `#[extend(Trait)]` additionally makes `Trait` a supertrait of the generated trait, `#[extend_where(...)]` adds predicates to the trait's own `where` clause, `#[impl_generics(...)]` inserts parameters into the impl generics only, and `#[use_type]`/`#[use_provider]` transform the trait and impl as documented on their own pages. Any attribute the parser does not recognize (for example `#[async_trait]` or `#[allow(...)]`) is preserved as a raw attribute and re-attached to both the trait and the impl.

**The visibility is moved, not copied.** `preprocess` takes the function's visibility off the inner `ItemFn` and re-applies it to the generated trait, so the emitted method inside the trait is always inherited-visibility while the trait itself carries the `pub` the user wrote.

**A malformed `#[implicit]` attribute is rejected at extraction.** `#[implicit]` is recognized only as a bare `Meta::Path`; the extraction pass additionally checks for an `implicit`-named attribute in any other form (`#[implicit(...)]` or `#[implicit = ...]`) and returns a spanned error, rather than leaving the stray attribute on the parameter to surface downstream as an obscure `cannot find attribute 'implicit'` error.

**A mutable implicit argument must be the only implicit argument.** The `field_mut` of each argument follows the argument's own type — set from a `&mut` in the type, whether the outer reference of a `&mut T`/`&mut [T]` or the inner reference of an `Option<&mut T>`, not from the receiver — so a mutable argument reads through `get_field_mut` while every immutable argument reads through `get_field`, even under a `&mut self` receiver. Because a `get_field_mut` read borrows the whole context exclusively for the rest of the body, a mutable implicit cannot coexist with any other implicit read; extraction rejects the combination (`has_mutable && count > 1`) rather than emit a blanket impl that fails to borrow-check. Any number of purely immutable implicits, by contrast, are shared borrows and combine freely. A mutable argument additionally requires the `&mut self` receiver (checked in `parse_field_type`), and a `mut` *pattern* on any implicit argument is rejected outright. These checks are enforced during implicit-argument extraction.

## Known issues

`#[cgp_fn]` does not support generics on the desugared *method* itself — generic parameters are only ever lifted onto the trait and impl. A method-level generic is silently treated as a trait/impl generic rather than rejected, which is the intended limitation rather than a bug: method-level generics are considered an advanced case better written as an explicit blanket impl or a [`#[cgp_component]`](../../reference/macros/cgp_component.md) provider.

## Snapshots

Every `snapshot_cgp_fn!` invocation across the suite is indexed here, since these snapshots all belong to this entrypoint:

- [implicit_arguments/cgp_fn_greet.rs](../../../crates/tests/cgp-tests/tests/implicit_arguments/cgp_fn_greet.rs) — the canonical plain case: one `#[implicit]` `&str` argument dropped from the signature and read via `HasField` with `.as_str()` applied.
- [implicit_arguments/cgp_fn_custom_trait_name.rs](../../../crates/tests/cgp-tests/tests/implicit_arguments/cgp_fn_custom_trait_name.rs) — `#[cgp_fn(CanCalculateRectangleArea)]` overrides the generated trait name; two owned `f64` implicits each `.clone()`d.
- [implicit_arguments/cgp_fn_mutable.rs](../../../crates/tests/cgp-tests/tests/implicit_arguments/cgp_fn_mutable.rs) — `&mut self` with a mutable implicit argument, reading through `HasFieldMut`/`get_field_mut`.
- [implicit_arguments/cgp_fn_mut_self_immutable.rs](../../../crates/tests/cgp-tests/tests/implicit_arguments/cgp_fn_mut_self_immutable.rs) — a `&mut self` receiver with two *immutable* implicit arguments, showing they read through `HasField`/`get_field` (the access mode follows the argument, not the receiver) and that several immutable implicits may share a `&mut self` receiver.
- [implicit_arguments/cgp_fn_slice.rs](../../../crates/tests/cgp-tests/tests/implicit_arguments/cgp_fn_slice.rs) — an immutable `&[u8]` implicit under a `&mut self` receiver, keeping its `Value: AsRef<[u8]> + 'static` bound and `.as_ref()` read because the slice mode follows the argument's own reference, not the receiver's; also mixes an implicit with a plain explicit argument.
- [implicit_arguments/cgp_fn_mut_slice.rs](../../../crates/tests/cgp-tests/tests/implicit_arguments/cgp_fn_mut_slice.rs) — a mutable `&mut [u8]` implicit, reading through `HasFieldMut` with a `Value: AsMut<[u8]> + 'static` bound and a `get_field_mut(...).as_mut()` read, the mutable mirror of the shared-slice case.
- [implicit_arguments/cgp_fn_option_mut.rs](../../../crates/tests/cgp-tests/tests/implicit_arguments/cgp_fn_option_mut.rs) — a mutable `Option<&mut u8>` implicit, reading an `Option<u8>` field through `HasFieldMut` with a `get_field_mut(...).as_mut()` read that yields `Option<&mut u8>`.
- [implicit_arguments/cgp_fn_owned_tuple.rs](../../../crates/tests/cgp-tests/tests/implicit_arguments/cgp_fn_owned_tuple.rs) — an owned, non-path implicit (a tuple) read by value with `.clone()` and a `Value = (f64, f64)` bound, showing owned types beyond path types are accepted.
- [implicit_arguments/cgp_fn_option_str.rs](../../../crates/tests/cgp-tests/tests/implicit_arguments/cgp_fn_option_str.rs) — an `Option<&str>` implicit backed by an `Option<String>` field and read with `.as_deref()`, composing the `&str` and option modes.
- [implicit_arguments/cgp_fn_calling_fn.rs](../../../crates/tests/cgp-tests/tests/implicit_arguments/cgp_fn_calling_fn.rs) — one `#[cgp_fn]` capability depending on another through an explicit `where Self:` bound.
- [implicit_arguments/cgp_fn_multi_and_use_type.rs](../../../crates/tests/cgp-tests/tests/implicit_arguments/cgp_fn_multi_and_use_type.rs) — explicit and implicit arguments mixed, generic type parameters lifted onto the trait, `#[async_trait]` preserved as a raw attribute, and `#[use_type]` importing and renaming abstract types.
- [async_and_send/cgp_fn_async.rs](../../../crates/tests/cgp-tests/tests/async_and_send/cgp_fn_async.rs) — the canonical async expansion, an `async fn` combined with `#[async_trait]`.
- [generic_components/fn_generic_param.rs](../../../crates/tests/cgp-tests/tests/generic_components/fn_generic_param.rs) — a function generic over a type parameter, showing the parameter moved onto both trait and impl.
- [generic_components/fn_impl_generics.rs](../../../crates/tests/cgp-tests/tests/generic_components/fn_impl_generics.rs) — `#[impl_generics(...)]` adding a generic parameter to the impl only, not the trait.
- [impl_side_dependencies/fn_extend.rs](../../../crates/tests/cgp-tests/tests/impl_side_dependencies/fn_extend.rs) — `#[extend(...)]` adding a supertrait bound to the generated trait.
- [impl_side_dependencies/fn_uses.rs](../../../crates/tests/cgp-tests/tests/impl_side_dependencies/fn_uses.rs) — `#[uses(...)]` importing a `Self` trait bound as an impl-side dependency.
- [higher_order_providers/use_provider_fn.rs](../../../crates/tests/cgp-tests/tests/higher_order_providers/use_provider_fn.rs) — `#[use_provider]` on a `#[cgp_fn]`, borrowing another provider's behavior.
- [abstract_types/use_type_fn_alias.rs](../../../crates/tests/cgp-tests/tests/abstract_types/use_type_fn_alias.rs), [use_type_fn_equality.rs](../../../crates/tests/cgp-tests/tests/abstract_types/use_type_fn_equality.rs), [use_type_fn_extend.rs](../../../crates/tests/cgp-tests/tests/abstract_types/use_type_fn_extend.rs), [use_type_fn_foreign.rs](../../../crates/tests/cgp-tests/tests/abstract_types/use_type_fn_foreign.rs), [use_type_fn_foreign_equality.rs](../../../crates/tests/cgp-tests/tests/abstract_types/use_type_fn_foreign_equality.rs), [use_type_fn_nested_foreign.rs](../../../crates/tests/cgp-tests/tests/abstract_types/use_type_fn_nested_foreign.rs), [use_type_fn_deep_foreign.rs](../../../crates/tests/cgp-tests/tests/abstract_types/use_type_fn_deep_foreign.rs), [use_type_fn_equality_cross_trait.rs](../../../crates/tests/cgp-tests/tests/abstract_types/use_type_fn_equality_cross_trait.rs), [use_type_fn_foreign_equality_cross_trait.rs](../../../crates/tests/cgp-tests/tests/abstract_types/use_type_fn_foreign_equality_cross_trait.rs) — the `#[use_type]` variants (local alias, type-equality bound, `#[extend]` form, foreign generic parameter, a three-hop foreign chain, and their combinations). The `#[extend_where(...)]` attribute's effect on the trait's own `where` clause is pinned by [use_type_fn_nested_foreign.rs](../../../crates/tests/cgp-tests/tests/abstract_types/use_type_fn_nested_foreign.rs), where it adds a `Scalar: Copy` bound (rewritten to the two-hop path) alongside the `#[use_type]` imports.

## Tests

Because `#[cgp_fn]` emits a blanket impl, its snapshot tests double as behavioral checks — the snapshot file also derives a concrete context and asserts it implements the generated trait, so a compile of the file is itself the wiring check. The runtime assertions worth calling out:

- [implicit_arguments/cgp_fn_greet.rs](../../../crates/tests/cgp-tests/tests/implicit_arguments/cgp_fn_greet.rs) proves any context with a `name: String` field implements the generated `Greet` trait via a `CheckPerson` bound.
- [implicit_arguments/cgp_fn_calling_fn.rs](../../../crates/tests/cgp-tests/tests/implicit_arguments/cgp_fn_calling_fn.rs) confirms one capability calling another resolves through both blanket impls at run time.
- [impl_side_dependencies/fn_uses.rs](../../../crates/tests/cgp-tests/tests/impl_side_dependencies/fn_uses.rs) and [impl_side_dependencies/fn_extend.rs](../../../crates/tests/cgp-tests/tests/impl_side_dependencies/fn_extend.rs) exercise the two ways of stating a dependency and confirm the resulting bounds are satisfiable.

The failure cases pin the inputs `#[cgp_fn]` refuses during expansion, each asserting the entrypoint returns `Err`:

- [parser_rejections/cgp_fn.rs](../../../crates/tests/cgp-macro-tests/tests/parser_rejections/cgp_fn.rs) covers an implicit argument on a function with no `self` receiver, a `mut` binding pattern on an implicit argument, a `&mut` implicit argument that is not the sole implicit (which would borrow the context mutably and again at once), a malformed `#[implicit]` attribute carrying arguments (`#[implicit(...)]` or `#[implicit = ...]`), and the two mutable-reference forms under a `&self` receiver — a `&mut [T]` slice and an `Option<&mut T>` — each of which reads through `get_field_mut` and so requires `&mut self`.

## Source

- Entry point: `cgp_fn` in [cgp-macro-lib/src/cgp_fn.rs](../../../crates/macros/cgp-macro-lib/src/cgp_fn.rs).
- Pipeline and its AST types: [cgp-macro-core/src/types/cgp_fn/](../../../crates/macros/cgp-macro-core/src/types/cgp_fn/), documented in [asts/cgp_fn.md](../asts/cgp_fn.md).
- Implicit-argument extraction and field-reading bindings: [cgp-macro-core/src/functions/implicits/](../../../crates/macros/cgp-macro-core/src/functions/implicits/).
- Field-mode helpers: [cgp-macro-core/src/functions/field/](../../../crates/macros/cgp-macro-core/src/functions/field/).
- Companion-attribute parsing: [cgp-macro-core/src/types/attributes/function.rs](../../../crates/macros/cgp-macro-core/src/types/attributes/function.rs).
- Fragment construction: [parse_internal!](../macros/parse_internal.md).
