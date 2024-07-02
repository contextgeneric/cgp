use syn::punctuated::Punctuated;
use syn::token::Plus;
use syn::{parse_quote, Ident, TypeParamBound};

use crate::delegate_components::ast::{ComponentAst, DelegateComponentsAst, DelegateEntriesAst};

pub fn define_delegates_to_trait(delegate_entries: &DelegateEntriesAst) {
    let mut trait_bounds: Punctuated<TypeParamBound, Plus> = Punctuated::new();

    for component in delegate_entries.all_components() {
        let component_type = &component.component_type;
        let trait_bound: TypeParamBound = parse_quote!(
            DelegateComponent<#component_type, Delegate = >
        );
    }
}
