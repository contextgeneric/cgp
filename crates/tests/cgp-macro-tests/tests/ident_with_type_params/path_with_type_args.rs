//! Corner cases for `PathWithTypeArgs` — a full Rust path followed by an
//! optional type-expression argument list, e.g. `path::to::Foo<A, B>`.

use cgp_macro_core::types::ident::{PathWithTypeArgs, TypeArg};
use quote::quote;
use syn::parse2;

use super::{assert_idempotent, assert_parses, assert_rejects};

type Subject = PathWithTypeArgs;

#[test]
fn accepts_single_segment() {
    assert_parses::<Subject>(quote!(Foo));
    assert_parses::<Subject>(quote!(Foo<A, B>));
}

#[test]
fn accepts_multi_segment_paths() {
    assert_parses::<Subject>(quote!(path::to::Foo));
    assert_parses::<Subject>(quote!(path::to::Foo<A, B>));
    assert_parses::<Subject>(quote!(path::to::Bar<(A, B), B>));
    assert_parses::<Subject>(quote!(crate::module::Foo<A>));
    assert_parses::<Subject>(quote!(self::Foo<A>));
}

#[test]
fn accepts_leading_colon() {
    assert_parses::<Subject>(quote!(::path::to::Foo));
    assert_parses::<Subject>(quote!(::path::to::Foo<'a, A>));
}

#[test]
fn accepts_same_argument_forms_as_ident_args() {
    assert_parses::<Subject>(quote!(path::to::Foo<'a, A, (A, B), Bar<C>, 3>));
}

#[test]
fn rejects_intermediate_segment_generics() {
    // Generic arguments are only meaningful on the final segment.
    assert_rejects::<Subject>(quote!(path::to<X>::Foo));
    assert_rejects::<Subject>(quote!(path<X>::to::Foo<A>));
}

#[test]
fn rejects_turbofish() {
    assert_rejects::<Subject>(quote!(path::to::Foo::<A>));
    assert_rejects::<Subject>(quote!(Foo::<A>));
}

#[test]
fn rejects_associated_bindings_and_bounds() {
    assert_rejects::<Subject>(quote!(path::to::Foo<A, B = C>));
    assert_rejects::<Subject>(quote!(path::to::Foo<Item = X>));
    assert_rejects::<Subject>(quote!(path::to::Foo<A: Clone>));
}

#[test]
fn rejects_parenthesized_arguments() {
    // `Fn(A) -> B` style parenthesized arguments are not allowed.
    assert_rejects::<Subject>(quote!(path::to::Fn(A) -> B));
}

#[test]
fn exposes_final_segment_ident() {
    let parsed: Subject = parse2(quote!(path::to::Foo<A, B>)).unwrap();
    assert_eq!(parsed.ident().to_string(), "Foo");

    let single: Subject = parse2(quote!(Foo)).unwrap();
    assert_eq!(single.ident().to_string(), "Foo");
}

#[test]
fn strips_arguments_from_stored_path() {
    let parsed: Subject = parse2(quote!(path::to::Foo<A, B>)).unwrap();

    // The arguments are lifted out into `type_args`, leaving the path itself
    // free of the final-segment arguments.
    let last = parsed.path.segments.last().unwrap();
    assert!(last.arguments.is_none());

    let args = &parsed.type_args.args;
    assert_eq!(args.len(), 2);
    assert!(matches!(args[0], TypeArg::Type(_)));
}

#[test]
fn single_segment_path_has_no_args_for_bare_ident() {
    let parsed: Subject = parse2(quote!(path::to::Foo)).unwrap();
    assert!(parsed.type_args.args.is_empty());
    assert_eq!(parsed.path.segments.len(), 3);
}

#[test]
fn round_trips() {
    assert_idempotent::<Subject>(quote!(Foo));
    assert_idempotent::<Subject>(quote!(path::to::Foo));
    assert_idempotent::<Subject>(quote!(path::to::Foo<A, B>));
    assert_idempotent::<Subject>(quote!(path::to::Bar<(A, B), Baz<C>>));
    assert_idempotent::<Subject>(quote!(::path::to::Foo<'a, A>));
}
