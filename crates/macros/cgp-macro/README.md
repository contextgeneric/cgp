# cgp-macro

`cgp-macro` provides the procedural macros of Context-Generic Programming (CGP), from
`#[cgp_component]`, which defines a component, and `#[cgp_impl]`, which writes a provider, to
`delegate_components!` and `check_components!`, which wire and check a context.

The `cgp` crate re-exports these macros under `cgp::prelude`. Most projects therefore depend on
[`cgp`](https://crates.io/crates/cgp) alone, without adding this crate to their manifest.

To learn more, read the [`cgp` crate's documentation](https://docs.rs/cgp) and the
[CGP website](https://contextgeneric.dev/), which covers everything from a first program to the
reference for each construct.
