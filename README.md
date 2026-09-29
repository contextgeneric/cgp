# Context-Generic Programming (CGP)

[![Apache 2.0 Licensed](https://img.shields.io/badge/license-Apache_2.0-blue.svg)](https://github.com/contextgeneric/cgp/blob/main/LICENSE)
[![Crates.io](https://img.shields.io/crates/v/cgp.svg)](https://crates.io/crates/cgp)
[![Tests](https://github.com/contextgeneric/cgp/actions/workflows/tests.yml/badge.svg)](https://github.com/contextgeneric/cgp/actions/workflows/tests.yml)
![Rust 1.89+](https://img.shields.io/badge/rustc-1.89+-blue.svg)

**Pluggable trait implementations, resolved at compile time.**

CGP lets one interface have many implementations, and lets each context — an application, a test, a deployment — choose the one it uses. The choice is written in one place and compiles down to a direct call.

This repository is **v0.9.0**. crates.io currently publishes **v0.7.0**, a separate line. The docs here describe v0.9.0.

**[Website](https://contextgeneric.dev/) · [Changelog](CHANGELOG.md) · [crates.io](https://crates.io/crates/cgp)**

## Install

```toml
cgp = { git = "https://github.com/contextgeneric/cgp" }
```

```rust
use cgp::prelude::*;
```

Rust **1.89+** on stable. `cargo add cgp` installs the crates.io release, v0.7.0.

For readable wiring errors, install [`cargo-cgp`](https://github.com/contextgeneric/cargo-cgp) and run `cargo cgp check`.

## Docs

Guides and examples: [contextgeneric.dev](https://contextgeneric.dev/).

Construct reference and macro internals: [cgp-knowledge-base](https://github.com/contextgeneric/cgp-knowledge-base). Repository conventions: [AGENTS.md](AGENTS.md). Related repositories: [sibling-projects.md](sibling-projects.md).

## License

Licensed under the [Apache License, Version 2.0](LICENSE).
