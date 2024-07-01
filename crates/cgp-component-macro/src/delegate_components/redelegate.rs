use syn::{parse_quote, ImplItem, ImplItemType, ItemImpl, Path, Type};

use crate::delegate_components::ast::{ComponentAst, DelegateComponentsAst};

pub fn impl_redelegate_components(ast: &DelegateComponentsAst) -> Vec<ItemImpl> {
    let target_type = &ast.target_type;

    ast.delegate_entries
        .iter()
        .flat_map(|entry| {
            entry
                .components
                .iter()
                .map(|component| impl_redelegate_component(target_type, component))
        })
        .collect()
}

pub fn impl_redelegate_component(target_type: &Type, component: &ComponentAst) -> ItemImpl {
    let component_type = &component.component_type;

    let trait_path: Path = parse_quote!(DelegateComponent < #component_type >);

    let delegate_type: ImplItemType = parse_quote!( type Delegate = #target_type; );

    let generics = component.component_generics.clone();

    let self_type = parse_quote!( $target );

    ItemImpl {
        attrs: Vec::new(),
        defaultness: None,
        unsafety: None,
        impl_token: Default::default(),
        generics,
        trait_: Some((None, trait_path, Default::default())),
        self_ty: Box::new(self_type),
        brace_token: Default::default(),
        items: vec![ImplItem::Type(delegate_type)],
    }
}
