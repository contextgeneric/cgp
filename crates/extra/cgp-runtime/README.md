# cgp-runtime

`cgp-runtime` provides the runtime abstraction of Context-Generic Programming (CGP), which lets each
context choose its async runtime while generic code reaches it through a trait. Projects reach it
through the `cgp` crate, which re-exports it as `cgp::extra::runtime`, so most depend on
[`cgp`](https://crates.io/crates/cgp) alone and never add this crate to their manifest.

To learn CGP itself, the [CGP website](https://contextgeneric.dev/) starts from a first program, and
the [`cgp` crate's documentation](https://docs.rs/cgp) covers the whole API.
