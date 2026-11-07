# AGENTS.md — maintaining the CGP test suite

The root [cgp/AGENTS.md](../../AGENTS.md) rules apply. Invoke the `/cgp` skill before adding, moving,
or refactoring a test. Read [README.md](README.md) for the target layout and the commands.

## Rules

- Group tests by the CGP concept under test, never by the macro that happens to appear.
- Split a category into finer ones when it grows too large to stay coherent.
- Put each unit test in its own file under the concept directory, self-contained at module scope.
- Open every test file with a brief comment on the behavior it exercises, and link the owning
  implementation document.
- Snapshot a macro only in the concept target that owns its feature; keep the snapshot inline.
- Invoke a macro plainly when it is incidental scaffolding; the owning target already pins its
  expansion.
- Put an input the macro rejects in `cgp-macro-tests` with `assert_macro_rejects`.
- Put an accepted input whose expansion fails to compile in a `cargo-cgp` UI fixture, filed under its
  error class, and catalog the class in the knowledge base's error catalog.
- Put a pin on the exact invalid tokens in the `invalid_expansion` target of `cgp-macro-tests`.
- Update the implementation document's Tests or Snapshots section in the same change as a test move.
- Run the suite with the commands in [README.md](README.md), and review every `insta` diff before
  accepting it.
- Check the post-codegen behavior in `cargo-cgp`'s UI suite when a construct change alters an accepted
  input or an expansion.
