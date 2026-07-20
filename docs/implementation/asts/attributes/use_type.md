# `#[use_type]` — the AST stack

`#[use_type(HasErrorType.Error)]` imports an abstract associated type: it rewrites the bare alias (`Error`) everywhere in the host's signatures into its fully-qualified `<Self as HasErrorType>::Error` form, and adds the owning trait as a bound so those paths are well-formed. It is the richest of the attribute modifiers, with a small stack of AST types and a three-step transform driven off them; this page covers that stack and the shared collection mechanism lives in the [attribute-modifier overview](README.md). For the user-facing syntax and expansion, read the reference document [reference/attributes/use_type.md](../../../reference/attributes/use_type.md).

## The stack at a glance

An import parses into a `UseTypeAttribute` per spec, each carrying one or more `UseTypeIdent` entries, and the specs a host collects are held together in a `UseTypeAttributes`. Application then runs in three steps off `UseTypeAttributes`: **ground** each spec's context, **substitute** every bare alias in one traversal with the `SubstituteAbstractTypes` visitor, and **add the bounds** to the trait or impl. The type-equality (`= T`) pins are derived separately by `derive_use_type_predicates`. The sections below follow the data through those types in order.

## `UseTypeIdent`

`UseTypeIdent` is one imported associated type within a spec. It captures three pieces: the `type_ident` (the associated type's name, e.g. `Error`), an optional `as_alias` (the `as NewName` rename), and an optional `equals` (the `= ConcreteType` pin). Its `alias_ident()` returns the alias when one is written and the type identifier otherwise — this is the name that appears bare in the host's signatures and that the substitution matches against. Parsing reads the identifier, then optionally `as <ident>`, then optionally `= <type>`, so the grammar is `Ident (as Ident)? (= Type)?`.

## `UseTypeAttribute`

`UseTypeAttribute` is one import spec — a trait path, the types imported from it, and the context they are projected against. It holds a `context_type` (the type whose associated type is imported, defaulting to `Self` or set by a trailing `in Context`), the `trait_path` (a `PathWithTypeArgs`, so the owning trait may be a full path or carry generic arguments such as `HasFooType<X>`), and a `Vec<UseTypeIdent>` of the types imported from that trait.

Its parser is where the `.`-versus-`::` separation is decided. It parses the trait path, consumes a `.`, and reads either a single `UseTypeIdent` or a brace-delimited comma list of them (`HasTypes.{A, B as C}`); it then reads an optional `in Context` suffix, parsing `Context` as a `PathWithTypeArgs`, with no suffix defaulting the context to `Self`. The `.` — not `::` — is what separates the trait from the associated type, so a trait that is itself a path keeps its `::` inside `trait_path` while the `.` unambiguously marks where the associated type begins; the `in` keyword, reserved in Rust, marks the context clause just as cleanly since it can never appear inside a type or path:

```rust
// #[use_type(foo::bar::HasScalarType.Scalar in Context)]
//            └────trait_path────┘ └assoc┘    └─ctx─┘
```

`UseTypeAttribute` also carries `replace_ident`, the per-spec lookup the substitution visitor calls: given an identifier, it returns the fully-qualified replacement identifier if the identifier matches one of the spec's `alias_ident()`s, and — importantly — **stamps the user's original span onto the replacement**. Preserving the span is what makes a caret on a mistyped imported type point at the identifier the user wrote rather than at the whole macro block.

## `UseTypeAttributes` and the three-step transform

`UseTypeAttributes` holds the `Vec<UseTypeAttribute>` a host collected and owns the transform that applies them. It exposes two entry points — `transform_item_trait` for a generated trait and `transform_item_impl` for a generated impl — and both run the same three steps, differing only in the bounds they add at the end. Each first calls `forbid_duplicate_aliases` and returns early when there are no specs.

**Step one — grounding.** `grounded_specs` resolves each spec's context type up front so the later steps agree on one fully-qualified context. An `in Context` whose `Context` is itself imported by another spec — as in `HasTypes.Types, HasScalarType.Scalar in Types` — is rewritten from the bare alias `Types` into `<Self as HasTypes>::Types`. Contexts that name a real generic parameter or `Self` are left alone. The pass iterates to a fixpoint, running the substitution visitor over each spec's `context_type` and stopping when a pass makes no change; because each pass grounds one more level, `attributes.len()` passes cover any acyclic chain (so `HasA.A, HasB.B in A, HasC.C in B` grounds `B` to `<<Self as HasA>::A as HasB>::B`), and a cyclic reference simply stops making progress rather than looping — it surfaces later as an ordinary unresolved-type error.

**Step two — substitution.** A single `SubstituteAbstractTypes` traversal, holding *every* grounded spec at once, rewrites each bare use of an alias into its fully-qualified associated type:

```rust
// #[use_type(HasErrorType.Error)]  turns a bare `Error` into:
<Self as HasErrorType>::Error
```

Grounding the contexts up front is what lets one pass suffice: the replacement a spec emits already contains no bare alias, so the visitor never revisits its own output, and because aliases are unique (guaranteed by `forbid_duplicate_aliases`) the order among specs is irrelevant. The visitor's matching rules are covered under [SubstituteAbstractTypes](#substituteabstracttypes) below.

**Step three — adding the bounds.** This is where the two entry points diverge:

- `transform_item_trait` pushes each `Self`-context spec's trait path onto the consumer trait's *supertraits*, so the abstract type is available to every signature. For a foreign `in Context` spec it instead adds a plain `Context: Trait` predicate to the trait's `where` clause, so the substituted `<Context as Trait>::Assoc` signatures are well-formed without the author declaring the bound. The type-equality (`= T`) form is an impl-side pin and is deliberately *not* added to a trait here.
- `transform_item_impl` derives the impl-side `where` predicates through `derive_use_type_predicates` and extends the impl's `where` clause with them. These carry the `= T` equality pins as associated-type bindings; the pins are impl-side only and never reach the trait.

## `derive_use_type_predicates` and the equality pins

`derive_use_type_predicates` (in `type_predicates.rs`) turns a set of grounded specs into the impl-side `where` predicates they contribute — one `context_type: trait_path` bound per spec, carrying any `= T` pins as associated-type bindings inside the trait path (`Context: Trait<Assoc = T>`). It reads each spec's already-grounded `context_type` directly rather than re-resolving aliases, which is why grounding must run first.

The pins can also **unify two imported abstract types**. `find_type_equality` handles the case where an equality's right-hand side names *another* imported alias: given `HasPasswordType.Password, HasHashedPasswordType.{HashedPassword = Password}`, it recognizes that `Password` is itself an imported alias and rewrites the pin's target into the other spec's fully-qualified projection (`<Self as HasPasswordType>::Password`), so the two abstract types are constrained equal. When the right-hand side is an ordinary concrete type it is used as written.

Two more functions in this module round out the impl-side logic. `forbid_duplicate_aliases` rejects any two imports that resolve to the same identifier or alias: it flattens every `UseTypeIdent` across all specs into one list and compares every pair by `alias_ident()`, so the check catches a collision across separate specs *and* within a single braced list, uniformly for components, impls, and functions. A shared alias would make the substitution silently pick the first match and drop the rest, so it must be an error.

## `SubstituteAbstractTypes`

`SubstituteAbstractTypes` (in [visitors/substitute_abstract_type.rs](../../../../crates/macros/cgp-macro-core/src/visitors/substitute_abstract_type.rs)) is the `VisitMut` that performs the bare-alias rewrite in a single traversal. Holding every spec at once — rather than running one visitor per spec — is what lets one pass over the item handle all imports regardless of the order they were written. It rewrites a type only when the type is a bare, single-segment, argument-free path (`qself: None`, no leading colon, one segment, `PathArguments::None`) whose identifier matches a spec's `alias_ident()`; it then replaces the type with `<context_type as trait_path>::replacement_ident` and records `is_changed`. The strict match guard is deliberate: a path that already carries a qualifier, arguments, or more than one segment is not a bare alias and is left untouched, so a genuine `Self::Error` or a generic `Foo<Error>` is not disturbed. `is_changed` is what the grounding fixpoint reads to decide whether a further pass would be a no-op.

## Behavior and corner cases

**A `=` equality is rejected outright on `#[cgp_component]`.** `CgpComponentAttributes::parse` scans each imported `UseTypeIdent` for an `equals` and returns a spanned "Type equality constraints cannot be used in component trait definition" error, because a component *definition* cannot pin an abstract type to a concrete one — the pin belongs on an impl. The impl and function collectors accept the equality form.

**A foreign `in Context` bound reaches the trait.** On a component or function trait, `transform_item_trait` adds the `Context: Trait` predicate to the trait's `where` clause for a foreign spec. Without this the constraint would be silently dropped, leaving a signature that only compiles when the author happens to supply the bound elsewhere; adding it is what makes the substituted `<Context as Trait>::Assoc` paths well-formed by construction.

**Nested-import order does not matter, but a cycle has no order.** Grounding runs a fixpoint over *all* specs at once, so a spec's `in Context` may name an alias imported by any other spec regardless of where it sits in the list — `HasC.C in B, HasB.B in A, HasA.A` grounds exactly like the front-to-back `HasA.A, HasB.B in A, HasC.C in B`. The one arrangement with no valid order is a cycle, where two contexts resolve through each other (`HasA.A in B, HasB.B in A`). The fixpoint stops after `attributes.len()` passes rather than looping, so the cyclic aliases are never grounded and stay bare in the emitted types; the compiler then reports `E0425` "cannot find type" at the offending `in` alias. CGP could in principle detect the cycle locally and reject it at macro time, but currently lowers it faithfully and defers to the compiler.

## Tests

The behavioral tests span every host and every form the attribute accepts:

- [abstract_types/use_type_component.rs](../../../../crates/tests/cgp-tests/tests/abstract_types/use_type_component.rs) covers the `#[cgp_component]` supertrait form; [use_type_foreign.rs](../../../../crates/tests/cgp-tests/tests/abstract_types/use_type_foreign.rs) the `in Context` foreign form on a component (with an *unbounded* generic parameter, guarding that the foreign bound reaches the trait); and [use_type_auto_getter.rs](../../../../crates/tests/cgp-tests/tests/abstract_types/use_type_auto_getter.rs) that a getter macro (`#[cgp_auto_getter]`) accepts `#[use_type]` through the same collector.
- [abstract_types/use_type_fn_alias.rs](../../../../crates/tests/cgp-tests/tests/abstract_types/use_type_fn_alias.rs), [use_type_fn_equality.rs](../../../../crates/tests/cgp-tests/tests/abstract_types/use_type_fn_equality.rs), and [use_type_fn_foreign.rs](../../../../crates/tests/cgp-tests/tests/abstract_types/use_type_fn_foreign.rs) cover the alias, equality, and foreign-context (`in`) forms on `#[cgp_fn]` — the last also using an unbounded generic parameter.
- [use_type_fn_equality_cross_trait.rs](../../../../crates/tests/cgp-tests/tests/abstract_types/use_type_fn_equality_cross_trait.rs), [use_type_fn_foreign_equality.rs](../../../../crates/tests/cgp-tests/tests/abstract_types/use_type_fn_foreign_equality.rs), [use_type_fn_foreign_equality_cross_trait.rs](../../../../crates/tests/cgp-tests/tests/abstract_types/use_type_fn_foreign_equality_cross_trait.rs), and [use_type_fn_nested_foreign.rs](../../../../crates/tests/cgp-tests/tests/abstract_types/use_type_fn_nested_foreign.rs) cover cross-spec and nested-foreign equality (the last combining a nested-foreign import with `#[extend_where]`); [use_type_fn_deep_foreign.rs](../../../../crates/tests/cgp-tests/tests/abstract_types/use_type_fn_deep_foreign.rs) pins a three-hop foreign chain front-to-back, and [use_type_fn_reverse_order.rs](../../../../crates/tests/cgp-tests/tests/abstract_types/use_type_fn_reverse_order.rs) writes the same chain back-to-front and asserts a value flows through it at runtime — together they exercise the transitive context grounding and pin its order-independence.
- [use_type_generic_param.rs](../../../../crates/tests/cgp-tests/tests/abstract_types/use_type_generic_param.rs) covers an alias that collides with a generic parameter, [use_type_path_qualified.rs](../../../../crates/tests/cgp-tests/tests/abstract_types/use_type_path_qualified.rs) the path-qualified trait form the `.` separator enables, and [implicit_arguments/cgp_fn_multi_and_use_type.rs](../../../../crates/tests/cgp-tests/tests/implicit_arguments/cgp_fn_multi_and_use_type.rs) the generic-argument trait form (`HasFooType<X>.Foo`).

The rejection cases are pinned in [parser_rejections/use_type.rs](../../../../crates/tests/cgp-macro-tests/tests/parser_rejections/use_type.rs): a `=` equality on a component, and a duplicate identifier or alias across specs, within one braced list, and on a component.

Four `cargo-cgp` UI fixtures pin the post-codegen failures the attribute defers to the compiler (`cargo-cgp` files each by the quality of the output it renders, so the cyclic-context case sits under `usability/`):

- [`acceptable/use-type/use_type_foreign_unsatisfied.rs`](https://github.com/contextgeneric/cargo-cgp/blob/main/tests/ui/acceptable/use-type/use_type_foreign_unsatisfied.rs) (a foreign `in Types` bound *enforced* on the trait rather than dropped) and [`acceptable/use-type/use_type_nested_unsatisfied.rs`](https://github.com/contextgeneric/cargo-cgp/blob/main/tests/ui/acceptable/use-type/use_type_nested_unsatisfied.rs) (the same at a nested two-hop depth) are both the [check-trait-failure](../../../errors/checks/check-trait-failure.md) class.
- [`acceptable/lowering/use_type_unknown_assoc.rs`](https://github.com/contextgeneric/cargo-cgp/blob/main/tests/ui/acceptable/lowering/use_type_unknown_assoc.rs) (a misnamed imported associated type lowered into an unresolvable path, whose caret confirms the substitution preserves the user's identifier span) is the [unresolved-imported-type](../../../errors/lowering/unresolved-imported-type.md) class.
- [`usability/lowering/use_type_cyclic_context.rs`](https://github.com/contextgeneric/cargo-cgp/blob/main/tests/ui/usability/lowering/use_type_cyclic_context.rs) (two `in Context` clauses that reference each other, so grounding never resolves either context and leaves the bare aliases in type position) surfaces as `E0425` "cannot find type" with the caret on the unresolved `in` alias — the sibling of the unresolved-imported-type class, but on the *context* rather than the associated-type name.

## Source

- The `use_type/` submodule in [cgp-macro-core/src/types/attributes/use_type/](../../../../crates/macros/cgp-macro-core/src/types/attributes/use_type/): `attribute.rs` (`UseTypeAttribute` and its parser), `ident.rs` (`UseTypeIdent`), `attributes.rs` (`UseTypeAttributes`, `grounded_specs`, and the two `transform_item_*` entry points), and `type_predicates.rs` (`derive_use_type_predicates`, `forbid_duplicate_aliases`, and the equality-unification helpers).
- The substitution visitor is `SubstituteAbstractTypes` in [cgp-macro-core/src/visitors/substitute_abstract_type.rs](../../../../crates/macros/cgp-macro-core/src/visitors/substitute_abstract_type.rs).
- The hosts that drive it: [entrypoints/cgp_component.md](../../entrypoints/cgp_component.md), [entrypoints/cgp_impl.md](../../entrypoints/cgp_impl.md), and [entrypoints/cgp_fn.md](../../entrypoints/cgp_fn.md).
