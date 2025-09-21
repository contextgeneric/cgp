use quote::quote;
use syn::{Arm, Ident, ItemEnum, ItemImpl, parse2};

use crate::derive_builder::to_generic_args;

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
            IsPresent
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
        impl #impl_generics HasExtractor
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
            'a
        })?,
    );

    extractor_generics.args.insert(
        1,
        parse2(quote! {
            IsRef
        })?,
    );

    let mut match_arms = Vec::<Arm>::new();

    for variant in context_enum.variants.iter() {
        extractor_generics.args.push(parse2(quote! {
            IsPresent
        })?);

        let variant_ident = &variant.ident;

        match_arms.push(parse2(quote! {
            Self :: #variant_ident ( value ) => {
                #extractor_ident:: #variant_ident ( value )
            }
        })?);
    }

    let item_impl = parse2(quote! {
        impl #impl_generics HasExtractorRef
            for #context_ident #ty_generics
        #where_clause
        {
            type ExtractorRef<'a> = #extractor_ident #extractor_generics
            where
                Self: 'a;

            fn extractor_ref<'a>(&'a self) -> Self::ExtractorRef<'a> {
                match self {
                    #(#match_arms)*
                }
            }
        }
    })?;

    Ok(item_impl)
}

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
            'a
        })?,
    );

    extractor_generics.args.insert(
        1,
        parse2(quote! {
            IsMut
        })?,
    );

    let mut match_arms = Vec::<Arm>::new();

    for variant in context_enum.variants.iter() {
        extractor_generics.args.push(parse2(quote! {
            IsPresent
        })?);

        let variant_ident = &variant.ident;

        match_arms.push(parse2(quote! {
            Self :: #variant_ident ( value ) => {
                #extractor_ident:: #variant_ident ( value )
            }
        })?);
    }

    let item_impl = parse2(quote! {
        impl #impl_generics HasExtractorMut
            for #context_ident #ty_generics
        #where_clause
        {
            type ExtractorMut<'a> = #extractor_ident #extractor_generics
            where
                Self: 'a;

            fn extractor_mut<'a>(&'a mut self) -> Self::ExtractorMut<'a> {
                match self {
                    #(#match_arms)*
                }
            }
        }
    })?;

    Ok(item_impl)
}
