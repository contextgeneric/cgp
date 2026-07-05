# Reading context fields

A provider reads values from its context's fields, and this guide is about doing that with an argument that looks like an ordinary parameter rather than a getter trait declared just to fetch it.

This guide is the value-level counterpart to [writing providers](writing-providers.md) — the same move from visible machinery to an ordinary-looking parameter, applied to reading a field.

## Read context fields with implicit arguments, not getter traits

Read a value from a context field with an [`#[implicit]`](../reference/attributes/implicit.md) argument — in a [`#[cgp_impl]`](../reference/macros/cgp_impl.md) provider method just as in a [`#[cgp_fn]`](../reference/macros/cgp_fn.md) — rather than declaring a getter trait with [`#[cgp_auto_getter]`](../reference/macros/cgp_auto_getter.md). An implicit argument names both a local variable and the field it is read from, so the field access reads like an ordinary parameter and the `HasField` machinery stays out of sight. This is the default way to pull a field into a provider, and it covers the great majority of field reads: a value used throughout a body is bound once at the top, and a value shared across several methods is simply declared as an implicit argument on each. The getter-trait version pairs a `#[cgp_auto_getter]` declaration with a `#[uses(...)]` import:

```rust
#[cgp_auto_getter]
pub trait HasDimensions {
    fn width(&self) -> &f64;
    fn height(&self) -> &f64;
}

#[cgp_impl(new RectangleArea)]
#[uses(HasDimensions)]
impl AreaCalculator {
    fn area(&self) -> f64 {
        self.width() * self.height()
    }
}
```

collapses to a provider that reads the two fields directly:

```rust
#[cgp_impl(new RectangleArea)]
impl AreaCalculator {
    fn area(&self, #[implicit] width: f64, #[implicit] height: f64) -> f64 {
        width * height
    }
}
```

Use `#[cgp_auto_getter]` sparingly — only where an implicit argument cannot reach the field. Because an implicit argument reads from the provider's own `self` and takes a plain `&T` by reference without cloning, it covers every same-context read, including a field several providers each consume. A getter trait earns its keep in the three cases an implicit argument cannot serve: a field that lives on a type *other* than the provider's context, so the getter is required as a `where` bound on that type (`Request: HasBasicAuthHeader<Self>`, with no `self` field to read); an accessor that must exist as a *named* capability other code depends on through `#[uses(HasName)]` or a supertrait; and a getter whose associated type is inferred from the field (`type Name; fn name(&self) -> &Self::Name;`) so the type stays abstract for callers. Both idioms desugar to the same `HasField` bounds and share the same access rules — `.clone()` for an owned value, `.as_str()` for a `&str`, a plain `&T` by reference — so the choice is only about whether an implicit argument can reach the value.

Avoid [`#[cgp_getter]`](../reference/macros/cgp_getter.md) in ordinary code. It builds a full wireable component so the source field name can be chosen at wiring time through a [`UseField`](../reference/providers/use_field.md) provider, and that flexibility is reserved for the advanced case where you want full control over the context implementation — deciding per context which field a getter reads from, or supplying the value by means other than a same-named field. For the common case of reading a field, an implicit argument is the form to write, with `#[cgp_auto_getter]` held back for the getter-only cases above.

## Related guides

- [Writing providers](writing-providers.md) — the `#[cgp_impl]` provider these arguments live in.
- [Importing abstract types](importing-abstract-types.md) — for a getter whose return type is an abstract type shared across contexts.
- [Guides summary](README.md#summary) — the cheat-sheet across all the guides, and the cases where a getter trait is still the right tool.
