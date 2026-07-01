# `#[cgp_getter]`

`#[cgp_getter]` defines a getter as a full CGP component — the same convenience as [`#[cgp_auto_getter]`](cgp_auto_getter.md), but wired through [`#[cgp_component]`](cgp_component.md) and backed by a [`UseField`](../providers/use_field.md) provider so the field name can differ from the method name and the getter can be swapped per context.

## Purpose

`#[cgp_getter]` exists for the advanced case where a getter must participate in CGP wiring rather than resolve to a single blanket impl. It is a specialized tool, not a default: most getters read a same-named field, which an [`#[implicit]`](../attributes/implicit.md) argument (for a private read) or [`#[cgp_auto_getter]`](cgp_auto_getter.md) (for a published accessor) handles with no wiring. Reserve `#[cgp_getter]` for when a context needs full control over the getter's implementation — storing the value under a different field name, or supplying it in different ways per context. It delivers that control by making the getter a genuine component with a provider trait, a component name, and a delegation entry.

The trade-off against `#[cgp_auto_getter]` is wireable versus blanket. `#[cgp_auto_getter]` emits one blanket impl that fires automatically for any context whose field name matches the method name; there is nothing to wire and nothing to choose. `#[cgp_getter]` emits a component that a context must wire to a provider, but in exchange the field name is decoupled from the method name and the implementation can be selected per context. You pay one line of wiring to gain that decoupling.

The decoupling is delivered through the `UseField` pattern. `#[cgp_getter]` automatically generates a `UseField` provider impl for the getter, so a context wires the getter to `UseField<Symbol!("...")>` and names whichever field it actually stores the value in — the field name lives in the wiring, not in the trait. Like any getter trait, a `#[cgp_getter]` trait can also be implemented directly on a concrete context when no wiring is desired.

## Syntax

The macro is applied to a getter trait the same way `#[cgp_auto_getter]` is, and accepts the same getter-method forms — `&self`/`&mut self` receivers and the `&str`, `Option<&T>`, `&[T]`, owned, and associated-type return shorthands. The simplest form takes no argument:

```rust
#[cgp_getter]
pub trait HasName {
    fn name(&self) -> &str;
}
```

Because `#[cgp_getter]` is an extension of `#[cgp_component]`, it needs a provider trait name, and it derives one from the trait name by default. When the trait name begins with `Has`, the macro strips that prefix and appends `Getter`, so `HasName` yields the provider `NameGetter` (and component name `NameGetterComponent`). You can override the provider name by passing it as an argument, exactly as with `#[cgp_component]`:

```rust
#[cgp_getter(GetName)]
pub trait HasName {
    fn name(&self) -> &str;
}
```

Here the provider trait is named `GetName` and the component `GetNameComponent`. The defaulting rule means `#[cgp_getter]` is at its most ergonomic when getter traits follow the `Has{Field}` naming convention.

## Syntax Grammar

The attribute argument of `#[cgp_getter]` is the same grammar as [`#[cgp_component]`](cgp_component.md)'s `CgpComponentArgs` — a bare provider name or the keyed `name`/`provider`/`context` form:

```ebnf
CgpGetterArgs -> CgpComponentArgs    // see #[cgp_component]
```

The only difference from `#[cgp_component]` is the default applied when `provider` is omitted: the macro derives the provider name from the trait name by stripping a leading `Has` and appending `Getter` (so `HasName` yields `NameGetter`). All other keys and their defaults behave exactly as documented for `#[cgp_component]`.

## Expansion

`#[cgp_getter]` expands to everything `#[cgp_component]` emits, plus a set of getter-specific provider impls. The component part is identical to a `#[cgp_component(NameGetter)]` definition — the consumer trait, the provider trait, the consumer and provider blanket impls, the `NameGetterComponent` marker, and the standard `UseContext` and `RedirectLookup` provider impls (see [`#[cgp_component]`](cgp_component.md) for that core expansion). On top of those, the macro adds the getter providers described below. A `UseFields` provider is always emitted; the `UseField` and `WithProvider` providers are emitted only when the getter trait has exactly one method, since both presuppose a single field to read.

The most important addition is the `UseField` provider impl, which is what lets the field name differ from the method name. Starting from the single-method trait:

```rust
#[cgp_getter]
pub trait HasName {
    fn name(&self) -> &str;
}
```

the macro generates this impl for `UseField<__Tag__>`, where `__Tag__` is a free generic parameter standing in for the field name the context will choose at wiring time:

```rust
impl<__Context__, __Tag__> NameGetter<__Context__> for UseField<__Tag__>
where
    __Context__: HasField<__Tag__, Value = String>,
{
    fn name(__context__: &__Context__) -> &str {
        __context__.get_field(PhantomData::<__Tag__>).as_str()
    }
}
```

This is the contrast with `#[cgp_auto_getter]`, whose blanket impl hard-codes the tag to `Symbol!("name")`. Here the tag is a parameter, so wiring to `UseField<Symbol!("first_name")>` supplies `first_name` as `__Tag__` and the getter reads that field instead. The `&str` shorthand is handled the same way in both macros: the `Value` is `String` and the body appends `.as_str()`.

When the getter trait contains exactly one method, the macro additionally generates a `WithProvider` impl, which adapts a [field-getter](../providers/use_field.md) provider into the getter component:

```rust
impl<__Context__, __Provider__> NameGetter<__Context__> for WithProvider<__Provider__>
where
    __Provider__: FieldGetter<__Context__, NameGetterComponent, Value = String>,
{
    fn name(__context__: &__Context__) -> &str {
        __Provider__::get_field(__context__, PhantomData::<NameGetterComponent>).as_str()
    }
}
```

Finally, the macro generates a `UseFields` impl — the analogue of the `#[cgp_auto_getter]` blanket impl, but as a provider — which reads each method's field keyed by the method name as a `Symbol!`:

```rust
impl<__Context__> NameGetter<__Context__> for UseFields
where
    __Context__: HasField<Symbol!("name"), Value = String>,
{
    fn name(__context__: &__Context__) -> &str {
        __context__.get_field(PhantomData::<Symbol!("name")>).as_str()
    }
}
```

Each of these provider impls is paired with a matching `IsProviderFor` impl carrying the same `where` bounds, so that delegation propagates the dependency and check traits can report missing fields precisely. As elsewhere, the desugarings show `Symbol!("name")` in sugared form rather than its expanded `Symbol<...>` representation.

## Examples

A typical use wires the getter to `UseField` with a field name that differs from the method name, which is the case `#[cgp_auto_getter]` cannot express. The context stores the value in `first_name`, while the trait method is `name`:

```rust
use cgp::prelude::*;

#[cgp_getter]
pub trait HasName {
    fn name(&self) -> &str;
}

#[derive(HasField)]
pub struct Person {
    pub first_name: String,
}

delegate_components! {
    Person {
        NameGetterComponent: UseField<Symbol!("first_name")>,
    }
}

fn greet(person: &Person) {
    println!("Hello, {}!", person.name()); // reads the first_name field
}
```

Because `Person` wires `NameGetterComponent` to `UseField<Symbol!("first_name")>`, the generated `UseField` provider reads `Person`'s `first_name` field to implement `name()`, so `person.name()` returns the value stored in `first_name`.

As with any getter trait, you can skip the wiring entirely and implement the consumer trait directly on a concrete context:

```rust
pub struct Person {
    pub full_name: String,
}

impl HasName for Person {
    fn name(&self) -> &str {
        &self.full_name
    }
}
```

The direct implementation is the most transparent option and shows that a `#[cgp_getter]` trait is, at bottom, an ordinary CGP component whose consumer trait can be implemented like any Rust trait.

## Related constructs

`#[cgp_getter]` is the wireable counterpart to [`#[cgp_auto_getter]`](cgp_auto_getter.md): the latter emits a single `HasField` blanket impl keyed by the method name, while `#[cgp_getter]` emits a full component plus a `UseField` provider so the field name can be chosen at wiring time. It is built on [`#[cgp_component]`](cgp_component.md), inheriting that macro's entire expansion and provider-name defaulting, and it is wired with [`delegate_components!`](delegate_components.md) and verified with [`check_components!`](check_components.md). The generated provider keys off [`UseField`](../providers/use_field.md) and reads fields produced by [`#[derive(HasField)]`](../derives/derive_has_field.md), keyed by [`Symbol!`](symbol.md). When the getter's return type is an abstract associated type, the construct overlaps with [`#[cgp_type]`](cgp_type.md).

## Source

The macro entry point is `cgp_getter` in [crates/macros/cgp-macro-lib/src/cgp_getter.rs](../../../crates/macros/cgp-macro-lib/src/cgp_getter.rs), which derives the default provider name (strip `Has`, append `Getter`), runs the `#[cgp_component]` `preprocess → eval` pipeline, then converts the result into `ItemCgpGetter` and emits the extra provider impls. The logic lives in [crates/macros/cgp-macro-core/src/types/cgp_getter/](../../../crates/macros/cgp-macro-core/src/types/cgp_getter/): `item.rs` assembles the items, `use_field.rs` builds the `UseField` impl with the free `__Tag__` parameter, `to_use_fields_impl.rs` builds the `UseFields` impl keyed by method name, and `with_provider.rs` builds the `WithProvider` impl. Getter-method parsing and the return-type shorthands are shared with `#[cgp_auto_getter]` in [crates/macros/cgp-macro-core/src/functions/getter/parse.rs](../../../crates/macros/cgp-macro-core/src/functions/getter/parse.rs) and [crates/macros/cgp-macro-core/src/types/getter/](../../../crates/macros/cgp-macro-core/src/types/getter/). Behavioral and expansion-snapshot tests are in [crates/tests/cgp-tests/tests/getters/](../../../crates/tests/cgp-tests/tests/getters/) (notably `string.rs` for the full `UseField`/`UseFields`/`WithProvider` expansion).
