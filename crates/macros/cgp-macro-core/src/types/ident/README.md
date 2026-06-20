# Identifiers and Paths with Type Parameters

This module provides the parsing constructs that CGP macros use to read
identifiers and paths that carry type parameters, such as `Foo<A, B>`,
`Bar<'a, C>`, or `path::to::Foo<(A, B), C>`.

There are two fundamentally different *positions* in which type parameters
appear, and CGP needs to parse both. Getting the distinction right matters,
because the two positions allow different syntax:

- **Definition site** — where generic parameters are *introduced*, e.g. the
  `<A, B>` in `struct Foo<A, B>` or the `<'a, C>` in `trait Bar<'a, C>`. Here
  each parameter is a *simple, unconstrained binder*: a lifetime, a type
  identifier, or a const parameter.
- **Type-expression site** — where generic arguments are *applied*, e.g. the
  `<A, B>` in `Foo<A, B>` used as a type. Here each argument may be an arbitrary
  type, including composite forms such as `(A, B)` or `Bar<A>`.

The constructs in this module fall into these two groups, plus the supporting
argument/parameter types they are built from.

| Construct | Position | Head | Status |
|---|---|---|---|
| `IdentWithTypeGenerics` | definition site | `Ident` | original |
| `IdentWithTypeArgs` | type-expression site | `Ident` | original |
| `NewIdentWithTypeGenerics` | definition site | `Ident` | new |
| `NewIdentWithTypeArgs` | type-expression site | `Ident` | new |
| `PathWithTypeArgs` | type-expression site | `syn::Path` | new |

The behavior described here is exercised by the test suite at
`crates/tests/cgp-tests/tests/ident_with_type_params_tests/`.

---

## Why `syn`'s constructs are not sufficient

CGP needs *faithful* parsers: parsers that accept exactly the forms that are
valid in a given position, and reject everything else at parse time with a clear
error. The off-the-shelf `syn` constructs do not give us this, for three
different reasons.

### 1. `syn::AngleBracketedGenericArguments` is too permissive for arguments

The original `IdentWithTypeArgs` models its argument list with
`syn::AngleBracketedGenericArguments`, whose elements are
[`syn::GenericArgument`]:

```rust
pub enum GenericArgument {
    Lifetime(Lifetime),     // 'a          — valid here
    Type(Type),             // A, (A, B)   — valid here
    Const(Expr),            // 3, { N }    — valid here
    AssocType(AssocType),   // Item = T    — NOT valid in a type-argument position
    AssocConst(AssocConst), // N = 1       — NOT valid in a type-argument position
    Constraint(Constraint), // Item: Clone — NOT valid in a type-argument position
}
```

The last three variants are only meaningful inside *trait bounds* (e.g.
`dyn Iterator<Item = T>`), not in a plain applied type like a struct field type.
But `AngleBracketedGenericArguments` accepts all six variants unconditionally, so
`IdentWithTypeArgs` silently accepts invalid input such as `Foo<A, B = C>` or
`Foo<A: Clone>`.

### 2. `syn::Generics` is the wrong shape for definition-site parameters

The original `IdentWithTypeGenerics` models its parameter list with
`syn::Generics`. But `Generics` is designed for full `impl`/`struct`/`fn`
headers: every parameter can carry bounds and a default, and the whole thing can
carry a trailing `where` clause. None of that is allowed in the simple
definition-site lists CGP cares about.

To compensate, the original `TypeGenerics` parser uses a *round-trip hack*: it
parses a full `Generics`, runs it through `split_for_impl()` to obtain the
"type generics" projection (which strips bounds and defaults), re-parses that,
and rejects the input if the two differ:

```rust
let generics: Generics = input.parse()?;
let (_, type_generics, _) = generics.split_for_impl();
let generics2: Generics = parse_internal(type_generics.to_token_stream())?;
if generics != generics2 {
    return Err(Error::new_spanned(generics, "invalid type generics syntax"));
}
```

This works for rejecting bounds and defaults, but it is opaque, it relies on a
structural-equality comparison as a validation mechanism, and it has a surprising
side effect: it rejects **const generics**. The type-generics projection of
`const N: usize` is the bare `N`, which re-parses as a *type* parameter, so the
equality check fails and `Bar<const N: usize>` is rejected — even though const
parameters are perfectly valid at a definition site.

### 3. `syn::Path` buries the final arguments

CGP frequently parses things that are genuinely Rust *paths* — provider trait
paths, namespace references, preset names — many of which are pulled out of a
real `syn::Path` (for instance from `item_impl.trait_`). The arguments we care
about live on the **final segment**, hidden inside
`path.segments.last().arguments` as a `PathArguments` enum. Reaching them, and
rewriting them (e.g. inserting the `Context` type as the first argument), is
awkward and repetitive.

Because the original `IdentWithTypeArgs` head is a single `Ident`, it cannot
parse a path at all: `path::to::Foo<A>` is rejected outright. So any site that
wanted to accept a qualified path had no construct to use.

---

## The constructs

### Supporting types

#### `TypeArg` / `TypeArgs` (type-expression arguments)

`TypeArg` is the faithful element of a type-argument list. It is the restriction
of `syn::GenericArgument` to the three valid variants:

```rust
pub enum TypeArg {
    Lifetime(Lifetime), // 'a
    Type(Type),         // A, (A, B), Bar<A>, &'a A, [A; 4], dyn Tr, ...
    Const(Expr),        // a literal (3, true) or a braced block ({ N })
}
```

Associated bindings (`Item = T`), associated const bindings (`N = 1`), and
associated bounds (`Item: Clone`) are rejected during parsing. `TypeArg` can
also be produced from an already-parsed `syn::GenericArgument` via
`TypeArg::from_generic_argument`, which applies the same validation (used by
`PathWithTypeArgs`).

`TypeArgs` is the optional angle-bracketed list:

```rust
pub struct TypeArgs {
    pub args: Option<Punctuated<TypeArg, Comma>>,
}
```

`None` means there were no angle brackets at all (`Foo`); `Some(empty)` means an
explicit empty list (`Foo<>`). `make_args(&mut self)` returns the inner
`Punctuated`, inserting an empty list if necessary — this mirrors the original
`GenericArguments::make_args` so migration is mechanical.

#### `TypeGenericParam` / `TypeGenericParams` (definition-site parameters)

`TypeGenericParam` is the faithful element of a definition-site parameter list:

```rust
pub enum TypeGenericParam {
    Lifetime(Lifetime),       // 'a
    Type(Ident),              // C
    Const(ConstGenericParam), // const N: usize
}
```

Each variant is a *bare* binder: bounds (`A: Clone`, `'a: 'b`) and defaults
(`A = B`, `const N: usize = 0`) are rejected, as are composite forms
(`(A, B)`, `Bar<A>`). `ConstGenericParam` deliberately has no default field.

`TypeGenericParams` is the optional list, with the same `None` / `Some(empty)`
convention as `TypeArgs` and a `make_params` accessor. It also provides
`to_generics()`, which lowers the parameters into a plain `syn::Generics` for
downstream code that builds struct/impl headers (e.g. `EmptyStruct`).

### Original constructs

#### `IdentWithTypeGenerics` = `Ident` + `TypeGenerics`

A definition-site construct. Accepts `Foo`, `Foo<A, B>`, `Bar<'a, C>`. Rejects
bounds, defaults, and composite parameters via the `split_for_impl` round-trip
hack — and, as a side effect of that hack, also rejects const parameters
(`Bar<const N: usize>`).

#### `IdentWithTypeArgs` = `Ident` + `GenericArguments`

A type-expression construct. Accepts `Foo`, `Foo<A, B>`, composite arguments
(`Foo<(A, B), Bar<C>>`), lifetimes, and const arguments. **Unfaithfully** also
accepts associated bindings and bounds (`Foo<A, B = C>`, `Foo<Item = X>`,
`Foo<N = 1>`, `Foo<A: Clone>`). The head is a single `Ident`, so paths
(`path::to::Foo<A>`) and turbofish (`Foo::<A>`) are rejected.

### New constructs

#### `NewIdentWithTypeGenerics` = `Ident` + `TypeGenericParams`

The intended replacement for `IdentWithTypeGenerics`. Same accepted forms, but:

- the parameter list is modelled directly, with no round-trip hack;
- **const parameters are accepted** (`Bar<const N: usize>`);
- defaults are rejected up front (the original happens to reject them too, but
  only as a side effect of the structural comparison).

#### `NewIdentWithTypeArgs` = `Ident` + `TypeArgs`

The intended replacement for `IdentWithTypeArgs`. Same valid forms, but
associated bindings/bounds are rejected at parse time. The head is still a single
`Ident` (paths and turbofish rejected).

#### `PathWithTypeArgs` = `syn::Path` + `TypeArgs`

A generalization of `NewIdentWithTypeArgs` from a single-identifier head to a
full path head. It parses a `syn::Path`, then **lifts the final segment's
arguments** out into a separate `type_args: TypeArgs` field, leaving `path` free
of those arguments. This makes the final arguments directly accessible and
rewritable, while `ident()` exposes the final segment's identifier.

Restrictions enforced during parsing:

- only the final segment may carry arguments — `path::to<X>::Foo` is rejected;
- turbofish is rejected — `path::to::Foo::<A>` must be written `path::to::Foo<A>`;
- parenthesized arguments are rejected — `Fn(A) -> B`;
- arguments obey the same `TypeArg` rules (no associated bindings/bounds).

Because any single identifier is also a valid one-segment path, every input that
`NewIdentWithTypeArgs` accepts is also accepted by `PathWithTypeArgs`.
`PathWithTypeArgs` is therefore a strict superset, and is the right default at a
type-expression site unless a qualified path must specifically be forbidden.

---

## Behavior comparison

### Type-expression position (`IdentWithTypeArgs` vs `NewIdentWithTypeArgs` / `PathWithTypeArgs`)

| Input | `IdentWithTypeArgs` | `NewIdentWithTypeArgs` | `PathWithTypeArgs` |
|---|---|---|---|
| `Foo` | accept | accept | accept |
| `Foo<>` | accept | accept | accept |
| `Foo<A, B>` | accept | accept | accept |
| `Foo<(A, B), C>` | accept | accept | accept |
| `Foo<Bar<A>, C>` | accept | accept | accept |
| `Foo<'a, A>` | accept | accept | accept |
| `Foo<3>`, `Foo<{ N }>` | accept | accept | accept |
| `Foo<A, B = C>` | **accept** ⚠ | reject | reject |
| `Foo<Item = X>` | **accept** ⚠ | reject | reject |
| `Foo<N = 1>` | **accept** ⚠ | reject | reject |
| `Foo<A: Clone>` | **accept** ⚠ | reject | reject |
| `path::to::Foo<A>` | reject | reject | **accept** |
| `Foo::<A>` (turbofish) | reject | reject | reject |
| `Fn(A) -> B` | reject | reject | reject |

⚠ marks the original construct's unfaithful acceptances.

### Definition-site position (`IdentWithTypeGenerics` vs `NewIdentWithTypeGenerics`)

| Input | `IdentWithTypeGenerics` | `NewIdentWithTypeGenerics` |
|---|---|---|
| `Foo` | accept | accept |
| `Foo<A, B>` | accept | accept |
| `Bar<'a, C>` | accept | accept |
| `Bar<const N: usize>` | **reject** | **accept** |
| `Foo<A: Clone>` | reject | reject |
| `Foo<'a: 'b>` | reject | reject |
| `Foo<A = B>` | reject | reject |
| `Foo<(A, B)>` | reject | reject |
| `Foo<Bar<A>>` | reject | reject |
| `path::to::Foo<A>` | reject | reject |

---

## Forms of generic parameters considered

For completeness, here is the full matrix of generic-parameter/argument forms in
the Rust grammar and how the new constructs treat each.

| Form | Example | Def site (`NewIdentWithTypeGenerics`) | Arg site (`NewIdentWithTypeArgs` / `PathWithTypeArgs`) |
|---|---|---|---|
| Lifetime | `'a` | accept | accept |
| Type identifier | `A` | accept | accept (as a `Type`) |
| Composite type | `(A, B)`, `Bar<A>`, `&'a A`, `[A; 4]`, `dyn Tr`, `fn(A) -> B` | reject | accept |
| Const parameter | `const N: usize` | accept | n/a |
| Const argument | `3`, `true`, `{ N }` | n/a | accept |
| Trait/lifetime bound | `A: Clone`, `'a: 'b` | reject | reject |
| Default | `A = B`, `const N: usize = 0` | reject | n/a |
| Associated type binding | `Item = T` | n/a | reject |
| Associated const binding | `N = 1` | n/a | reject |
| Associated type bound | `Item: Clone` | n/a | reject |
| Where clause | `where A: Clone` | not consumed (out of scope) | not consumed |
| Turbofish | `::<A>` | n/a | reject |
| Path head | `path::to::Foo` | reject | `PathWithTypeArgs` only |
| Intermediate-segment args | `path::to<X>::Foo` | n/a | reject |
| Parenthesized args | `Fn(A) -> B` | n/a | reject |

Two design decisions worth highlighting:

- **Const generics are supported.** CGP does not appear to use them today, but
  they are a legitimate Rust form, so the new constructs accept them rather than
  rejecting them by accident (as the original definition-site construct does).
- **A bare-identifier const argument is parsed as a `Type`.** `Foo<N>` is
  classified as `TypeArg::Type`, not `TypeArg::Const`, exactly as `syn` does,
  because the two are syntactically indistinguishable without resolution. Only
  literal or braced const arguments are classified as `Const`.

---

## Migration guide

The replacements come in two flavors:

- **Definition sites** → `NewIdentWithTypeGenerics` (a drop-in for
  `IdentWithTypeGenerics`).
- **Type-expression sites** → `PathWithTypeArgs` by default, or
  `NewIdentWithTypeArgs` where a qualified path must be forbidden.

The original public surface is mirrored on the new types (`From<Ident>`,
`ToTokens`, `to_type`/`Into<Type>`, `make_args`/`make_params`), so most call
sites change only the type name. The notable structural changes are: argument
iteration now yields `TypeArg` instead of `syn::GenericArgument`, and the path
head is reached through `ident()` / `path` rather than a bare `ident` field.

### Definition sites → `NewIdentWithTypeGenerics`

These all introduce a fresh local name, so an ident head is correct.

| Location | Field |
|---|---|
| `types/cgp_component/args/component_args.rs`, `args/raw.rs` | `component_name` |
| `types/attributes/prefix.rs` | `component_name` parameter |
| `types/namespace/table.rs` | `namespace` (table head, possibly `new`) |
| `types/delegate_component/table/main.rs` | `struct_type` (the `new` table struct) |
| `types/cgp_provider/item.rs` | `provider_type` (the provider struct being defined) |
| `cgp-macro-lib/parse/component_spec.rs` | `component_name` |

Downstream code that consumes `.type_generics.generics` (a `syn::Generics`)
should call `.type_generics.to_generics()` instead.

### Type-expression sites → `PathWithTypeArgs`

These name an existing item (a trait, namespace, preset, or component-name type)
that may legitimately be written as a qualified path. Several already extract
their value from a real `syn::Path`, so moving to `PathWithTypeArgs` both fixes
the faithfulness gap *and* unlocks the `path::to::Foo<A, B>` use case.

| Location | Field / use | Notes |
|---|---|---|
| `types/cgp_provider/item.rs` | `provider_trait` | parsed from `item_impl.trait_`; use `.ident()` to build `{Trait}Component` |
| `types/provider_impl.rs` | `provider_path` destructuring | replace `IdentWithTypeArgs { ident, type_args }` with `path`/`type_args` + `ident()` |
| `types/cgp_impl/lowered.rs` | `provider_trait_path` | `make_args().insert(0, ..)` still works (now over `TypeArg`) |
| `types/attributes/use_type/attribute.rs` | `trait_path` | already named a "path" |
| `types/attributes/use_provider/attribute.rs` | `provider_trait_bounds` | trait names may be paths |
| `types/attributes/uses.rs`, `attributes/function.rs`, `attributes/cgp_impl_attributes.rs` | `uses` / `imports` | `#[uses(Trait<..>)]` trait names |
| `types/attributes/prefix.rs`, `attributes/default_impl/attribute.rs` | `namespace` | namespace references |
| `types/namespace/table.rs` | `parent_namespace` | |
| `types/namespace/inherit.rs` | `namespace` | |
| `types/delegate_component/statement/for_loop.rs`, `statement/eval.rs` | `namespace` | |
| `cgp-macro-lib/entrypoints/cgp_inherit.rs` | `preset` | preset names may be paths |
| `cgp-macro-lib/parse/define_preset.rs` | `parent_type`, preset entries | |
| `cgp-macro-lib/entrypoints/cgp_preset.rs`, `preset/impl_is_preset.rs`, `for_each_replace.rs` | `DelegateEntry`/`DelegateKey<IdentWithTypeArgs>` | component-name keys |
| `cgp-macro-lib/parse/delegate_components.rs` | `DelegateEntry<IdentWithTypeArgs>` | preset form |
| `cgp-macro-lib/parse/check_components.rs` | `context_type` for naming | only `.ident()` is needed |

### Type-expression sites → `NewIdentWithTypeArgs`

Use this only where the grammar should specifically forbid a qualified path —
for example a freshly declared local name that nonetheless appears in
argument-list syntax. In practice the "must be a single ident" cases in CGP are
all *definition* sites (covered by `NewIdentWithTypeGenerics`), so
`NewIdentWithTypeArgs` is mostly useful as the precise, path-rejecting option
when reviewing a specific site shows that a path would be meaningless there.

### `ProviderImplArgs::from_generic_args`

`types/cgp_provider/provider_impl_args.rs` currently iterates
`syn::GenericArgument` to split the leading `Context` type from the remaining
impl arguments. After migration it should take a `&TypeArgs` (or
`&Punctuated<TypeArg, Comma>`) and match on `TypeArg::{Lifetime, Type, Const}` —
which is a more direct mapping than the current `GenericArgument` match (the
`_ => Err(..)` arm for unsupported variants disappears, since `TypeArg` has no
invalid variants).
