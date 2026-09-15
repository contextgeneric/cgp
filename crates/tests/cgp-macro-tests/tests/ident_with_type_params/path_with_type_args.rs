//! Corner cases for `PathWithTypeArgs` — a full Rust path followed by an
//! optional type-expression argument list, e.g. `path::to::Foo<A, B>`.

use cgp_macro_core::types::ident::{PathWithTypeArgs, TypeArg};
use quote::quote;
use syn::{WherePredicate, parse2};

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
fn merges_bindings_into_the_argument_list() {
    // Associated-type bindings are appended to the path's *own* arguments as one
    // list, which is the only spelling Rust accepts for a bound that both
    // instantiates a generic trait and pins its associated type. Rendering the
    // path's arguments and then a second group — `Foo<A><Item = X>` — is not a trait
    // bound in any position.
    let bare: Subject = parse2(quote!(Foo)).unwrap();
    assert_eq!(
        bare.to_bound_tokens(&[quote!(Item = X)]).to_string(),
        quote!(Foo<Item = X>).to_string(),
    );

    let generic: Subject = parse2(quote!(Foo<A>)).unwrap();
    assert_eq!(
        generic.to_bound_tokens(&[quote!(Item = X)]).to_string(),
        quote!(Foo<A, Item = X>).to_string(),
    );

    // A lifetime has to keep leading the list while the binding trails it.
    let lifetime: Subject = parse2(quote!(Foo<'a>)).unwrap();
    assert_eq!(
        lifetime.to_bound_tokens(&[quote!(Item = X)]).to_string(),
        quote!(Foo<'a, Item = X>).to_string(),
    );

    let several: Subject = parse2(quote!(path::to::Foo<A>)).unwrap();
    assert_eq!(
        several
            .to_bound_tokens(&[quote!(Item = X), quote!(Other = Y)])
            .to_string(),
        quote!(path::to::Foo<A, Item = X, Other = Y>).to_string(),
    );
}

#[test]
fn renders_unchanged_without_bindings() {
    // An empty binding list is the unpinned case, and must leave the path exactly as
    // `ToTokens` would render it, so one code path serves both.
    for tokens in [quote!(Foo), quote!(Foo<A>), quote!(::path::to::Foo<'a, A>)] {
        let parsed: Subject = parse2(tokens.clone()).unwrap();
        assert_eq!(
            parsed.to_bound_tokens(&[]).to_string(),
            tokens.to_string(),
            "empty bindings must not alter the rendering",
        );
    }
}

#[test]
fn bound_output_parses_as_a_trait_bound() {
    // The property that matters: whatever the merge emits has to be usable as a
    // trait bound. Emitting two argument groups instead once failed *inside* the
    // macro with a bare "failed to parse internal tokens" naming no cause, so the
    // malformed form never reached the compiler where it could be diagnosed.
    for path in [quote!(Foo), quote!(Foo<A>), quote!(path::to::Foo<'a, A>)] {
        let parsed: Subject = parse2(path.clone()).unwrap();
        let bound = parsed.to_bound_tokens(&[quote!(Item = X)]);

        parse2::<WherePredicate>(quote!(Self: #bound)).unwrap_or_else(|error| {
            panic!("`{path}` with a binding must parse as a trait bound, got: {error}")
        });
    }
}

#[test]
fn round_trips() {
    assert_idempotent::<Subject>(quote!(Foo));
    assert_idempotent::<Subject>(quote!(path::to::Foo));
    assert_idempotent::<Subject>(quote!(path::to::Foo<A, B>));
    assert_idempotent::<Subject>(quote!(path::to::Bar<(A, B), Baz<C>>));
    assert_idempotent::<Subject>(quote!(::path::to::Foo<'a, A>));
}
