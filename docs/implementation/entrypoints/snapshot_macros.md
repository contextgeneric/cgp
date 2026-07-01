# The `snapshot_*!` macro family — implementation

The `snapshot_*!` macros are the test-only proc macros that pin a CGP macro's expansion: each one invokes the real `cgp-macro-lib` entry function on an inline invocation, emits the generated code into the module *so it still compiles*, and generates a `#[test]` that asserts a pretty-printed `insta` snapshot of that same code. This document covers how the family is built; it has no reference document, because these macros are internal test utilities rather than user-facing CGP constructs.

## Entry point

The proc-macro shims live in [cgp-macro-test-util/src/lib.rs](../../../crates/macros/cgp-macro-test-util/src/lib.rs) — one `#[proc_macro]` per snapshot macro, each forwarding to a same-named function in [cgp-macro-test-util-lib](../../../crates/macros/cgp-macro-test-util-lib/) and converting a `syn::Error` into a compile error. The real logic is in `cgp-macro-test-util-lib`, split into the per-macro entrypoints under [src/entrypoints/](../../../crates/macros/cgp-macro-test-util-lib/src/entrypoints/) and the shared snapshot types and helpers. This crate is a peer of `cgp-macro-lib`, not part of `cgp-macro-core`; it depends on `cgp-macro-lib` so that a snapshot runs the *actual* macro rather than a reimplementation.

The family members mirror the CGP macros they pin. The current set is `snapshot_cgp_component!`, `snapshot_cgp_impl!`, `snapshot_cgp_provider!`, `snapshot_cgp_new_provider!`, `snapshot_cgp_fn!`, `snapshot_cgp_getter!`, `snapshot_cgp_auto_getter!`, `snapshot_cgp_type!`, `snapshot_cgp_namespace!`, `snapshot_blanket_trait!`, `snapshot_delegate_components!`, `snapshot_check_components!`, `snapshot_delegate_and_check_components!`, `snapshot_derive_has_field!`, `snapshot_derive_has_fields!`, and `snapshot_derive_cgp_data!`. There is deliberately no snapshot macro for `#[cgp_computer]`, `#[cgp_producer]`, `#[cgp_auto_dispatch]`, or `#[async_trait]` — those extra-feature macros are pinned only behaviorally, through the handler, dispatch, and async tests indexed in their own implementation documents.

## Pipeline

Every family member follows the same three-step shape, differing only in which parser reads the invocation and which `cgp-macro-lib` function it calls:

- **Parse** the macro body into a snapshot wrapper — a wrapper that captures the macro invocation to expand plus the trailing `#[test]` scaffold (the test name, the output binding, and the assertion expression).
- **Expand** by calling the corresponding `cgp-macro-lib` entry function (for example `cgp_macro_lib::cgp_component(attr, body)`) on the captured invocation, producing the same `TokenStream` a user would get.
- **Wrap** the expansion with `MacroSnapshot::wrap_output`, which pretty-prints it and stitches together the final output.

Which parser is used depends on how the pinned macro is invoked, and the three shapes correspond to three wrapper types in [src/types/](../../../crates/macros/cgp-macro-test-util-lib/src/types/):

- **`AttributeMacroSnapshot<Keyword, Item>`** — for an attribute macro. It parses a `#[keyword(...)]` or `#[keyword { ... }]` attribute (its argument tokens become `attr`) followed by the annotated item (`ItemTrait`, `ItemImpl`, …). Used by `snapshot_cgp_component!`, `snapshot_cgp_impl!`, and the other attribute-macro members.
- **`StatementMacroSnapshot<Keyword>`** — for a function-like macro. It parses `keyword! { … }` and hands the brace body straight to the entry function. Used by `snapshot_delegate_components!` and its check-component siblings.
- **`DeriveMacroSnapshot<Keyword, Item>`** — for a derive. It parses `#[derive(Keyword)]` followed by the item, and the entrypoint re-attaches the `#[derive(...)]` before calling the derive expander. Used by `snapshot_derive_has_field!` and the other derive members.

The keyword each wrapper matches is a compile-time marker from [src/keywords.rs](../../../crates/macros/cgp-macro-test-util-lib/src/keywords.rs), built with `cgp-macro-core`'s `define_keyword!`, so the wrapper only accepts the attribute or macro name it is meant to pin.

## Generated items

A `snapshot_*!` invocation expands to the real generated code followed by a `#[test]` that binds the pretty-printed expansion to the declared name and runs the assertion. From this invocation —

```rust
snapshot_cgp_component! {
    #[cgp_component(FooProvider)]
    pub trait CanDoFoo {
        fn foo(&self, value: u32) -> String;
    }

    expand_foo_component(output) {
        insta::assert_snapshot!(output, @"…");
    }
}
```

— `MacroSnapshot::wrap_output` emits the component expansion verbatim (the consumer trait, provider trait, blanket impls, marker struct, and provider impls) and then:

```rust
#[test]
fn expand_foo_component() {
    let output = "<pretty-printed expansion as a string literal>";
    insta::assert_snapshot!(output, @"…");
}
```

Because the expansion is emitted into the module, the snapshot participates in normal compilation exactly like a plain macro invocation would — adding or removing a snapshot changes only the golden assertion, never the compile or runtime coverage. The pretty-printed string is produced by `pretty_format` in [src/functions/pretty_format.rs](../../../crates/macros/cgp-macro-test-util-lib/src/functions/pretty_format.rs): it runs `cgp-macro-core`'s `strip_macro_prelude` over the tokens to drop the `::cgp::macro_prelude::` hygiene prefix, then formats with `prettyplease`, so the snapshot reads as the code a user would write rather than as fully-qualified hygienic output.

## Behavior and corner cases

The snapshot **runs the production macro**, not a copy: any change to a `cgp-macro-lib` entry function's output flows straight into the pinned snapshot string, which is the point — a diff in the golden output is the signal that the expansion changed.

The **derive members re-emit the annotated item themselves**. Because a derive macro produces only the *added* code and not the struct or enum it decorates, an entrypoint like `snapshot_derive_has_field` prepends the original item to the output so the generated impls have a type to attach to.

The **assertion is inline by convention**, following [crates/tests/CLAUDE.md](../../../crates/tests/CLAUDE.md): the snapshot string is written as an `insta` inline snapshot (`@"…"`) in the test file, so the golden output lives beside the invocation rather than in a separate `.snap` file.

## Tests

The snapshot macros are the test harness rather than the thing under test, so their coverage is the set of snapshot invocations across the suite. Per [crates/tests/CLAUDE.md](../../../crates/tests/CLAUDE.md), each CGP macro's expansion is snapshotted only in the concept target that owns its feature, and each owning target's snapshots are indexed by that macro's own implementation document. Representative usages:

- [basic_delegation/component_macro.rs](../../../crates/tests/cgp-tests/tests/basic_delegation/component_macro.rs) — the canonical `snapshot_cgp_component!` invocation, using the `AttributeMacroSnapshot` shape.
- [async_and_send/cgp_fn_async.rs](../../../crates/tests/cgp-tests/tests/async_and_send/cgp_fn_async.rs) — a `snapshot_cgp_fn!` over a stacked `#[cgp_fn]` / `#[async_trait]`.
- [dispatching/use_delegate_getter.rs](../../../crates/tests/cgp-tests/tests/dispatching/use_delegate_getter.rs) — `snapshot_delegate_components!` and `snapshot_delegate_and_check_components!`, using the `StatementMacroSnapshot` shape.

The per-macro Snapshots sections — for example [cgp_component.md](cgp_component.md) — are the canonical index of which expansion variants each family member pins and which are still missing.

## Source

- Proc-macro shims: [cgp-macro-test-util/src/lib.rs](../../../crates/macros/cgp-macro-test-util/src/lib.rs).
- Per-macro entrypoints: [cgp-macro-test-util-lib/src/entrypoints/](../../../crates/macros/cgp-macro-test-util-lib/src/entrypoints/).
- Snapshot wrapper types: [src/types/](../../../crates/macros/cgp-macro-test-util-lib/src/types/).
- Pretty-printer: [src/functions/pretty_format.rs](../../../crates/macros/cgp-macro-test-util-lib/src/functions/pretty_format.rs).
- Keyword markers: [src/keywords.rs](../../../crates/macros/cgp-macro-test-util-lib/src/keywords.rs).
- Each snapshot calls the matching production entry function in [cgp-macro-lib](../../../crates/macros/cgp-macro-lib/).
