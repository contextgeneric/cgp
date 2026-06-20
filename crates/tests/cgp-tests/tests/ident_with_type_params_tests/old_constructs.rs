//! Behavior of the original constructs, `IdentWithTypeArgs` and
//! `IdentWithTypeGenerics`.
//!
//! These tests document the existing behavior as-is, including the cases where
//! the original constructs are *too permissive* (accepting forms that are not
//! valid in the relevant position). The new constructs tighten exactly these
//! cases; see `old_vs_new.rs` for the side-by-side comparison.

use cgp_macro_core::types::ident::{IdentWithTypeArgs, IdentWithTypeGenerics};
use quote::quote;

use super::{assert_parses, assert_rejects};

mod ident_with_type_args {
    use super::*;

    type Subject = IdentWithTypeArgs;

    #[test]
    fn accepts_valid_forms() {
        assert_parses::<Subject>(quote!(Foo));
        assert_parses::<Subject>(quote!(Foo));
        assert_parses::<Subject>(quote!(Foo<A, B>));
        assert_parses::<Subject>(quote!(Foo<(A, B), C>));
        assert_parses::<Subject>(quote!(Foo<Bar<A>>));
        assert_parses::<Subject>(quote!(Foo<'a, A>));
        assert_parses::<Subject>(quote!(Foo<3>));
    }

    // The following four cases demonstrate the unfaithful behavior: these are
    // accepted because `IdentWithTypeArgs` delegates to
    // `syn::AngleBracketedGenericArguments`, which permits associated bindings
    // and bounds that are invalid in a plain type-argument position.

    #[test]
    fn unfaithfully_accepts_associated_type_binding() {
        assert_parses::<Subject>(quote!(Foo<A, B = C>));
        assert_parses::<Subject>(quote!(Foo<Item = X>));
    }

    #[test]
    fn unfaithfully_accepts_associated_const_binding() {
        assert_parses::<Subject>(quote!(Foo<N = 1>));
    }

    #[test]
    fn unfaithfully_accepts_associated_type_bound() {
        assert_parses::<Subject>(quote!(Foo<A: Clone>));
    }

    #[test]
    fn rejects_path_head() {
        // The head is always a single `Ident`, so paths cannot be parsed even
        // where the source genuinely is a `syn::Path`.
        assert_rejects::<Subject>(quote!(path::to::Foo<A>));
    }

    #[test]
    fn rejects_turbofish() {
        assert_rejects::<Subject>(quote!(Foo::<A>));
    }
}

mod ident_with_type_generics {
    use super::*;

    type Subject = IdentWithTypeGenerics;

    #[test]
    fn accepts_valid_forms() {
        assert_parses::<Subject>(quote!(Foo));
        assert_parses::<Subject>(quote!(Foo<A, B>));
        assert_parses::<Subject>(quote!(Bar<'a, C>));
    }

    #[test]
    fn rejects_bounds_via_roundtrip_hack() {
        assert_rejects::<Subject>(quote!(Foo<A: Clone>));
    }

    #[test]
    fn rejects_defaults_via_roundtrip_hack() {
        assert_rejects::<Subject>(quote!(Foo<A = B>));
    }

    #[test]
    fn rejects_composite_parameters() {
        assert_rejects::<Subject>(quote!(Foo<(A, B)>));
        assert_rejects::<Subject>(quote!(Foo<Bar<A>>));
    }

    #[test]
    fn rejects_const_parameters() {
        // The `split_for_impl` round-trip collapses a const parameter into a
        // bare type parameter, so the equality check fails and the input is
        // rejected. (The new `NewIdentWithTypeGenerics` accepts this form.)
        assert_rejects::<Subject>(quote!(Bar<const N: usize>));
    }
}
