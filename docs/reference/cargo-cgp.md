# cargo-cgp — the CGP error toolchain

`cargo-cgp` is CGP's first-class toolchain: a cargo subcommand that stands in for `cargo check` and rewrites CGP's compiler errors into a compact, root-cause-first form, and that also expands a target's macros with CGP's type-level constructs made legible. It is the **recommended way to build and check CGP code and to read a CGP compile error**, and an agent working in this repository should reach for it before plain `cargo check` when diagnosing a wiring failure. This document is enough to install and use it without leaving `cgp`; the tool itself lives in a separate repository, [github.com/contextgeneric/cargo-cgp](https://github.com/contextgeneric/cargo-cgp), and its own docs are the exhaustive reference.

This is a *tooling* reference, not a CGP construct — it is the one document under `reference/` that describes an external tool rather than a macro, trait, or provider. The [error catalog](../errors/README.md) is its companion: the catalog documents each error *class* and how `cargo-cgp` presents it, while this document covers running the tool — both its `check` and its [`expand`](#expanding-the-generated-code) command.

## Why it exists

A CGP macro expands to ordinary Rust, so the compiler type-checks the *generated* code, and a small wiring mistake surfaces as a wall of errors naming types the programmer never wrote — with the real cause buried under machinery like `IsProviderFor` and `CanUseComponent`, encoded as a nested `Symbol<…>` spine, or hidden from the output entirely (the [hidden-versus-surfaced axis](../errors/README.md#the-central-axis-hidden-versus-surfaced) the catalog is built around). `cargo-cgp` reads those diagnostics inside the compiler and re-presents them: it names the real cause, renders the dependency chain that leads to it as a `cargo tree`-style tree, and tags each rewritten message with a `[CGP-Exxx]` code — much as Clippy layers its own analysis on top of `rustc`.

Two mechanisms do the work, and both matter when reading its output. It compiles through a `rustc` wrapper that turns on the **next-generation trait solver** (`-Znext-solver`), which surfaces dependency errors the default solver hides — this alone recovers the root cause of the [hidden unsatisfied-dependency](../errors/hidden/unsatisfied-dependency.md) class that plain `cargo check` suppresses. On top of that it **rewrites the classes it recognizes** into the coded, root-cause-first form. Classes it does not yet rewrite pass through as the compiler wrote them.

## Installing

`cargo-cgp` is two binaries — the `cargo-cgp` front-end and a `cargo-cgp-driver` that links the compiler internals — and it requires an exact Rust nightly (the one the driver is built against). That nightly is installed by `cargo cgp setup` (or built by the Nix flake), and cargo-cgp forces it only for its own check, so **your `cgp` project keeps its own toolchain** — the pinned stable in [`rust-toolchain.toml`](../../rust-toolchain.toml) — for its ordinary builds.

With **cargo** (rustup present):

```sh
cargo install cargo-cgp      # the front-end; builds on any toolchain
cargo cgp setup              # provisions the pinned nightly + driver, in lockstep
```

With **Nix** (pinning the pre-release tag for a reproducible install):

```sh
nix profile install github:contextgeneric/cargo-cgp/v0.1.0-alpha
```

The full matrix — installing from source, updating, and uninstalling — is in cargo-cgp's [installation guide](https://github.com/contextgeneric/cargo-cgp/blob/main/docs/reference/installation.md).

## Running it without installing

To check a project through the tool without installing anything — the quickest way to try it on this repository's examples or a scratch reproduction — run the flake's default app from the project directory, pinning the tag:

```sh
cd /path/to/a/cgp/project
nix run github:contextgeneric/cargo-cgp/v0.1.0-alpha -- check
```

Everything after `--` is forwarded to `cargo check`. This needs no rustup and leaves the project's toolchain and `target/` untouched.

## Using it

Run it wherever you would run `cargo check`:

```sh
cargo cgp check                 # like `cargo check`, with CGP errors reshaped
cargo cgp check --workspace     # every argument after `check` is forwarded to `cargo check`
```

`check` is the command you reach for day to day; its companion [`expand`](#expanding-the-generated-code) shows the Rust your macros generate. Neither changes how you build: both re-compile your code under the pinned nightly purely to show you something, and there is no `cargo cgp build`, `run`, or `test`; run those with plain cargo (besides these two, only `setup` and `update` exist, and they merely provision the tool). **cargo-cgp is optional, and its advantage is readable errors — and readable expansions — during development.** CGP itself is an ordinary library that compiles on any stable Rust ≥ 1.89, so `cargo check`, `cargo build`, `cargo run`, and `cargo test` all work on a CGP project unchanged. Reach for `cargo cgp check` when you hit — or expect — a wiring error and want it readable; use plain `cargo check` when you do not; and always build, run, and test with ordinary cargo.

Read its output as ordinary rustc/cargo diagnostics with the recognized CGP errors rewritten. When the tool rewrites an error into a known class it stamps a short code in brackets and leads with the cause:

```text
error[E0277]: [CGP-E001] the consumer trait `CanCalculateArea` is not implemented for context `Rectangle`
   = note: root cause: [CGP-E106] missing field `height` on `Rectangle`
           this is required through the dependency chain:
             [CGP-E101] consumer trait impl `CanCalculateArea` for context `Rectangle`
             └─ [CGP-E102] provider trait impl `AreaCalculator` with context `Rectangle` for provider `RectangleArea`
               └─ [CGP-E106] missing field `height` on `Rectangle`
```

The diagnostic keeps its own Rust code (`E0277`), so `rustc --explain` still works; the `[CGP-Exxx]` tag rides inside the message. The codes are catalogued in cargo-cgp's [error-code catalog](https://github.com/contextgeneric/cargo-cgp/blob/main/docs/error-code.md), and each class in this repository's [error catalog](../errors/README.md) records which code it maps to under its *How cargo-cgp presents it* section.

To use it as **Rust Analyzer's** on-save check backend so the reshaped errors appear inline, wire it through `check.overrideCommand` (it is a two-word command and must emit JSON):

```jsonc
"rust-analyzer.check.overrideCommand": [
  "cargo", "cgp", "check", "--workspace", "--all-targets", "--message-format=json"
]
```

Never apply this to a user's Rust Analyzer configuration on your own initiative — present it as an option and edit their editor settings only when they explicitly ask.

## Expanding the generated code

`cargo cgp expand` prints the crate as the compiler sees it after macro expansion, which answers the question a CGP error cannot: *what did that macro actually generate?* Reach for it when a wiring failure stops making sense from the message alone — the failure is always "the emitted impls do not resolve," and you cannot reason about which impl is wrong until you read it — or simply to confirm what a construct produces before trusting your memory of it.

What makes it worth using over plain `cargo expand` is that CGP's type-level constructs come back **resugared**: a field tag reads `Symbol!("width")` rather than the raw `Symbol<5, Chars<'w', …>>` spine the compiler prints, a handler pipeline reads `Product![StepOne, StepTwo]`, and a namespace key reads `Path!(@app.GreeterComponent)`. Everything else is a full expansion — `#[derive(Debug)]` and `println!` appear in their generated form too — so what you get is the whole program with the CGP parts legible. Expanding the [area-calculation](../examples/area-calculation.md) example's `rectangle_area` function shows the shape:

```sh
cargo cgp expand --lib --item RectangleArea
```

```rust
pub trait RectangleArea {
    fn rectangle_area(&self) -> f64;
}
impl<__Context__> RectangleArea for __Context__
where
    Self: HasField<Symbol!("width"), Value = f64>
        + HasField<Symbol!("height"), Value = f64>,
{
    fn rectangle_area(&self) -> f64 {
        let width: f64 = self
            .get_field(::core::marker::PhantomData::<Symbol!("width")>)
            .clone();
        let height: f64 = self
            .get_field(::core::marker::PhantomData::<Symbol!("height")>)
            .clone();
        width * height
    }
}
```

Reading that answers two questions at once: what the trait `#[cgp_fn]` generated looks like, and what the `#[implicit]` arguments turned into — a `HasField` bound per argument and a `get_field` call that clones the value.

**It expands exactly one target**, so a package with both a library and a binary needs `--lib` or `--bin NAME`; without one, cargo declines with *"extra arguments to `rustc` can only be passed to one target"*. Everything else is forwarded to `cargo rustc`, so `-p`, `--features`, and the rest work as usual, and the expansion goes to stdout so it pipes and redirects:

```sh
cargo cgp expand --lib                       # the whole library target
cargo cgp expand --bin server                # one binary
cargo cgp expand --lib > expanded.rs         # stdout, so redirect or pipe freely
```

**A whole crate's expansion is long, so `--item <path>` narrows it to one part.** The path names a module or item inside the crate being expanded, written as a `::`-separated path with an optional leading `crate::` (`self::` and a bare `::` work too), and what it selects depends on what it names:

```sh
cargo cgp expand --lib --item contexts              # a module: its contents
cargo cgp expand --lib --item contexts::MockApp     # a type: its declaration and every impl for it
cargo cgp expand --lib --item AreaCalculator        # a trait: its definition and every impl of it
```

The **trait** form is the one to reach for on CGP code, because a component's generated items are almost all impls and an impl has no name of its own. Naming a component's *provider* trait (`--item AreaCalculator`) gives you the provider trait definition, the delegation blanket impl every wired context resolves through, the [`UseContext`](providers/use_context.md) and [`RedirectLookup`](providers/redirect_lookup.md) impls, and each provider's own impl of it — the whole set of things that can answer "which provider serves this component, and under what bounds?" Naming the *consumer* trait (`--item CanCalculateArea`) is the smaller companion: the consumer trait and the one blanket impl that routes it to the provider trait.

The **type** form is what you want for a context. Naming a context (`--item Rectangle`) gives its struct, the `HasField`/`HasFieldMut` impls its [`#[derive(HasField)]`](derives/derive_has_field.md) generated, and its `DelegateComponent` wiring entries with the real key and provider types — which is how you check that a wiring table produced the keys you intended rather than inferring them from the table's syntax.

If a path matches nothing you get an error saying so, naming the path, rather than a silent whole-crate expansion.

Two limits are worth knowing. `expand` is **not a check**: it stops as soon as the macros are expanded, so type-checking never runs and it reports nothing about wiring — a malformed macro invocation still fails, since that happens during expansion, but a missing field does not. And the output is meant to be *read* rather than compiled: the `cgp::macro_prelude::` qualifier the macros emit is stripped for legibility, and an `open` statement's per-key wiring entry keeps its raw `PathCons<…>` key, since its tail is a generic parameter that no `Path!` spelling covers.

`expand` is newer than the v0.1.0-alpha release, so a `cargo install cargo-cgp` from crates.io does not yet carry it. Until the next release, get it by dropping the tag from the Nix reference (`nix run github:contextgeneric/cargo-cgp -- expand --lib`, which tracks `main`) or by [building from a checkout](https://github.com/contextgeneric/cargo-cgp/blob/main/docs/reference/installation.md#installing-from-source).

## When cargo-cgp is not available

If the tool is not installed and cannot be run, fall back to reading the raw compiler output directly. Every class in the [error catalog](../errors/README.md) documents that raw diagnostic — its code, shape, and whether the root cause is present — as the fallback for exactly this case, and the [error-extraction sub-skill](https://github.com/contextgeneric/cgp/blob/main/docs/skills/cgp/references/error-extraction.md) is the technique for reducing a raw cascade to its cause by hand. The key difference to remember: without `cargo-cgp`'s next-gen solver, the [hidden classes](../errors/hidden/unsatisfied-dependency.md) genuinely omit the cause, so promote them with a `check_components!` at the wiring site to make it appear.

## Version compatibility

This documentation is written for **`cgp` v0.8.0** and **`cargo-cgp` v0.1.0-alpha**. The two version independently: `cargo-cgp` reads `cgp`'s stable, macro-generated surface (the consumer/provider traits, `DelegateComponent`, `HasField`, and the rest), so a newer `cargo-cgp` works against this `cgp`, and a newer `cgp` generally works under this `cargo-cgp`. If the tool reports a version far behind this one, recommend updating it (`cargo cgp update`, or refresh the Nix flake); the `/cgp` skill records the versions it was written against and how to reconcile a mismatch.

## Further reading

All in the `cargo-cgp` repository (link as GitHub URLs, read from `../cargo-cgp` locally when checked out):

- [Usage](https://github.com/contextgeneric/cargo-cgp/blob/main/docs/reference/usage.md) — running the check, reading output, expanding a target, editor integration.
- [Installation](https://github.com/contextgeneric/cargo-cgp/blob/main/docs/reference/installation.md) — every install/update/uninstall path.
- [Error-code catalog](https://github.com/contextgeneric/cargo-cgp/blob/main/docs/error-code.md) — what each `[CGP-Exxx]` means.
- [Troubleshooting](https://github.com/contextgeneric/cargo-cgp/blob/main/docs/reference/troubleshooting.md) — when the tool itself will not run.
