use alloc::vec::Vec;

use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{Ident, ItemImpl, Type, parse_quote};

use crate::parse::{DelegateEntry, DelegateKey, ImplGenerics, SimpleType};

pub fn impl_components_is_preset(
    trait_name: &Ident,
    preset_type: &Type,
    preset_generics: &ImplGenerics,
    delegate_entries: &Punctuated<DelegateEntry<SimpleType>, Comma>,
) -> Vec<ItemImpl> {
    delegate_entries
        .iter()
        .flat_map(|entry| {
            entry.keys.iter().map(|component| {
                impl_component_is_preset(trait_name, preset_type, preset_generics, component)
            })
        })
        .collect()
}

pub fn impl_component_is_preset(
    trait_name: &Ident,
    _preset_type: &Type,
    _preset_generics: &ImplGenerics,
    component: &DelegateKey<SimpleType>,
) -> ItemImpl {
    let component_type = &component.ty;

    let mut generics = component.generics.generics.clone();
    generics.params.push(parse_quote!(__Self__));

    let impl_generics = generics.split_for_impl().0;

    parse_quote! {
        impl #impl_generics #trait_name < #component_type > for __Self__ {}
    }
}
