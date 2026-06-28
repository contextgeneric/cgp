use syn::spanned::Spanned;
use syn::{Fields, ItemImpl, ItemStruct, LitInt};

use crate::exports::{HasField, HasFieldMut};
use crate::parse_internal;
use crate::types::field::{Index, Symbol};

pub fn derive_has_field_impls_from_struct(item_struct: &ItemStruct) -> syn::Result<Vec<ItemImpl>> {
    let struct_ident = &item_struct.ident;

    let (impl_generics, ty_generics, where_clause) = item_struct.generics.split_for_impl();

    let mut item_impls = Vec::new();

    match &item_struct.fields {
        Fields::Named(fields) => {
            for field in fields.named.iter() {
                let field_ident = field.ident.as_ref().unwrap();

                let field_symbol = Symbol::from_ident(field_ident.clone());

                let field_type = &field.ty;

                let has_field_impl: ItemImpl = parse_internal! {
                    impl #impl_generics #HasField< #field_symbol >
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

                let has_field_mut_impl: ItemImpl = parse_internal! {
                    impl #impl_generics #HasFieldMut< #field_symbol >
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
                let field_tag = Index {
                    index: i,
                    span: field.span(),
                };

                let field_ident = LitInt::new(&format!("{i}"), field.span());

                let field_type = &field.ty;

                let has_field_impl: ItemImpl = parse_internal! {
                    impl #impl_generics #HasField< #field_tag >
                        for #struct_ident #ty_generics
                    #where_clause
                    {
                        type Value = #field_type;

                        fn get_field(
                            &self,
                            key: ::core::marker::PhantomData< #field_tag >,
                        ) -> &Self::Value
                        {
                            &self. #field_ident
                        }
                    }
                };

                let has_field_mut_impl: ItemImpl = parse_internal! {
                    impl #impl_generics #HasFieldMut< #field_tag >
                        for #struct_ident #ty_generics
                    #where_clause
                    {
                        fn get_field_mut(
                            &mut self,
                            key: ::core::marker::PhantomData< #field_tag >,
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

    Ok(item_impls)
}
