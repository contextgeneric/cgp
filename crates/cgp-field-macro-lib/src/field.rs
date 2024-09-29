use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{parse_quote, Fields, ItemImpl, ItemStruct};

use crate::symbol::symbol_from_string;

pub fn derive_has_field_impls(item_struct: &ItemStruct) -> Vec<ItemImpl> {
    let struct_ident = &item_struct.ident;

    let (impl_generics, ty_generics, where_clause) = item_struct.generics.split_for_impl();

    let mut item_impls = Vec::new();

    if let Fields::Named(fields) = &item_struct.fields {
        for field in fields.named.iter() {
            let field_ident = field.ident.as_ref().unwrap();

            let field_symbol = symbol_from_string(&field_ident.to_string());

            let field_type = &field.ty;

            let has_field_impl: ItemImpl = parse_quote! {
                impl #impl_generics HasField< #field_symbol >
                    for #struct_ident #ty_generics
                #where_clause
                {
                    type Field = #field_type;

                    fn get_field(
                        &self,
                        key: ::core::marker::PhantomData< #field_symbol >,
                    ) -> &Self::Field
                    {
                        &self. #field_ident
                    }
                }
            };

            let has_field_mut_impl: ItemImpl = parse_quote! {
                impl #impl_generics HasFieldMut< #field_symbol >
                    for #struct_ident #ty_generics
                #where_clause
                {
                    fn get_field_mut(
                        &mut self,
                        key: ::core::marker::PhantomData< #field_symbol >,
                    ) -> &mut Self::Field
                    {
                        &mut self. #field_ident
                    }
                }
            };

            item_impls.push(has_field_impl);
            item_impls.push(has_field_mut_impl);
        }
    }

    item_impls
}

pub fn derive_fields(input: TokenStream) -> TokenStream {
    let item_struct: ItemStruct = syn::parse2(input).unwrap();

    let item_impls = derive_has_field_impls(&item_struct);

    let mut output = TokenStream::new();

    for item_impl in item_impls {
        output.extend(item_impl.to_token_stream());
    }

    output
}
