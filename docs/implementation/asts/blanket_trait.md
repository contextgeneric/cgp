# The `blanket_trait` AST stack

The `blanket_trait` stack is a single type. `#[blanket_trait]` has no multi-stage pipeline and no bespoke argument type — the entry function parses the optional context identifier and a `syn::ItemTrait` directly — so all the codegen lives in `ItemBlanketTrait`, whose `to_items` emits the trait unchanged plus one generated blanket impl. This document covers that type; the [entrypoint document](../entrypoints/blanket_trait.md) covers the shape of the items it produces.

## `ItemBlanketTrait`

`ItemBlanketTrait` holds the context identifier and the parsed trait, and does everything in `to_items`, which returns the cloned input trait followed by the impl built by `to_item_impl`. Because the trait is emitted verbatim, its default bodies and supertraits survive into the output untouched; the derivation is entirely on the impl.

`to_item_impl` walks the trait's items once to build the blanket impl. It first collects the associated-type identifiers and runs a `RemoveSelfPathVisitor` over the whole trait to rewrite `Self::<AssocType>` references to the bare parameter name, then processes each item by kind:

- a **type** contributes a `type <Name> = <Name>;` assignment (from the lifted parameter), a new impl generic parameter, and — if it declared bounds — a `<Name>: <bounds>` predicate collected before its default is stripped;
- a **method** contributes an impl method whose body is the trait's default block, erroring if the default is absent;
- a **const** contributes an impl constant whose expression is the trait's default, erroring if the default is absent;
- any other item is rejected as unsupported.

It then assembles the impl generics — the trait's own generics, plus the context parameter, plus one parameter per associated type — and builds the `where` clause: a single `#context: <supertraits>` predicate carrying the trait's supertraits as the hidden dependency, followed by the collected associated-type bounds. The impl targets the trait (with the trait's own type generics) for the context type. The [entrypoint document](../entrypoints/blanket_trait.md) shows the resulting shapes.

## Tests

- `ItemBlanketTrait` is exercised end-to-end by the expansion snapshots indexed in the [entrypoint document's Snapshots section](../entrypoints/blanket_trait.md); the missing-default-body error path has no dedicated `cgp-macro-tests` failure case yet.

## Source

- `ItemBlanketTrait` lives in [cgp-macro-core/src/types/blanket_trait.rs](../../../crates/macros/cgp-macro-core/src/types/blanket_trait.rs).
- The `Self::<AssocType>`-to-parameter rewriting is done by `RemoveSelfPathVisitor` in [cgp-macro-core/src/visitors/remove_self_path.rs](../../../crates/macros/cgp-macro-core/src/visitors/remove_self_path.rs).
