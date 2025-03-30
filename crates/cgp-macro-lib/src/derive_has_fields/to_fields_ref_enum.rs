use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse2, Ident, ItemEnum, ItemImpl};

use crate::derive_has_fields::to_fields_enum::extract_variant_args;
use crate::derive_has_fields::to_fields_struct::{derive_to_fields_constructor, FieldLabel};

pub fn derive_to_fields_ref_for_enum(item_enum: &ItemEnum) -> syn::Result<ItemImpl> {
    let struct_name = &item_enum.ident;
    let (impl_generics, type_generics, where_clause) = item_enum.generics.split_for_impl();

    let mut match_arms = quote! {};
    let mut inject_prefix: Box<dyn Fn(TokenStream) -> TokenStream> =
        Box::new(|inner: TokenStream| quote! { #inner });

    for variant in item_enum.variants.iter() {
        let variant_ident = &variant.ident;

        let constructor = derive_to_fields_constructor(&variant.fields, |label| match label {
            FieldLabel::Named(label) => quote! {
                &#label .into()
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

    let life = quote! { '__a };

    let item_impl = quote! {
        impl #impl_generics
            ToFieldsRef for #struct_name #type_generics
        #where_clause
        {
            fn to_fields_ref< #life >(
                & #life self,
            ) -> Self::FieldsRef< #life >
            where
                Self: #life,
            {
                match self {
                    #match_arms
                }
            }
        }
    };

    parse2(item_impl)
}
