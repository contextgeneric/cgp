# Modern idioms: a migration guide

CGP offers a set of newer, higher-level idioms for writing components, providers, and wiring that read much closer to ordinary Rust — and this guide maps each older, more explicit form to the modern one you should prefer.

The explicit forms came first. Early CGP exposed the machinery directly: a provider was an inside-out `impl` of a provider trait, dependencies were spelled as `where` clauses, abstract types were pulled from supertraits and written in fully-qualified `<Self as Trait>::Type` form, and per-type dispatch went through a `UseDelegate` table. Those forms still work and are exactly what the macros desugar to, so you will keep reading them in generated code, in expansion documentation, and in existing codebases. The newer idioms exist to lower the barrier to entry: they let a provider look like an ordinary trait `impl`, a dependency look like a `use` import, and an abstract type look like a plain generic, so that a reader who knows Rust but not CGP can follow the code. **Prefer the modern idioms in all new code, and reach for an explicit form only when a construct genuinely cannot express the case.**

This guide is organized by the shift each idiom makes. The concepts it draws on are documented in full elsewhere: writing providers in [consumer and provider traits](consumer-and-provider-traits.md), dependency injection in [impl-side dependencies](impl-side-dependencies.md), field injection in [implicit arguments](implicit-arguments.md), abstract types in [abstract types](abstract-types.md), the provider-parameterized pattern in [higher-order providers](higher-order-providers.md), and per-type dispatch in [dispatching](dispatching.md) and [namespaces](namespaces.md). Each section below links to the reference document that owns the construct.

## Write providers with `#[cgp_impl]`, not the raw provider forms

A provider should be written with [`#[cgp_impl]`](../reference/macros/cgp_impl.md), which keeps `self`, `Self`, and the consumer method signatures, rather than with the lower-level [`#[cgp_provider]`](../reference/macros/cgp_provider.md) or [`#[cgp_new_provider]`](../reference/macros/cgp_new_provider.md), which require the inside-out provider-trait shape. The lower forms move the context into an explicit leading type parameter and force the method to take `context: &Context` instead of `&self`; `#[cgp_impl]` restores the familiar shape and performs that rewrite for you. The legacy form:

```rust
#[cgp_new_provider]
impl<Context> AreaCalculator<Context> for RectangleArea
where
    Context: HasField<Symbol!("width"), Value = f64>,
    Context: HasField<Symbol!("height"), Value = f64>,
{
    fn area(context: &Context) -> f64 {
        *context.get_field(PhantomData::<Symbol!("width")>)
            * *context.get_field(PhantomData::<Symbol!("height")>)
    }
}
```

becomes, with the modern idiom and [`#[implicit]`](../reference/attributes/implicit.md) arguments:

```rust
#[cgp_impl(new RectangleArea)]
impl AreaCalculator {
    fn area(&self, #[implicit] width: f64, #[implicit] height: f64) -> f64 {
        width * height
    }
}
```

`#[cgp_impl]` desugars back to `#[cgp_provider]`/`#[cgp_new_provider]`, so the raw forms are still what the reference documents show in their Expansion sections and what you read in generated code. Write the raw form yourself only when you specifically need the inside-out shape — for instance, to state a bound the sugar cannot express.

## Omit the context parameter

Inside a `#[cgp_impl]` block, prefer the unqualified `impl AreaCalculator` and let the macro insert the context parameter, rather than naming it explicitly as `impl<Context> AreaCalculator for Context`. Omitting `for Context` is what makes the provider read like an ordinary trait `impl`; the macro supplies a reserved context parameter and treats `self`/`Self` as the context. Write the context out by hand:

```rust
#[cgp_impl(new RectangleArea)]
impl<Context> AreaCalculator for Context
where
    Context: HasDimensions,
{
    fn area(&self) -> f64 {
        self.width() * self.height()
    }
}
```

only when you must name it — to bound it with a lifetime or higher-ranked bound the sugar cannot spell, or to refer to it by a readable name. Otherwise write the shorter form and use `#[uses(...)]` for the bound:

```rust
#[cgp_impl(new RectangleArea)]
#[uses(HasDimensions)]
impl AreaCalculator {
    fn area(&self) -> f64 {
        self.width() * self.height()
    }
}
```

## Declare dependencies with `#[uses]` and `#[use_provider]`

State a provider's impl-side dependencies with [`#[uses(...)]`](../reference/attributes/uses.md) and [`#[use_provider(...)]`](../reference/attributes/use_provider.md) rather than hand-written `where` clauses, so a dependency reads like a `use` import instead of a trait bound. A capability the body calls on the context is imported with `#[uses]`: writing `#[uses(CanCalculateArea)]` adds `Self: CanCalculateArea` to the generated impl. An inner provider a higher-order provider delegates to is declared with `#[use_provider]`: writing `#[use_provider(InnerCalculator: AreaCalculator)]` adds the bound `InnerCalculator: AreaCalculator<Self>`, filling in the `<Self>` argument that a provider trait inserts. The legacy `where` forms:

```rust
#[cgp_impl(new ScaledArea<InnerCalculator>)]
impl<InnerCalculator> AreaCalculator for Context
where
    Self: HasField<Symbol!("scale_factor"), Value = f64>,
    InnerCalculator: AreaCalculator<Self>,
{
    fn area(&self) -> f64 { /* ... */ }
}
```

become:

```rust
#[cgp_impl(new ScaledArea<InnerCalculator>)]
#[use_provider(InnerCalculator: AreaCalculator)]
impl<InnerCalculator> AreaCalculator {
    fn area(&self, #[implicit] scale_factor: f64) -> f64 { /* ... */ }
}
```

`#[uses(...)]` accepts only the simple `Trait<Params>` form, so a bound with associated-type equality such as `Iterator<Item = u8>` must still be written as an explicit `where` clause. Both attributes desugar to the same `where` predicates they replace.

## Read context fields with implicit arguments, not getter traits

Read a value from a context field with an [`#[implicit]`](../reference/attributes/implicit.md) argument — in a [`#[cgp_impl]`](../reference/macros/cgp_impl.md) provider method just as in a [`#[cgp_fn]`](../reference/macros/cgp_fn.md) — rather than declaring a getter trait with [`#[cgp_auto_getter]`](../reference/macros/cgp_auto_getter.md). An implicit argument names both a local variable and the field it is read from, so the field access reads like an ordinary parameter and the `HasField` machinery stays out of sight — the same shift the provider idioms make, applied to values. This is the default way to pull a field into a provider, and it covers the great majority of field reads: a value used throughout a body is bound once at the top and used freely thereafter, and a value shared across several methods is simply declared as an implicit argument on each. The getter-trait version pairs a `#[cgp_auto_getter]` declaration with a `#[uses(...)]` import:

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

Reserve `#[cgp_auto_getter]` for when you genuinely want to publish a reusable getter *capability* rather than read a field for your own use — a named `self.name()` accessor that other providers depend on through `#[uses(HasName)]`, or a getter whose associated type is inferred from the field (`type Name; fn name(&self) -> &Self::Name;`). Both idioms desugar to the same `HasField` bounds and share the same access rules — `.clone()` for an owned value, `.as_str()` for a `&str` — so choosing between them is about whether the value is a private input or a published capability, not about mechanics.

Avoid [`#[cgp_getter]`](../reference/macros/cgp_getter.md) in ordinary code. It builds a full wireable component so the source field name can be chosen at wiring time through a [`UseField`](../reference/providers/use_field.md) provider, and that flexibility is reserved for the advanced case where you want full control over the context implementation — deciding per context which field a getter reads from, or supplying the value by means other than a same-named field. For the common case of reading a field, an implicit argument (or, for a published accessor, `#[cgp_auto_getter]`) is the form to write.

## Import abstract types with `#[use_type]`

Bring an abstract type into a definition with [`#[use_type]`](../reference/attributes/use_type.md) and write it as a bare alias, rather than declaring the owning trait as a supertrait and qualifying every use as `Self::Type`. The attribute does both jobs at once: `#[use_type(HasScalarType::Scalar)]` adds the trait as a supertrait (on a `#[cgp_component]`) or a `where` bound (on a `#[cgp_impl]`/`#[cgp_fn]`), and rewrites each bare `Scalar` to `<Self as HasScalarType>::Scalar`. This is the preferred form even for the built-in error type: the legacy component definition

```rust
#[cgp_component(Loader)]
pub trait CanLoad: HasErrorType {
    fn load(&self, path: &str) -> Result<String, Self::Error>;
}
```

becomes

```rust
#[cgp_component(Loader)]
#[use_type(HasErrorType::Error)]
pub trait CanLoad {
    fn load(&self, path: &str) -> Result<String, Error>;
}
```

One rule bounds the rewrite: it fires only on the bare identifier of an *imported* type. A construct's own **local associated type always stays qualified as `Self::Assoc`** — a handler that declares `type Output` writes `Self::Output`, never a bare `Output`, because `Output` is the trait's own type rather than one imported from another trait. A mixed signature such as `Result<Self::Output, Error>` is therefore exactly right: the local `Self::Output` stays qualified while the imported foreign `Error` is written bare. When a capability supertrait has no associated type to import, add it with [`#[extend]`](../reference/attributes/extend.md) rather than `#[use_type]`, as the next section describes.

## Add supertraits with `#[extend]`, not native `:` syntax

Add a non-type capability supertrait to a [`#[cgp_component]`](../reference/macros/cgp_component.md) trait with [`#[extend(...)]`](../reference/attributes/extend.md), rather than writing the native `pub trait CanDoX: Supertrait` form. Both produce the same trait with the same supertrait, but the attribute reads as an import — a capability the trait re-exports — which matches how CGP actually uses supertraits: as declared dependencies, not as a base class. Native `:` supertrait syntax tends to read as inheritance to programmers coming from object-oriented languages, suggesting an is-a relationship to a parent that a CGP component does not have. `#[extend(...)]` avoids that misreading and pairs symmetrically with [`#[uses(...)]`](../reference/attributes/uses.md): `#[uses]` imports a capability for the implementation's private use, `#[extend]` re-exports one as part of the trait's public contract. The native form:

```rust
#[cgp_component(Greeter)]
pub trait CanGreet: HasName {
    fn greet(&self) -> String;
}
```

becomes:

```rust
#[cgp_component(Greeter)]
#[extend(HasName)]
pub trait CanGreet {
    fn greet(&self) -> String;
}
```

`#[extend]` is the tool for a supertrait that contributes only a *capability* — like `HasName` here, which `CanGreet` depends on but whose value it reads through the getter rather than naming an abstract type in the signature. When the supertrait is instead an **abstract-type component** whose associated type the signature does name, use [`#[use_type]`](../reference/attributes/use_type.md) instead, exactly as the previous section showed with `HasErrorType`: `#[use_type]` adds the supertrait *and* rewrites the bare type, which `#[extend]` does not, so it is the recommended form for abstract-type components. In [`#[cgp_fn]`](../reference/macros/cgp_fn.md), whose `where` clauses are impl-side dependencies, `#[extend]` is the only way to declare a supertrait at all.

## Dispatch per type with `open` and namespaces, not `UseDelegate`

Route a generic-parameter component to a different provider per type with the [`open` statement](../reference/macros/delegate_components.md) or a [namespace](namespaces.md), rather than the legacy [`UseDelegate`](../reference/providers/use_delegate.md) nested-table pattern. Both the `open` statement and namespaces dispatch through the `RedirectLookup` impl that every [`#[cgp_component]`](../reference/macros/cgp_component.md) already generates, so they store the per-type entries directly on the context and need no wrapper type. The legacy form nests a `UseDelegate` table:

```rust
delegate_components! {
    MyApp {
        AreaCalculatorComponent:
            UseDelegate<new AreaCalculatorComponents {
                Rectangle: RectangleArea,
                Circle: CircleArea,
            }>,
    }
}
```

while the modern form dispatches inline with `open`:

```rust
delegate_components! {
    MyApp {
        open { AreaCalculatorComponent };

        @AreaCalculatorComponent.Rectangle: RectangleArea,
        @AreaCalculatorComponent.Circle: CircleArea,
    }
}
```

Because `open` and namespaces ride `RedirectLookup`, **a new component you intend to dispatch this way does not need the [`#[derive_delegate(UseDelegate<Param>)]`](../reference/attributes/derive_delegate.md) attribute at all** — that attribute exists only to generate the `UseDelegate` provider the legacy nested-table form relies on. You will still see `#[derive_delegate]` on some CGP-shipped components, such as the error and handler families, which carry it so existing `UseDelegate`-based wiring keeps working; but code that dispatches only through `open` or a namespace can omit it. Prefer `open` for a self-contained context wiring its own components, and a [namespace](namespaces.md) when a reusable, inheritable dispatch table is worth sharing across contexts.

## When the explicit forms are still right

A handful of cases genuinely need an explicit form, and reaching for one there is not a regression. Keep an explicit `where` clause for a bound `#[uses]` cannot express — anything with associated-type equality. Name the context explicitly, as `impl<Context> Trait for Context`, when you must attach a lifetime or higher-ranked bound the sugar cannot carry. Reach for [`#[cgp_getter]`](../reference/macros/cgp_getter.md) when you specifically want full control over which field a getter reads from, chosen per context at wiring time. Write a raw provider-trait `impl` when you need the inside-out shape directly, for instance a provider whose `Self` is a concrete context rather than a generic one. And keep a local associated type qualified as `Self::Output` always — it is never a `#[use_type]` import. In every other case, the modern idiom is the one to write.

Further reference: the per-construct mechanics live in the reference documents linked above; [modularity hierarchy](modularity-hierarchy.md) frames how much CGP a problem needs, and [consumer and provider traits](consumer-and-provider-traits.md) explains the duality the provider idioms rest on.
