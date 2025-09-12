use quote::quote;
use syn::{parse2, Ident, ItemImpl, ItemStruct};

use crate::derive_builder::{index_to_generic_ident, to_generic_args};

pub fn derive_partial_data_impl(
    context_struct: &ItemStruct,
    builder_ident: &Ident,
) -> syn::Result<ItemImpl> {
    let mut generics = context_struct.generics.clone();
    let mut source_generic_args = to_generic_args(&context_struct.generics)?.args;

    for (index, _) in context_struct.fields.iter().enumerate() {
        let generic_param_name = index_to_generic_ident(index);

        generics.params.push(parse2(quote! {
            #generic_param_name: MapType
        })?);

        source_generic_args.push(parse2(quote! {
            #generic_param_name
        })?);
    }

    let (impl_generics, _, where_clause) = generics.split_for_impl();

    let context_ident = &context_struct.ident;
    let context_generics = context_struct.generics.split_for_impl().1;

    let item_impl = parse2(quote! {
        impl #impl_generics PartialData
            for #builder_ident < #source_generic_args >
        #where_clause
        {
            type Target = #context_ident #context_generics;
        }
    })?;

    Ok(item_impl)
}
