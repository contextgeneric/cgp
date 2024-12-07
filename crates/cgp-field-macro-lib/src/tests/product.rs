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
        Cons<
            Foo,
            Cons<
                Bar<T>,
                Cons<
                    Baz<T, U>,
                    Nil> > >
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
        Cons(
            foo,
            Cons(
                bar,
                Cons(
                    baz,
                    Nil ) ) )
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
        Cons(
            foo.0,
            Cons(
                Bar { bar },
                Cons(
                    Baz::baz(),
                    Nil ) ) )
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
        Either<
            Foo,
            Either<
                Bar<T>,
                Either<
                    Baz<T, U>,
                    Void> > >
    };

    assert_eq!(derived.to_string(), expected.to_string());
}
