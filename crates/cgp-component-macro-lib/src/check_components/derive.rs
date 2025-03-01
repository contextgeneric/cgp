use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{parse2, ItemImpl, Type};

use crate::parse::CheckEntries;

pub fn derive_check_components(
    context_type: &Type,
    check_entries: &CheckEntries,
) -> syn::Result<Vec<ItemImpl>> {
    let mut item_impls = Vec::new();
    let unit: Type = parse2(quote!(()))?;

    for (component_type, component_param) in check_entries.entries.iter() {
        let component_span = component_type.span();

        let span = if component_param.is_some() {
            component_span
                .join(component_param.span())
                .unwrap_or(component_span)
        } else {
            component_span
        };

        let component_param = component_param.as_ref().unwrap_or(&unit);

        let item_impl: ItemImpl = parse2(quote_spanned! {
            span =>
            impl CheckCanUseComponent< #component_type, #component_param >
                for #context_type
            {}
        })?;

        item_impls.push(item_impl);
    }

    Ok(item_impls)
}
