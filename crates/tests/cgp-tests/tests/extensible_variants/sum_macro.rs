//! The `Sum!` type-level sum list: the variant-level analogue of `Product!`.
//!
//! `Sum![A, B, C]` expands to a right-nested chain of `Either`, terminated by
//! the uninhabited `Void` (`Either<A, Either<B, Either<C, Void>>>`). A value of
//! the sum holds exactly one of the listed types: `Left` selects the head,
//! `Right` defers to the rest of the chain, and `Void` — being unconstructible —
//! closes it off. This is the structure an enum's variants desugar to via
//! `#[derive(HasFields)]`, and the spine every extractor/cast in this concept
//! walks.
//!
//! See docs/reference/macros/sum.md and docs/reference/types/either.md.

use cgp::prelude::*;

// A standalone three-element sum, written with the `Sum!` sugar.
type Token = Sum![u32, String, bool];

// The same type spelled out in terms of the `Either`/`Void` spine. Asserting
// the two are the *same type* pins the expansion of `Sum!`.
type TokenExpanded = Either<u32, Either<String, Either<bool, Void>>>;

fn _assert_same_type(token: Token) -> TokenExpanded {
    // Compiles only if `Sum![u32, String, bool]` and the hand-written chain are
    // the identical type.
    token
}

// A value of a `Sum!` is built by nesting `Left`/`Right` to select a branch.
fn make_u32(value: u32) -> Token {
    Either::Left(value)
}

fn make_string(value: String) -> Token {
    Either::Right(Either::Left(value))
}

fn make_bool(value: bool) -> Token {
    Either::Right(Either::Right(Either::Left(value)))
}

// Matching a `Sum!` walks the chain; the final `Void` arm is a `match {}`,
// since `Void` can never be constructed and so needs no value.
fn describe(token: Token) -> String {
    match token {
        Either::Left(value) => format!("u32: {value}"),
        Either::Right(Either::Left(value)) => format!("string: {value}"),
        Either::Right(Either::Right(Either::Left(value))) => format!("bool: {value}"),
        Either::Right(Either::Right(Either::Right(void))) => match void {},
    }
}

#[test]
fn test_sum_variants() {
    assert_eq!(describe(make_u32(42)), "u32: 42");
    assert_eq!(describe(make_string("hello".to_owned())), "string: hello");
    assert_eq!(describe(make_bool(true)), "bool: true");
}

// The empty sum `Sum![]` is just `Void`, a type with no values.
type EmptySum = Sum![];

fn _empty_is_void(value: EmptySum) -> Void {
    value
}
