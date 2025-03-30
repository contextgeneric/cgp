use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::spanned::Spanned;
use syn::{parse2, Error, Fields, ItemImpl, ItemStruct, LitInt};

pub fn derive_to_fields_for_struct(item_struct: &ItemStruct) -> syn::Result<ItemImpl> {
    let struct_name = &item_struct.ident;
    let (impl_generics, type_generics, where_clause) = item_struct.generics.split_for_impl();

    let constructor = derive_to_fields_constructor(&item_struct.fields, |field_name| {
        quote! {
            self . #field_name .into()
        }
    })?;

    let item_impl = parse2(quote! {
        impl #impl_generics
            ToFields for #struct_name #type_generics
        #where_clause
        {
            fn to_fields(
                self,
            ) -> Self::Fields {
                #constructor
            }
        }
    })?;

    Ok(item_impl)
}

pub fn derive_to_fields_constructor(
    fields: &Fields,
    construct_field: impl Fn(TokenStream) -> TokenStream,
) -> syn::Result<TokenStream> {
    let mut constructors = quote! { Nil };

    match &fields {
        Fields::Named(fields) => {
            for field in fields.named.iter().rev() {
                let field_name = field.ident.as_ref().ok_or_else(|| {
                    Error::new_spanned(field, "expect struct field to contain name identifier")
                })?;

                let constructor = construct_field(field_name.to_token_stream());

                constructors = quote! {
                    Cons(
                        #constructor,
                        #constructors
                    )
                };
            }
        }
        Fields::Unnamed(fields) => {
            for (i, field) in fields.unnamed.iter().enumerate().rev() {
                let field_name = LitInt::new(&format!("{i}"), field.span());

                let constructor = construct_field(field_name.to_token_stream());

                constructors = quote! {
                    Cons(
                        #constructor,
                        #constructors
                    )
                };
            }
        }
        Fields::Unit => {}
    };

    Ok(constructors)
}
