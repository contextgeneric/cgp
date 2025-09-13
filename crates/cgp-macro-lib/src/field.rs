use alloc::string::ToString;
use alloc::vec::Vec;

use proc_macro2::TokenStream;
use quote::quote;
use syn::spanned::Spanned;
use syn::{parse_quote, Fields, ItemImpl, ItemStruct, LitInt};

use crate::symbol::symbol_from_string;

pub fn derive_has_field_impls_from_struct(item_struct: &ItemStruct) -> Vec<ItemImpl> {
    let struct_ident = &item_struct.ident;

    let (impl_generics, ty_generics, where_clause) = item_struct.generics.split_for_impl();

    let mut item_impls = Vec::new();

    match &item_struct.fields {
        Fields::Named(fields) => {
            for field in fields.named.iter() {
                let field_ident = field.ident.as_ref().unwrap();

                let field_symbol = symbol_from_string(&field_ident.to_string());

                let field_type = &field.ty;

                let has_field_impl: ItemImpl = parse_quote! {
                    impl #impl_generics HasField< #field_symbol >
                        for #struct_ident #ty_generics
                    #where_clause
                    {
                        type Value = #field_type;

                        fn get_field(
                            &self,
                            key: ::core::marker::PhantomData< #field_symbol >,
                        ) -> &Self::Value
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
                        ) -> &mut Self::Value
                        {
                            &mut self. #field_ident
                        }
                    }
                };

                item_impls.push(has_field_impl);
                item_impls.push(has_field_mut_impl);
            }
        }
        Fields::Unnamed(fields) => {
            for (i, field) in fields.unnamed.iter().enumerate() {
                let field_ident = LitInt::new(&format!("{i}"), field.span());
                let field_symbol = quote! { δ< #field_ident > };

                let field_type = &field.ty;

                let has_field_impl: ItemImpl = parse_quote! {
                    impl #impl_generics HasField< #field_symbol >
                        for #struct_ident #ty_generics
                    #where_clause
                    {
                        type Value = #field_type;

                        fn get_field(
                            &self,
                            key: ::core::marker::PhantomData< #field_symbol >,
                        ) -> &Self::Value
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
                        ) -> &mut Self::Value
                        {
                            &mut self. #field_ident
                        }
                    }
                };

                item_impls.push(has_field_impl);
                item_impls.push(has_field_mut_impl);
            }
        }
        _ => {}
    }

    item_impls
}

pub fn derive_has_field(input: TokenStream) -> TokenStream {
    let item_struct: ItemStruct = syn::parse2(input).unwrap();

    let item_impls = derive_has_field_impls_from_struct(&item_struct);

    quote! {
        #( #item_impls )*
    }
}
