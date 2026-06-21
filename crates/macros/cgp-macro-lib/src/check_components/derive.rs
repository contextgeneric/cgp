use cgp_macro_core::functions::merge_generics;
use cgp_macro_core::types::check_components::{
    CheckComponentsTable, EvaluatedCheckEntry, TypeWithGenerics,
};
use quote::quote;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{ItemImpl, ItemTrait, Type, parse2};

use crate::check_components::override_span;

pub fn derive_check_components(
    spec: &CheckComponentsTable,
) -> syn::Result<(ItemTrait, Vec<ItemImpl>)> {
    if let Some(check_providers) = &spec.check_providers {
        return derive_check_provider(spec, check_providers);
    }

    let mut item_impls = Vec::new();
    let unit: Type = parse2(quote!(()))?;

    let context_type = &spec.context_type;
    let trait_name = &spec.trait_name;
    let impl_generics = &spec.impl_generics;
    let where_clause = &spec.where_clause;

    let item_trait = parse2(quote! {
        trait #trait_name <__Component__, __Params__: ?Sized>: CanUseComponent<__Component__, __Params__> {}
    })?;

    for EvaluatedCheckEntry {
        key: component_type,
        value: component_params,
        span,
    } in spec.check_entries.eval()
    {
        // Override the span of the context type so that any unsatisfied constraint
        // error is highlighted on the component type instead
        let context_type = override_span(&span, context_type)?;

        let TypeWithGenerics {
            ty: component_param,
            generics: check_generics,
        } = component_params.unwrap_or_else(|| unit.clone().into());

        let generics = merge_generics(&check_generics.generics, &impl_generics.generics);

        let impl_generics = generics.split_for_impl().0;

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

pub fn derive_check_provider(
    spec: &CheckComponentsTable,
    providers: &Punctuated<Type, Comma>,
) -> syn::Result<(ItemTrait, Vec<ItemImpl>)> {
    let mut item_impls = Vec::new();
    let unit: Type = parse2(quote!(()))?;

    let context_type = &spec.context_type;
    let trait_name = &spec.trait_name;
    let impl_generics = &spec.impl_generics;
    let where_clause = &spec.where_clause;

    let item_trait = parse2(quote! {
        trait #trait_name <__Component__, __Params__: ?Sized>: IsProviderFor<__Component__, #context_type, __Params__> {}
    })?;

    for EvaluatedCheckEntry {
        key: component_type,
        value: component_params,
        ..
    } in spec.check_entries.eval()
    {
        let TypeWithGenerics {
            ty: component_param,
            generics: check_generics,
        } = component_params.unwrap_or_else(|| unit.clone().into());

        let generics = merge_generics(&check_generics.generics, &impl_generics.generics);
        let impl_generics = generics.split_for_impl().0;

        for provider in providers {
            let item_impl: ItemImpl = parse2(quote! {
                impl #impl_generics
                    #trait_name < #component_type, #component_param >
                    for #provider
                #where_clause
                {}
            })?;

            item_impls.push(item_impl);
        }
    }

    Ok((item_trait, item_impls))
}
