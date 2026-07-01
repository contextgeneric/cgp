# `#[cgp_new_provider]` — implementation

`#[cgp_new_provider]` is [`#[cgp_provider]`](cgp_provider.md) with the `new` keyword forced on: it runs the identical lowering but also declares the provider struct, so a provider impl and its `Self` type are defined in one place. This document covers only what distinguishes it from `#[cgp_provider]`; for the accepted syntax and the full expansion, read the reference document [reference/macros/cgp_new_provider.md](../../reference/macros/cgp_new_provider.md).

## Entry point

The macro is driven by the `cgp_new_provider` function in [cgp-macro-lib/src/cgp_new_provider.rs](../../../crates/macros/cgp-macro-lib/src/cgp_new_provider.rs). It parses the same `ProviderArgs` and the same `syn::ItemImpl` as `#[cgp_provider]`, sets `args.new` to enabled, and then runs the identical `ItemCgpProvider::lower`:

```rust
let mut args: ProviderArgs = parse2(attr)?;
args.new = Some(Keyword::default());
let lowered = ItemCgpProvider { args, item_impl }.lower()?;
```

Because everything downstream is shared, the argument grammar, the error paths, and the corner cases are exactly those of `#[cgp_provider]` — the attribute takes only the optional component-type override (the `new` keyword is implied by the macro name, not written in the argument).

## Pipeline

The pipeline is `#[cgp_provider]`'s single `ItemCgpProvider::lower` stage; the [`cgp_provider` AST stack](../asts/cgp_provider.md) documents it in full. The only behavioral difference is that `new` is always set, so `to_provider_struct` always emits a struct rather than returning `None`.

## Generated items

The macro emits the provider impl (verbatim), the derived `IsProviderFor` impl, and the provider struct — the same three items as `#[cgp_impl(new …)]` and the same first two as `#[cgp_provider]`. See [`#[cgp_provider]`'s Generated items](cgp_provider.md#generated-items) for how the `IsProviderFor` arguments are assembled, and its [Behavior and corner cases](cgp_provider.md#behavior-and-corner-cases) for the struct shape (unit struct for a plain name, `PhantomData` field for a generic provider).

## Known issues

None beyond those inherited from [`#[cgp_provider]`](cgp_provider.md#known-issues): a const argument in the provider trait's arguments is rejected with a spanned error.

## Snapshots

`#[cgp_new_provider]` has no `snapshot_cgp_new_provider!` invocation across the suite, so its expansion is not pinned directly. The equivalent output is covered indirectly by the `#[cgp_impl(new …)]` snapshots — which desugar to `#[cgp_new_provider]` — indexed in [`#[cgp_impl]`'s Snapshots section](cgp_impl.md#snapshots), and by the const-generic and lifetime `snapshot_cgp_provider!` cases indexed in [`#[cgp_provider]`'s Tests section](cgp_provider.md#tests). A dedicated `#[cgp_new_provider]` snapshot — showing the struct declaration emitted directly rather than via the `#[cgp_impl]` sugar — is a missing variant.

## Tests

`#[cgp_new_provider]` is exercised directly in real wiring:

- [dispatching/compose.rs](../../../crates/tests/cgp-tests/tests/dispatching/compose.rs) defines composed providers with `#[cgp_new_provider]` and wires them into a context.
- [async_and_send/spawn.rs](../../../crates/tests/cgp-tests/tests/async_and_send/spawn.rs) defines a generic `SpawnAndRun<InCode>` provider with `#[cgp_new_provider]`, exercising the `PhantomData`-field struct shape.

## Source

- Entry point: `cgp_new_provider` in [cgp-macro-lib/src/cgp_new_provider.rs](../../../crates/macros/cgp-macro-lib/src/cgp_new_provider.rs).
- Generation logic — including the struct declaration built by `to_provider_struct` when `new` is set — shared with `#[cgp_provider]`: [cgp-macro-core/src/types/cgp_provider/](../../../crates/macros/cgp-macro-core/src/types/cgp_provider/), documented in [asts/cgp_provider.md](../asts/cgp_provider.md).
- The struct-declaring sugar [`#[cgp_impl(new …)]`](cgp_impl.md) desugars to this macro.
