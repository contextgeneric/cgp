# cgp-macro-test-util-lib

`cgp-macro-test-util-lib` implements the snapshot-testing macros that `cgp-macro-test-util` exports.
The tests of Context-Generic Programming (CGP) use them to emit the code a CGP macro generates and
pin it as an inline snapshot, so a project that uses CGP has no reason to depend on this crate.

To learn CGP itself, the [CGP website](https://contextgeneric.dev/) starts from a first program, and
the [`cgp` crate's documentation](https://docs.rs/cgp) covers the whole API.
