# cgp-type

`cgp-type` provides the abstract types of Context-Generic Programming (CGP), which let each context
choose a type that generic code names only through a trait.

The `cgp` crate re-exports it as `cgp::core::types`. Most projects therefore depend on
[`cgp`](https://crates.io/crates/cgp) alone, without adding this crate to their manifest.

To learn more, read the [`cgp` crate's documentation](https://docs.rs/cgp) and the
[CGP website](https://contextgeneric.dev/), which covers everything from a first program to the
reference for each construct.
