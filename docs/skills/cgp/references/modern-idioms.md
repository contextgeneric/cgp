# Modern idioms: reading and modernizing CGP

The map from each older, explicit CGP form to the modern idiom you should prefer — so you can recognize legacy syntax in existing code, understand what it desugars to, and rewrite it in the vanilla-looking form.

CGP's explicit forms came first, and they are still exactly what the macros desugar to, so you will keep meeting them in generated code, in reference Expansion sections, and in any codebase written before the newer idioms landed. The modern idioms exist to lower the barrier to entry: they let a provider look like an ordinary trait `impl`, a dependency look like a `use` import, and an abstract type look like a plain generic. **Prefer the modern idiom in all new code, and reach for an explicit form only when a construct genuinely cannot express the case** (the closing section lists those exceptions). This file is the bidirectional reference: read it forward to write modern CGP, and backward to decode legacy CGP you are asked to read or update. Assume `use cgp::prelude::*;`; the CGP version is v0.8.0.

Each shift below shows the legacy form, the modern equivalent, and the mechanical rule that connects them. All of them desugar to the same generated code, so a rewrite is a readability change, never a behavioral one.

## Provider shape: `#[cgp_impl]` over the raw provider forms

Write a provider with [`#[cgp_impl]`](components.md), which keeps `self`, `Self`, and the consumer method signatures, rather than the lower-level [`#[cgp_provider]`/`#[cgp_new_provider]`](components.md), which require the inside-out provider-trait shape (the context moved to a leading type parameter, the receiver written as `context: &Context`). The legacy form:

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

becomes, with `#[cgp_impl]` and [`#[implicit]`](functions-and-getters.md) arguments:

```rust
#[cgp_impl(new RectangleArea)]
impl AreaCalculator {
    fn area(&self, #[implicit] width: f64, #[implicit] height: f64) -> f64 {
        width * height
    }
}
```

`#[cgp_impl]` desugars back to `#[cgp_provider]`/`#[cgp_new_provider]`, so the raw forms remain what you read in expansions. Write the raw form yourself only when you need the inside-out shape directly — for instance a provider whose `Self` is a concrete context, or a bound the sugar cannot spell.

## Context parameter: omit `for Context`

Inside a `#[cgp_impl]` block, prefer the unqualified `impl AreaCalculator` and let the macro insert the reserved context parameter, rather than naming it as `impl<Context> AreaCalculator for Context`. Omitting `for Context` is what makes the provider read like an ordinary trait `impl`. Name the context explicitly only when you must bound it with a lifetime or higher-ranked bound the sugar cannot carry, or refer to it by a readable name. So the legacy

```rust
#[cgp_impl(new RectangleArea)]
impl<Context> AreaCalculator for Context
where
    Context: HasDimensions,
{
    fn area(&self) -> f64 { self.width() * self.height() }
}
```

shortens to the bare header, with the bound moved to `#[uses]`:

```rust
#[cgp_impl(new RectangleArea)]
#[uses(HasDimensions)]
impl AreaCalculator {
    fn area(&self) -> f64 { self.width() * self.height() }
}
```

## Dependencies: `#[uses]` and `#[use_provider]` over hand-written `where`

State a provider's [impl-side dependencies](components.md) with [`#[uses(...)]`](functions-and-getters.md) and [`#[use_provider(...)]`](higher-order-providers.md) rather than hand-written `where` clauses, so a dependency reads like a `use` import. `#[uses(CanCalculateArea)]` adds `Self: CanCalculateArea`; `#[use_provider(Inner: AreaCalculator)]` adds `Inner: AreaCalculator<Self>`, filling in the `<Self>` context argument a provider-trait bound needs. The legacy `where` forms:

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

become the attribute forms plus an implicit argument:

```rust
#[cgp_impl(new ScaledArea<InnerCalculator>)]
#[use_provider(InnerCalculator: AreaCalculator)]
impl<InnerCalculator> AreaCalculator {
    fn area(&self, #[implicit] scale_factor: f64) -> f64 { /* ... */ }
}
```

`#[uses]` accepts any bound a `where` clause allows, including one carrying associated-type equality, though the simple `Trait<Params>` form is idiomatic. Where that bound pins an **abstract type** — `Self: HasErrorType<Error = AppError>` — express it with the [`#[use_type]` equality form](#abstract-types-use_type-over-supertrait--selftype) below, rather than spelling the equality in `#[uses]` or writing a hand-written `where` clause; only equality on a trait you would never import from with `#[use_type]` (`Iterator<Item = u8>`) stays an explicit `where` clause. When a provider imports several capabilities or binds several inner providers, combine them into one attribute separated by commas — `#[uses(CanQueryUserBalance, CanRaiseHttpError<ErrNotFound, String>)]`, `#[use_provider(A: TraitA, B: TraitB)]` — rather than stacking the same attribute repeatedly; one combined attribute reads as a single dependency list. Both attributes also accept being split across repeated attributes, but do that only when a real reason calls for it.

## Field reads: `#[implicit]` over a getter trait

Read a value from a context field with an [`#[implicit]`](functions-and-getters.md) argument — in a `#[cgp_impl]` method exactly as in `#[cgp_fn]` — rather than declaring a getter trait and importing it. This is the default for *any* field a provider reads from its own context, since an implicit argument reads from `self` and takes a plain `&T` by reference without cloning — even a field several providers each read is written as the same implicit argument in each, not promoted to a getter. An implicit argument names both a local and the field it comes from, keeping the `HasField` machinery out of sight. The getter-trait version:

```rust
#[cgp_auto_getter]
pub trait HasDimensions {
    fn width(&self) -> &f64;
    fn height(&self) -> &f64;
}

#[cgp_impl(new RectangleArea)]
#[uses(HasDimensions)]
impl AreaCalculator {
    fn area(&self) -> f64 { self.width() * self.height() }
}
```

collapses to reading the fields directly:

```rust
#[cgp_impl(new RectangleArea)]
impl AreaCalculator {
    fn area(&self, #[implicit] width: f64, #[implicit] height: f64) -> f64 {
        width * height
    }
}
```

Use `#[cgp_auto_getter]` sparingly — only where an implicit argument cannot reach. There are three such cases. The field lives on a type *other* than the provider's own context, so the getter is required as a `where` bound on that type (`Request: HasBasicAuthHeader<Self>`) and there is no `self` field for an implicit argument to read; the accessor must exist as a *named* capability other code depends on through `#[uses(HasName)]` or a supertrait; or the getter carries an *associated type inferred from the field* so the type stays abstract for callers. Everywhere else — including a same-context field read shared by several providers — prefer the implicit argument. Avoid `#[cgp_getter]` in ordinary code, since its full wireable component is for the advanced case of choosing the source field per context at wiring time.

## Abstract types: `#[use_type]` over supertrait + `Self::Type`

Bring an abstract type into a definition with [`#[use_type]`](abstract-types.md) and write it as a bare alias, rather than declaring the owning trait as a supertrait and qualifying every use as `Self::Type`. The attribute does both jobs: it adds the supertrait (on `#[cgp_component]`) or `where` bound (on `#[cgp_impl]`/`#[cgp_fn]`) *and* rewrites each bare `Error`/`Scalar` to its fully-qualified path. This is preferred even for the built-in error type — the legacy

```rust
#[cgp_component(Loader)]
pub trait CanLoad: HasErrorType {
    fn load(&self, path: &str) -> Result<String, Self::Error>;
}
```

becomes

```rust
#[cgp_component(Loader)]
#[use_type(HasErrorType.Error)]
pub trait CanLoad {
    fn load(&self, path: &str) -> Result<String, Error>;
}
```

One rule bounds the rewrite: it fires only on the bare identifier of an *imported* type. A construct's own **local associated type stays qualified as `Self::Assoc`** — a handler that declares `type Output` writes `Self::Output`, never a bare `Output`. So a mixed signature like `Result<Self::Output, Error>` is exactly right: the local `Self::Output` stays qualified, the imported `Error` is bare.

When a definition imports types from several traits, combine them into one `#[use_type]` attribute with the trait paths comma-separated — `#[use_type(HasUserIdType.UserId, HasCurrencyType.Currency, HasErrorType.Error)]` — rather than stacking one attribute per trait; the combined form reads as a single import list. Several types from one trait use the braced list (`#[use_type(HasFooType.{Foo, Bar})]`). Stacking repeated `#[use_type]` attributes behaves identically, but reach for it only when a real reason calls for it.

Prefer `#[use_type]` even when the type lives on a **foreign** type rather than `Self` — a type named by a generic parameter — using the trailing `in Context` clause. It rewrites the bare alias to `<Context as Trait>::Assoc` *and* supplies `Context: Trait`, so the hand-written

```rust
#[cgp_auto_getter]
pub trait HasLoggedInUser<App>
where
    App: HasUserIdType,
{
    fn logged_in_user(&self) -> &Option<App::UserId>;
}
```

becomes

```rust
#[cgp_auto_getter]
#[use_type(HasUserIdType.UserId in App)]
pub trait HasLoggedInUser<App> {
    fn logged_in_user(&self) -> &Option<UserId>;
}
```

with `App: HasUserIdType` supplied for you, so the plain `<App>` parameter and the bare `UserId` are enough.

Import a type with `#[use_type]` even when it *already* arrives transitively — as the supertrait of a trait pulled in by `#[uses]`. When `CanCreateFoo` has `HasFooType` as a supertrait, prefer re-importing over leaning on the transitive `Self::Foo`:

```rust
#[cgp_fn]
#[uses(CanCreateFoo)]
#[use_type(HasFooType.Foo)]        // re-import: bare `Foo`, explicit `HasFooType` dep
fn bar(&self) -> Foo { self.create_foo(42) }
```

instead of

```rust
#[cgp_fn]
#[uses(CanCreateFoo)]
fn bar(&self) -> Self::Foo { self.create_foo(42) }   // relies on the reachable supertrait
```

The re-import re-adds a harmless (already-implied) `Self: HasFooType` and rewrites the bare `Foo`, so `#[uses]` states the *capability* dependency and `#[use_type]` states the *type* dependency, both visible rather than one riding in silently.

`#[use_type]` also *pins* an abstract type with the equality form `{Assoc = Type}`, which is the modern replacement for a hand-written `where Self: HasXType<Assoc = Concrete>` clause — reach for it whenever a provider constrains an abstract type to a concrete one, rather than leaving the equality as an explicit `where`. On a `#[cgp_impl]`, the legacy

```rust
#[cgp_impl(new DisplayHttpError)]
impl<Code, Detail> HttpErrorRaiser<Code, Detail>
where
    Self: HasErrorType<Error = AppError>,
    Code: IsStatusCode,
    Detail: Display,
{
    fn raise_http_error(_code: Code, detail: Detail) -> AppError { /* ... */ }
}
```

becomes, with the equality moved into the attribute:

```rust
#[cgp_impl(new DisplayHttpError)]
#[use_type(HasErrorType.{Error = AppError})]
impl<Code, Detail> HttpErrorRaiser<Code, Detail>
where
    Code: IsStatusCode,
    Detail: Display,
{
    fn raise_http_error(_code: Code, detail: Detail) -> AppError { /* ... */ }
}
```

The attribute emits the same `Self: HasErrorType<Error = AppError>` bound (and would rewrite any bare `Error`, though here the body names the concrete `AppError` directly). The right-hand side of `=` may even name *another* imported alias, which unifies two abstract types: `#[use_type(HasPasswordType.Password, HasHashedPasswordType.{HashedPassword = Password})]` emits `Self: HasHashedPasswordType<HashedPassword = <Self as HasPasswordType>::Password>`. Because the equality form produces an impl-side bound, it belongs on `#[cgp_impl]` and `#[cgp_fn]` only — it is rejected on `#[cgp_component]`. The one equality bound that *stays* a hand-written `where` clause is one on a trait you would never `#[use_type]` from, such as `Iterator<Item = u8>`.

## Supertraits: `#[extend]` over native `:` syntax

Add a non-type capability supertrait to a `#[cgp_component]` trait with [`#[extend(...)]`](functions-and-getters.md) rather than native `pub trait CanDoX: Supertrait` syntax. Both produce the same trait, but `#[extend]` reads as importing a capability the trait re-exports, which is what a CGP supertrait actually is — a declared dependency, not a base class. It pairs symmetrically with `#[uses]`: `#[uses]` imports a capability for private use, `#[extend]` re-exports one as part of the trait's contract. The native

```rust
#[cgp_component(Greeter)]
pub trait CanGreet: HasName {
    fn greet(&self) -> String;
}
```

becomes

```rust
#[cgp_component(Greeter)]
#[extend(HasName)]
pub trait CanGreet {
    fn greet(&self) -> String;
}
```

Use `#[extend]` for a supertrait that contributes only a *capability* (like `HasName`, read through the getter); use `#[use_type]` instead when the supertrait is an abstract-type component whose associated type the signature names, since `#[use_type]` also rewrites the type. In `#[cgp_fn]`, whose `where` clauses are impl-side dependencies, `#[extend]` is the only way to declare a supertrait at all.

## Per-type dispatch: `open` and namespaces over `UseDelegate`

Route a generic-parameter component to a different provider per type with the [`open` statement](wiring.md) or a [namespace](namespaces.md), rather than the legacy [`UseDelegate`](higher-order-providers.md) nested-table pattern. Both ride the `RedirectLookup` impl every `#[cgp_component]` already generates, so they store the per-type entries directly on the context with no wrapper type. The legacy nested table:

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

becomes the inline `open` form:

```rust
delegate_components! {
    MyApp {
        open AreaCalculatorComponent;

        @AreaCalculatorComponent.Rectangle: RectangleArea,
        @AreaCalculatorComponent.Circle: CircleArea,
    }
}
```

Because `open` and namespaces ride `RedirectLookup`, a **new** component you dispatch this way needs no [`#[derive_delegate(UseDelegate<Param>)]`](macro-grammar.md) attribute — that attribute exists only to generate the `UseDelegate` provider the nested-table form relies on. You will still see `#[derive_delegate]` on some CGP-shipped components (the error and handler families) so their existing `UseDelegate` wiring keeps working, but code dispatching only through `open` or a namespace can omit it. Prefer `open` for a context wiring its own components, and a namespace when a reusable, inheritable dispatch table is worth sharing.

## When the explicit forms are still right

A handful of cases genuinely need an explicit form, and choosing one there is not a regression. Keep an explicit `where` clause for an associated-type-equality bound on a non-`#[use_type]` trait — `Iterator<Item = u8>`, `From<X>`, and the like — which reads more clearly as a `where` clause than in an import-shaped `#[uses]` (which does accept it); but note the exception's own exception: an equality bound on an **abstract-type** trait (`Self: HasErrorType<Error = AppError>`) is *not* one of these, because the [`#[use_type]` equality form](#abstract-types-use_type-over-supertrait--selftype) `#[use_type(HasErrorType.{Error = AppError})]` does express it and is preferred; leave only equality on a non-`#[use_type]` trait as a hand-written `where`. Name the context explicitly, `impl<Context> Trait for Context`, to attach a lifetime or higher-ranked bound the sugar cannot carry, or when `Self` must be a concrete context (the `#[cgp_impl(Self)]` passthrough is the direct-impl case). Reach for `#[cgp_getter]` when you specifically want to choose which field a getter reads per context at wiring time. Write a raw provider-trait `impl` when you need the inside-out shape directly. And keep a local associated type qualified as `Self::Output` always — it is never a `#[use_type]` import.

## Reading pre-0.7 code: renamed and removed names

Very old code and pre-0.7 write-ups can use names that no longer exist, as opposed to the still-valid legacy *forms* above. These do not compile against current CGP, so treat them as signals to translate, not to copy: the attribute `#[cgp_context]` was removed (a context is now assembled with `delegate_components!` and the derive/getter machinery rather than a dedicated context macro), and the abstract-type provider trait once called `ProvideType` is now `TypeProvider`. When you meet a name the current prelude does not export, assume it was renamed or removed rather than that you are misremembering, and fetch the online knowledge base's changelog and reference to find its modern spelling; do not reintroduce a removed name into new code.

## Related sub-skills

Every shift here is a shortcut over a construct documented in full elsewhere: provider forms in [components](components.md), field injection and `#[uses]`/`#[extend]` in [functions-and-getters](functions-and-getters.md), abstract types and `#[use_type]` in [abstract-types](abstract-types.md), the inner-provider pattern in [higher-order-providers](higher-order-providers.md), per-type dispatch in [wiring](wiring.md) and [namespaces](namespaces.md), and the grammar and expansion of every form in [macro-grammar](macro-grammar.md). For how much CGP a problem needs in the first place, see [modularity-hierarchy](modularity-hierarchy.md).
