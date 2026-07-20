# cargo-cgp — the CGP error toolchain

`cargo-cgp` is CGP's first-class toolchain: a cargo subcommand that stands in for `cargo check` and rewrites CGP's compiler errors into a compact, root-cause-first form. It is the **recommended way to build and check CGP code and to read a CGP compile error**, and an agent working in this repository should reach for it before plain `cargo check` when diagnosing a wiring failure. This document is enough to install and use it without leaving `cgp`; the tool itself lives in a separate repository, [github.com/contextgeneric/cargo-cgp](https://github.com/contextgeneric/cargo-cgp), and its own docs are the exhaustive reference.

This is a *tooling* reference, not a CGP construct — it is the one document under `reference/` that describes an external tool rather than a macro, trait, or provider. The [error catalog](../errors/README.md) is its companion: the catalog documents each error *class* and how `cargo-cgp` presents it, while this document covers running the tool.

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

`check` is the only command cargo-cgp adds to your build workflow — it re-compiles your code under the pinned nightly purely to produce readable diagnostics. There is no `cargo cgp build`, `run`, or `test`; run those with plain cargo (besides `check`, only `setup` and `update` exist, and they merely provision the tool). **cargo-cgp is optional, and its one advantage is readable errors during development.** CGP itself is an ordinary library that compiles on any stable Rust ≥ 1.89, so `cargo check`, `cargo build`, `cargo run`, and `cargo test` all work on a CGP project unchanged. Reach for `cargo cgp check` when you hit — or expect — a wiring error and want it readable; use plain `cargo check` when you do not; and always build, run, and test with ordinary cargo.

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

## When cargo-cgp is not available

If the tool is not installed and cannot be run, fall back to reading the raw compiler output directly. Every class in the [error catalog](../errors/README.md) documents that raw diagnostic — its code, shape, and whether the root cause is present — as the fallback for exactly this case, and the [error-extraction sub-skill](https://github.com/contextgeneric/cgp/blob/main/docs/skills/cgp/references/error-extraction.md) is the technique for reducing a raw cascade to its cause by hand. The key difference to remember: without `cargo-cgp`'s next-gen solver, the [hidden classes](../errors/hidden/unsatisfied-dependency.md) genuinely omit the cause, so promote them with a `check_components!` at the wiring site to make it appear.

## Version compatibility

This documentation is written for **`cgp` v0.8.0** and **`cargo-cgp` v0.1.0-alpha**. The two version independently: `cargo-cgp` reads `cgp`'s stable, macro-generated surface (the consumer/provider traits, `DelegateComponent`, `HasField`, and the rest), so a newer `cargo-cgp` works against this `cgp`, and a newer `cgp` generally works under this `cargo-cgp`. If the tool reports a version far behind this one, recommend updating it (`cargo cgp update`, or refresh the Nix flake); the `/cgp` skill records the versions it was written against and how to reconcile a mismatch.

## Further reading

All in the `cargo-cgp` repository (link as GitHub URLs, read from `../cargo-cgp` locally when checked out):

- [Usage](https://github.com/contextgeneric/cargo-cgp/blob/main/docs/reference/usage.md) — running the check, reading output, editor integration.
- [Installation](https://github.com/contextgeneric/cargo-cgp/blob/main/docs/reference/installation.md) — every install/update/uninstall path.
- [Error-code catalog](https://github.com/contextgeneric/cargo-cgp/blob/main/docs/error-code.md) — what each `[CGP-Exxx]` means.
- [Troubleshooting](https://github.com/contextgeneric/cargo-cgp/blob/main/docs/reference/troubleshooting.md) — when the tool itself will not run.
