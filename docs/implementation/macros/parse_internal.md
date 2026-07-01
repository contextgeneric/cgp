# `parse_internal!`

`parse_internal!` is the internal macro the CGP codegen uses to build a `syn` AST node from quasi-quoted tokens, attaching a descriptive error if the tokens do not parse. It is the workhorse of the whole macro implementation: nearly every generated fragment — a `where` predicate, a trait path, a type, an impl item — is produced by quoting tokens and parsing them into the target `syn` type through this macro, rather than by constructing the `syn` node field-by-field.

The macro is a thin wrapper over a function of the same name. `parse_internal!( #tokens … )` expands to `crate::functions::parse_internal(crate::vendor::quote!( #tokens … ))?` — it quotes its body with `quote!` (routed through the crate's vendored re-export so exported `macro_rules!` can reach it) and passes the resulting `TokenStream` to the `parse_internal` function. The trailing `?` is significant: the macro expands to a `?` expression, so it must be invoked inside a function that returns `syn::Result`, and a parse failure propagates as an early return rather than a panic.

The `parse_internal` function is where the descriptive error is attached. It calls `syn::parse2` for the inferred target type `T: Parse`, and on failure combines the original parser error with a second error spanning the input tokens and reading "failed to parse internal tokens to type `<T>`", followed by the offending tokens rendered with the `::cgp::macro_prelude::` prefix stripped for readability (via `strip_macro_prelude`). This turns an opaque "expected …" parser error into one that names both the target AST type and the concrete tokens that failed, which is what makes a codegen bug diagnosable.

## Behavior and corner cases

The target type is inferred from context, so the same invocation parses into whatever `syn` type the surrounding code expects. `let ty: Type = parse_internal!(#ident);` parses a type while `let p: Path = parse_internal!(#ident #generics);` parses a path from similar tokens; the `T: Parse` bound is resolved by the binding or argument position. When no type can be inferred, annotate the binding.

A subtle interaction with the `?` expansion is that `parse_internal!` cannot be used where a `?` is not valid — outside a `syn::Result`-returning function, or in a `const` context. The sibling plain function `parse_internal(tokens)?` is available for the cases where the token stream is already built (for example when a `quote!` block was assembled conditionally), and the blanket-impl builder uses it directly for exactly that reason.

## Tests

- `parse_internal!` has no dedicated test; it is exercised by every macro-expansion snapshot in the suite, since essentially all generated code passes through it. Its error path is observed indirectly whenever a codegen change produces unparseable tokens during development.

## Source

- The macro is defined in [cgp-macro-core/src/macros/parse.rs](../../../crates/macros/cgp-macro-core/src/macros/parse.rs) and the backing function in [cgp-macro-core/src/functions/parse_internal.rs](../../../crates/macros/cgp-macro-core/src/functions/parse_internal.rs).
- The `quote!` re-export it depends on is in `cgp-macro-core/src/vendor.rs`, and the prefix-stripping helper is `strip_macro_prelude` in `cgp-macro-core/src/functions/strip.rs`.
- The convention that all AST nodes are built through this macro is recorded in [cgp-macro-core/CLAUDE.md](../../../crates/macros/cgp-macro-core/CLAUDE.md).
