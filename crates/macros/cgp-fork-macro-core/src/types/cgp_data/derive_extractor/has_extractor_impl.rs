use quote::quote;
use syn::{Arm, Ident, ItemEnum, ItemImpl};

use crate::exports::{HasExtractor, HasExtractorMut, HasExtractorRef, IsMut, IsPresent, IsRef};
use crate::functions::override_item_span;
use crate::parse_internal;
use crate::types::cgp_data::to_generic_args;

/// Emit the owned `HasExtractor` impl: `to_extractor`/`from_extractor` map each
/// variant to and from the all-`IsPresent` configuration of the partial enum.
pub fn derive_has_extractor_impl(
    context_enum: &ItemEnum,
    extractor_ident: &Ident,
) -> syn::Result<ItemImpl> {
    let (impl_generics, ty_generics, where_clause) = context_enum.generics.split_for_impl();

    let context_ident = &context_enum.ident;

    let mut extractor_generics = to_generic_args(&context_enum.generics)?;

    let mut to_match_arms = Vec::<Arm>::new();

    let mut from_match_arms = Vec::<Arm>::new();

    for variant in context_enum.variants.iter() {
        extractor_generics.args.push(parse_internal! {
            #IsPresent
        });

        let variant_ident = &variant.ident;

        to_match_arms.push(parse_internal! {
            Self :: #variant_ident ( value ) => {
                #extractor_ident:: #variant_ident ( value )
            }
        });

        from_match_arms.push(parse_internal! {
            #extractor_ident:: #variant_ident ( value ) => {
                Self :: #variant_ident ( value )
            }
        });
    }

    let item_impl: ItemImpl = parse_internal! {
        impl #impl_generics #HasExtractor
            for #context_ident #ty_generics
        #where_clause
        {
            type Extractor = #extractor_ident #extractor_generics;

            fn to_extractor(self) -> Self::Extractor {
                match self {
                    #(#to_match_arms)*
                }
            }

            fn from_extractor(extractor: Self::Extractor) -> Self {
                match extractor {
                    #(#from_match_arms)*
                }
            }
        }
    };

    // Key the error span on the enum name the user wrote, not the whole derive.
    // See cgp-knowledge-base/cgp/implementation/README.md#spans.
    override_item_span(context_ident.span(), &item_impl)
}

/// Emit the `HasExtractorRef` impl over the borrowed partial enum (`IsRef`). Its
/// GAT and method use the reserved `'__a__` lifetime rather than a bare `'a` so
/// they never collide with an enum whose own lifetime parameter is named `'a`.
pub fn derive_has_extractor_ref_impl(
    context_enum: &ItemEnum,
    extractor_ident: &Ident,
) -> syn::Result<ItemImpl> {
    let (impl_generics, ty_generics, where_clause) = context_enum.generics.split_for_impl();

    let context_ident = &context_enum.ident;

    let is_empty = context_enum.variants.is_empty();

    let mut extractor_generics = to_generic_args(&context_enum.generics)?;

    // A variantless enum drops the borrowed extractor's `'__a__`/`__R__`
    // parameters (see `derive_extractor_enum_ref`), so pass no borrow arguments
    // and match on the dereferenced uninhabited place — a bare `match self {}`
    // over `&Self` is non-exhaustive because a reference is always inhabited.
    if !is_empty {
        extractor_generics.args.insert(
            0,
            parse_internal! {
                '__a__
            },
        );

        extractor_generics.args.insert(
            1,
            parse_internal! {
                #IsRef
            },
        );
    }

    let mut match_arms = Vec::<Arm>::new();

    for variant in context_enum.variants.iter() {
        extractor_generics.args.push(parse_internal! {
            #IsPresent
        });

        let variant_ident = &variant.ident;

        match_arms.push(parse_internal! {
            Self :: #variant_ident ( value ) => {
                #extractor_ident:: #variant_ident ( value )
            }
        });
    }

    let body = if is_empty {
        quote! { match *self {} }
    } else {
        quote! { match self { #(#match_arms)* } }
    };

    let item_impl: ItemImpl = parse_internal! {
        impl #impl_generics #HasExtractorRef
            for #context_ident #ty_generics
        #where_clause
        {
            type ExtractorRef<'__a__> = #extractor_ident #extractor_generics
            where
                Self: '__a__;

            fn extractor_ref<'__a__>(&'__a__ self) -> Self::ExtractorRef<'__a__> {
                #body
            }
        }
    };

    // Key the error span on the enum name the user wrote, not the whole derive.
    // See cgp-knowledge-base/cgp/implementation/README.md#spans.
    override_item_span(context_ident.span(), &item_impl)
}

/// Emit the `HasExtractorMut` impl: the `IsMut` mirror of
/// [`derive_has_extractor_ref_impl`], likewise using the reserved `'__a__`.
pub fn derive_has_extractor_mut_impl(
    context_enum: &ItemEnum,
    extractor_ident: &Ident,
) -> syn::Result<ItemImpl> {
    let (impl_generics, ty_generics, where_clause) = context_enum.generics.split_for_impl();

    let context_ident = &context_enum.ident;

    let is_empty = context_enum.variants.is_empty();

    let mut extractor_generics = to_generic_args(&context_enum.generics)?;

    // See `derive_has_extractor_ref_impl`: a variantless enum drops the borrowed
    // extractor's `'__a__`/`__R__` parameters and matches the dereferenced place.
    if !is_empty {
        extractor_generics.args.insert(
            0,
            parse_internal! {
                '__a__
            },
        );

        extractor_generics.args.insert(
            1,
            parse_internal! {
                #IsMut
            },
        );
    }

    let mut match_arms = Vec::<Arm>::new();

    for variant in context_enum.variants.iter() {
        extractor_generics.args.push(parse_internal! {
            #IsPresent
        });

        let variant_ident = &variant.ident;

        match_arms.push(parse_internal! {
            Self :: #variant_ident ( value ) => {
                #extractor_ident:: #variant_ident ( value )
            }
        });
    }

    let body = if is_empty {
        quote! { match *self {} }
    } else {
        quote! { match self { #(#match_arms)* } }
    };

    let item_impl: ItemImpl = parse_internal! {
        impl #impl_generics #HasExtractorMut
            for #context_ident #ty_generics
        #where_clause
        {
            type ExtractorMut<'__a__> = #extractor_ident #extractor_generics
            where
                Self: '__a__;

            fn extractor_mut<'__a__>(&'__a__ mut self) -> Self::ExtractorMut<'__a__> {
                #body
            }
        }
    };

    // Key the error span on the enum name the user wrote, not the whole derive.
    // See cgp-knowledge-base/cgp/implementation/README.md#spans.
    override_item_span(context_ident.span(), &item_impl)
}
