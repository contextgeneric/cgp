use alloc::boxed::Box;
use alloc::vec;
use alloc::vec::Vec;

use syn::{parse_quote, Generics, ImplItem, ImplItemType, ItemImpl, Path, Type};

use crate::delegate_components::ast::{ComponentAst, DelegateEntriesAst};
use crate::delegate_components::merge_generics::merge_generics;

pub fn impl_delegate_components(
    target_type: &Type,
    target_generics: &Generics,
    delegate_entries: &DelegateEntriesAst,
) -> Vec<ItemImpl> {
    delegate_entries
        .entries
        .iter()
        .flat_map(|entry| {
            let source = &entry.source;

            entry.components.iter().flat_map(|component| {
                impl_delegate_component(target_type, target_generics, component, source)
            })
        })
        .collect()
}

pub fn impl_delegate_component(
    target_type: &Type,
    target_generics: &Generics,
    component: &ComponentAst,
    source: &Type,
) -> Vec<ItemImpl> {
    let component_type = &component.component_type;

    let delegate_trait_path: Path = parse_quote!(DelegateComponent < #component_type >);

    let delegate_type: ImplItemType = parse_quote!(type Delegate = #source;);

    let delegate_generics = merge_generics(target_generics, &component.component_generics);

    let is_provider_generics = {
        let mut generics = delegate_generics.clone();

        generics.params.push(parse_quote!(__Context__));
        generics.params.push(parse_quote!(__Params__));

        let where_clause = generics.make_where_clause();
        where_clause.predicates.push(
            parse_quote!( #source : IsProviderFor< #component_type, __Context__, __Params__ > ),
        );

        generics
    };

    let delegate_impl = ItemImpl {
        attrs: Vec::new(),
        defaultness: None,
        unsafety: None,
        impl_token: Default::default(),
        generics: delegate_generics,
        trait_: Some((None, delegate_trait_path, Default::default())),
        self_ty: Box::new(target_type.clone()),
        brace_token: Default::default(),
        items: vec![ImplItem::Type(delegate_type)],
    };

    let is_provider_trait_path: Path =
        parse_quote!( IsProviderFor< #component_type, __Context__, __Params__ > );

    let is_provider_impl = ItemImpl {
        attrs: Vec::new(),
        defaultness: None,
        unsafety: None,
        impl_token: Default::default(),
        generics: is_provider_generics,
        trait_: Some((None, is_provider_trait_path, Default::default())),
        self_ty: Box::new(target_type.clone()),
        brace_token: Default::default(),
        items: Default::default(),
    };

    vec![delegate_impl, is_provider_impl]
}
