use quote::quote;
use syn::{Arm, Ident, ItemEnum, ItemImpl, parse2};

use crate::exports::{HasExtractor, HasExtractorMut, HasExtractorRef, IsMut, IsPresent, IsRef};
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
        extractor_generics.args.push(parse2(quote! {
            #IsPresent
        })?);

        let variant_ident = &variant.ident;

        to_match_arms.push(parse2(quote! {
            Self :: #variant_ident ( value ) => {
                #extractor_ident:: #variant_ident ( value )
            }
        })?);

        from_match_arms.push(parse2(quote! {
            #extractor_ident:: #variant_ident ( value ) => {
                Self :: #variant_ident ( value )
            }
        })?);
    }

    let item_impl = parse2(quote! {
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
    })?;

    Ok(item_impl)
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

    let mut extractor_generics = to_generic_args(&context_enum.generics)?;
    extractor_generics.args.insert(
        0,
        parse2(quote! {
            '__a__
        })?,
    );

    extractor_generics.args.insert(
        1,
        parse2(quote! {
            #IsRef
        })?,
    );

    let mut match_arms = Vec::<Arm>::new();

    for variant in context_enum.variants.iter() {
        extractor_generics.args.push(parse2(quote! {
            #IsPresent
        })?);

        let variant_ident = &variant.ident;

        match_arms.push(parse2(quote! {
            Self :: #variant_ident ( value ) => {
                #extractor_ident:: #variant_ident ( value )
            }
        })?);
    }

    let item_impl = parse2(quote! {
        impl #impl_generics #HasExtractorRef
            for #context_ident #ty_generics
        #where_clause
        {
            type ExtractorRef<'__a__> = #extractor_ident #extractor_generics
            where
                Self: '__a__;

            fn extractor_ref<'__a__>(&'__a__ self) -> Self::ExtractorRef<'__a__> {
                match self {
                    #(#match_arms)*
                }
            }
        }
    })?;

    Ok(item_impl)
}

/// Emit the `HasExtractorMut` impl: the `IsMut` mirror of
/// [`derive_has_extractor_ref_impl`], likewise using the reserved `'__a__`.
pub fn derive_has_extractor_mut_impl(
    context_enum: &ItemEnum,
    extractor_ident: &Ident,
) -> syn::Result<ItemImpl> {
    let (impl_generics, ty_generics, where_clause) = context_enum.generics.split_for_impl();

    let context_ident = &context_enum.ident;

    let mut extractor_generics = to_generic_args(&context_enum.generics)?;
    extractor_generics.args.insert(
        0,
        parse2(quote! {
            '__a__
        })?,
    );

    extractor_generics.args.insert(
        1,
        parse2(quote! {
            #IsMut
        })?,
    );

    let mut match_arms = Vec::<Arm>::new();

    for variant in context_enum.variants.iter() {
        extractor_generics.args.push(parse2(quote! {
            #IsPresent
        })?);

        let variant_ident = &variant.ident;

        match_arms.push(parse2(quote! {
            Self :: #variant_ident ( value ) => {
                #extractor_ident:: #variant_ident ( value )
            }
        })?);
    }

    let item_impl = parse2(quote! {
        impl #impl_generics #HasExtractorMut
            for #context_ident #ty_generics
        #where_clause
        {
            type ExtractorMut<'__a__> = #extractor_ident #extractor_generics
            where
                Self: '__a__;

            fn extractor_mut<'__a__>(&'__a__ mut self) -> Self::ExtractorMut<'__a__> {
                match self {
                    #(#match_arms)*
                }
            }
        }
    })?;

    Ok(item_impl)
}
