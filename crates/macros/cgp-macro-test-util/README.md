# `cgp-macro-test-util`

Snapshot-testing macros for the CGP procedural macros.

This crate exposes a family of `snapshot_*!` procedural macros that pin the code the
core CGP macros generate. Each one does **two** things at once: it emits the real
generated code into the surrounding module, exactly as the underlying CGP macro
would — so the traits, structs, and impls stay live and the rest of the module can
still wire them up and assert their runtime behavior — and it generates a `#[test]`
that asserts a pretty-printed string of that same code against an inline
[`insta`](https://insta.rs) snapshot. Adding or removing a snapshot therefore
changes only the golden assertion, never the compile-time or runtime coverage.

Two documents carry what this README does not. The
[implementation document](https://github.com/contextgeneric/cgp-knowledge-base/blob/main/cgp/implementation/entrypoints/snapshot_macros.md)
in the knowledge base explains how the macros are built and what an invocation
expands to; [crates/tests/AGENTS.md](../../tests/AGENTS.md) sets the convention for
*when* to snapshot — a macro's expansion is pinned only in the concept target that
owns that macro's feature, and written plainly everywhere else.

## Crate layout

This is the proc-macro crate, a thin shell: every macro forwards to
[`cgp-macro-test-util-lib`](../cgp-macro-test-util-lib), a normal library crate so
the logic can be unit-tested without the `proc-macro = true` restriction.

```
cgp-macro-test-util/         # proc-macro entry points (#[proc_macro] fns)
└── cgp-macro-test-util-lib/ # the actual implementation
    ├── entrypoints/         # one function per macro
    ├── types/               # parsers (syn `Parse` impls) + MacroSnapshot
    └── functions/           # parse_attribute, pretty_format
```

The implementation calls the real macro logic in `cgp-macro-lib`, not a copy of it,
so a snapshot is guaranteed to show what the production macros generate.

## Available macros

| Snapshot macro              | Wraps                 |
| --------------------------- | --------------------- |
| `snapshot_cgp_component!`   | `#[cgp_component]`    |
| `snapshot_cgp_impl!`        | `#[cgp_impl]`         |
| `snapshot_cgp_provider!`    | `#[cgp_provider]`     |
| `snapshot_cgp_new_provider!`| `#[cgp_new_provider]` |
| `snapshot_cgp_auto_getter!` | `#[cgp_auto_getter]`  |
| `snapshot_cgp_getter!`      | `#[cgp_getter]`       |
| `snapshot_cgp_fn!`          | `#[cgp_fn]`           |
| `snapshot_cgp_type!`        | `#[cgp_type]`         |
| `snapshot_derive_has_field!`| `#[derive(HasField)]` |
| `snapshot_derive_has_fields!`| `#[derive(HasFields)]` |
| `snapshot_derive_cgp_data!` | `#[derive(CgpData)]`  |
| `snapshot_delegate_components!` | `delegate_components!` |
| `snapshot_check_components!` | `check_components!`   |
| `snapshot_delegate_and_check_components!` | `delegate_and_check_components!` |
| `snapshot_cgp_namespace!`   | `cgp_namespace!`      |

Each accepts the same argument forms as the macro it wraps — `snapshot_cgp_component!`
takes both `#[cgp_component(Greeter)]` and the brace form, for instance — precisely
because it drives the production entry function.

## Anatomy of an invocation

Every snapshot macro takes the item under test, written exactly as you would write
the underlying macro invocation, followed by a test block naming the generated
`#[test]` and the identifier bound to the pretty-printed expansion:

```rust
use cgp::prelude::*;
use cgp_macro_test_util::snapshot_cgp_getter;
use insta::assert_snapshot;

snapshot_cgp_getter! {
    #[cgp_getter]
    pub trait HasName {
        fn name(&self) -> &str;
    }

    expand_has_name(output) {
        assert_snapshot!(output, @"...generated code...")
    }
}
```

That emits the real `#[cgp_getter]` expansion — the consumer trait and its blanket
impl, the provider trait, the marker struct, and the `UseField`/`UseFields`/
`RedirectLookup` provider impls — and then a `fn expand_has_name()` holding the same
code as a string. The test-function name must be unique within its module, so give
each snapshot in a file a distinct one.

## Workflow with `insta`

Write the test with an **empty** inline snapshot first (`assert_snapshot!(output, @"")`),
then let `insta` fill it in:

```bash
cargo insta test -p cgp-tests --test getter   # then: cargo insta review
cargo insta test -p cgp-tests --accept        # accept everything non-interactively
INSTA_UPDATE=always cargo test -p cgp-tests   # or via the env var
```

After the first accept, the inline `@"..."` holds the pretty-printed generated code.
On later runs the test fails if that code changes, showing a diff to re-accept once
you have confirmed the change is intended.

## Notes and limitations

- Pretty-printing goes through [`prettyplease`](https://crates.io/crates/prettyplease),
  with macro-prelude noise stripped first (`cgp_macro_core::functions::strip_macro_prelude`),
  so a snapshot is stable, readable Rust source.
- A derive member re-emits the annotated item itself, since a derive macro produces
  only the code it adds and not the struct or enum it decorates.
