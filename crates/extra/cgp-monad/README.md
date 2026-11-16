# cgp-monad

`cgp-monad` provides the monadic handlers of Context-Generic Programming (CGP). They compose
handlers through a monad that decides whether each step runs, such as one that stops a pipeline at
its first error. Projects reach them through the `cgp` crate, which re-exports them as
`cgp::extra::monad`, so most depend on [`cgp`](https://crates.io/crates/cgp) alone and never add
this crate to their manifest.

To learn CGP itself, the [CGP website](https://contextgeneric.dev/) starts from a first program, and
the [`cgp` crate's documentation](https://docs.rs/cgp) covers the whole API.
