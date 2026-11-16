# cgp-macro-test-util-lib

`cgp-macro-test-util-lib` implements the snapshot-testing macros of Context-Generic Programming
(CGP) that `cgp-macro-test-util` exports. CGP's own tests use them to emit the code a CGP macro
generates and pin it as an inline snapshot.

Projects that use CGP have no reason to depend on this crate.

The [CGP website](https://contextgeneric.dev/) teaches CGP from a first program to the reference for
each construct, and the [`cgp` crate's documentation](https://docs.rs/cgp) covers the API.