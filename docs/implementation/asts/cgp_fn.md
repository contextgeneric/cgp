# The `cgp_fn` AST stack

The `cgp_fn` stack is the short sequence of AST types that `#[cgp_fn]` parses into and transforms through: the raw `ItemCgpFn` and the `PreprocessedItemCgpFn` it normalizes into. Data flows in one direction — an optional trait-name `Ident` plus a `syn::ItemFn` become `ItemCgpFn`, which `preprocess`es into `PreprocessedItemCgpFn`, whose `to_items` renders the trait and blanket impl to a `Vec<syn::Item>`. The [entrypoint document](../entrypoints/cgp_fn.md) covers what each stage produces; this document covers the types and the implicit-argument helpers they lean on.

## `ItemCgpFn`

`ItemCgpFn` is the raw input stage — the parsed attribute identifier and function before any normalization. Its only field beyond the function is the optional trait-name `Ident`, left `None` when the user wrote `#[cgp_fn]` with no argument.

Its `preprocess` step does all the up-front work that the emit stage assumes is already done. It resolves the trait name (the attribute identifier, or the function name run through `to_camel_case_str` to PascalCase), moves the function's visibility aside so the trait can carry it, extracts the `#[implicit]` arguments and prepends their field-reading `let` bindings to the body, parses the companion attributes into a `FunctionAttributes` record, and takes the function's generics out into a separate field. Everything it produces is packaged into `PreprocessedItemCgpFn`.

## `PreprocessedItemCgpFn`

`PreprocessedItemCgpFn` owns the emit stage. It holds the resolved trait name, the normalized `ItemFn` (implicit arguments already removed, body bindings already prepended), the parsed `ImplicitArgFields`, the `FunctionAttributes`, the saved visibility, and the saved generics. Its `to_items` produces the two output items by calling `to_item_trait` and `to_item_impl`.

`to_item_trait` builds the trait: it wraps the function's signature as a `TraitItemFn` with no body, applies the saved generics (dropping the `where` clause, which is impl-side only), extends the supertraits with the `#[extend(...)]` bounds, adds any `#[extend_where(...)]` predicates to the trait's own `where` clause, runs the `#[use_type]` transform, re-attaches the raw attributes, and sets the saved visibility. `to_item_impl` builds the blanket impl: it emits `impl #ident #type_generics for __Context__` with the full function body, inserts `__Context__` as the leading generic parameter, appends any `#[impl_generics(...)]` parameters, then layers the `where` clause — first the `#[uses]`/`#[extend]` bounds as a `Self: …` predicate, then the `#[extend_where]` predicates, then the implicit `HasField` bounds last, and finally the `#[use_type]`/`#[use_provider]` transforms.

The ordering here is the contract the snapshots pin: the implicit-argument bounds are always appended after the attribute-contributed predicates, so a reader of a generated impl sees user-declared dependencies before field requirements.

## Implicit arguments: `ImplicitArgFields` and `ImplicitArgField`

The implicit-argument types are shared building blocks that `#[cgp_fn]` and `#[cgp_impl]` both use, so they live under `types/implicits/` rather than in the `cgp_fn` module. An `ImplicitArgField` records one extracted argument — its field name, the field type to require, the receiver mutability, the field mode (the conversion to apply), and the original argument type — and `ImplicitArgFields` is the collected list.

The extraction is driven by `extract_and_parse_implicit_args`, which pulls every `#[implicit]`-marked argument out of a signature's inputs and parses each into an `ImplicitArgField`. `ImplicitArgField` then contributes in two directions: `to_has_field_bound` produces the `HasField`/`HasFieldMut` bound the impl requires, and `to_statement` produces the `let #name: #arg_type = self.get_field(...) <conversion>;` binding that `prepend_to_block` splices onto the front of the body. The conversion is chosen by `parse_field_type`, the same field-mode logic the getter macros use.

```rust
// for `#[implicit] name: &str` on `&self`, to_statement produces:
let name: &str = self.get_field(PhantomData::<Symbol!("name")>).as_str();
```

`ImplicitArgFields` also carries `extract_from_impl_items`, used by `#[cgp_impl]` to collect implicit arguments across all methods of a provider impl and deduplicate them; `#[cgp_fn]` uses only the single-function `extract_and_parse_implicit_args` path.

## Tests

- The stage transforms are exercised end-to-end by the expansion snapshots indexed in the [entrypoint document's Snapshots section](../entrypoints/cgp_fn.md); there is no separate parser-rejection test file for `#[cgp_fn]` in `cgp-macro-tests`.
- The `&mut self`-with-multiple-implicits rejection and the mutable-pattern rejection enforced during extraction are currently unpinned by any test.

## Source

- The stack lives in [cgp-macro-core/src/types/cgp_fn/](../../../crates/macros/cgp-macro-core/src/types/cgp_fn/): `ItemCgpFn` and its `preprocess` in `item.rs`, `PreprocessedItemCgpFn` and its `to_item_trait`/`to_item_impl` in `preprocessed.rs`.
- The implicit-argument types are in [cgp-macro-core/src/types/implicits/](../../../crates/macros/cgp-macro-core/src/types/implicits/) and their extraction in [cgp-macro-core/src/functions/implicits/](../../../crates/macros/cgp-macro-core/src/functions/implicits/); the field-mode conversion (`parse_field_type`) is in [cgp-macro-core/src/functions/field/parse.rs](../../../crates/macros/cgp-macro-core/src/functions/field/parse.rs), shared with the getter stack in [asts/cgp_getter.md](cgp_getter.md).
- Companion-attribute parsing is in [cgp-macro-core/src/types/attributes/function.rs](../../../crates/macros/cgp-macro-core/src/types/attributes/function.rs).
