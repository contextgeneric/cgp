# cgp-run

`cgp-run` provides the task runners of Context-Generic Programming (CGP): the components that run a
task named by a type-level code. Projects reach them through the `cgp` crate, which re-exports them
as `cgp::extra::run`, so most depend on [`cgp`](https://crates.io/crates/cgp) alone and never add
this crate to their manifest.

To learn CGP itself, the [CGP website](https://contextgeneric.dev/) starts from a first program, and
the [`cgp` crate's documentation](https://docs.rs/cgp) covers the whole API.
