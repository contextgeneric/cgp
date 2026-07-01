# Identifier case conversion

The case-conversion helpers turn identifiers between the naming conventions CGP's generated code uses — PascalCase for type and trait names, snake_case for values and method names. The macros derive one name from another all over the codebase (a provider trait name from a function name, a context value name from the context type name), and these functions are the shared way to do it.

`to_camel_case_str` produces a PascalCase string by splitting on underscores, dropping empty segments, and upper-casing the first letter of each word. `to_snake_case_str` produces a snake_case string by inserting an underscore before each interior uppercase letter and lower-casing the result.

The one non-obvious helper is `to_snake_case_ident`, which additionally wraps its result in the reserved `__…__` form: unless the identifier already starts with an underscore, `Context` becomes `__context__` rather than plain `context`. This is what produces the reserved value name that pairs with a reserved type name like `__Context__`, keeping generated bindings from clashing with a user's own identifiers — the same convention behind the reserved names described in [entrypoints/cgp_component.md](../../entrypoints/cgp_component.md).

## Tests

- These helpers have no dedicated test; they are covered through the expansion snapshots, where the derived names appear in the generated code.

## Source

- The functions live in [cgp-macro-core/src/functions/camel_case.rs](../../../../crates/macros/cgp-macro-core/src/functions/camel_case.rs) and [cgp-macro-core/src/functions/snake_case.rs](../../../../crates/macros/cgp-macro-core/src/functions/snake_case.rs).
