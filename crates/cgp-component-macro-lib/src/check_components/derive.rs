use proc_macro2::TokenStream;
use quote::{quote, quote_spanned, ToTokens};
use syn::spanned::Spanned;
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
    } in check_entries.entries.iter()
    {
        let component_span = component_type.span();

        let span = if component_params.is_some() {
            component_span
                .join(component_params.span())
                .unwrap_or(component_span)
        } else {
            component_span
        };

        // Override the span of the context type so that any unsatisfied constraint
        // error is highlighted on the component type instead
        let context_type: TokenStream = context_type
            .to_token_stream()
            .into_iter()
            .map(|mut tree| {
                tree.set_span(span);
                tree
            })
            .collect();

        let component_param = component_params.as_ref().unwrap_or(&unit);

        let item_impl: ItemImpl = parse2(quote_spanned! {
            span=>
            impl CheckCanUseComponent< #component_type, #component_param >
                for #context_type
            {}
        })?;

        item_impls.push(item_impl);
    }

    Ok(item_impls)
}
