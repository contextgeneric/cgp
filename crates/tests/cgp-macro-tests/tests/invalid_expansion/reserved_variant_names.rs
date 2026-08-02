//! The variant derives emit `Self::<AssocType>` to name their own associated types, so a variant
//! whose name collides with one of those makes the generated path ambiguous and the expansion does
//! not compile.
//!
//! **Why the output is wrong.** `#[derive(FromVariant)]` writes the payload's type as `Self::Value`,
//! and `#[derive(ExtractField)]` writes `Self::Value`, `Self::Remainder`, `Self::Extractor`,
//! `Self::ExtractorRef`, and `Self::ExtractorMut`. Inside an impl for an enum, `Self::Value` can
//! resolve to either the trait's associated type or a *variant* of that name, so an enum carrying a
//! variant called `Value` makes the path ambiguous and `rustc` reports
//! `error: ambiguous associated item`, headlined at the `#[derive(...)]` attribute.
//!
//! How readable that error is depends on the derive. `#[derive(HasFields)]` and
//! `#[derive(FromVariant)]` write their impls `for` the user's enum, so the colliding variant keeps its
//! own span and a note points straight at it. `#[derive(ExtractField)]` writes its impls for the
//! generated `__Partial…` companions, whose variant identifiers the codegen rebuilds — so both notes
//! land on the derive attribute and nothing in the output names the variant to rename. That is the case
//! worth guarding against. `#[derive(HasFields)]` also reserves `FieldsRef`, so an enum deriving the
//! whole family has seven names it cannot use.
//!
//! **What the correct output should be.** The codegen should write each associated type as a fully
//! qualified projection — `<Self as FromVariant<Tag>>::Value` rather than `Self::Value` — which is
//! unambiguous whatever the enum's variants are named. That is the same hygiene discipline the rest of
//! the suite follows for CGP's own paths, applied to the traits these derives implement.
//!
//! The snapshots below capture the emitted expansion as a string, so this test compiles even though
//! the code it describes would not. Only the offending lines are pinned; the surrounding items are the
//! ordinary output the reference documents already describe.
//!
//! Recorded in cgp-knowledge-base/cgp/reference/derives/derive_from_variant.md and
//! derive_extract_field.md, under `## Known issues`.

use cgp_macro_test_util_lib::functions::pretty_format;
use quote::quote;

/// `#[derive(FromVariant)]` on an enum with a variant named `Value`.
///
/// The emitted `value: Self::Value` parameter is the ambiguous path: `Self::Value` could be the
/// `FromVariant::Value` associated type or the `Value` variant.
#[test]
fn test_from_variant_reserves_value() {
    let output = cgp_macro_lib::derive_from_variant(quote! {
        #[derive(FromVariant)]
        pub enum Tagged {
            Value(u32),
        }
    })
    .unwrap();

    let formatted = pretty_format(output).unwrap();

    assert!(
        formatted.contains("value: Self::Value"),
        "expected the ambiguous `Self::Value` parameter, got:\n{formatted}"
    );

    insta::assert_snapshot!(formatted, @r"
    impl FromVariant<
        Symbol<5, Chars<'V', Chars<'a', Chars<'l', Chars<'u', Chars<'e', Nil>>>>>>,
    > for Tagged {
        type Value = u32;
        fn from_variant(
            _tag: ::core::marker::PhantomData<
                Symbol<5, Chars<'V', Chars<'a', Chars<'l', Chars<'u', Chars<'e', Nil>>>>>>,
            >,
            value: Self::Value,
        ) -> Self {
            Self::Value(value)
        }
    }
    ");
}

/// `#[derive(ExtractField)]` on an enum with a variant named `Remainder`.
///
/// `Self::Remainder` in the `extract_field` return type is the ambiguous path here. The same enum
/// would also collide on `Value`, `Extractor`, `ExtractorRef`, and `ExtractorMut`.
#[test]
fn test_extract_field_reserves_remainder() {
    let output = cgp_macro_lib::derive_extract_field(quote! {
        #[derive(ExtractField)]
        pub enum Tagged {
            Remainder(u32),
        }
    })
    .unwrap();

    let formatted = pretty_format(output).unwrap();

    assert!(
        formatted.contains("Result<Self::Value, Self::Remainder>"),
        "expected the ambiguous `Self::Remainder` return type, got:\n{formatted}"
    );

    // `Self::Extractor` appears in the owned accessor for the same reason.
    assert!(
        formatted.contains("-> Self::Extractor"),
        "expected the ambiguous `Self::Extractor` return type, got:\n{formatted}"
    );
}

/// `#[derive(HasFields)]` on an enum with a variant named `Fields`.
///
/// `Self::Fields` in the `from_fields` parameter and `Self::FieldsRef` in the borrowed accessor are
/// the ambiguous paths. This is the same defect in the representation slice, which is why an enum
/// deriving the whole family has seven reserved names rather than five.
#[test]
fn test_has_fields_reserves_fields() {
    let output = cgp_macro_lib::derive_has_fields(quote! {
        #[derive(HasFields)]
        pub enum Tagged {
            Fields(u32),
        }
    })
    .unwrap();

    let formatted = pretty_format(output).unwrap();

    assert!(
        formatted.contains("rest: Self::Fields"),
        "expected the ambiguous `Self::Fields` parameter, got:\n{formatted}"
    );

    assert!(
        formatted.contains("Self::FieldsRef<'__a>"),
        "expected the ambiguous `Self::FieldsRef` projection, got:\n{formatted}"
    );
}
