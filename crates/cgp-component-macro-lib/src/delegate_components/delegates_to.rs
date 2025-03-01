use quote::quote;
use syn::punctuated::Punctuated;
use syn::token::Plus;
use syn::{parse2, Ident, ItemImpl, ItemTrait, Type, TypeParamBound};

use crate::parse::{DelegateComponentEntries, ImplGenerics};

pub fn define_delegate_component_trait_bounds(
    target_type: &Type,
    delegate_entries: &DelegateComponentEntries,
) -> syn::Result<Punctuated<TypeParamBound, Plus>> {
    let mut trait_bounds: Punctuated<TypeParamBound, Plus> = Punctuated::new();

    for component in delegate_entries.all_components() {
        let component_type = &component.component_type;
        let trait_bound: TypeParamBound = parse2(quote!(
            DelegateComponent<#component_type, Delegate = #target_type>
        ))?;
        trait_bounds.push(trait_bound);
    }

    Ok(trait_bounds)
}

pub fn define_delegates_to_trait(
    trait_name: &Ident,
    target_type: &Type,
    target_generics: &ImplGenerics,
    delegate_entries: &DelegateComponentEntries,
) -> syn::Result<(ItemTrait, ItemImpl)> {
    let trait_bounds = define_delegate_component_trait_bounds(target_type, delegate_entries)?;

    let item_trait = parse2(quote! {
        #[doc(hidden)]
        pub trait #trait_name #target_generics: #trait_bounds {}
    })?;

    let mut impl_generics = target_generics.generics.clone();
    impl_generics.params.push(parse2(quote!(Components))?);

    let type_generics = target_generics.generics.split_for_impl().1;

    let item_impl = parse2(quote! {
        impl #impl_generics #trait_name #type_generics  for Components
        where
            Components: #trait_bounds
        {}
    })?;

    Ok((item_trait, item_impl))
}
