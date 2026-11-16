# cgp-base

`cgp-base` gathers the base layer of Context-Generic Programming (CGP): the type-level primitives,
the component machinery, and the procedural macros, which CGP's higher layers build on.

The `cgp` crate re-exports it as `cgp::core::base`. Most projects therefore depend on
[`cgp`](https://crates.io/crates/cgp) alone, without adding this crate to their manifest.

To learn more, read the [`cgp` crate's documentation](https://docs.rs/cgp) and the
[CGP website](https://contextgeneric.dev/), which covers everything from a first program to the
reference for each construct.
