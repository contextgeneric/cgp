# Macro grammar and expansion rules

The grammar every CGP macro parses, the code each one expands to, and how to read the compiler errors that result — the reference for writing CGP that parses, and for decoding what goes wrong when it does not.

CGP is almost entirely procedural macros, so "valid CGP code" means two separate things: input the macro's own parser accepts, and output that expands to Rust the compiler accepts. This file covers both. It gives the formal grammar of each macro's custom syntax, states the invariant each expansion preserves, and closes with a decoder for the error messages the two layers produce. Read it to check a form you are unsure about, to predict what a macro emits, or to trace an error back to its cause. Assume `use cgp::prelude::*;` throughout; the CGP version is v0.7.0.

## How to read the grammars

The grammars use the notation of the [Rust Reference](https://doc.rust-lang.org/nightly/reference/notation.html), in production-rule form. A production is `Name -> Expression`, one per line inside an `ebnf` block. `x?` is optional, `x*` is zero-or-more, `x+` is one-or-more, `A | B` is a choice, `( … )` groups, and juxtaposition is sequence. A terminal is written in backticks (`` `new` ``, `` `:` ``, `` `@` ``); a nonterminal is a CamelCase name.

A CamelCase name this file does not define is a production of the **Rust grammar**, reused rather than re-specified — `Type`, `Generics` (the `< … >` parameter list), `GenericArgs` (the `< … >` argument list), `WhereClause`, `TypePath`, and `Expression`. An `ALL_CAPS` name is a Rust lexer token: `IDENTIFIER` is a Rust identifier and `STRING_LITERAL` a string literal. Each grammar describes only the tokens the macro itself parses — the argument tokens inside an attribute's `#[name(…)]` or `#[name{…}]` delimiters, or the body tokens inside a function-like macro's `name!{ … }` — never the delimiters themselves, and never the plain Rust item an attribute is applied to.

## Reserved identifiers the expansions introduce

Every macro that turns a trait or impl "inside out" invents identifiers, and it wraps them in double underscores so they cannot collide with a user's names. Recognizing them is what lets you read raw expansions and the errors that quote them. The context type parameter is `__Context__` (overridable), the provider parameter is `__Provider__`, a rewritten `self` receiver is `__context__`, the component key generic is `__Component__`, an inner delegation table is `__Components__`, a looked-up delegate is `__Delegate__`, the `IsProviderFor` params tuple is `__Params__`, a getter's free field tag is `__Tag__`, and a namespace's table generic is `__Table__`. Throughout this file the readable names `Context`/`Provider`/`Params` stand in for these where clarity is served, but the emitted code uses the reserved forms.

Two more names appear in expansions and resolve through the prelude: any CGP construct the output references is emitted as `::cgp::macro_prelude::<Name>`, so an expansion compiles in a crate that has only `cgp` in scope. You never write that path yourself.

---

# Component-definition macros

## `#[cgp_component]`

The argument is either a bare provider name or a comma-separated set of keyed values:

```ebnf
CgpComponentArgs -> ProviderName
                  | KeyValueArg ( `,` KeyValueArg )* `,`?

ProviderName     -> IDENTIFIER

KeyValueArg      -> `name` `:` ComponentName
                  | `provider` `:` IDENTIFIER
                  | `context` `:` IDENTIFIER

ComponentName    -> IDENTIFIER GenericArgs?
```

`ProviderName` is shorthand for setting `provider` alone. In the keyed form — written with brace delimiters, `#[cgp_component { … }]` — each of the three keys appears at most once, in any order, and `provider` is required. The `name` key defaults to the provider name with a `Component` suffix (`AreaCalculator` → `AreaCalculatorComponent`), and `context` defaults to `__Context__`. The provider name may not carry generics; the component name may.

**Expansion invariant.** From one trait, `#[cgp_component]` emits five items plus standard provider impls, and this shape never varies: the **consumer trait** unchanged; the **provider trait** with `Self` moved to a leading `Context` parameter, every `self`/`Self` rewritten to `context`/`Context`, and an `IsProviderFor<{Name}Component, Context, Params>` supertrait; the **consumer blanket impl** (`Context: {Provider}<Context>` ⟹ consumer trait); the **provider blanket impl** (delegates through `DelegateComponent<{Name}Component>`); and the zero-sized **`{Name}Component` marker**. Alongside these it emits a `UseContext` impl, a `RedirectLookup` impl (what the `open` statement and namespaces dispatch through), one `UseDelegate` impl per `#[derive_delegate]`, and prefix impls per `#[prefix]`. Any trait generics land *after* the context in the provider trait and are grouped into the `IsProviderFor` `Params` tuple. See [components](components.md) for the full worked expansion.

## `#[cgp_type]` and `#[cgp_getter]`

Both reuse `CgpComponentArgs` verbatim and differ only in the default when `provider` is omitted:

```ebnf
CgpTypeArgs   -> CgpComponentArgs   // default provider: {AssocType}TypeProvider
CgpGetterArgs -> CgpComponentArgs   // default provider: strip leading `Has`, append `Getter`
```

`#[cgp_type]` keys its default off the *associated type's* name (`type Scalar;` → `ScalarTypeProvider`, marker `ScalarTypeProviderComponent`); `#[cgp_getter]` strips a leading `Has` from the trait name and appends `Getter` (`HasName` → `NameGetter`). Beyond the shared component output, `#[cgp_type]` adds a `UseType<T>` blanket impl (the associated type *is* `T`) and a `WithProvider` impl adapting the built-in `TypeProvider`; `#[cgp_getter]` adds a `UseFields` impl (always) plus a `UseField<Tag>` impl and a `WithProvider` impl (only when the trait has exactly one method). See [abstract-types](abstract-types.md) and [functions-and-getters](functions-and-getters.md).

---

# Provider-writing macros

## `#[cgp_impl]`

The argument names the provider, optionally preceded by `new` and followed by a component override:

```ebnf
CgpImplArgs   -> `new`? ProviderType ( `:` ComponentType )?

ProviderType  -> Type
ComponentType -> Type
```

`new` makes the macro also emit `pub struct <ProviderType>;`. `ProviderType` takes the `Self` position of the generated provider impl — a plain name, a generic provider (`ScaledArea<Inner>`), or the literal `Self`. The optional `: ComponentType` overrides the component in the generated `IsProviderFor` impl, defaulting to the provider trait name plus `Component`.

Two forms of the impl header matter. **Omit `for Context`** and the macro inserts `__Context__` for you — this is the preferred form, and it reads like an ordinary trait impl. Write `impl<Context> Trait for Context` explicitly only to bound or name the context readably (a lifetime or HRTB the sugar cannot spell). Naming the provider **`Self`** — `#[cgp_impl(Self)]` — is a passthrough: it emits the block unchanged as a direct consumer-trait impl on the concrete context (the `for Context` clause is then *required*), which is how you apply companion attributes like `#[use_provider]` to a hand-written impl; `new` and the component override are ignored in this form.

**Expansion invariant.** `#[cgp_impl]` desugars to `#[cgp_provider]` (or `#[cgp_new_provider]` with `new`): it moves the context back to the provider trait's leading position, swaps the provider name into `Self`, rewrites `&self` to `__context__: &__Context__`, every `self` to `__context__`, and every `Self` to the context type. The rewrite is scoped to the method bodies — a nested item's own `self`/`Self` is left alone (except inside a `macro!(…)` invocation, which the token rewrite cannot scope and so rewrites anyway). See [components](components.md).

## `#[cgp_provider]` and `#[cgp_new_provider]`

Both take a single optional component type; `#[cgp_new_provider]` additionally declares the provider struct (implied by the name, not the argument):

```ebnf
CgpProviderArgs    -> ComponentType?
CgpNewProviderArgs -> ComponentType?

ComponentType      -> Type
```

Applied to a provider-trait impl written directly on a provider struct, the macro passes the impl through unchanged and derives the matching `IsProviderFor` impl from it — a copy with the body and associated types stripped, the trait swapped to `IsProviderFor`, and the same `where` clause kept. The trait arguments become `IsProviderFor<{Component}, {Context}, ({trailing params})>`: component first (the argument overrides the default), context second (the provider trait's leading argument), and a tuple of every remaining provider-trait parameter third (`()` when none). A **const argument in the provider trait's own argument list is rejected** with a spanned error, because a const cannot key the type-based `Params` tuple; a const generic on the provider *struct* flows through untouched.

---

# Function, getter, and handler macros

These macros are applied to a plain Rust function or trait, so most take only an optional name and read the rest of their behavior off the item's shape. Their grammars are small:

```ebnf
CgpFnArgs       -> TraitName?      // #[cgp_fn]:       default = fn name in PascalCase
CgpComputerArgs -> ProviderName?   // #[cgp_computer]: default = fn name in PascalCase
CgpProducerArgs -> ProviderName?   // #[cgp_producer]: default = fn name in PascalCase
BlanketTraitArgs -> ContextName?   // #[blanket_trait]: default = __Context__

TraitName    -> IDENTIFIER
ProviderName -> IDENTIFIER
ContextName  -> IDENTIFIER
```

`#[cgp_auto_getter]`, `#[cgp_auto_dispatch]`, and `#[async_trait]` take **no argument** — they are plain attributes on a trait, with no custom grammar. Their behavior comes entirely from the annotated trait and from separate companion attributes.

The rest of each function macro's contract is read from the item, not the argument. `#[cgp_fn]` lifts function generics onto both the generated trait and impl while keeping the `where` clause impl-only, turns `#[implicit]` parameters into `HasField` reads, and forbids generics on the desugared *method* (a method-level generic is silently treated as a trait/impl generic). `#[cgp_computer]` collects the function's parameters into one tuple `Input`, chooses the base component and promotion bundle from whether the function is `async` and whether it returns `Result`, and wires the rest of the handler family by promotion; `#[cgp_producer]` requires a zero-parameter, non-`async`, non-generic function. `#[async_trait]` rewrites each `async fn` to `-> impl Future`. See [functions-and-getters](functions-and-getters.md) and [handlers](handlers.md) for the expansions.

---

# Attribute modifiers

These attributes refine what a host macro generates; none is a standalone macro. The table names each one's form, what it contributes, and which host macros accept it.

- **`#[implicit]`** — a bare marker on a typed, plain-identifier function argument (`#[implicit] width: f64`); no argument of its own, and `#[implicit(x)]` or `#[implicit = "x"]` is rejected. Removes the argument, adds a `HasField<Symbol!("width")>` bound, and binds the value at the top of the body. A *mutable* implicit (its type carries a `&mut`) must be the sole implicit and needs `&mut self`. Host: `#[cgp_fn]`, `#[cgp_impl]`.
- **`#[uses(Trait, Trait<P>, …)]`** — a comma-separated list of simple trait paths, accumulating across repeats. Adds each as a `Self: Trait` impl-side bound. No associated-type-equality forms (`Iterator<Item = u8>`); write those as a body `where` clause. Host: `#[cgp_fn]`, `#[cgp_impl]`.
- **`#[extend(Trait, Trait<P>, …)]`** — same simple-path list. Adds each as a *supertrait* of the generated trait (and, in `#[cgp_fn]`, also as an impl bound). Host: `#[cgp_fn]`, `#[cgp_component]`. Not `#[cgp_impl]` (a provider impl has no trait to attach supertraits to).
- **`#[extend_where(Pred, …)]`** — a list of full `where`-clause predicates, including associated-type equality, added verbatim to the generated trait's `where` clause. Host: `#[cgp_fn]` only.
- **`#[impl_generics(Param: Bound, …)]`** — bounded generic parameters added to the generated impl **only**, never the trait. Host: `#[cgp_fn]` only.
- **`#[use_provider(Provider: Trait<…> + …, …)]`** — completes an inner provider bound by inserting the context as the trait's leading argument (you write `: AreaCalculator`, it means `AreaCalculator<Self>`) and moving it into the `where` clause. Several bounds via commas or stacked attributes. It only *completes the bound* — it never rewrites the body, which must still call `Provider::method(self)` explicitly. Host: `#[cgp_impl]`, `#[cgp_fn]`.
- **`#[derive_delegate(Wrapper<Param>)]`** — generates a `UseDelegate`-style dispatcher impl keyed on `Param`; repeat for several dispatchers; the key may be a tuple `Wrapper<(A, B)>`. **Legacy** — prefer the `open` statement; see [wiring](wiring.md). Host: `#[cgp_component]`.
- **`#[prefix(@Path in Namespace)]`** — registers the component into `Namespace` under `@Path`, emitting one `RedirectLookup` impl; repeat for several namespaces. Host: `#[cgp_component]`.
- **`#[default_impl(T in DefaultImpls1<Component>)]`** — registers a `#[cgp_impl]` provider as a namespace's per-type default for `T`. Host: `#[cgp_impl]`.

The check macros carry their own modifiers, covered with them below: `#[check_trait(Name)]` and `#[check_providers(…)]` on a table, `#[check_params(…)]` and `#[skip_check]` on an entry.

## `#[use_type]`

This attribute has enough structure to warrant a grammar of its own:

```ebnf
UseTypeArgs  -> UseTypeSpec ( `,` UseTypeSpec )* `,`?

UseTypeSpec  -> ( `@` ContextPath `.` )? TraitPath `.` TypeItems

ContextPath  -> TypePath
TraitPath    -> TypePath

TypeItems    -> UseTypeIdent
              | `{` UseTypeIdent ( `,` UseTypeIdent )* `,`? `}`

UseTypeIdent -> IDENTIFIER ( `as` IDENTIFIER )? ( `=` Type )?
```

The `.` (not `::`) after the trait path starts the associated-type list — a trait path keeps its own `::` segments and may carry generics. Omitting the `@ContextPath .` prefix defaults the rewrite target to `Self`; a `@Types.` prefix rewrites against a named generic parameter and adds `Types: Trait` as a `where` bound rather than a supertrait. In each `UseTypeIdent`, `as` gives a local alias to write in signatures, and `= Type` pins the type with an equality bound (accepted on `#[cgp_fn]`/`#[cgp_impl]`, **rejected on `#[cgp_component]`**, whose trait definition cannot carry the impl-side equality). The macro rewrites every bare mention of the imported identifier (or alias) to the fully-qualified `<Target as Trait>::Type`. Host: `#[cgp_fn]`, `#[cgp_impl]`, `#[cgp_component]`. See [abstract-types](abstract-types.md).

---

# Wiring and checking macros

## `delegate_components!`

The body is an optional generic list and `new` keyword, a target type, and a brace-delimited table:

```ebnf
DelegateComponents -> Generics? `new`? TargetType `{` TableBody `}`

TargetType    -> Type

TableBody     -> Statement* ( Mapping ( `,` Mapping )* `,`? )?

Statement     -> OpenStmt | NamespaceStmt | ForStmt

OpenStmt      -> `open` ( `{` Type ( `,` Type )* `,`? `}` | Type ) `;`

Mapping       -> Key `:`  ProviderValue
               | Key `->` ProviderValue
               | Key `=>` Path

Key           -> SingleKey | MultiKey | PathKey
SingleKey     -> Generics? Type
MultiKey      -> `[` SingleKey ( `,` SingleKey )* `,`? `]`
PathKey       -> Generics? Path

ProviderValue -> Type
               | IDENTIFIER `<` `new` TargetType `{` TableBody `}` `>`

Path          -> `@` PathSegment ( `.` PathSegment )*
PathSegment   -> Type
```

Every **statement must lead the block, before any mapping** — the parser reads statements first, so an `open`, `namespace`, or `for` written after a `Key: Value` line fails to parse (see error decoding below). A leading `Generics` list makes the whole table generic over the target; `new` also emits the target struct. Of the three mapping operators, `` `:` `` (map to a provider) is the common one, while `` `->` `` (delegate to the value's own entry) and `` `=>` `` (redirect along an `@`-path) belong to the namespace machinery. A `Key` may be one type, a bracketed list expanding to one entry each, or an `@`-path. The nested-table `ProviderValue` form (`UseDelegate<new Inner { … }>`) defines an inner table in place — the legacy dispatch form. An `OpenStmt` opens one component (braces optional) or several (braces required) for per-value `@Component.Key: Provider` wiring folded into the context's own table. **No attributes are accepted** on the table or any entry; an unknown attribute is a spanned error, not silently dropped. See [wiring](wiring.md).

**Expansion invariant.** Each plain `Key: Provider` entry emits a `DelegateComponent<Key>` impl with `Delegate = Provider` plus a forwarding `IsProviderFor` impl that threads the provider's dependencies back through the target. An `open Component;` header emits `DelegateComponent<Component>` = `RedirectLookup<Target, PathCons<Component, Nil>>`, and each `@Component.Value: Provider` entry stores that provider under the path key `PathCons<Component, PathCons<Value, Nil>>` on the target; the `RedirectLookup` impl appends the dispatch parameter onto the path at resolution time.

## `delegate_and_check_components!`

The same table shape, with a table-level check-trait attribute and per-entry check attributes:

```ebnf
DelegateAndCheck -> TableAttr* Generics? `new`? TargetType `{` TableBody `}`

TableAttr        -> `#` `[` `check_trait` `(` IDENTIFIER `)` `]`

TableBody        -> Statement* ( CheckedMapping ( `,` CheckedMapping )* `,`? )?

CheckedMapping   -> EntryAttr? Mapping

EntryAttr        -> `#` `[` `check_params` `(` Type ( `,` Type )* `,`? `)` `]`
                  | `#` `[` `skip_check` `]`
```

`Mapping`, `Key`, `ProviderValue`, and `Statement` are exactly `delegate_components!`'s. This macro wires each entry and derives a check for it, so its check trait defaults to `__CanUse{Context}` (distinct from `check_components!`'s `__Check{Context}`, so both fit one module). Each mapping carries at most one `EntryAttr`; `#[check_params(…)]` supplies the concrete generic parameters a parameterized component's check needs, and `#[skip_check]` wires without checking — the two are mutually exclusive. See [checking](checking.md).

## `check_components!`

One or more check tables, each with optional attributes, generics, a context, an optional `where` clause, and a brace list of entries:

```ebnf
CheckComponents -> CheckTable+

CheckTable      -> TableAttr* Generics? ContextType WhereClause? `{` CheckEntries `}`

TableAttr       -> `#` `[` `check_trait` `(` IDENTIFIER `)` `]`
                 | `#` `[` `check_providers` `(` Type ( `,` Type )* `,`? `)` `]`

ContextType     -> Type

CheckEntries    -> ( CheckEntry ( `,` CheckEntry )* `,`? )?

CheckEntry      -> CheckKey ( `:` CheckValue )?

CheckKey        -> Type
                 | `[` Type ( `,` Type )* `,`? `]`

CheckValue      -> CheckParam
                 | `[` CheckParam ( `,` CheckParam )* `,`? `]`

CheckParam      -> Generics? Type
```

A `CheckEntry`'s value is omitted for a parameterless component and required otherwise; a bracketed key or value expands to the cartesian product, so a set of components is checked against a set of parameters. `#[check_trait(Name)]` overrides the derived `__Check{Context}` name. `#[check_providers(…)]` changes the check: instead of asserting `CanUseComponent` on the context, it asserts `IsProviderFor` on each listed provider, so each layer of a higher-order stack is verified on its own line. See [checking](checking.md).

**Expansion invariant.** A check table emits one marker trait aliasing the asserted bound plus one empty impl per entry — the impl compiles only if the bound holds. The default form aliases `CanUseComponent<Component, Params>` and implements it for the context; `#[check_providers(…)]` aliases `IsProviderFor<Component, Context, Params>` and implements it for each provider. A successful build *is* the passing assertion; the checks have no runtime existence.

## `cgp_namespace!`

The body is an optional generic list and `new`, a namespace name, an optional parent, and a table:

```ebnf
CgpNamespace    -> Generics? `new`? NamespaceName ( `:` ParentNamespace )? `{` NamespaceBody `}`

NamespaceName   -> IDENTIFIER GenericArgs?
ParentNamespace -> TypePath GenericArgs?

NamespaceBody   -> Statement* ( Mapping ( `,` Mapping )* `,`? )?
```

The mappings are `delegate_components!`'s `Mapping` — most often a `` `=>` `` redirect to an `@`-path or a `` `:` `` direct provider. The colon between `NamespaceName` and `ParentNamespace` is the *inheritance* colon, distinct from a mapping's `:`; naming a parent makes the namespace resolve everything the parent does plus its own entries. This macro also owns the two statement forms a context's `delegate_components!` table uses to consume a namespace:

```ebnf
Statement     -> NamespaceStmt | ForStmt

NamespaceStmt -> `namespace` IDENTIFIER `;`

ForStmt       -> `for` `<` IDENTIFIER `,` IDENTIFIER `>` `in` TypePath WhereClause?
                 `{` ( NormalMapping ( `,` NormalMapping )* `,`? )? `}`

NormalMapping -> Key `:` ProviderValue
```

A `NamespaceStmt` forwards every unresolved lookup through the named namespace. A `ForStmt` binds a key variable and a provider variable, reads each entry of the table named after `in`, and emits one `:` mapping per entry; its optional `where` clause is merged into every impl it generates. Like `delegate_components!`, the body accepts no attributes on any entry. See [namespaces](namespaces.md).

---

# Type-level construction macros

The four type-level macros build the vocabulary the rest of CGP keys on, and their grammars are minimal:

```ebnf
SymbolInput  -> STRING_LITERAL

ProductInput -> ( Type ( `,` Type )* `,`? )?          // Product!  (type position)
ProductExpr  -> ( Expression ( `,` Expression )* `,`? )?  // product! (value position)

SumInput     -> ( Type ( `,` Type )* `,`? )?

PathInput    -> `@` PathSegment ( `.` PathSegment )*
PathSegment  -> Type
```

Each expands to a fixed spine, and knowing the spine is what lets you read a printed type. `Symbol!("abc")` becomes `Symbol<3, Chars<'a', Chars<'b', Chars<'c', Nil>>>>` — the leading const is the *byte* length (so `Symbol!("世界")` records `6`), present only because stable Rust cannot compute a `Chars` length in const position. `Product![A, B]` becomes `Cons<A, Cons<B, Nil>>` and `product![…]` builds the matching value; the empty forms are `Product![]`/`product![]` over `Nil`. `Sum![A, B]` becomes `Either<A, Either<B, Void>>`, terminating in the uninhabited `Void` rather than `Nil`. `Path!(@app.error.FooComponent)` becomes a `PathCons` chain in which a lowercase, non-primitive segment is a `Symbol!` and a capitalized or primitive segment stays the named type. See [type-level-primitives](type-level-primitives.md).

---

# Derives

The data derives (`#[derive(HasField)]`, `HasFields`, `CgpData`, `CgpRecord`, `CgpVariant`, `BuildField`, `ExtractField`, `FromVariant`) take no custom arguments — they are ordinary derives whose behavior is fixed by the item they annotate. Their one grammar-shaped constraint is on **variant shape**: the enum derives (`CgpVariant`, `CgpData` on an enum, `ExtractField`, `FromVariant`) require every variant to be a single unnamed-field tuple variant (`Circle(Circle)`), because each variant's payload must be one nameable type. A fieldless (`Empty`), multi-field (`Pair(A, B)`), or struct-style (`Named { x: A }`) variant fails with **"Expected variant to contain exactly one unnamed field"** — wrap a richer payload in a dedicated struct. A named struct field is keyed by `Symbol!`, a tuple-struct field by `Index<N>`. See [extensible-data](extensible-data.md).

---

# Reading error messages

CGP errors come from two layers — the macro's own parser, and the compiler resolving the expanded code — and each has a recognizable vocabulary. The parser errors are usually clear because most macros validate with spanned messages; the type errors are the ones that need decoding, because they name generated traits rather than the mistake you made.

## Wiring and dependency errors

**`the trait bound X: IsProviderFor<SomeComponent, Ctx, …> is not satisfied`** is the most common, and it never means "write an `IsProviderFor` impl." Read it as *the provider `X` is not a valid provider for this component on `Ctx`, because one of its impl-side dependencies is unmet*. The truly missing bound is named nearby (a `HasField`, an abstract type, another capability); that is the thing to supply. `IsProviderFor` exists precisely to surface that named bound instead of a bare "trait not implemented" — see [components](components.md).

**`the trait bound Ctx: DelegateComponent<SomeComponent> is not satisfied`** means the component was never wired on `Ctx` at all — add the `delegate_components!` entry. Contrast this with the `IsProviderFor` failure above, which means the component *was* wired but to a provider whose dependencies fail. A [check](checking.md) distinguishes the two for you: a failed `DelegateComponent` bound is "not wired," a failed `IsProviderFor` bound is "wired but unsatisfied."

**A `CanUseComponent` error** comes from a `check_components!` or `delegate_and_check_components!` assertion firing at the wiring site — which is the point of checking. The named unmet bound is the first impl-side dependency the compiler could not satisfy; trace it through the provider chain to its root. Not every unmet bound is a CGP component: a plain trait or blanket-impl bound a provider also needs has no `…Component` marker and cannot be surfaced through wiring, so it must be satisfied by ordinary Rust means.

**`overflow evaluating the requirement …`** on a component usually means a `UseContext` cycle — a component wired directly to `UseContext` whose only impl of that component *is* that delegation, so the context implements the consumer trait by calling a provider that implements the provider trait by calling the consumer trait. `UseContext` belongs as another provider's inner provider, not as a context's own delegate for the same component; see [wiring](wiring.md).

## Decoding printed type-level values

A long `Symbol<N, Chars<'n', Chars<'a', …>>>` in an error is just a **field-name string** — read the `Chars` characters in order (here `name`) and ignore the leading length. A `Cons<…, Cons<…, Nil>>` is a record field list, an `Either<…, Either<…, Void>>` an enum variant list, and a `PathCons<…, …>` a namespace/redirect route. The [type-level-primitives](type-level-primitives.md) sub-skill is the full decoder ring; the shortcut is that any nested type ending in `Nil` is a product list and any ending in `Void` is a sum.

## Errors that underline the whole macro block

A coherence conflict (**`E0119`, conflicting implementations**) or an unconstrained-parameter error (**`E0207`**) whose caret covers an *entire* macro invocation rather than one entry is a span artifact, not a sign the whole block is wrong. The macros stamp generated items with a re-span onto the originating token, but a synthesized token that has lost its span falls back to the invocation site. When you see `E0119` between two generated impls, look for a **duplicated wiring entry** or two `open`/dispatch keys that overlap on the same type; when you see `E0207`, look for a generic parameter that appears in a provider or `#[cgp_fn]` header but is not bound by any argument or `where` clause.

## Macro-parser errors

These fire before expansion and are worth recognizing by shape. An **`expected `:`` near an `open`/`namespace`/`for`** almost always means a statement was written *after* a mapping — statements must lead the block. An **"unsupported attribute"** (or a spanned rejection) on a `delegate_components!` / `cgp_namespace!` entry means those macros accept no attributes there at all. A **const generic** on a `#[cgp_component]` trait or in a provider trait's argument list is rejected, because CGP dispatches on types, not values (an associated `const` item on the trait is fine). **"Expected variant to contain exactly one unnamed field"** is the variant-derive shape rule above. A **default-bodied `async fn`** inside `#[async_trait]` is unsupported (the body is not wrapped in `async {}`), and a **generic method on a `#[cgp_auto_dispatch]` trait** is rejected because Rust lacks the quantified bound the blanket impl would need.

---

# Further reference

This file is a companion to the per-topic sub-skills, which show the same expansions in worked context: [components](components.md), [wiring](wiring.md), [checking](checking.md), [functions-and-getters](functions-and-getters.md), [abstract-types](abstract-types.md), [higher-order-providers](higher-order-providers.md), [namespaces](namespaces.md), [handlers](handlers.md), [extensible-data](extensible-data.md), and [type-level-primitives](type-level-primitives.md). For the authoritative, exhaustive grammar and expansion of any single construct — every accepted form, every corner case, and the implementing source — fetch the online knowledge base at `https://github.com/contextgeneric/cgp/tree/main/docs`, whose `reference/macros/` documents carry the formal Syntax Grammar and Expansion sections these grammars are drawn from.
