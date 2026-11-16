# cgp-base-types

`cgp-base-types` provides the type-level primitives of Context-Generic Programming (CGP): the
symbols that name fields, the lists and sums that describe a type's fields and variants, and the
paths that namespaces route along.

The `cgp` crate re-exports these types under `cgp::core::base`, and the macros that build them, such
as `Symbol!` and `Product!`, under `cgp::prelude`. Most projects therefore depend on
[`cgp`](https://crates.io/crates/cgp) alone, without adding this crate to their manifest.

To learn more, read the [`cgp` crate's documentation](https://docs.rs/cgp) and the
[CGP website](https://contextgeneric.dev/), which covers everything from a first program to the
reference for each construct.
