use syn::{parse_quote, Ident, ItemImpl, Path};

use crate::delegate_components::ast::{ComponentAst, DelegateEntriesAst};

pub fn impl_components_is_preset(
    preset_name: &Ident,
    delegate_entries: &DelegateEntriesAst,
) -> Vec<ItemImpl> {
    delegate_entries
        .entries
        .iter()
        .flat_map(|entry| {
            entry
                .components
                .iter()
                .map(|component| impl_component_is_preset(preset_name, component))
        })
        .collect()
}

pub fn impl_component_is_preset(preset_name: &Ident, component: &ComponentAst) -> ItemImpl {
    let component_type = &component.component_type;

    let trait_path: Path = parse_quote!(#preset_name);

    ItemImpl {
        attrs: Vec::new(),
        defaultness: None,
        unsafety: None,
        impl_token: Default::default(),
        generics: component.component_generics.clone(),
        trait_: Some((None, trait_path, Default::default())),
        self_ty: Box::new(component_type.clone()),
        brace_token: Default::default(),
        items: vec![],
    }
}
