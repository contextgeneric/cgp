
# `cgp` - Context-Generic Programming Libraries in Rust

[![Apache 2.0 Licensed](https://img.shields.io/badge/license-Apache_2.0-blue.svg)](https://github.com/informalsystems/cgp/blob/master/LICENSE)
![Rust Stable](https://img.shields.io/badge/rustc-stable-blue.svg)
![Rust 1.79+](https://img.shields.io/badge/rustc-1.79+-blue.svg)

## Overview

The `cgp` project contains a collection of micro Rust crates that empowers 
_context-generic programming_ (CGP), a new programming paradigm in Rust.
To learn more about context-generic programming, check out the book
[Context-Generic Programming Patterns](https://patterns.contextgeneric.dev/).

## Crates Organization

The CGP core constructs are organized as many child crates that are intended
to be minimal and stable. Having each construct defined in separate crate 
helps us avoid introducing breaking changes in semantic  versioning, in case 
an unrelated construct is updated.

We also offers meta-crates that aggregate the dependencies from many CGP
child crates into one place, so that users can use CGP by specifying only one 
dependency:

- [`cgp`](./crates/cgp/) - The main crate that includes all child crates defined in this project.
- [`cgp-core`](./crates/cgp-core) - Meta crate that includes core CGP crates that
- [`cgp-extra`](./crates/cgp-extra) - Meta crate that includes additional CGP crates that may be non-essential or unstable.

The following library crates are provided:

- [`cgp-component`](./crates/cgp-component) - Defines the core CGP component traits and macros.
- [`cgp-async`](./crates/cgp-async/) - Defines the `Async` trait as an alias for async-safe types.
- [`cgp-error`](./crates/cgp-error/) - Defines the `HasErrorType` component and error handling constructs.
- [`cgp-field`](./crates/cgp-field/) - Defines the `HasField` trait to enable the use of data-generic programming with CGP.
- [`cgp-inner`](./crates/cgp-inner/) - Defines the `HasInner` component which enables composition-based implementation in CGP.
- [`cgp-run`](./crates/cgp-run) - Defines the `CanRun` component for implementing async runners.