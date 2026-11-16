# cgp-runtime

`cgp-runtime` provides the runtime abstraction of Context-Generic Programming (CGP), which lets each
context choose its async runtime while generic code reaches it through a trait.

The `cgp` crate re-exports it as `cgp::extra::runtime`. Most projects therefore depend on
[`cgp`](https://crates.io/crates/cgp) alone, without adding this crate to their manifest.

To learn more, read the [`cgp` crate's documentation](https://docs.rs/cgp) and the
[CGP website](https://contextgeneric.dev/), which covers everything from a first program to the
reference for each construct.
