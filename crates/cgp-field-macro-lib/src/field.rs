use syn::{parse_quote, Fields, ItemImpl, ItemStruct};

use crate::symbol::symbol_from_string;

pub fn derive_fields(item_struct: &ItemStruct) -> Vec<ItemImpl> {
    let struct_ident = &item_struct.ident;

    let (impl_generics, ty_generics, where_clause) = item_struct.generics.split_for_impl();

    let mut item_impls = Vec::new();

    match &item_struct.fields {
        Fields::Named(fields) => {
            for field in fields.named.iter() {
                let field_ident = field.ident.as_ref().unwrap();

                let field_symbol = symbol_from_string(&field_ident.to_string());

                let field_type = &field.ty;

                let item_impl: ItemImpl = parse_quote! {
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

                item_impls.push(item_impl);
            }
        }
        _ => {}
    }

    item_impls
}
