use syn::{parse_quote, Generics, Ident, ItemImpl, Path, Type};

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

    let trait_path: Path = parse_quote!(#trait_name);

    // FIXME: The preset generic would be absent if the if it is used as part of the
    // component name's generic.
    // let generics = merge_generics(preset_generics, &component.component_generics);
    let generics = component.component_generics.clone();

    ItemImpl {
        attrs: Vec::new(),
        defaultness: None,
        unsafety: None,
        impl_token: Default::default(),
        generics,
        trait_: Some((None, trait_path, Default::default())),
        self_ty: Box::new(component_type.clone()),
        brace_token: Default::default(),
        items: vec![],
    }
}
