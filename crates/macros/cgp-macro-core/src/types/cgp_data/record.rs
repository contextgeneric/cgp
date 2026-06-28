use syn::spanned::Spanned;
use syn::{Fields, Ident, Item, ItemImpl, ItemStruct, LitInt, parse_quote};

use crate::types::cgp_data::{
    derive_builder_struct, derive_finalize_build_impl, derive_has_builder_impl,
    derive_has_field_impls, derive_has_fields_impls_from_struct, derive_into_builder_impl,
    derive_partial_data_impl_from_struct, derive_update_field_impls,
};
use crate::types::field::{Index, Symbol};

pub struct ItemCgpRecord {
    pub item_struct: ItemStruct,
}

impl ItemCgpRecord {
    pub fn to_has_field_impls(&self) -> syn::Result<Vec<ItemImpl>> {
        let item_struct = &self.item_struct;

        let struct_ident = &item_struct.ident;

        let (impl_generics, ty_generics, where_clause) = item_struct.generics.split_for_impl();

        let mut item_impls = Vec::new();

        match &item_struct.fields {
            Fields::Named(fields) => {
                for field in fields.named.iter() {
                    let field_ident = field.ident.as_ref().unwrap();

                    let field_symbol = Symbol::from_ident(field_ident.clone());

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
                    let field_tag = Index {
                        index: i,
                        span: field.span(),
                    };

                    let field_ident = LitInt::new(&format!("{i}"), field.span());

                    let field_type = &field.ty;

                    let has_field_impl: ItemImpl = parse_quote! {
                        impl #impl_generics HasField< #field_tag >
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

                    let has_field_mut_impl: ItemImpl = parse_quote! {
                        impl #impl_generics HasFieldMut< #field_tag >
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

    pub fn to_has_fields_impls(&self) -> syn::Result<Vec<ItemImpl>> {
        derive_has_fields_impls_from_struct(&self.item_struct)
    }

    pub fn to_build_field_items(&self) -> syn::Result<Vec<Item>> {
        let item_struct = &self.item_struct;

        let context_ident = &item_struct.ident;
        let builder_ident = Ident::new(&format!("__Partial{context_ident}"), context_ident.span());

        let builder_struct = derive_builder_struct(item_struct, &builder_ident)?;

        let has_builder_impl = derive_has_builder_impl(item_struct, &builder_ident)?;

        let into_builder_impl = derive_into_builder_impl(item_struct, &builder_ident)?;

        let partial_data_impl = derive_partial_data_impl_from_struct(item_struct, &builder_ident)?;

        let finalize_build_impl = derive_finalize_build_impl(item_struct, &builder_ident)?;

        let update_field_impls = derive_update_field_impls(item_struct, &builder_ident)?;

        let has_field_impls = derive_has_field_impls(item_struct, &builder_ident)?;

        let mut out = vec![
            builder_struct.into(),
            has_builder_impl.into(),
            into_builder_impl.into(),
            partial_data_impl.into(),
            finalize_build_impl.into(),
        ];

        out.extend(update_field_impls.into_iter().map(Item::from));
        out.extend(has_field_impls.into_iter().map(Item::from));

        Ok(out)
    }
}
