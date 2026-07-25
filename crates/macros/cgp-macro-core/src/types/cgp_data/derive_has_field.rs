use syn::spanned::Spanned;
use syn::{Fields, ItemImpl, ItemStruct, LitInt};

use crate::exports::{HasField, HasFieldMut};
use crate::functions::override_item_span;
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

                // Aim a compiler error on either generated impl — a coherence
                // conflict (`E0119`) with a hand-written `HasField` impl for the
                // same tag, say — at the field the user wrote rather than at the
                // whole `#[derive(HasField)]`, which is where the impl's
                // `call_site`-spanned `impl`/`{ … }` boundary would otherwise put
                // the caret. See cgp-knowledge-base/cgp/implementation/README.md#spans.
                let field_span = field_ident.span();

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

                item_impls.push(override_item_span(field_span, &has_field_impl)?);
                item_impls.push(override_item_span(field_span, &has_field_mut_impl)?);
            }
        }
        Fields::Unnamed(fields) => {
            for (i, field) in fields.unnamed.iter().enumerate() {
                // A tuple field has no identifier, so its whole `syn::Field` span
                // is the narrowest token the user wrote; re-span each generated
                // impl onto it for the same reason as the named case above.
                let field_span = field.span();

                let field_tag = Index {
                    index: i,
                    span: field_span,
                };

                let field_ident = LitInt::new(&format!("{i}"), field_span);

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

                item_impls.push(override_item_span(field_span, &has_field_impl)?);
                item_impls.push(override_item_span(field_span, &has_field_mut_impl)?);
            }
        }
        _ => {}
    }

    Ok(item_impls)
}
