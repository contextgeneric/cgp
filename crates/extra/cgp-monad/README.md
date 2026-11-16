# cgp-monad

`cgp-monad` provides the monadic handlers of Context-Generic Programming (CGP), which compose
handlers through a monad that decides whether each step runs, such as one that stops a pipeline at
its first error.

The `cgp` crate re-exports it as `cgp::extra::monad`. Most projects therefore depend on
[`cgp`](https://crates.io/crates/cgp) alone, without adding this crate to their manifest.

The [CGP website](https://contextgeneric.dev/) teaches CGP from a first program to the reference for
each construct, and the [`cgp` crate's documentation](https://docs.rs/cgp) covers the API.