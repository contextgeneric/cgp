# cgp-base-extra

`cgp-base-extra` gathers the base layer of Context-Generic Programming (CGP) together with its
abstract types, the foundation that CGP's error components are built on.

It is an internal layer of the `cgp` crate, which re-exports everything it holds under `cgp::core`.
Most projects therefore depend on [`cgp`](https://crates.io/crates/cgp) alone, without adding this
crate to their manifest.

To learn more, read the [`cgp` crate's documentation](https://docs.rs/cgp) and the
[CGP website](https://contextgeneric.dev/), which covers everything from a first program to the
reference for each construct.
