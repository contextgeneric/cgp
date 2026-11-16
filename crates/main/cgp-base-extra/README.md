# cgp-base-extra

`cgp-base-extra` gathers the base layer of Context-Generic Programming (CGP) together with its
abstract types, the foundation that CGP's error components are built on.

It is an internal layer of the `cgp` crate, which re-exports everything it holds under `cgp::core`.
Most projects therefore depend on [`cgp`](https://crates.io/crates/cgp) alone, without adding this
crate to their manifest.

The [CGP website](https://contextgeneric.dev/) teaches CGP from a first program to the reference for
each construct, and the [`cgp` crate's documentation](https://docs.rs/cgp) covers the API.