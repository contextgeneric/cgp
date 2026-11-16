# cgp-run

`cgp-run` provides the task runners of Context-Generic Programming (CGP), the components that run a
task named by a type-level code.

The `cgp` crate re-exports it as `cgp::extra::run`. Most projects therefore depend on
[`cgp`](https://crates.io/crates/cgp) alone, without adding this crate to their manifest.

To learn more, read the [`cgp` crate's documentation](https://docs.rs/cgp) and the
[CGP website](https://contextgeneric.dev/), which covers everything from a first program to the
reference for each construct.
