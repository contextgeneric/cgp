use syn::{parse_quote, Generics, Ident, ItemImpl, Type};

use crate::delegate_components::ast::{ComponentAst, DelegateEntriesAst};

pub fn impl_components_is_preset(
    trait_name: &Ident,
    preset_type: &Type,
    preset_generics: &Generics,
    delegate_entries: &DelegateEntriesAst,
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
    _preset_generics: &Generics,
    component: &ComponentAst,
) -> ItemImpl {
    let component_type = &component.component_type;

    // FIXME: The preset generic would be absent if the if it is used as part of the
    // component name's generic.
    // let generics = merge_generics(preset_generics, &component.component_generics);

    let mut generics = component.component_generics.clone();
    generics.params.push(parse_quote!(T));

    let impl_generics = generics.split_for_impl().0;

    parse_quote! {
        impl #impl_generics #trait_name < #component_type > for T {}
    }
}
