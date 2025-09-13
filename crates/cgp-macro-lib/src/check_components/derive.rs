use quote::quote;
use syn::{parse2, ItemImpl, ItemTrait, Type};

use crate::check_components::override_span;
use crate::parse::{CheckComponents, CheckEntry};

pub fn derive_check_components(spec: &CheckComponents) -> syn::Result<(ItemTrait, Vec<ItemImpl>)> {
    let mut item_impls = Vec::new();
    let unit: Type = parse2(quote!(()))?;

    let context_type = &spec.context_type;
    let trait_name = &spec.trait_name;
    let impl_generics = &spec.impl_generics;
    let where_clause = &spec.where_clause;

    let item_trait = parse2(quote! {
        trait #trait_name <__Component__, __Params__>: CanUseComponent<__Component__, __Params__> {}
    })?;

    for CheckEntry {
        component_type,
        component_params,
        span,
    } in spec.check_entries.entries.iter()
    {
        // Override the span of the context type so that any unsatisfied constraint
        // error is highlighted on the component type instead
        let context_type = override_span(span, context_type)?;

        let component_param = component_params.as_ref().unwrap_or(&unit);

        let item_impl: ItemImpl = parse2(quote! {
            impl #impl_generics
                #trait_name < #component_type, #component_param >
                for #context_type
            #where_clause
            {}
        })?;

        item_impls.push(item_impl);
    }

    Ok((item_trait, item_impls))
}
