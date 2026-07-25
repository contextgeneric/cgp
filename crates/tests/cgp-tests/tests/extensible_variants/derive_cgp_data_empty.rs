//! `#[derive(CgpData)]` on a variantless enum. An empty enum is uninhabited, so
//! the machinery is degenerate — there is no value to convert or extract — but
//! the derive must still emit code that compiles. Two shapes are special-cased:
//! the borrowed partial enum `__PartialRefNever` becomes a bare empty enum with
//! no `'__a__`/`__R__` parameters (otherwise they would be unused, `E0392`), and
//! every borrowed accessor matches the dereferenced place (`match *self {}`)
//! rather than a bare `match self {}` over a reference, which is non-exhaustive
//! because a reference is always inhabited (`E0004`). The owned side needs no
//! special case: it matches owned uninhabited values directly.
//!
//! This concept owns the variant expansion of `#[derive(CgpData)]`; this file is
//! the empty-enum snapshot.
//!
//! See cgp-knowledge-base/cgp/reference/derives/derive_cgp_data.md and
//! cgp-knowledge-base/cgp/reference/derives/derive_extract_field.md.

use cgp_macro_test_util::snapshot_derive_cgp_data;

snapshot_derive_cgp_data! {
    #[derive(CgpData)]
    pub enum Never {}

    expand_never(output) {
        insta::assert_snapshot!(output, @"
        impl HasFields for Never {
            type Fields = Void;
        }
        impl HasFieldsRef for Never {
            type FieldsRef<'__a> = Void where Self: '__a;
        }
        impl FromFields for Never {
            fn from_fields(rest: Self::Fields) -> Self {
                match rest {}
            }
        }
        impl ToFields for Never {
            fn to_fields(self) -> Self::Fields {
                match self {}
            }
        }
        impl ToFieldsRef for Never {
            fn to_fields_ref<'__a>(&'__a self) -> Self::FieldsRef<'__a>
            where
                Self: '__a,
            {
                match *self {}
            }
        }
        pub enum __PartialNever {}
        pub enum __PartialRefNever {}
        impl PartialData for __PartialNever {
            type Target = Never;
        }
        impl PartialData for __PartialRefNever {
            type Target = Never;
        }
        impl HasExtractor for Never {
            type Extractor = __PartialNever;
            fn to_extractor(self) -> Self::Extractor {
                match self {}
            }
            fn from_extractor(extractor: Self::Extractor) -> Self {
                match extractor {}
            }
        }
        impl HasExtractorRef for Never {
            type ExtractorRef<'__a__> = __PartialRefNever where Self: '__a__;
            fn extractor_ref<'__a__>(&'__a__ self) -> Self::ExtractorRef<'__a__> {
                match *self {}
            }
        }
        impl HasExtractorMut for Never {
            type ExtractorMut<'__a__> = __PartialRefNever where Self: '__a__;
            fn extractor_mut<'__a__>(&'__a__ mut self) -> Self::ExtractorMut<'__a__> {
                match *self {}
            }
        }
        impl FinalizeExtract for __PartialNever {
            fn finalize_extract<__T__>(self) -> __T__ {
                match self {}
            }
        }
        impl FinalizeExtract for __PartialRefNever {
            fn finalize_extract<__T__>(self) -> __T__ {
                match self {}
            }
        }
        ")
    }
}
