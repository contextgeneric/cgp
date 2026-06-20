//! Corner cases for `IdentWithTypeArgs` — an identifier followed by an
//! optional *type-expression* argument list, e.g. `Foo<A, B>`.

use cgp_macro_core::types::ident::{IdentWithTypeArgs, TypeArg};
use quote::quote;
use syn::parse2;

use super::{assert_idempotent, assert_parses, assert_rejects};

type Subject = IdentWithTypeArgs;

#[test]
fn accepts_bare_ident() {
    assert_parses::<Subject>(quote!(Foo));
}

#[test]
fn accepts_empty_argument_list() {
    // An explicit empty `<>` is allowed and parses to an empty argument list,
    // indistinguishable from no brackets at all.
    assert_parses::<Subject>(quote!(Foo));
}

#[test]
fn accepts_type_arguments() {
    assert_parses::<Subject>(quote!(Foo<A>));
    assert_parses::<Subject>(quote!(Foo<A, B>));
    assert_parses::<Subject>(quote!(Foo<A, B, C>));
}

#[test]
fn accepts_composite_type_arguments() {
    // The key distinguishing feature versus the definition-site generics:
    // arguments may be arbitrary types, not just simple identifiers.
    assert_parses::<Subject>(quote!(Foo<(A, B), C>));
    assert_parses::<Subject>(quote!(Foo<Bar<A>, C>));
    assert_parses::<Subject>(quote!(Foo<Bar<Baz<A>>>));
    assert_parses::<Subject>(quote!(Foo<[A; 4]>));
    assert_parses::<Subject>(quote!(Foo<&'a A>));
    assert_parses::<Subject>(quote!(Foo<fn(A) -> B>));
    assert_parses::<Subject>(quote!(Foo<dyn Bar + Send>));
    assert_parses::<Subject>(quote!(Foo<path::to::Bar>));
    assert_parses::<Subject>(quote!(Foo<path::to::Bar<A>>));
}

#[test]
fn accepts_lifetime_arguments() {
    assert_parses::<Subject>(quote!(Foo<'a>));
    assert_parses::<Subject>(quote!(Foo<'a, A>));
    assert_parses::<Subject>(quote!(Foo<'a, 'b, A>));
}

#[test]
fn accepts_const_arguments() {
    // Const arguments are recognized when written as a literal or braced block.
    assert_parses::<Subject>(quote!(Foo<3>));
    assert_parses::<Subject>(quote!(Foo<{ N }>));
    assert_parses::<Subject>(quote!(Foo<A, 3>));
    assert_parses::<Subject>(quote!(Foo<true>));
}

#[test]
fn rejects_associated_type_binding() {
    // `syn::AngleBracketedGenericArguments` would accept these, but they are
    // not valid in a plain type-argument position.
    assert_rejects::<Subject>(quote!(Foo<A, B = C>));
    assert_rejects::<Subject>(quote!(Foo<Item = X>));
}

#[test]
fn rejects_associated_const_binding() {
    assert_rejects::<Subject>(quote!(Foo<N = 1>));
}

#[test]
fn rejects_associated_type_bound() {
    assert_rejects::<Subject>(quote!(Foo<A: Clone>));
    assert_rejects::<Subject>(quote!(Foo<Item: Iterator>));
}

#[test]
fn rejects_path_head() {
    // The head must be a single identifier; use `PathWithTypeArgs` for paths.
    assert_rejects::<Subject>(quote!(path::to::Foo<A>));
    assert_rejects::<Subject>(quote!(path::to::Foo));
}

#[test]
fn rejects_turbofish() {
    assert_rejects::<Subject>(quote!(Foo::<A>));
}

#[test]
fn rejects_unterminated_arguments() {
    assert_rejects::<Subject>(quote!(Foo < A));
    assert_rejects::<Subject>(quote!(Foo < A,));
}

#[test]
fn classifies_each_argument_form() {
    let parsed: Subject = parse2(quote!(Foo<'a, A, (A, B), Bar<C>, 3, { N }>)).unwrap();

    let args = &parsed.type_args.args;

    let kinds: Vec<&str> = args
        .iter()
        .map(|arg| match arg {
            TypeArg::Lifetime(_) => "lifetime",
            TypeArg::Type(_) => "type",
            TypeArg::Const(_) => "const",
        })
        .collect();

    assert_eq!(
        kinds,
        ["lifetime", "type", "type", "type", "const", "const"],
    );
}

#[test]
fn bare_ident_has_no_arguments() {
    let parsed: Subject = parse2(quote!(Foo)).unwrap();
    assert!(parsed.type_args.args.is_empty());
    assert!(parsed.type_args.is_empty());
}

#[test]
fn round_trips() {
    assert_idempotent::<Subject>(quote!(Foo));
    assert_idempotent::<Subject>(quote!(Foo<A, B>));
    assert_idempotent::<Subject>(quote!(Foo<(A, B), Bar<C>>));
    assert_idempotent::<Subject>(quote!(Foo<'a, A, 3>));
}
