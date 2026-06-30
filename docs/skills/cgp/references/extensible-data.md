# Extensible data

Treat a struct as a product of named fields and an enum as a sum of named variants, so that generic code can read, build, deconstruct, and convert any data type by its type-level field and variant names rather than by naming the concrete type.

A plain Rust `struct` or `enum` is opaque to generic code: nothing about "the `first_name` field" or "the `Circle` variant" can be named through a type parameter, so a struct literal must list every field at one site and a `match` must spell out every variant. Extensible data breaks that by deriving a type-level description of a type's shape — its fields or variants, each tagged by name — and the machinery to build, take apart, and convert values through that description. This brings row polymorphism and structural sum types to Rust, resolved entirely at compile time, and it is what lets independent components each contribute one field or handle one variant without knowing the whole type. The result feeds directly into CGP wiring: builders, variant dispatchers, and structural casts all consume the field-level machinery these derives generate. A context is just such a data type when it flows through providers this way.

The whole family is dual. A record is a product (every field present at once); an enum is a sum (exactly one variant present). The two share their presence markers, their casting traits, and their dispatch machinery, so reading one half tells you the shape of the other.

## The umbrella derive and its shape-specific faces

`#[derive(CgpData)]` is the high-level entry point: applied to a struct it emits the full record machinery, applied to an enum the full variant machinery, dispatching on the item kind. `#[derive(CgpRecord)]` and `#[derive(CgpVariant)]` are the same code paths restricted to one shape — use them when the type is always a struct or always an enum and you want the name to say so; applying the wrong one is a type error. The narrower building-block derives (`HasFields`, `HasField`, `BuildField`, `ExtractField`, `FromVariant`) each emit one slice of that output when you want only part of it.

```rust
#[derive(CgpData)]
pub struct Person { pub first_name: String, pub last_name: String }

#[derive(CgpData)]
pub enum Shape { Circle(Circle), Rectangle(Rectangle) }
```

Field tags drive everything generated. A named struct field or an enum variant is keyed by the type-level string `Symbol!("name")`, and an unnamed field of a tuple struct by its positional `Index<N>`; that tag is what addresses the field across every generated impl. One restriction shapes the variant side: a derivable enum must follow the sum-of-products form, where each variant holds exactly one unnamed payload — wrap a richer payload in a dedicated struct so the variant's value stays a single nameable type. A fieldless, multi-field, or struct-style variant is a compile error.

## Records: the whole-struct field view

`#[derive(HasFields)]` gives a type its whole-shape view, the foundation everything else builds on. For a struct it implements `HasFields` with a `Fields` associated type that is a [`Product!`](type-level-primitives.md) of `Field<Symbol!("name"), Type>` entries — the type-level spelling of the struct's layout, one entry per field tagged by name:

```rust
impl HasFields for Person {
    type Fields = Product![
        Field<Symbol!("first_name"), String>,
        Field<Symbol!("last_name"), String>,
    ];
}
```

The derive also emits the conversions that move values in and out of that representation: `ToFields` turns an owned value into its `Fields` product, `FromFields` rebuilds the value from one, and `ToFieldsRef` borrows it as a product of references. Generic algorithms bound `Context: HasFields` (or `ToFields`/`FromFields`) and fold over `Context::Fields` structurally, never naming the concrete struct. This whole-shape view is distinct from per-field indexed access: reading a single field by name is `HasField<Tag>`, the dependency-injection capability that getter traits resolve against (see [functions and getters](functions-and-getters.md)). A struct that wants both derives both.

## Records: building a struct field by field

The capability `#[derive(CgpRecord)]` and `#[derive(BuildField)]` add on top of reading is incremental, type-checked construction, exposed through the builder trait family. Construction runs through a *partial record* — a generated companion struct `__Partial{Name}` carrying one `MapType` marker per field that records whether that field is present yet. The marker `IsPresent` stores the field's real value, `IsNothing` stores `()`, so a partial record with every marker `IsNothing` is an empty builder and one with every marker `IsPresent` is a fully populated value. The family walks between these states.

```rust
let employee: Employee = Employee::builder()                    // every field IsNothing
    .build_from(person)                                         // first_name + last_name now IsPresent
    .build_field(PhantomData::<Symbol!("employee_id")>, 1)      // employee_id now IsPresent
    .finalize_build();                                          // exists only at the all-present configuration
```

`HasBuilder::builder()` produces the empty partial record; `BuildField::build_field` flips one marker from `IsNothing` to `IsPresent`; and `FinalizeBuild::finalize_build` turns the partial record back into the concrete struct. Underneath, both `BuildField` and its reverse `TakeField` (the `IsPresent → IsNothing` direction) are blanket impls over a single per-field primitive `UpdateField<Tag, M>`, which changes one field's marker and returns the old value alongside the rebuilt partial. The tracking is what makes the pattern safe: `finalize_build` is implemented *only* for the all-`IsPresent` configuration, so finalizing with any field still absent is a compile error rather than a runtime panic, and the order in which fields are built does not matter. The reverse direction is equally useful — `IntoBuilder` turns a complete struct into an all-present partial, and `TakeField` removes fields one at a time.

## Records: the extensible builder pattern

`CanBuildFrom` (one of the structural casts below) is what lets a builder absorb the shared fields of an entire source struct in one `build_from` step: it recurses over the source's field product, using `TakeField` to pull each field out and `BuildField` to write it into the target builder. So a small struct like a database client can be merged into a larger application struct without either type naming the other — they share only field names, matched at the type level. This is the basis of the *extensible builder pattern*, where the construction of one context is split across several independent providers, one per subsystem, none of which knows the final type or each other. Each provider builds a small output struct, and a dispatcher merges every output into the target's builder before finalizing.

```rust
delegate_components! {
    FullAppBuilder {
        HandlerComponent:
            BuildAndMergeOutputs<App, Product![
                BuildSqliteClient,
                BuildHttpClient,
                BuildOpenAiClient,
            ]>,
    }
}
```

Because the dispatcher is generic over the target struct and the provider list, swapping a subsystem means changing one entry, and selecting among several target structs is a matter of code-based dispatch. The `BuildAndMergeOutputs` combinator and the rest of this routing live in [handlers](handlers.md).

## Variants: constructing and deconstructing an enum

For an enum, `#[derive(HasFields)]` produces a `Fields` that is a [`Sum!`](type-level-primitives.md) of `Field<Symbol!("Variant"), Type>` entries built on the `Either`/`Void` spine, mirroring the product side. Construction is `FromVariant`, generated per variant by `#[derive(FromVariant)]`: it builds the enum from one variant chosen by a type-level tag, so generic code parameterized over a `Tag` can construct whichever variant it was asked for.

```rust
fn wrap_circle(circle: Circle) -> Shape {
    Shape::from_variant(PhantomData::<Symbol!("Circle")>, circle)   // == Shape::Circle(circle)
}
```

Deconstruction is the dual of the builder and runs through a *partial variant* — the companion enum `__Partial{Name}` generated by `#[derive(ExtractField)]`, carrying one `MapType` marker per variant just as a partial record carries one per field. The crucial difference is the absence marker: a record uses `IsNothing` (storing `()`), while a variant uses `IsVoid`, mapping a ruled-out variant to the uninhabited `Void` type. `HasExtractor::to_extractor` converts the enum into a partial variant with every variant still `IsPresent`, and `ExtractField::extract_field` tries to pull out one variant, returning `Ok(value)` if the value is that variant or `Err(remainder)` — the partial variant with that variant flipped to `IsVoid` — otherwise.

```rust
fn area(shape: Shape) -> f64 {
    match shape.to_extractor().extract_field(PhantomData::<Symbol!("Circle")>) {
        Ok(circle) => core::f64::consts::PI * circle.radius * circle.radius,
        Err(remainder) => {
            // remainder's type now has Circle ruled out (IsVoid); try the next variant
            let rect = remainder
                .extract_field(PhantomData::<Symbol!("Rectangle")>)
                .finalize_extract_result();   // remainder is now empty; this cannot fail
            rect.width * rect.height
        }
    }
}
```

Marking extracted variants as `IsVoid` is what gives compile-time exhaustiveness without a wildcard arm. Each failed `extract_field` rules out one more variant in the type, so after every variant has been tried the remainder has every marker `IsVoid` and is therefore uninhabited. `FinalizeExtract::finalize_extract` discharges such a remainder with an empty `match`, sound precisely because no value can reach it — and `FinalizeExtractResult::finalize_extract_result` is the convenience wrapper that collapses the final `Result` into its `Ok` value. Add a variant to the enum without handling it and the final remainder becomes inhabited again, so the code fails to compile until the new variant is covered, recovering the guarantee a concrete `match` gives. `HasExtractorRef` and `HasExtractorMut` provide the same machinery over borrows.

## Variants: the extensible visitor pattern

Routing a value to the handler for its current variant is the *extensible visitor pattern*, which solves the expression problem: new variants can be added without touching the handlers for the others, and the same handler set can serve several enums that share variants. The logic for each variant lives in its own provider, and a dispatcher derives one extract-and-handle step per variant from the enum's `Fields`, running them as a pipeline that short-circuits on the first matching variant and threads the remainder forward otherwise.

```rust
delegate_components! {
    Interpreter {
        ComputerComponent:
            UseInputDelegate<new EvalComponents {
                MathExpr: DispatchEval,         // the whole enum → variant dispatcher
                Plus<MathExpr>: EvalAdd,         // one provider per variant
                Times<MathExpr>: EvalMultiply,
                Literal<u64>: EvalLiteral,
            }>,
    }
}
```

The `MathExpr` entry routes the whole enum through a thin context-specific provider that defers to the matcher combinator; that wrapper exists to break the trait-resolution cycle between the matcher and the per-variant providers it dispatches to. The matcher combinators (`MatchWithValueHandlers` and its by-reference form) and the per-variant handler families live in [handlers](handlers.md).

## The type-level spines underneath

Both halves rest on the same right-nested type-level lists, kept brief here and covered fully in [type-level primitives](type-level-primitives.md). A struct's `Fields` is a `Product![A, B, C]`, which desugars to `Cons<A, Cons<B, Cons<C, Nil>>>` over the `Cons`/`Nil` record spine — a list terminated by the constructible empty `Nil`, because an empty record is a valid value. An enum's `Fields` is a `Sum![A, B]`, which desugars to `Either<A, Either<B, Void>>` over the `Either`/`Void` variant spine — a chain that branches at each step and terminates in the uninhabited `Void`, because an empty choice has no value to pick. The lowercase `product![..]` builds a value of the matching `Product!` type. Generic providers walk these spines one element at a time, which is exactly what no plain tuple or enum permits in generic code.

## Structural casts between records and variants

Two types that share a subset of named fields or variants convert into one another generically, with no hand-written `From`/`TryFrom`, through the casting traits — every conversion is just routing each named entry to the matching slot in the target. `CanUpcast` lifts a value of a narrow enum into a wider one whose variants are a superset; it always succeeds, since every source variant has a home in the target, and it walks the source's variants, extracting each and reconstructing it via `FromVariant`. `CanDowncast` goes the other way, narrowing a wide enum into a smaller one and succeeding only if the value's current variant exists in the target, otherwise handing back a remainder; `CanDowncastFields` is the same operation on a remainder, so downcasting against several candidates chains a `downcast` followed by `downcast_fields`. `CanBuildFrom` is the record counterpart already met above, assembling a target builder out of the fields of one or more sources.

```rust
let wide = FooBar::Foo(1).upcast(PhantomData::<FooBarBaz>);          // always succeeds
assert_eq!(wide, FooBarBaz::Foo(1));

FooBarBaz::Bar("hi".into()).downcast(PhantomData::<FooBar>).ok();    // Some(FooBar::Bar(..))
FooBarBaz::Baz(true).downcast(PhantomData::<FooBar>).ok();           // None — Baz has no home in FooBar
```

Upcasting is also how a provider constructs a value using only the subset of variants it cares about: it builds a small local enum and upcasts it into the full type — the variant-side analog of reading a field through a getter.

## Dispatching over extensible data

The payoff of exposing data shape at the type level is that *dispatch* becomes generic: a record builder routes each field to the provider that produces it, and a variant visitor routes each value to the provider for its current variant. Both are realized with the dispatch combinators — `BuildAndMergeOutputs` on the record side, `MatchWithValueHandlers` on the variant side — which derive one per-field or per-variant step from the type's `Fields` and sequence them. Those combinators and the handler families they build on are documented in [handlers](handlers.md); the wiring tables that name a provider per [component](components.md) are the ordinary `delegate_components!` entries shown above.

## Further reference

Online docs (current as of CGP v0.7.0): [concepts/extensible-records.md](https://github.com/contextgeneric/cgp/blob/main/docs/concepts/extensible-records.md), [concepts/extensible-variants.md](https://github.com/contextgeneric/cgp/blob/main/docs/concepts/extensible-variants.md), the derive references under [reference/derives/](https://github.com/contextgeneric/cgp/tree/main/docs/reference/derives) (`derive_cgp_data.md`, `derive_cgp_record.md`, `derive_cgp_variant.md`, `derive_has_fields.md`, `derive_build_field.md`, `derive_extract_field.md`, `derive_from_variant.md`), the trait references [traits/has_builder.md](https://github.com/contextgeneric/cgp/blob/main/docs/reference/traits/has_builder.md), [traits/extract_field.md](https://github.com/contextgeneric/cgp/blob/main/docs/reference/traits/extract_field.md), [traits/from_variant.md](https://github.com/contextgeneric/cgp/blob/main/docs/reference/traits/from_variant.md), [traits/has_fields.md](https://github.com/contextgeneric/cgp/blob/main/docs/reference/traits/has_fields.md), and [traits/cast.md](https://github.com/contextgeneric/cgp/blob/main/docs/reference/traits/cast.md), and the type macros [macros/product.md](https://github.com/contextgeneric/cgp/blob/main/docs/reference/macros/product.md) and [macros/sum.md](https://github.com/contextgeneric/cgp/blob/main/docs/reference/macros/sum.md).
