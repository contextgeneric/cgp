#![cfg(test)]

use quote::quote;
use syn::parse2;

use crate::types::ident::{NewIdentWithTypeArgs, NewIdentWithTypeGenerics, PathWithTypeArgs};

fn args_ok(ts: proc_macro2::TokenStream) {
    parse2::<NewIdentWithTypeArgs>(ts.clone())
        .unwrap_or_else(|e| panic!("expected ok for `{ts}`: {e}"));
}
fn args_err(ts: proc_macro2::TokenStream) {
    assert!(
        parse2::<NewIdentWithTypeArgs>(ts.clone()).is_err(),
        "expected err for `{ts}`"
    );
}
fn gen_ok(ts: proc_macro2::TokenStream) {
    parse2::<NewIdentWithTypeGenerics>(ts.clone())
        .unwrap_or_else(|e| panic!("expected ok for `{ts}`: {e}"));
}
fn gen_err(ts: proc_macro2::TokenStream) {
    assert!(
        parse2::<NewIdentWithTypeGenerics>(ts.clone()).is_err(),
        "expected err for `{ts}`"
    );
}
fn path_ok(ts: proc_macro2::TokenStream) {
    parse2::<PathWithTypeArgs>(ts.clone())
        .unwrap_or_else(|e| panic!("expected ok for `{ts}`: {e}"));
}
fn path_err(ts: proc_macro2::TokenStream) {
    assert!(
        parse2::<PathWithTypeArgs>(ts.clone()).is_err(),
        "expected err for `{ts}`"
    );
}

// Compare re-emitted tokens by re-parsing both into `syn::Type`, so that
// purely cosmetic spacing differences (e.g. `> >` vs `>>`) are ignored.
fn roundtrip_args(ts: proc_macro2::TokenStream) {
    let parsed: NewIdentWithTypeArgs = parse2(ts.clone()).unwrap();
    let a: syn::Type = parse2(quote!(#parsed)).unwrap();
    let b: syn::Type = parse2(ts).unwrap();
    assert_eq!(a, b);
}
fn roundtrip_path(ts: proc_macro2::TokenStream) {
    let parsed: PathWithTypeArgs = parse2(ts.clone()).unwrap();
    let a: syn::Type = parse2(quote!(#parsed)).unwrap();
    let b: syn::Type = parse2(ts).unwrap();
    assert_eq!(a, b);
}

#[test]
fn type_args() {
    args_ok(quote!(Foo));
    args_ok(quote!(Foo<A, B>));
    args_ok(quote!(Foo<(A, B), C>));
    args_ok(quote!(Foo<Bar<A>, C>));
    args_ok(quote!(Foo<'a, A>));
    args_ok(quote!(Foo<3>));
    args_ok(quote!(Foo<{ N }>));

    args_err(quote!(Foo<A, B = C>));
    args_err(quote!(Foo<Item = X>));
    args_err(quote!(Foo<A: Clone>));
}

#[test]
fn type_generics() {
    gen_ok(quote!(Foo));
    gen_ok(quote!(Foo<A, B>));
    gen_ok(quote!(Bar<'a, C>));
    gen_ok(quote!(Bar<const N: usize>));

    gen_err(quote!(Foo<A: Clone>));
    gen_err(quote!(Foo<A = B>));
    gen_err(quote!(Foo<(A, B)>));
    gen_err(quote!(Foo<Bar<A>>));
    gen_err(quote!(Bar<const N: usize = 0>));
}

#[test]
fn paths() {
    path_ok(quote!(Foo));
    path_ok(quote!(Foo<A, B>));
    path_ok(quote!(path::to::Foo));
    path_ok(quote!(path::to::Foo<A, B>));
    path_ok(quote!(path::to::Bar<(A, B), B>));

    path_err(quote!(path::to<X>::Foo));
    path_err(quote!(path::to::Foo<A, B = C>));
    path_err(quote!(path::to::Foo::<A>));

    let parsed: PathWithTypeArgs = parse2(quote!(path::to::Foo<A, B>)).unwrap();
    assert_eq!(parsed.ident().to_string(), "Foo");
}

#[test]
fn roundtrips() {
    roundtrip_args(quote!(Foo<A, B>));
    roundtrip_args(quote!(Foo<(A, B), Bar<C>>));
    roundtrip_path(quote!(path::to::Foo<A, B>));
    roundtrip_path(quote!(::path::to::Foo<'a, A>));
}
