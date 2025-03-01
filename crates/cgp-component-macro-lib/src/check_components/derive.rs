use quote::quote;
use syn::{parse2, ItemImpl, Type};

use crate::parse::CheckEntries;

pub fn derive_check_components(
    context_type: &Type,
    check_entries: &CheckEntries,
) -> syn::Result<Vec<ItemImpl>> {
    let mut item_impls = Vec::new();

    for (component_type, component_param) in check_entries.entries.iter() {
        let item_impl: ItemImpl = parse2(quote! {
            impl CheckCanUseComponent< #component_type, #component_param >
                for #context_type
            {}
        })?;

        item_impls.push(item_impl);
    }

    Ok(item_impls)
}
