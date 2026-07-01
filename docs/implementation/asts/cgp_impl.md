# The `cgp_impl` AST stack

The `cgp_impl` stack is the sequence of AST types that `#[cgp_impl]` parses into and lowers through: the argument type `ImplArgs`, then the two lowering stages `ItemCgpImpl` and `LoweredCgpImpl`, then the `CgpProviderOrBareImpl` output that either forwards to the [`cgp_provider` stack](cgp_provider.md) or emits an untouched consumer impl. The data flows one way — args plus a `syn::ItemImpl` become `ItemCgpImpl`, which `lower`s into `LoweredCgpImpl`, which `lower`s again into `CgpProviderOrBareImpl`. The [entrypoint document](../entrypoints/cgp_impl.md) covers what each stage produces; this document covers the types. The `self`/`Self` rewrite that these stages depend on is done by the [`replace_self` visitors](#the-replace_self-visitors) described at the end.

## `ImplArgs`

`ImplArgs` is the parsed attribute argument: an optional `new` keyword, the provider type, and an optional `: ComponentType` override. Its `Parse` impl reads the keyword, then the provider type, then a component type if a colon follows:

```rust
pub struct ImplArgs {
    pub new: Option<Keyword<New>>,
    pub provider_type: Type,
    pub component_type: Option<Type>,
}
```

When the macro lowers to the provider stage, these fields are copied straight into a [`ProviderArgs`](cgp_provider.md#providerargs) (the `new` and `component_type` fields), and the `provider_type` becomes the `Self` type of the generated provider impl.

## `ItemCgpImpl`

`ItemCgpImpl` is the raw input stage — the parsed args and the `impl` block before any lowering. Its `lower` step does the attribute processing and header normalization: it parses the CGP modifier attributes off the block into a `CgpImplAttributes`, extracts `#[implicit]` parameters into `HasField` bounds on `Self`, folds `#[uses]`/`#[use_type]`/`#[use_provider]` into the impl generics and bodies, builds a delegation impl for each `#[default_impl]`, and resolves the provider trait path and context type.

The header handling is the one structural point worth knowing. When the block has a `for` clause, the trait path is the provider trait and the block's `Self` type is the context. When the `for` clause is omitted, the block's `Self` type *is* the provider trait path, and `lower` inserts the reserved `__Context__` as the leading impl generic so the next stage always has an explicit context to move into position:

```rust
// input, no `for`            // recorded on LoweredCgpImpl
impl AreaCalculator {         provider_trait_path = AreaCalculator
    fn area(&self) -> f64      context_type        = __Context__
    { … }                      item_impl generics gain a leading __Context__
}
```

`lower` hands the cleaned `item_impl`, the args, the `context_type`, the `provider_trait_path`, and the `default_impls` to `LoweredCgpImpl`.

## `LoweredCgpImpl`

`LoweredCgpImpl` owns the consumer-to-provider rewrite. Its `lower` step branches on the provider type. For the ordinary case it calls `to_raw_item_impl` to produce a provider-trait impl and wraps it in an [`ItemCgpProvider`](cgp_provider.md#itemcgpprovider), reusing that stack's `IsProviderFor` derivation and struct emission. For the `#[cgp_impl(Self)]` case — where the provider type is the literal `Self` — it returns the original block unchanged as a bare consumer impl, requiring a `for` clause and erroring otherwise.

`to_raw_item_impl` is where the rewrite happens: it swaps the block's `Self` type to the provider type, inserts the context type as the provider trait's leading type argument, computes the receiver identifier (the context type snake-cased and double-underscored, or the literal `__context__` when the context is not a plain identifier), and runs the three `replace_self` visitors. It first collects the block's own associated types so the type visitor skips rewriting `Self::Output` and other local associated types.

## `CgpProviderOrBareImpl`

`CgpProviderOrBareImpl` is the output of the second `lower` — an enum with a `Bare` variant holding the untouched consumer impl and a `Provider` variant holding a [`LoweredCgpProvider`](cgp_provider.md#loweredcgpprovider). Its `ToTokens` renders whichever variant it holds, so the `Bare` case emits a single consumer impl while the `Provider` case emits the provider impl, its `IsProviderFor` impl, and any provider struct. The entrypoint appends the `default_impls` after this.

## The `replace_self` visitors

Three `VisitMut` passes in [cgp-macro-core/src/visitors/replace_self/](../../../crates/macros/cgp-macro-core/src/visitors/replace_self/) perform the rewrite that turns consumer-style syntax into provider-style, and they run in this order:

- **`ReplaceSelfTypeVisitor`** rewrites the `Self` type to the context type and `Self::Foo` paths to `Context::Foo`, but skips any path whose associated type is in the block's local associated-type list, so a trait's own `Self::Output` is left intact. It handles macro bodies at the token level too, since `VisitMut` does not see inside a `macro!( … )`.
- **`ReplaceSelfReceiverVisitor`** rewrites the method receiver into an explicit context parameter, preserving the reference and mutability shape: `&self` becomes `ctx: &Context`, `&mut self` becomes `ctx: &mut Context`, `self` becomes `ctx: Context`, and a lifetime on the receiver is carried onto the parameter type.
- **`ReplaceSelfValueVisitor`** rewrites every `self` *value* expression to the context identifier, again descending into macro bodies at the token level, and stops at nested `fn` items since they do not capture the outer `self`.

The receiver identifier and the context type these visitors substitute are the ones `to_raw_item_impl` computed. The same visitors are not used by the [`cgp_provider` stack](cgp_provider.md), which already receives provider-form input.

## Tests

- The stage transforms are exercised end-to-end by the `snapshot_cgp_impl!` expansion snapshots and the behavioral tests indexed in the [entrypoint document](../entrypoints/cgp_impl.md).

## Source

- The stack lives in [cgp-macro-core/src/types/cgp_impl/](../../../crates/macros/cgp-macro-core/src/types/cgp_impl/): `ImplArgs` in `args.rs`, `ItemCgpImpl` in `item.rs`, `LoweredCgpImpl` and `to_raw_item_impl` in `lowered.rs`, and `CgpProviderOrBareImpl` in `provider_or_bare.rs`.
- The companion-attribute parsing is in [cgp-macro-core/src/types/attributes/cgp_impl_attributes.rs](../../../crates/macros/cgp-macro-core/src/types/attributes/cgp_impl_attributes.rs), and the `self`/`Self` rewriting in [cgp-macro-core/src/visitors/replace_self/](../../../crates/macros/cgp-macro-core/src/visitors/replace_self/).
- The provider stage this stack hands off to is documented in [asts/cgp_provider.md](cgp_provider.md).
