use alloc::string::ToString;

use quote::quote;

use crate::product::{make_product_expr, make_product_type, make_sum_type};

#[test]
fn test_product_type() {
    let derived = make_product_type(quote! {
        Foo,
        Bar<T>,
        Baz<T, U>,
    });

    let expected = quote! {
        π<
            Foo,
            π<
                Bar<T>,
                π<
                    Baz<T, U>,
                    ε> > >
    };

    assert_eq!(derived.to_string(), expected.to_string());
}

#[test]
fn test_product_ident() {
    let derived = make_product_expr(quote! {
        foo,
        bar,
        baz,
    });

    let expected = quote! {
        π(
            foo,
            π(
                bar,
                π(
                    baz,
                    ε ) ) )
    };

    assert_eq!(derived.to_string(), expected.to_string());
}

#[test]
fn test_product_expr() {
    let derived = make_product_expr(quote! {
        foo.0,
        Bar { bar },
        Baz::baz(),
    });

    let expected = quote! {
        π(
            foo.0,
            π(
                Bar { bar },
                π(
                    Baz::baz(),
                    ε ) ) )
    };

    assert_eq!(derived.to_string(), expected.to_string());
}

#[test]
fn test_sum_type() {
    let derived = make_sum_type(quote! {
        Foo,
        Bar<T>,
        Baz<T, U>,
    });

    let expected = quote! {
        σ<
            Foo,
            σ<
                Bar<T>,
                σ<
                    Baz<T, U>,
                    θ> > >
    };

    assert_eq!(derived.to_string(), expected.to_string());
}
