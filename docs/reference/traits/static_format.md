# `StaticFormat`, `StaticString`, `ConcatPath`

`StaticFormat`, `StaticString`, and `ConcatPath` turn CGP's type-level strings and paths back into runtime data — formatting a [`Chars`](../types/chars.md) chain character by character, decoding a [`Symbol!`](../macros/symbol.md) into a `&'static str` constant, and concatenating two type-level paths into one.

## Purpose

These traits close the loop between CGP's compile-time names and the runtime world. CGP encodes field and variant names as types — a [`Symbol!`](../macros/symbol.md) is a length plus a [`Chars`](../types/chars.md) list, one node per character — so that names can drive trait resolution. But a program eventually needs those names as ordinary strings: to print a field name in an error message, to build a key, or to render a navigation path. `StaticFormat` and `StaticString` are the two ways to recover the string, and `ConcatPath` is the corresponding type-level operation on paths.

`StaticFormat` recovers the string lazily, by writing into a formatter; it is what backs the `Display` impl on `Symbol` and `Chars`, so a type-level string can be printed with `to_string()` or `{}`. `StaticString` recovers it eagerly, as a compile-time `&'static str` constant computed by const evaluation, so the decoded string is available wherever a `const` is needed and costs nothing at runtime. `ConcatPath` works one level up: a [`PathCons`](../types/path_cons.md) path is a type-level list of path segments, and joining two paths — the common operation when composing nested accessors — is splicing one such list onto another.

## Definition

`StaticFormat` is a trait with a single associated function that writes the type-level string into a `Formatter`. The implementor is the type-level string itself, so the function takes no `self` — there is no runtime value, only the type:

```rust
pub trait StaticFormat {
    fn fmt(f: &mut Formatter<'_>) -> Result<(), fmt::Error>;
}
```

It is implemented by recursion over the [`Chars`](../types/chars.md) spine. Each `Chars<CHAR, Tail>` writes its own `CHAR` and then defers to `Tail`, and the terminating `Nil` writes nothing:

```rust
impl<const CHAR: char, Tail> StaticFormat for Chars<CHAR, Tail>
where
    Tail: StaticFormat,
{
    fn fmt(f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{CHAR}")?;
        Tail::fmt(f)
    }
}

impl StaticFormat for Nil { /* writes nothing */ }
```

`StaticString` exposes the decoded string as an associated constant rather than a formatting action:

```rust
pub trait StaticString {
    const VALUE: &'static str;
}
```

It is implemented for every type via a blanket impl over an internal `StaticBytes` trait. `Symbol<LEN, Chars>` computes a `[u8; LEN]` byte array at const-evaluation time by walking the `Chars` list and UTF-8-encoding each character into the array, then `StaticString::VALUE` validates those bytes as UTF-8 and exposes the result as a `&'static str`. The `LEN` const parameter of the `Symbol` is the precomputed byte length that sizes this array, which is why `Symbol!` records a byte length rather than a character count.

`ConcatPath<Other>` joins two type-level paths, exposing the joined path as `Output`. Both the input and output may be unsized, since path types are markers:

```rust
pub trait ConcatPath<Other: ?Sized> {
    type Output: ?Sized;
}
```

It recurses over the [`PathCons`](../types/path_cons.md) spine just as `ConcatProduct` does over a product: each `PathCons<Head, Tail>` keeps its `Head` and concatenates `Other` onto the `Tail`, and the terminating `Nil` becomes `Other` itself, so the result is the first path's segments followed by the second's.

## Behavior

`StaticFormat` and `StaticString` decode the same type-level string but differ in when and how. `StaticFormat` runs at the moment of formatting, recursing through the `Chars` nodes and emitting each character into the formatter; because both `Symbol` and `Chars` implement `Display` by delegating to it, any type-level string can be turned into an owned `String` or interpolated directly. `StaticString` instead produces the string once, in a `const` context, so `<Symbol!("name") as StaticString>::VALUE` is a plain `&'static str` with no per-call work — preferable when the name is needed as a constant or in a hot path. Both faithfully round-trip multi-byte Unicode: the byte length in `Symbol` accounts for UTF-8 width, and the empty symbol decodes to `""`.

`ConcatPath` is a pure type-level computation evaluated during trait resolution. It never touches values; it only names the combined path type, which a getter or accessor then uses to descend through nested fields.

## Examples

A type-level string can be recovered both ways. The `Display` route reconstructs it at runtime, and the `StaticString` route exposes it as a constant:

```rust
use cgp::prelude::*;
use cgp::core::field::traits::StaticString;

// via StaticFormat / Display
let s = <Symbol!("hello")>::default();
assert_eq!(s.to_string(), "hello");

// via StaticString — a compile-time constant, multi-byte safe
assert_eq!(<Symbol!("世界你好") as StaticString>::VALUE, "世界你好");
assert_eq!(<Symbol!("") as StaticString>::VALUE, "");
```

`ConcatPath` composes two paths into one at the type level, the operation behind chaining nested accessors:

```rust
type Outer = Path!(a.b);
type Inner = Path!(c.d);
type Joined = <Outer as ConcatPath<Inner>>::Output;
// the path a.b.c.d
```

## Related constructs

`StaticFormat` and `StaticString` decode the [`Chars`](../types/chars.md) chain and [`Symbol`](../macros/symbol.md) wrapper that the [`Symbol!`](../macros/symbol.md) macro builds from a string literal — the type-level string at the heart of CGP's field naming. `ConcatPath` operates on the [`PathCons`](../types/path_cons.md) spine constructed by the [`Path!`](../macros/path.md) macro, and is the path-level analogue of the product-level `ConcatProduct`. Together they let the names that drive [`HasField`](has_field.md) lookups and nested-getter composition surface as ordinary strings and paths.

## Source

- `StaticFormat` and its `Chars`/`Nil` impls are defined in [crates/core/cgp-base-types/src/traits/static_format.rs](../../../crates/core/cgp-base-types/src/traits/static_format.rs); the `Display` impls that delegate to it are on the [`Chars`](../types/chars.md) and [`Symbol`](../macros/symbol.md) types in [crates/core/cgp-base-types/src/types/](../../../crates/core/cgp-base-types/src/types/).
- `StaticString`, with its const-evaluated UTF-8 decoding, is in [crates/core/cgp-field/src/traits/static_string.rs](../../../crates/core/cgp-field/src/traits/static_string.rs).
- `ConcatPath` is in [crates/core/cgp-base-types/src/traits/concat_path.rs](../../../crates/core/cgp-base-types/src/traits/concat_path.rs), and `PathCons` in [crates/core/cgp-base-types/src/types/path.rs](../../../crates/core/cgp-base-types/src/types/path.rs).
