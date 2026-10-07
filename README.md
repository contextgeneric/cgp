# cgp-fork

[![Apache 2.0 Licensed](https://img.shields.io/badge/license-Apache_2.0-blue.svg)](https://github.com/Just-Replicant/cgp-fork/blob/main/LICENSE)
[![Tests](https://github.com/Just-Replicant/cgp-fork/actions/workflows/tests.yml/badge.svg)](https://github.com/Just-Replicant/cgp-fork/actions/workflows/tests.yml)
![Rust 1.89+](https://img.shields.io/badge/rustc-1.89+-blue.svg)

**Pluggable trait implementations, resolved at compile time.**

This repository is **cgp-fork**, a fork of Context-Generic Programming (CGP). CGP lets one interface have many implementations, and lets each context — an application, a test, a deployment — choose the one it uses. The choice is written in one place and compiles down to a direct call.

The crates in this tree are **v0.9.0**. The package name is `cgp-fork`. In Rust that crate is `cgp_fork`.

This fork includes upstream [contextgeneric/cgp](https://github.com/contextgeneric/cgp) through commit [`69d2854`](https://github.com/contextgeneric/cgp/commit/69d285410fc2f7b40eceaafeef8536642a9390b1) (7 October 2026, pull request [#278](https://github.com/contextgeneric/cgp/pull/278)).

**[Website](https://contextgeneric.dev/) · [Changelog](CHANGELOG.md) · [Repository](https://github.com/Just-Replicant/cgp-fork)**

## Install

```toml
cgp-fork = { git = "https://github.com/Just-Replicant/cgp-fork" }
```

```rust
use cgp_fork::prelude::*;
```

Rust **1.89+** on stable. Depend on this repository with the git line above.

For readable wiring errors, install [`cargo-cgp`](https://github.com/contextgeneric/cargo-cgp) and run `cargo cgp check`.

## Docs

Guides and examples: [contextgeneric.dev](https://contextgeneric.dev/).

Construct reference and macro internals: [cgp-knowledge-base-fork](https://github.com/Just-Replicant/cgp-knowledge-base-fork). Repository conventions: [AGENTS.md](AGENTS.md). Related repositories: [sibling-projects.md](sibling-projects.md).

## License

Licensed under the [Apache License, Version 2.0](LICENSE).
