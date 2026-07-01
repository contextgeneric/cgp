# Implicit arguments

Implicit arguments are a programming model in which a provider is written like an ordinary function taking arguments, where arguments marked `#[implicit]` are sourced not from the caller but from fields of the context.

## The idea

A provider almost always needs values from its context, and CGP makes asking for them look like asking for function parameters. Instead of declaring a `HasField` bound and calling `get_field` inside the body, the author writes a normal-looking parameter — `#[implicit] width: f64` — that reads as "this function needs a `width` of type `f64`." The macro removes that parameter from the public signature, adds the matching field bound, and binds the value at the top of the body, so the code keeps the shape of a plain function while behaving like a provider that pulls its inputs from the context.

This framing is the recommended on-ramp to CGP precisely because it requires no new vocabulary. A programmer who understands functions and arguments can write a complete provider without first meeting `HasField`, type-level symbols, or `PhantomData` tags; those mechanics stay hidden behind a parameter that looks ordinary. The model defers the underlying machinery until the author has a reason to care about it.

## How CGP expresses it

An implicit argument is a function parameter carrying the [`#[implicit]`](../reference/attributes/implicit.md) attribute, whose name doubles as the context field name it reads from. The attribute is meaningful inside the two macros that rewrite function bodies into providers: [`#[cgp_fn]`](../reference/macros/cgp_fn.md), which turns a free function into a single-implementation capability, and [`#[cgp_impl]`](../reference/macros/cgp_impl.md), which writes a provider for an existing component. In both, the marked argument disappears from the signature and is fetched from the context instead.

```rust
use cgp::prelude::*;

#[cgp_fn]
pub fn rectangle_area(&self, #[implicit] width: f64, #[implicit] height: f64) -> f64 {
    width * height
}
```

The desugaring is the impl-side dependency pattern applied to values: each implicit argument becomes a [`HasField`](../reference/traits/has_field.md) bound on the generated impl and a `let` binding that reads the field at the top of the body. The function above generates a `RectangleArea` trait whose method takes no arguments, and a blanket impl requiring `HasField<Symbol!("width"), Value = f64>` and the matching `height` bound, with `width` and `height` read from the context before the body runs. The same rewrite happens identically inside a [`#[cgp_impl]`](../reference/macros/cgp_impl.md) method, where the bounds join the provider impl's `where` clause. The field bounds are satisfied by any context that derives [`#[derive(HasField)]`](../reference/derives/derive_has_field.md) and carries fields of the matching names — no wiring is involved for `#[cgp_fn]`, since its blanket impl applies to every context that has the fields.

Because the argument name is the field name, the values are addressed by name through the type-level symbol [`HasField`](../reference/traits/has_field.md) uses internally, and the whole `Symbol!`/`get_field`/`PhantomData` apparatus stays out of sight. What the author sees is a function with arguments; what the compiler sees is a provider with injected dependencies.

## Access rules

The argument type controls how the field is read, following a small set of rules so that the body receives a value of exactly the declared type. The default rule is that an owned argument type, such as `f64` or `String`, reads the field by reference and appends `.clone()`, so the body gets an owned value while the context keeps its field. The one special case worth memorizing is `&str`: an argument typed `&str` is backed by a `String` field and read with `.as_str()` rather than `.clone()`, letting the body borrow without forcing the context to store a `&str`.

These same rules govern getter access in [`#[cgp_auto_getter]`](../reference/macros/cgp_auto_getter.md), so an author who learns them once applies them everywhere field values are read. The shared rule of thumb is that the declared type is what the body works with, and the macro inserts whatever conversion — a clone for owned values, an `.as_str()` for `&str` — bridges it to the stored field. A few further forms exist for references, options, and slices, all documented with [`#[implicit]`](../reference/attributes/implicit.md).

## When to use which

An implicit argument is the default way to read a context field, and a getter trait is the exception reserved for publishing a reusable capability. Both read fields through `HasField` and share the same access rules, so the choice is not about mechanics but about what the value *is*. An implicit argument treats the value as a private input to one provider: it is bound as a local at the top of the method and used freely from there, which covers reading a field once, using it throughout a body, or declaring it on each of several methods that need it. This is the form to reach for whenever a provider simply needs a value from its context.

A getter trait written with [`#[cgp_auto_getter]`](../reference/macros/cgp_auto_getter.md) is worth defining only when the value is a *published capability* rather than a private input — a named `self.name()` accessor that other providers depend on through `#[uses(HasName)]`, or a getter whose associated type is inferred from the field so the type stays abstract. Declaring such a trait promotes the field to part of the context's capability surface. When all a provider wants is to read a field for its own computation, an implicit argument says exactly that with less ceremony, so prefer it and treat the getter trait as the deliberate step of exposing a shared capability. The full wireable getter, [`#[cgp_getter]`](../reference/macros/cgp_getter.md), is a further step still, reserved for the advanced case of choosing the source field per context at wiring time.

## Related constructs

The [`#[implicit]`](../reference/attributes/implicit.md) attribute is the construct itself, usable inside [`#[cgp_fn]`](../reference/macros/cgp_fn.md) and [`#[cgp_impl]`](../reference/macros/cgp_impl.md) — the former producing a single-implementation capability with no wiring, the latter a provider for a [`#[cgp_component]`](../reference/macros/cgp_component.md). All of them desugar implicit arguments into [`HasField`](../reference/traits/has_field.md) bounds, which a context supplies by deriving [`#[derive(HasField)]`](../reference/derives/derive_has_field.md).

For publishing a field as a shared, reusable accessor, [`#[cgp_auto_getter]`](../reference/macros/cgp_auto_getter.md) is the getter-trait counterpart that shares the same `.clone()`/`.as_str()` access rules — but a provider reading a field for its own use should prefer an implicit argument. The whole model is value-level [impl-side dependencies](impl-side-dependencies.md): an implicit argument is a context dependency injected through a `where`-clause `HasField` bound and dressed as a function parameter.
