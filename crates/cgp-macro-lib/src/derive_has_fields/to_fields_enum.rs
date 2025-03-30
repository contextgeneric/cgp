use proc_macro2::TokenStream;
use quote::quote;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::token::Comma;
use syn::{parse2, Error, Fields, Ident, ItemEnum, ItemImpl, Variant};

use crate::derive_has_fields::to_fields_struct::{derive_to_fields_constructor, FieldLabel};

pub fn derive_to_fields_for_enum(item_enum: &ItemEnum) -> syn::Result<ItemImpl> {
    let enum_name = &item_enum.ident;
    let (impl_generics, type_generics, where_clause) = item_enum.generics.split_for_impl();

    let match_arms = derive_to_fields_match_arms(&item_enum.variants)?;

    let item_impl = quote! {
        impl #impl_generics
            ToFields for #enum_name #type_generics
        #where_clause
        {
            fn to_fields(
                self,
            ) -> Self::Fields {
                match self {
                    #match_arms
                }
            }
        }
    };

    parse2(item_impl)
}

pub fn derive_to_fields_match_arms(
    variants: &Punctuated<Variant, Comma>,
) -> syn::Result<TokenStream> {
    let mut match_arms = quote! {};
    let mut inject_prefix: Box<dyn Fn(TokenStream) -> TokenStream> =
        Box::new(|inner: TokenStream| quote! { #inner });

    for variant in variants.iter() {
        let variant_ident = &variant.ident;

        let constructor = derive_to_fields_constructor(&variant.fields, |label| match label {
            FieldLabel::Named(label) => quote! {
                #label .into()
            },
            FieldLabel::Unnamed(label) => {
                let field_name = Ident::new(&format!("field_{label}"), label.span());

                quote! {
                    #field_name .into()
                }
            }
        })?;

        let variant_args = extract_variant_args(&variant.fields)?;

        let inject_variant = inject_prefix(quote! {
            Either::Left( #constructor .into() )
        });

        inject_prefix = Box::new(move |inner| {
            let outer = inject_prefix(inner);

            quote! {
                Either::Right( #outer )
            }
        });

        match_arms = quote! {
            #match_arms
            Self :: #variant_ident #variant_args => {
                #inject_variant
            }
        };
    }

    Ok(match_arms)
}

pub fn extract_variant_args(fields: &Fields) -> syn::Result<TokenStream> {
    match fields {
        Fields::Named(fields) => {
            let mut args = TokenStream::new();

            for field in fields.named.iter().rev() {
                let field_name = field.ident.as_ref().ok_or_else(|| {
                    Error::new_spanned(field, "expect struct field to contain name identifier")
                })?;

                args = quote! { #field_name , #args };
            }

            Ok(quote! { { #args } })
        }
        Fields::Unnamed(fields) => {
            let mut args = TokenStream::new();
            let mut constructor = quote! { Nil };

            for (i, field) in fields.unnamed.iter().enumerate().rev() {
                let field_name = Ident::new(&format!("field_{i}"), field.span());

                args = quote! { #field_name , #args };
                constructor = quote! { Cons( #field_name .into(), #constructor ) }
            }

            Ok(quote! { ( #args ) })
        }
        Fields::Unit => Ok(TokenStream::new()),
    }
}
