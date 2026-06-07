use quote::quote;
use syn::{Ident, ItemImpl, ItemStruct, parse2};

use crate::derive_builder::index_to_generic_ident;

pub fn derive_partial_data_impl(
    context_struct: &ItemStruct,
    builder_ident: &Ident,
) -> syn::Result<ItemImpl> {
    let mut generics = context_struct.generics.clone();

    for (index, _) in context_struct.fields.iter().enumerate() {
        let generic_param_name = index_to_generic_ident(index);

        generics.params.push(parse2(quote! {
            #generic_param_name: MapType
        })?);
    }

    let (impl_generics, type_generics, where_clause) = generics.split_for_impl();

    let context_ident = &context_struct.ident;
    let context_generics = context_struct.generics.split_for_impl().1;

    let item_impl = parse2(quote! {
        impl #impl_generics PartialData
            for #builder_ident #type_generics
        #where_clause
        {
            type Target = #context_ident #context_generics;
        }
    })?;

    Ok(item_impl)
}
