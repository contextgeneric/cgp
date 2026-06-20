//! Side-by-side comparison of the old and new constructs, documenting exactly
//! where their parsing behavior diverges.

use cgp_macro_core::types::ident::{
    IdentWithTypeArgs, IdentWithTypeGenerics, NewIdentWithTypeArgs, NewIdentWithTypeGenerics,
};
use quote::quote;

use super::{assert_parses, assert_rejects};

/// Type-argument position: forms that the *old* `IdentWithTypeArgs` accepts but
/// the *new* `NewIdentWithTypeArgs` correctly rejects.
#[test]
fn args_new_is_stricter_about_associated_bindings_and_bounds() {
    for tokens in [
        quote!(Foo<A, B = C>),
        quote!(Foo<Item = X>),
        quote!(Foo<N = 1>),
        quote!(Foo<A: Clone>),
    ] {
        assert_parses::<IdentWithTypeArgs>(tokens.clone());
        assert_rejects::<NewIdentWithTypeArgs>(tokens);
    }
}

/// Type-argument position: forms that both constructs accept identically.
#[test]
fn args_agree_on_valid_forms() {
    for tokens in [
        quote!(Foo),
        quote!(Foo<A, B>),
        quote!(Foo<(A, B), C>),
        quote!(Foo<Bar<A>, C>),
        quote!(Foo<'a, A>),
        quote!(Foo<3>),
    ] {
        assert_parses::<IdentWithTypeArgs>(tokens.clone());
        assert_parses::<NewIdentWithTypeArgs>(tokens);
    }
}

/// Type-argument position: forms that both constructs reject identically.
#[test]
fn args_agree_on_rejected_forms() {
    for tokens in [quote!(path::to::Foo<A>), quote!(Foo::<A>)] {
        assert_rejects::<IdentWithTypeArgs>(tokens.clone());
        assert_rejects::<NewIdentWithTypeArgs>(tokens);
    }
}

/// Definition-site position: forms that both constructs reject identically.
#[test]
fn generics_agree_on_rejected_forms() {
    for tokens in [
        quote!(Foo<A: Clone>),
        quote!(Foo<A = B>),
        quote!(Foo<(A, B)>),
        quote!(Foo<Bar<A>>),
    ] {
        assert_rejects::<IdentWithTypeGenerics>(tokens.clone());
        assert_rejects::<NewIdentWithTypeGenerics>(tokens);
    }
}

/// Definition-site position: const generics are the one form where the *new*
/// construct is more permissive than the old one — the old construct rejects
/// them as a side effect of its `split_for_impl` round-trip check.
#[test]
fn generics_new_accepts_const_parameters() {
    let tokens = quote!(Bar<const N: usize>);

    assert_rejects::<IdentWithTypeGenerics>(tokens.clone());
    assert_parses::<NewIdentWithTypeGenerics>(tokens);
}

/// Definition-site position: forms that both constructs accept identically.
#[test]
fn generics_agree_on_valid_forms() {
    for tokens in [quote!(Foo), quote!(Foo<A, B>), quote!(Bar<'a, C>)] {
        assert_parses::<IdentWithTypeGenerics>(tokens.clone());
        assert_parses::<NewIdentWithTypeGenerics>(tokens);
    }
}
