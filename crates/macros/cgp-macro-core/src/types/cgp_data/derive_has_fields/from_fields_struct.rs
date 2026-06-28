use proc_macro2::TokenStream;
use quote::quote;
use syn::spanned::Spanned;
use syn::{Error, Fields, Ident, ItemImpl, ItemStruct, parse2};

pub fn derive_from_fields_for_struct(item_struct: &ItemStruct) -> syn::Result<ItemImpl> {
    let struct_name = &item_struct.ident;
    let (impl_generics, type_generics, where_clause) = item_struct.generics.split_for_impl();

    let (fields_arg, constructor_args) = derive_from_field_params(&item_struct.fields)?;

    let item_impl: ItemImpl = parse2(quote! {
        impl #impl_generics
            FromFields for #struct_name #type_generics
        #where_clause
        {
            fn from_fields(
                #fields_arg: Self::Fields,
            ) -> Self {
                Self #constructor_args
            }
        }
    })?;

    Ok(item_impl)
}

pub fn derive_from_field_params(fields: &Fields) -> syn::Result<(TokenStream, TokenStream)> {
    match fields {
        Fields::Named(fields) => {
            let mut fields_arg = quote! { ε };
            let mut constructor_args = quote! {};

            for field in fields.named.iter().rev() {
                let field_name = field.ident.as_ref().ok_or_else(|| {
                    Error::new_spanned(field, "expect struct field to contain name identifier")
                })?;

                fields_arg = quote! {
                    π( #field_name, #fields_arg )
                };

                constructor_args = quote! {
                    #field_name : #field_name .value ,
                    #constructor_args
                };
            }

            Ok((
                fields_arg,
                quote! {
                    { #constructor_args }
                },
            ))
        }
        Fields::Unnamed(fields) => {
            if fields.unnamed.len() == 1 {
                let fields_arg = quote! { field };
                let constructor_args = quote! { ( field ) };

                Ok((fields_arg, constructor_args))
            } else {
                let mut fields_arg = quote! { ε };
                let mut constructor_args = quote! {};

                for (i, field) in fields.unnamed.iter().enumerate() {
                    let field_name: Ident = Ident::new(&format!("field_{i}"), field.span());

                    fields_arg = quote! {
                        π( #field_name, #fields_arg )
                    };

                    constructor_args = quote! {
                        #field_name .value ,
                        #constructor_args
                    };
                }

                Ok((
                    fields_arg,
                    quote! {
                        ( #constructor_args )
                    },
                ))
            }
        }
        Fields::Unit => Ok((quote! { ε }, TokenStream::new())),
    }
}
