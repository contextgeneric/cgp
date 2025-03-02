use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse2, ItemImpl, Type};

use crate::parse::{CheckEntries, CheckEntry};

pub fn derive_check_components(
    context_type: &Type,
    check_entries: &CheckEntries,
) -> syn::Result<Vec<ItemImpl>> {
    let mut item_impls = Vec::new();
    let unit: Type = parse2(quote!(()))?;

    for CheckEntry {
        component_type,
        component_params,
        span,
    } in check_entries.entries.iter()
    {
        // Override the span of the context type so that any unsatisfied constraint
        // error is highlighted on the component type instead
        let context_type: TokenStream = context_type
            .to_token_stream()
            .into_iter()
            .map(|mut tree| {
                tree.set_span(span.clone());
                tree
            })
            .collect();

        let component_param = component_params.as_ref().unwrap_or(&unit);

        let item_impl: ItemImpl = parse2(quote! {
            impl CheckCanUseComponent< #component_type, #component_param >
                for #context_type
            {}
        })?;

        item_impls.push(item_impl);
    }

    Ok(item_impls)
}
