# The attribute-modifier AST stacks

The attribute modifiers — `#[uses]`, `#[use_type]`, `#[use_provider]`, `#[extend]`, `#[extend_where]`, `#[derive_delegate]`, and `#[default_impl]` — are not standalone macros; each is an `#[…]` attribute that a host macro strips off its input, parses into an AST type, and folds into the code it generates. This directory documents one modifier per page: each page covers that modifier's AST types, what it parses from, and what it injects into its host's output. This overview covers what the modifiers share — how a host collects them and which host accepts which — so the per-modifier pages can stay focused on their own types. For the user-facing syntax and expansion of each, read the reference documents in the [reference `attributes/` subdirectory](../../../reference/attributes/uses.md); for how the hosts drive them, see [entrypoints/cgp_component.md](../../entrypoints/cgp_component.md), [entrypoints/cgp_impl.md](../../entrypoints/cgp_impl.md), and [entrypoints/cgp_fn.md](../../entrypoints/cgp_fn.md).

## The pages

Each modifier has its own page in this directory:

- [`#[uses]`](uses.md) — import `Self` trait bounds onto a provider impl, reading like a `use` statement.
- [`#[use_type]`](use_type.md) — import an abstract associated type: rewrite the bare alias everywhere and add the owning trait as a bound.
- [`#[use_provider]`](use_provider.md) — complete an inner provider's bound for a higher-order provider.
- [`#[extend]`](extend.md) — add *supertrait* bounds to a generated trait.
- [`#[extend_where]`](extend_where.md) — add `where` predicates to a generated trait definition.
- [`#[derive_delegate]`](derive_delegate.md) — generate a `UseDelegate` dispatcher provider impl for a component.
- [`#[default_impl]`](default_impl.md) — register a provider as a namespace's per-path default.

## How a host collects a modifier

The modifiers do not parse themselves out of the token stream on their own; a host collects them first. Each host macro owns a collector type that walks the item's attribute list once, matches the leading identifier (`uses`, `use_type`, …) of every attribute, parses that attribute's arguments into the corresponding AST type, and passes any unrecognized attribute through untouched onto the generated code. There are three collectors, one per host:

- `CgpComponentAttributes` (in `cgp_component_attributes.rs`) collects the modifiers `#[cgp_component]` accepts, during its `preprocess` stage.
- `CgpImplAttributes` (in `cgp_impl_attributes.rs`) collects the modifiers `#[cgp_impl]` accepts, during `ItemCgpImpl::lower`.
- `FunctionAttributes` (in `function.rs`) collects the modifiers `#[cgp_fn]` accepts (and the getter macros reuse), during `preprocess`.

An unrecognized attribute is never an error: a collector that does not match an attribute's leading identifier pushes it back onto a `raw_attributes` list (or straight back onto the item, for the component collector), which the host re-attaches to the generated code. This is what lets `#[async_trait]`, `#[allow(...)]`, and any other foreign attribute ride through a host macro untouched.

## Which host accepts which modifier

Which modifiers a host accepts differs, because a modifier is only meaningful on the construct that can consume it — `#[derive_delegate]` needs a component's provider trait to dispatch, `#[default_impl]` needs a provider to register, and `#[extend_where]` needs a generated trait whose own `where` clause it can extend. A modifier therefore appears only in the collectors of the hosts that consume it:

| Modifier | `#[cgp_component]` | `#[cgp_impl]` | `#[cgp_fn]` |
|---|:---:|:---:|:---:|
| `#[uses]` | | ✓ | ✓ |
| `#[use_type]` | ✓ | ✓ | ✓ |
| `#[use_provider]` | | ✓ | ✓ |
| `#[extend]` | ✓ | | ✓ |
| `#[extend_where]` | | | ✓ |
| `#[derive_delegate]` | ✓ | | |
| `#[default_impl]` | | ✓ | |

Two further modifiers the same collectors handle are documented elsewhere rather than here, because they belong to another construct's story. `CgpComponentAttributes` also collects `#[prefix(@path in Namespace)]`, which registers a component into a namespace and is documented with the [namespace machinery](../namespace.md). `FunctionAttributes` also collects `#[impl_generics(Param: Bound)]`, which adds a bounded generic parameter to a `#[cgp_fn]`'s impl alone and is documented with [`#[cgp_fn]`](../../entrypoints/cgp_fn.md).

## Source

- The modifiers live in [cgp-macro-core/src/types/attributes/](../../../../crates/macros/cgp-macro-core/src/types/attributes/), one module or submodule per modifier, re-exported from `mod.rs`.
- The host collectors are `CgpComponentAttributes` in [cgp_component_attributes.rs](../../../../crates/macros/cgp-macro-core/src/types/attributes/cgp_component_attributes.rs), `CgpImplAttributes` in [cgp_impl_attributes.rs](../../../../crates/macros/cgp-macro-core/src/types/attributes/cgp_impl_attributes.rs), and `FunctionAttributes` in [function.rs](../../../../crates/macros/cgp-macro-core/src/types/attributes/function.rs).
