# cgp-macro-test-util-lib

`cgp-macro-test-util-lib` implements the snapshot-testing macros of Context-Generic Programming
(CGP) that `cgp-macro-test-util` exports. CGP's own tests use them to emit the code a CGP macro
generates and pin it as an inline snapshot.

Projects that use CGP have no reason to depend on this crate.

To learn more, read the [`cgp` crate's documentation](https://docs.rs/cgp) and the
[CGP website](https://contextgeneric.dev/), which covers everything from a first program to the
reference for each construct.
