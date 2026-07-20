# Ill-formed generated type

A macro accepts a field- or argument-type *shorthand* it has no dedicated rule for, lowers it literally, and the generated bound names a type that is not well-formed — most often an unsized type — so the compiler rejects it with `E0277` (the `Sized` form).

## What triggers it

This class is distinct from every other in the catalog: it is neither a wiring mistake nor a lazy dependency, but a macro *lowering* that produces a type the compiler cannot accept. CGP's getter and implicit-argument macros recognize a small set of field-type shorthands — `Option<&T>` for an optional field, `&[T]` for a slice field, `&str` for a `String` field — and lower each to a `HasField` bound and an accessor. Each shorthand covers a single field shape. A *combination* they provide no rule for is not rejected at macro time; it is lowered literally by whichever single rule matches first, and the resulting bound names an ill-formed type that only the compiler catches.

The worked case is a getter typed `Option<&[T]>`, which combines the `Option<&T>` and `&[T]` shorthands:

```rust
#[cgp_auto_getter]
pub trait HasItems {
    fn items(&self) -> Option<&[u8]>;
}
```

The shared `parse_field_type` applies the `Option<&T>` rule, reading an `Option<T>` field whose `T` is the slice `[u8]`, so the generated `HasField` bound names the unsized `Option<[u8]>`. Because `[u8]` is a dynamically sized type, `Option<[u8]>` is ill-formed, and the compiler rejects it. This boundary is shared by [`#[cgp_auto_getter]`](../../reference/macros/cgp_auto_getter.md), [`#[cgp_getter]`](../../reference/macros/cgp_getter.md), and a [`#[cgp_fn]`](../../reference/macros/cgp_fn.md) [`#[implicit]`](../../guides/reading-context-fields.md) argument, since all three lower field types through `parse_field_type`. CGP is working as designed: the shorthands ease the common single-shape cases, and an unsupported combination is deferred to the compiler rather than given a bespoke rule — an [acceptable failure](../../implementation/AGENTS.md), not a defect.

## The raw diagnostic

This section describes what plain `cargo check` prints — the fallback when `cargo-cgp` is not on hand; [How cargo-cgp presents it](#how-cargo-cgp-presents-it) below covers the readable form. This is a **surfaced** class, and unusually the root cause is the *primary* error, stated in plain terms. The compiler reports **`E0277`** in its dedicated `Sized` form — "the size for values of type `[u8]` cannot be known at compilation time," with the label "doesn't have a size known at compile-time" and a `help:` note "the trait `Sized` is not implemented for `[u8]`." A following `note: required by an implicit Sized bound in Option` names *where* the size is required: the `T` in `Option<T>` carries an implicit `Sized` bound, and `[u8]` cannot meet it. The caret lands on the macro attribute (`#[cgp_auto_getter]`) because the offending type is synthesized in the expansion, and a closing note attributes the error to that attribute macro.

Recognizing the exact message form matters, because rustc special-cases the missing-`Sized` case rather than printing a generic trait-bound line. It does **not** say "the trait bound `[u8]: Sized` is not satisfied"; it says "the size for values of type `[u8]` cannot be known at compilation time" and names `Sized` only in the `help:` note. The underlying unmet trait is still `Sized` under error code `E0277`, but the headline is the size-specific phrasing. This is the standard `rustc` diagnostic for a [dynamically sized type](https://doc.rust-lang.org/reference/dynamically-sized-types.html) reaching a position that requires a statically known size — a generic type parameter like `Option<T>`'s `T`, which carries an implicit `Sized` bound unless written `T: ?Sized`.

A second error usually follows and is derived noise, not a separate cause. The getter body calls `.as_ref()` on the ill-formed value, so the compiler adds an **`E0599`** — "the method `as_ref` exists for reference `&Option<[u8]>`, but its trait bounds were not satisfied" — whose notes restate `[u8]: Sized` and `Option<[u8]>: AsRef<_>` as the unmet bounds. It is a consequence of the same unsized type, downstream of the `E0277`, and points at no new mistake.

## Where the root cause is

The root cause is **present and sits at the top of the output**, in the primary `E0277` and its `help:` note naming `Sized` for the concrete unsized type. This is among the clearest diagnostics in the catalog: the compiler states exactly which type has no known size and where the size is required, so there is no note chain to walk and nothing suppressed. The `E0599` on `as_ref` below it is a derived consequence to skip. What the raw message does not supply is the CGP-specific reading — that the unsized type came from an *unsupported shorthand combination* the macro lowered literally, and that the fix is to change the field type rather than the wiring — because the compiler sees only the synthesized type, not the shorthand the user wrote.

## How cargo-cgp presents it

`cargo-cgp` does not yet rewrite this class — it passes rustc's diagnostic through unchanged. For the fixture, the tool's `.cgp.stderr` is byte-for-byte its raw `.rust.stderr`: the primary `E0277` "the size for values of type `[u8]` cannot be known at compilation time", its `Sized` `help:` note, the "required by an implicit `Sized` bound in `Option`" note, and the trailing `E0599` on `as_ref` — no `[CGP-Exxx]` code stamped, no note suppressed. The pass-through is why the fixture sits in cargo-cgp's `usability/` tier: the cause is present and stated plainly, but two things bury it for a CGP reader. The caret points at the `#[cgp_auto_getter]` attribute rather than at any type the user wrote, because the offending `Option<[u8]>` is synthesized in the expansion; and the derived `E0599` on `as_ref` adds noise below the real error. The improvement left for the tool is translation, not recovery — suppress the derived `E0599`, and read the synthesized `Option<[u8]>` back to its cause, that the field type combined the `Option<&T>` and `&[T]` shorthands into a shape CGP has no rule for. The codes cargo-cgp stamps on the classes it does rewrite are defined in the [cargo-cgp error-code catalog](https://github.com/contextgeneric/cargo-cgp/blob/main/docs/error-code.md).

## Resolving it

Change the field or argument type to a shape a single shorthand supports, so the macro lowers it to a well-formed bound. Use an owned optional container instead of an optional slice — `Option<Vec<u8>>` (returned by reference, `Option<&Vec<u8>>`, or by value) or `Option<&'static [u8]>` where a borrow of static data fits — rather than `Option<&[u8]>`, whose lowering names the unsized `Option<[u8]>`. When neither shorthand fits the shape you need, write the getter or accessor by hand: the macros only save boilerplate over a plain `HasField` impl, so a hand-written trait impl can name whatever well-formed type the field actually has. The rule to carry away is that the shorthands compose only where their single-shape lowering yields a `Sized` type; a combination that would produce a dynamically sized type is out of scope by design.

## Backing fixtures

- [`usability/lowering/option_slice.rs`](https://github.com/contextgeneric/cargo-cgp/blob/main/tests/ui/usability/lowering/option_slice.rs) — a `#[cgp_auto_getter]` returning `Option<&[u8]>`, lowered to a `HasField` bound over the unsized `Option<[u8]>`; its `.rust.stderr` pins the primary `E0277` "the size for values of type `[u8]` cannot be known at compilation time" with the `Sized` `help:` note, the "required by an implicit `Sized` bound in `Option`" note, and the derived `E0599` on `as_ref`. Its `.cgp.stderr` is identical — the pass-through that places the fixture in the `usability/` tier, since cargo-cgp does not yet suppress the derived `E0599` or read the synthesized type back to the shorthand combination.

## Related

- [`#[cgp_auto_getter]`](../../reference/macros/cgp_auto_getter.md), [`#[cgp_getter]`](../../reference/macros/cgp_getter.md), [`#[cgp_fn]`](../../reference/macros/cgp_fn.md), and [reading context fields](../../guides/reading-context-fields.md) — the macros whose shared `parse_field_type` lowers the shorthands, and the field-type forms they support.
- [`cgp_auto_getter` implementation document](../../implementation/entrypoints/cgp_auto_getter.md) — the Behavior-and-corner-cases and Failure-modes account of why the unsupported combination is deferred to the compiler.
- [`HasField`](../../reference/traits/has_field.md) — the trait the generated bound targets, whose value type must be `Sized` here.
- [`E0277`](../error_codes/e0277.md) and [dynamically sized types](https://doc.rust-lang.org/reference/dynamically-sized-types.html) — the error-code reference (including the `Sized` special case) and the Rust reference for the size requirement the generated type violates.
