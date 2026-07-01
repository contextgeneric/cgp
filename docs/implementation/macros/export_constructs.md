# `export_construct!` / `export_constructs!`

`export_construct!` and `export_constructs!` declare the hygienic markers that let generated code refer to CGP library items by a short name that always expands to a fully-qualified path. Every CGP item the macros emit — `IsProviderFor`, `DelegateComponent`, `UseField`, `HasField`, and the rest — is represented in the codegen by a zero-sized marker struct whose `ToTokens` emits `::cgp::macro_prelude::<Name>`, so a user only needs `cgp` in scope for the expansion to resolve.

`export_construct!(Name)` declares `pub struct Name;` and an impl of `quote::ToTokens` that emits `::cgp::macro_prelude::Name`. The two-argument form `export_construct!(From => To)` emits a different target path than the marker's own name, for the case where the codegen name and the exported item name differ. `export_constructs!` is the plural convenience wrapper: it takes a comma-separated list of `Name` or `From => To` entries and expands each through `export_construct!`, which is how [`exports.rs`](../../../crates/macros/cgp-macro-core/src/exports.rs) declares the whole set in one block.

The point of the indirection is hygiene. Generated code interpolates the marker (`#is_provider_for`) rather than a literal `::cgp::...` path, so the emitted tokens carry the fully-qualified path without the macro author writing it out and without depending on what the user has imported. To emit a new CGP item from a macro, add it to the `export_constructs!` list in `exports.rs` and interpolate its marker; do not write `::cgp::...` path literals inline.

## Tests

- These macros have no dedicated test; they are exercised by every expansion snapshot in the suite, since the fully-qualified paths in generated code all originate from these markers.

## Source

- The macros are defined in [cgp-macro-core/src/macros/export.rs](../../../crates/macros/cgp-macro-core/src/macros/export.rs); the marker set they generate lives in [cgp-macro-core/src/exports.rs](../../../crates/macros/cgp-macro-core/src/exports.rs), and the `::cgp::macro_prelude` re-export surface is what makes the emitted paths resolve.
- The convention that all CGP items are referenced through these markers is recorded in [cgp-macro-core/CLAUDE.md](../../../crates/macros/cgp-macro-core/CLAUDE.md).
