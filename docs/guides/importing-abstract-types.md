# Importing abstract types

CGP abstracts over types with associated types on components, and this guide is about bringing such a type into a definition as a plain alias rather than a supertrait plus a fully-qualified `Self::Type` at every use.

It applies inside [`#[cgp_component]`](../reference/macros/cgp_component.md) definitions and [`#[cgp_impl]`](../reference/macros/cgp_impl.md)/[`#[cgp_fn]`](../reference/macros/cgp_fn.md) providers alike, and is the recommended form for the built-in error type as much as for a domain type.

## Import abstract types with `#[use_type]`

Bring an abstract type into a definition with [`#[use_type]`](../reference/attributes/use_type.md) and write it as a bare alias, rather than declaring the owning trait as a supertrait and qualifying every use as `Self::Type`. The attribute does both jobs at once: `#[use_type(HasScalarType.Scalar)]` adds the trait as a supertrait (on a `#[cgp_component]`) or a `where` bound (on a `#[cgp_impl]`/`#[cgp_fn]`), and rewrites each bare `Scalar` to `<Self as HasScalarType>::Scalar`. This is the preferred form even for the built-in error type: the legacy component definition

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

One rule bounds the rewrite: it fires only on the bare identifier of an *imported* type. A construct's own **local associated type always stays qualified as `Self::Assoc`** — a handler that declares `type Output` writes `Self::Output`, never a bare `Output`, because `Output` is the trait's own type rather than one imported from another trait. A mixed signature such as `Result<Self::Output, Error>` is therefore exactly right: the local `Self::Output` stays qualified while the imported foreign `Error` is written bare.

When a definition imports types from several traits, combine them into one `#[use_type]` attribute by separating the trait paths with commas — `#[use_type(HasUserIdType.UserId, HasCurrencyType.Currency, HasErrorType.Error)]` — rather than stacking one attribute per trait; the combined form reads as a single import list. Several types from one trait use a braced list (`#[use_type(HasFooType.{Foo, Bar})]`).

## Pinning an abstract type to a concrete one

On a `#[cgp_impl]` or `#[cgp_fn]`, `#[use_type]` also *pins* an abstract type to a concrete one with the equality form `{Assoc = Type}`, which is the replacement for a hand-written `where Self: HasXType<Assoc = Concrete>` clause. Writing `#[use_type(HasErrorType.{Error = AppError})]` emits `Self: HasErrorType<Error = AppError>` (and rewrites any bare `Error`), so a provider fixed to a concrete error type moves that pin out of its `where` clause and into the import. The right-hand side may name another imported alias to *unify* two abstract types — `#[use_type(HasPasswordType.Password, HasHashedPasswordType.{HashedPassword = Password})]` emits `Self: HasHashedPasswordType<HashedPassword = <Self as HasPasswordType>::Password>`. The equality form is rejected on `#[cgp_component]`, since a trait definition cannot carry the impl-side constraint it produces — this is the one place a pin stays in a hand-written `where` clause, and only for equality on a trait you would never `#[use_type]` from.

When a capability supertrait has no associated type to import — a plain capability like `HasName` — add it with [`#[extend]`](capability-supertraits.md) rather than `#[use_type]`. Use `#[use_type]` when the signature names the trait's associated type; use `#[extend]` when it only calls the trait's methods.

## Related guides

- [Capability supertraits](capability-supertraits.md) — the companion for a supertrait that contributes a capability rather than a type.
- [Declaring dependencies](declaring-dependencies.md) — where an abstract-type pin moves *from* (a `#[uses]` or hand-written `where`).
- [Guides summary](README.md#summary) — the cheat-sheet across all the guides, with the local-associated-type exception restated among the other still-explicit cases.
