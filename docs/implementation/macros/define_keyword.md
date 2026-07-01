# `define_keyword!`

`define_keyword!` declares a custom-keyword marker type for the CGP parsers — a zero-sized struct paired with the string it matches. The CGP macros parse several bespoke keywords in their bodies (`new` in `#[cgp_impl]` and `delegate_components!`, `open` in the `open` dispatch statement, and the rest), and each such keyword is represented at the type level by a struct that carries the keyword's spelling so the parser can recognize it and error on a mismatch.

The macro expands to two items. `define_keyword!(Foo, "foo")` emits `pub struct Foo;` and an `impl crate::traits::IsKeyword for Foo` whose associated const `IDENT` is `"foo"`. The `IsKeyword` trait is the shared interface the parsing machinery uses: a parser peeks the next identifier, compares it against `<Keyword as IsKeyword>::IDENT`, and consumes it as that keyword when they match. Defining a keyword is therefore just declaring the marker and wiring its spelling into the trait; the actual peek-and-consume logic lives with the parser that uses the keyword.

## Behavior and corner cases

The keyword string and the struct name are independent, so the marker can be named for its role rather than its spelling, though in practice they match (`New`/`"new"`, `Open`/`"open"`). Because the generated struct is an ordinary public type, a keyword marker can also appear in generated code or as a type-level tag where that is useful, not only in the parser.

## Tests

- `define_keyword!` has no dedicated test; the keywords it defines are exercised through the parser tests and expansion snapshots of the macros that use them — for example the `new`-prefixed forms pinned in the `basic_delegation` snapshots and the `open` statement pinned in the `namespaces` and `dispatching` targets.

## Source

- The macro is defined in [cgp-macro-core/src/macros/keyword.rs](../../../crates/macros/cgp-macro-core/src/macros/keyword.rs); the `IsKeyword` trait it implements lives in `cgp-macro-core/src/traits/`, and the keyword marker types that use it live in `cgp-macro-core/src/types/keyword*.rs`.
- The convention that custom keywords go through this macro is recorded in [cgp-macro-core/CLAUDE.md](../../../crates/macros/cgp-macro-core/CLAUDE.md).
