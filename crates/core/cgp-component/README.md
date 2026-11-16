# cgp-component

`cgp-component` provides the component machinery of Context-Generic Programming (CGP): the
delegation traits that let a context choose a provider for each component, the lookup that finds
that provider, and the providers CGP ships with.

The `cgp` crate re-exports it as `cgp::core::component`. Most projects therefore depend on
[`cgp`](https://crates.io/crates/cgp) alone, without adding this crate to their manifest.

To learn more, read the [`cgp` crate's documentation](https://docs.rs/cgp) and the
[CGP website](https://contextgeneric.dev/), which covers everything from a first program to the
reference for each construct.
