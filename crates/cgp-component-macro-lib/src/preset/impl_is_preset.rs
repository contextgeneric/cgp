use alloc::vec::Vec;

use syn::{parse_quote, Ident, ItemImpl, Type};

use crate::parse::{DelegateComponentEntries, DelegateComponentName, ImplGenerics};

pub fn impl_components_is_preset(
    trait_name: &Ident,
    preset_type: &Type,
    preset_generics: &ImplGenerics,
    delegate_entries: &DelegateComponentEntries,
) -> Vec<ItemImpl> {
    delegate_entries
        .entries
        .iter()
        .flat_map(|entry| {
            entry.components.iter().map(|component| {
                impl_component_is_preset(trait_name, preset_type, preset_generics, component)
            })
        })
        .collect()
}

pub fn impl_component_is_preset(
    trait_name: &Ident,
    _preset_type: &Type,
    _preset_generics: &ImplGenerics,
    component: &DelegateComponentName,
) -> ItemImpl {
    let component_type = &component.component_type;

    // FIXME: The preset generic would be absent if the if it is used as part of the
    // component name's generic.
    // let generics = merge_generics(preset_generics, &component.component_generics);

    let mut generics = component.component_generics.generics.clone();
    generics.params.push(parse_quote!(T));

    let impl_generics = generics.split_for_impl().0;

    parse_quote! {
        impl #impl_generics #trait_name < #component_type > for T {}
    }
}
