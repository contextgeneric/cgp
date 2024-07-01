use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{
    parse_quote, GenericParam, Generics, Ident, ImplItem, ImplItemType, ItemImpl, Path, Type,
};

use crate::delegate_components::ast::DelegateComponentsAst;

pub fn impl_delegate_components(ast: &DelegateComponentsAst) -> Vec<ItemImpl> {
    let target_type = &ast.target_type;
    let target_generics = &ast.target_generics;

    ast.delegate_entries
        .iter()
        .flat_map(|entry| {
            let source = &entry.source;

            entry.components.iter().map(|component| {
                impl_delegate_component(target_type, target_generics, component, source)
            })
        })
        .collect()
}

pub fn impl_delegate_component(
    target_type: &Type,
    target_generic_params: &Punctuated<GenericParam, Comma>,
    component: &Type,
    source: &Type,
) -> ItemImpl {
    let trait_path: Path = parse_quote!(DelegateComponent < #component >);

    let target_generics = parse_quote!(< #target_generic_params >);

    let delegate_type: ImplItemType = parse_quote!(type Delegate = #source;);

    ItemImpl {
        attrs: Vec::new(),
        defaultness: None,
        unsafety: None,
        impl_token: Default::default(),
        generics: target_generics,
        trait_: Some((None, trait_path, Default::default())),
        self_ty: Box::new(target_type.clone()),
        brace_token: Default::default(),
        items: vec![ImplItem::Type(delegate_type)],
    }
}
