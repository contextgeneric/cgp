//! Corner cases for `IdentWithTypeGenerics` — an identifier followed by an
//! optional *definition-site* generic parameter list, e.g. `Foo<A, B>` or
//! `Bar<'a, C>`.

use cgp_macro_core::types::ident::{IdentWithTypeGenerics, TypeGenericParam};
use quote::quote;
use syn::parse2;

use super::{assert_idempotent, assert_parses, assert_rejects};

type Subject = IdentWithTypeGenerics;

#[test]
fn accepts_bare_ident() {
    assert_parses::<Subject>(quote!(Foo));
}

#[test]
fn accepts_empty_parameter_list() {
    assert_parses::<Subject>(quote!(Foo));
}

#[test]
fn accepts_simple_type_parameters() {
    assert_parses::<Subject>(quote!(Foo<A>));
    assert_parses::<Subject>(quote!(Foo<A, B>));
    assert_parses::<Subject>(quote!(Foo<A, B, C>));
}

#[test]
fn accepts_lifetime_parameters() {
    assert_parses::<Subject>(quote!(Bar<'a>));
    assert_parses::<Subject>(quote!(Bar<'a, C>));
    assert_parses::<Subject>(quote!(Bar<'a, 'b, C>));
}

#[test]
fn accepts_const_parameters() {
    assert_parses::<Subject>(quote!(Bar<const N: usize>));
    assert_parses::<Subject>(quote!(Bar<A, const N: usize>));
}

#[test]
fn rejects_trait_bounds() {
    assert_rejects::<Subject>(quote!(Foo<A: Clone>));
    assert_rejects::<Subject>(quote!(Foo<A, B: Clone>));
}

#[test]
fn rejects_lifetime_bounds() {
    assert_rejects::<Subject>(quote!(Foo<'a: 'b>));
}

#[test]
fn rejects_defaults() {
    assert_rejects::<Subject>(quote!(Foo<A = B>));
    assert_rejects::<Subject>(quote!(Bar<const N: usize = 0>));
}

#[test]
fn rejects_composite_parameters() {
    // Definition-site parameters must be simple; composite forms that are
    // valid as *arguments* are not valid as *parameters*.
    assert_rejects::<Subject>(quote!(Foo<(A, B)>));
    assert_rejects::<Subject>(quote!(Foo<Bar<A>>));
    assert_rejects::<Subject>(quote!(Foo<&'a A>));
}

#[test]
fn rejects_path_head() {
    assert_rejects::<Subject>(quote!(path::to::Foo<A>));
}

#[test]
fn classifies_each_parameter_form() {
    let parsed: Subject = parse2(quote!(Bar<'a, C, const N: usize>)).unwrap();

    let params = &parsed.type_generics.params;

    let kinds: Vec<&str> = params
        .iter()
        .map(|param| match param {
            TypeGenericParam::Lifetime(_) => "lifetime",
            TypeGenericParam::Type(_) => "type",
            TypeGenericParam::Const(_) => "const",
        })
        .collect();

    assert_eq!(kinds, ["lifetime", "type", "const"]);
}

#[test]
fn lowers_to_syn_generics() {
    let parsed: Subject = parse2(quote!(Bar<'a, C, const N: usize>)).unwrap();
    let generics = parsed.type_generics.to_generics();

    assert_eq!(generics.params.len(), 3);
    assert!(matches!(generics.params[0], syn::GenericParam::Lifetime(_)));
    assert!(matches!(generics.params[1], syn::GenericParam::Type(_)));
    assert!(matches!(generics.params[2], syn::GenericParam::Const(_)));
}

#[test]
fn empty_generics_lower_to_empty_syn_generics() {
    let parsed: Subject = parse2(quote!(Foo)).unwrap();
    let generics = parsed.type_generics.to_generics();
    assert!(generics.params.is_empty());
}

#[test]
fn round_trips() {
    assert_idempotent::<Subject>(quote!(Foo));
    assert_idempotent::<Subject>(quote!(Foo<A, B>));
    assert_idempotent::<Subject>(quote!(Bar<'a, C>));
    assert_idempotent::<Subject>(quote!(Bar<'a, C, const N: usize>));
}
