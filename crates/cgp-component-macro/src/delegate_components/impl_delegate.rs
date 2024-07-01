use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{
    parse_quote, Generics, ImplItem, ImplItemType, ItemImpl, Path, Type, WhereClause,
    WherePredicate,
};

use crate::delegate_components::ast::{ComponentAst, DelegateComponentsAst};

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
    target_generics: &Generics,
    component: &ComponentAst,
    source: &Type,
) -> ItemImpl {
    let component_type = &component.component_type;

    let trait_path: Path = parse_quote!(DelegateComponent < #component_type >);

    let delegate_type: ImplItemType = parse_quote!(type Delegate = #source;);

    let generics = merge_generics(target_generics, &component.component_generics);

    ItemImpl {
        attrs: Vec::new(),
        defaultness: None,
        unsafety: None,
        impl_token: Default::default(),
        generics,
        trait_: Some((None, trait_path, Default::default())),
        self_ty: Box::new(target_type.clone()),
        brace_token: Default::default(),
        items: vec![ImplItem::Type(delegate_type)],
    }
}

pub fn merge_generics(generics_a: &Generics, generics_b: &Generics) -> Generics {
    let mut params = generics_a.params.clone();
    params.extend(generics_b.params.clone());

    let mut predicates: Punctuated<WherePredicate, Comma> = Default::default();

    if let Some(where_clause) = &generics_a.where_clause {
        predicates.extend(where_clause.predicates.clone());
    }

    if let Some(where_clause) = &generics_b.where_clause {
        predicates.extend(where_clause.predicates.clone());
    }

    let where_clause = if predicates.is_empty() {
        None
    } else {
        Some(WhereClause {
            where_token: Default::default(),
            predicates,
        })
    };

    Generics {
        lt_token: generics_a.lt_token.clone(),
        params,
        gt_token: generics_a.gt_token.clone(),
        where_clause,
    }
}
