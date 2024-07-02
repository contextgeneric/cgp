use syn::punctuated::Punctuated;
use syn::token::Plus;
use syn::{parse_quote, Ident, TypeParamBound};

use crate::delegate_components::ast::{ComponentAst, DelegateComponentsAst};

pub fn define_delegates_to_trait(ast: &DelegateComponentsAst) {
    let mut trait_bounds: Punctuated<TypeParamBound, Plus> = Punctuated::new();

    for component in ast.delegate_entries.all_components() {
        let component_type = &component.component_type;
        // let trait_bound = parse_quote!(
        //     DelegateComponent<#component_type >
        // );
    }
}
