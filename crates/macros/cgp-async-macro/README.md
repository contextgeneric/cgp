# cgp-async-macro

`cgp-async-macro` provides the `#[async_trait]` macro of Context-Generic Programming (CGP), which
rewrites each `async fn` in a trait to return `impl Future`, without boxing the future.

The `cgp` crate re-exports the macro under `cgp::prelude`. Most projects therefore depend on
[`cgp`](https://crates.io/crates/cgp) alone, without adding this crate to their manifest.

To learn more, read the [`cgp` crate's documentation](https://docs.rs/cgp) and the
[CGP website](https://contextgeneric.dev/), which covers everything from a first program to the
reference for each construct.
