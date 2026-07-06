# CGP Implementation Reference

This directory documents the *internals* of the CGP macros — how each macro is implemented in [crates/macros/cgp-macro-core](../../crates/macros/cgp-macro-core) and [crates/macros/cgp-macro-lib](../../crates/macros/cgp-macro-lib), including corner-case behavior, known limitations and bugs, and the test suite that exercises each construct. It is the documentation an agent reviewing or maintaining the macro source reads first: it records the current state of the code in one place so an agent can pick up a construct's implementation from where the last one left off. The authoring rules, document templates, and the synchronization rule that binds these documents to the code live in [AGENTS.md](AGENTS.md).

These documents complement the [construct reference](../reference/README.md) rather than repeating it. The reference explains what each construct does for a *user* — its accepted syntax and the code it expands to — and points only at library source. The implementation documents explain how the macro *produces* that behavior, and they are the sole home for every pointer into the [test suite](../../crates/tests): behavioral tests, failure cases, and macro-expansion snapshots all index from here. A reference document links to its implementation counterpart to elaborate a corner case; an implementation document links back to the reference for the user-facing semantics.

Before reading any single document, skim [Cross-cutting implementation notes](#cross-cutting-implementation-notes) at the end of this file. It records the recurring, easy-to-misread mechanics — the leading-generic insertion that survives lifetimes, how the generic kinds are kept apart, spans, parsing, and hygiene — that a reviewer needs in mind for every macro.

## Directory layout

The tree is organized by the kind of source construct each document describes, so an agent has an obvious place to start for "the macro entry point", "the AST type behind a stage", "a helper function", or "an internal macro". A new document goes in the matching subdirectory and registers itself in the catalog below in the same change.

The [entrypoints/](entrypoints/) directory holds one document per CGP macro — the top-level procedural macro a programmer invokes — describing its `cgp-macro-lib` entry function, the transform pipeline it drives, the items it emits, its corner cases, known issues, tests, and snapshots. The [asts/](asts/) directory holds one document per evaluation stack of AST constructs — the `cgp-macro-core` types implementing `Parse` or `ToTokens`, or serving as an intermediate representation — with the types of one pipeline grouped into a single document. The [functions/](functions/) directory holds the standalone helper functions, split into [functions/parse/](functions/parse/) for parsing helpers and [functions/derive/](functions/derive/) for code-synthesis helpers. The [macros/](macros/) directory holds the internal `macro_rules!` macros the implementation is written in, such as `parse_internal!` and `define_keyword!`.

## Catalog

This section is the index of implementation documents. When you add a document, register it here in the same change.

### Entrypoints — [entrypoints/](entrypoints/)

The component and provider macros:

- [`#[cgp_component]`](entrypoints/cgp_component.md) — the foundational component-definition macro and its `preprocess → eval → to_items` pipeline.
- [`#[cgp_impl]`](entrypoints/cgp_impl.md) — lowers consumer-style syntax into a provider impl and hands it to `#[cgp_provider]`.
- [`#[cgp_provider]`](entrypoints/cgp_provider.md) — passes a provider-trait impl through and derives its `IsProviderFor` impl.
- [`#[cgp_new_provider]`](entrypoints/cgp_new_provider.md) — `#[cgp_provider]` with the provider struct also declared.

Functions and getters:

- [`#[cgp_fn]`](entrypoints/cgp_fn.md) — a single-implementation capability as a blanket-impl trait, with `#[implicit]` argument lowering.
- [`#[cgp_getter]`](entrypoints/cgp_getter.md) — a getter component wired through CGP, adding `UseField`/`UseFields` provider impls.
- [`#[cgp_auto_getter]`](entrypoints/cgp_auto_getter.md) — a getter as a blanket impl over `HasField`.

Abstract types and blanket traits:

- [`#[cgp_type]`](entrypoints/cgp_type.md) — an abstract-type component, reusing the `#[cgp_component]` pipeline and adding `UseType`.
- [`#[blanket_trait]`](entrypoints/blanket_trait.md) — a blanket impl generated from a trait with default methods.

Wiring and checking:

- [`delegate_components!`](entrypoints/delegate_components.md) — the context wiring table and its mapping/statement grammar.
- [`check_components!`](entrypoints/check_components.md) — compile-time wiring assertions.
- [`delegate_and_check_components!`](entrypoints/delegate_and_check_components.md) — wire and check in one macro.
- [`cgp_namespace!`](entrypoints/cgp_namespace.md) — reusable, inheritable wiring tables via `RedirectLookup`.

Type-level construction macros:

- [`Symbol!`](entrypoints/symbol.md), [`Product!`](entrypoints/product.md), [`Sum!`](entrypoints/sum.md), [`Path!`](entrypoints/path.md) — the type-level string, list, sum, and path macros.

Data derives:

- [`#[derive(HasField)]`](entrypoints/derive_has_field.md), [`#[derive(HasFields)]`](entrypoints/derive_has_fields.md) — field-access derives.
- [`#[derive(CgpData)]`](entrypoints/derive_cgp_data.md), [`#[derive(CgpRecord)]`](entrypoints/derive_cgp_record.md), [`#[derive(CgpVariant)]`](entrypoints/derive_cgp_variant.md) — the extensible-data derives.
- [`#[derive(BuildField)]`](entrypoints/derive_build_field.md), [`#[derive(ExtractField)]`](entrypoints/derive_extract_field.md), [`#[derive(FromVariant)]`](entrypoints/derive_from_variant.md) — builder/extractor/variant support.

Handlers and other extra macros:

- [`#[cgp_computer]`](entrypoints/cgp_computer.md), [`#[cgp_producer]`](entrypoints/cgp_producer.md) — define `Computer`/`Producer` providers from functions.
- [`#[cgp_auto_dispatch]`](entrypoints/cgp_auto_dispatch.md) — generate a dispatching handler.
- [`#[async_trait]`](entrypoints/async_trait.md) — rewrite trait `async fn` to `-> impl Future`.
- [The `snapshot_*!` family](entrypoints/snapshot_macros.md) — the `cgp-macro-test-util` macros that pin macro expansions as `insta` snapshots.

### AST stacks — [asts/](asts/)

One document per evaluation stack, grouped by the macro that owns it:

- [cgp_component](asts/cgp_component.md), [cgp_impl](asts/cgp_impl.md), [cgp_provider](asts/cgp_provider.md), [cgp_type](asts/cgp_type.md), [cgp_fn](asts/cgp_fn.md), [cgp_getter](asts/cgp_getter.md), [blanket_trait](asts/blanket_trait.md).
- [delegate_component](asts/delegate_component.md), [check_components](asts/check_components.md), [namespace](asts/namespace.md).
- [cgp_data](asts/cgp_data.md) — the shared extensible-data derive stack.
- [product](asts/product.md), [sum](asts/sum.md), [path](asts/path.md), [symbol](asts/symbol.md) — the type-level construction stacks.
- [attributes/](asts/attributes/README.md) — the modifier-attribute AST stacks, one page per modifier: [`#[uses]`](asts/attributes/uses.md), [`#[use_type]`](asts/attributes/use_type.md), [`#[use_provider]`](asts/attributes/use_provider.md), [`#[extend]`](asts/attributes/extend.md), [`#[extend_where]`](asts/attributes/extend_where.md), [`#[derive_delegate]`](asts/attributes/derive_delegate.md), and [`#[default_impl]`](asts/attributes/default_impl.md).

### Functions — [functions/](functions/)

The cross-cutting helper functions; construct-specific parse/derive helpers are documented inside the owning macro's entrypoint or AST document.

- [Delegated-impl synthesis](functions/derive/delegated_impls.md) — the forwarding-impl machinery shared by the component impls.
- [`parse_is_provider_params`](functions/parse/is_provider_params.md) — building the `IsProviderFor` params tuple from trait generics.
- [`merge_generics`](functions/derive/generics.md) — combining two `Generics` into one.
- [Identifier case conversion](functions/derive/idents.md) — the PascalCase/snake_case/reserved-name helpers.

### Internal macros — [macros/](macros/)

- [`parse_internal!`](macros/parse_internal.md) — build a `syn` node from quoted tokens with a descriptive parse error.
- [`define_keyword!`](macros/define_keyword.md) — declare a custom-keyword marker type implementing `IsKeyword`.
- [`export_construct!` / `export_constructs!`](macros/export_constructs.md) — declare the hygienic markers backing `exports.rs`.

## Cross-cutting implementation notes

The macros share a handful of mechanics that are easy to misread and that recur across many constructs, so a reviewer should hold them in mind before reading any single document. This section explains *why the code behaves as it does*; the review workflow in [../../AGENTS.md](../../AGENTS.md) under "Scrutinize the macro codegen" lists *what to check* against these same mechanics for each macro, and the per-construct documents show where each one bites.

### Generic-parameter insertion and lifetime ordering

Almost every component and provider macro turns a trait or impl "inside out" by inserting the context — or the provider — as the *leading* generic parameter, and it does so with `insert(0, …)`, pushing the new parameter to the front of a parameter or argument list. This is the single detail that most often misleads a reader, whether an AI agent or a human: Rust requires lifetime parameters to come *before* type and const parameters, so inserting a type at position 0 looks like it places the type ahead of an existing lifetime and emits invalid Rust — `impl<Ctx, 'a>` or `Trait<Ctx, 'a>`, which the compiler rejects with `E0747`. The insertion is nonetheless correct, and understanding why prevents both a wrong "bug report" against working code and a real bug in new code that skips the step that saves it.

It works because the built tokens are always emitted through `syn`, which re-orders lifetimes to the front. There are two paths, one for each kind of list the macros insert into:

- When the target is a **`syn::Generics`** — the `impl<…>` parameter list — `Generics::to_tokens` itself re-emits lifetimes ahead of type and const parameters. Inserting `__Context__` at position 0 of `<'a, T>` leaves the `Punctuated` in the order `[__Context__, 'a, T]`, yet the emitted tokens are `<'a, __Context__, T>`.
- When the target is a **type-argument list** — the `<…>` in `Foo<…>` — the code builds the list with CGP's own `TypeArgs`, whose `to_tokens` does *not* re-order, and then round-trips it through `parse_internal(path.to_token_stream())` into a `syn::Path`. The `syn::AngleBracketedGenericArguments` behind that path re-emits lifetimes first, so the ordering is normalized by the re-parse rather than at insertion.

The `#[cgp_impl]` lifetime case makes both effects concrete. Writing

```rust
#[cgp_impl(new GetReference)]
impl<'a> ReferenceGetter<'a, str> for App<'a> { /* … */ }
```

builds the provider trait path as `ReferenceGetter<App<'a>, 'a, str>` — the context inserted ahead of the lifetime — but the `parse_internal` round-trip emits it as `ReferenceGetter<'a, App<'a>, str>`, and the impl generics likewise emit lifetimes-first.

The caveat is the reason both paths matter: **CGP's `TypeArgs` (and its `to_tokens_angle_bracketed`) does not re-order on its own.** Position-0 insertion into a `TypeArgs` is safe *only* because a `syn` re-parse follows it; a new code path that inserts into a `TypeArgs` and emits it directly, without that round-trip, would keep the invalid `<Ctx, 'a>` ordering and produce `E0747`. When adding such a site, either round-trip the tokens through `parse_internal` or insert after the lifetimes.

Every insertion site carries an inline comment flagging this; grep `cgp-macro-core` for `insert(0` to find them. The canonical examples are `to_raw_item_impl` in the [`cgp_impl` stack](asts/cgp_impl.md) for the provider macros, and the provider-trait and consumer-blanket-impl builders in the [`cgp_component` stack](asts/cgp_component.md) for `#[cgp_component]`.

### Generics: keep the kinds and roles distinct

Generics are the most error-prone part of the codegen beyond insertion order, because a parameter's *kind* — lifetime, type, or const — and its *role* — an `impl<T>` parameter versus the `<T>` in `Foo<T>` — each need different handling, and mixing them emits subtly wrong output. CGP keeps them in distinct types: `syn::Generics` for impl-position parameters, and its own `TypeGenerics`/`TypeArgs`/`TypeArg` types (under `cgp-macro-core/src/types/ident` and `types/generics`) for argument-position lists. A review should confirm a parameter never crosses from one to the other in the wrong form — a type parameter emitted where an argument belongs, or a lifetime dropped into an argument tuple as if it were a type.

A few rules recur across the suite and are worth stating plainly. A lifetime is lifted into `Life<'a>` wherever it must stand in type position — inside the `IsProviderFor` params tuple, built by [`parse_is_provider_params`](functions/parse/is_provider_params.md), and inside a provider struct's `PhantomData`. A const argument is rejected in a provider trait's own argument list, because it cannot key the type-based `IsProviderFor` tuple, yet it flows through untouched as a const generic on the provider *struct*. Parameters merged from two sources — the trait's own generics plus an inserted context, say — pass through [`merge_generics`](functions/derive/generics.md) so they cannot collide. Finally, every parameter that appears in a generated header must be bound in that header, or the compiler reports the free parameter as `E0207`.

### Spans: aim generated items at the token the user wrote

A generated item's tokens carry whatever span the macro stamps on them, and by default `parse_internal!`/`quote!` stamp every token they synthesize with the macro's `call_site` span — the whole invocation. Placing those spans correctly matters for two separate tools, and a stray `call_site` span misleads each in its own way: the **compiler** aims an error caret at the wrong place, and **rust-analyzer** resolves go-to-definition to the wrong place. Both symptoms trace to the same cause — a token that should carry the narrow span of what the user wrote instead carries `call_site` — so the macros re-span their generated tokens onto the originating user token. A reviewer must weigh both consequences, not just the error carets, because only the carets are pinned by a test.

The compiler derives an item's error span by joining that item's first and last tokens. A coherence conflict (`E0119`) between two generated impls, an unsatisfied bound, or a name-resolution failure on a generated header therefore underlines the *entire macro block* whenever the item sits at `call_site`, saying nothing about which entry to fix. The [`override_item_span`](../../crates/macros/cgp-macro-core/src/functions/override_span.rs) helper corrects this by re-spanning only the item's two *boundary* tokens — its leading `impl` keyword and its trailing `{ … }` body — onto the entry the user wrote; joining just those two collapses the caret onto the entry. It moves a boundary token only when that token is itself synthesized, telling a synthesized token from a user one by source text: a `call_site` token's source text is the whole invocation, which no narrower user token can equal.

rust-analyzer maps a source token to its expanded counterpart purely by source range, ignoring hygiene, so a token re-spanned onto a user token's range becomes, as far as the editor is concerned, that user token — and go-to-definition on it offers every construct that now shares the range. Two distinct `call_site` leaks cause this, and both are subtle because the compiler is indifferent to them:

- **Re-spanning a resolvable reference onto the entry.** If `override_item_span` moved the whole impl rather than its boundary, a synthesized reference such as `IsProviderFor` or `DelegateComponent` would land on the delegate key's range, and a jump on that key would offer those traits alongside the component the user meant. This is exactly why the helper moves only the boundary — a keyword and a delimiter, never a reference — and leaves the interior at its own spans. Leaving the interior alone is also what keeps a wired provider navigable and keeps a per-entry generic's unconstrained-parameter `E0207` pointing at the `<T>` the user wrote.
- **A derived name stamped with `call_site`.** The `#[cgp_component]` marker struct `{Provider}Component`, when its name is not given explicitly, is derived from the provider identifier. Deriving it with `Span::call_site()` gave the generated `struct {Provider}Component` a definition span covering the whole `#[cgp_component(..)]` attribute — a range that also contains the `cgp_component` macro path — so go-to-definition on a delegate key naming that component offered both the component and the macro. Deriving the name from the provider identifier's own span, the way [`derive_check_trait_ident`](../../crates/macros/cgp-macro-core/src/types/check_components/table.rs) already spans its derived check trait, points the definition at a single user token and removes the ambiguity. It surfaced only across crates: within the defining crate rust-analyzer resolves from the live macro expansion, whereas a dependent crate relies on the recorded definition span, which is where the leak shows.

Two more re-spanning techniques cover the rest. `override_item_span`'s sibling `override_span` re-spans *every* token unconditionally, which is right only for `check_components!`, where the intent is to clobber one shared user token — the context type — onto each checked component in turn. Where the originating token is itself synthesized and has lost its span (an `@`-path's `PathCons<..>` nest), the evaluated form carries an explicit span field forward, as `EvaluatedCheckEntry.span` and `EvaluatedDelegateEntry.span` do; and the provider macros reuse the user's own `for` token so the middle of a generated impl header never falls on `call_site`.

Only the compiler side is pinned by a test: a `trybuild` `.stderr` fixture records the exact line and column of each caret, so an error-span regression changes a snapshot. The rust-analyzer behavior has no fixture and must be checked by hand in an editor — and checked across crates as well as within one, since the derived-name leak above is invisible from inside the defining crate. The per-macro detail lives in each document's error-spans discussion; [`delegate_components!`](entrypoints/delegate_components.md) and [`#[cgp_impl]`](entrypoints/cgp_impl.md) are the worked examples.

### Parsing: build with `parse_internal!`, and distrust `syn`'s leniency

Build every `syn` node from quasi-quoted tokens with [`parse_internal!`](macros/parse_internal.md) rather than `parse2` or `parse_quote!`, so a malformed fragment fails with an error naming the target type and the offending tokens instead of a bare parse error or a panic that aborts the compiler. A function that parses anything should thread `syn::Result` and propagate the error; reserve the panicking `parse_quote!` for tokens that are trivially, locally guaranteed to parse. The other half of this concern is that **`syn` parses far more leniently than Rust accepts** — it takes a type argument before a lifetime, a turbofish where CGP wants none, and associated-type bindings in plain argument positions — so CGP defines its own restricted argument types — `TypeArg`, `TypeArgs`, and `PathWithTypeArgs` — to reject at parse time what `syn` would silently wave through, and a review should confirm user-facing input is validated against those rather than raw `syn` nodes. The same leniency is what makes the lifetime round-trip above work: `syn` accepts the mis-ordered `<Ctx, 'a>` on parse and quietly re-emits it correctly.

### Hygiene: exports markers and reserved identifiers

The expansion must compile in a crate that has only `cgp` in scope, so every CGP item it references is emitted through a [`crate::exports`](macros/export_constructs.md) marker that resolves to `::cgp::macro_prelude::<Name>`, never as a bare or hand-written path; a review greps the codegen for any CGP name not interpolated from an `exports` marker. The reserved identifiers the expansion introduces — `__Context__`, `__Provider__`, `__Component__`, `__Components__`, `__context__`, and the like — are wrapped in double underscores so they cannot collide with a user's own names, and a new identifier the codegen invents should follow the same convention.
