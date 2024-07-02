use syn::punctuated::Punctuated;
use syn::token::Plus;
use syn::{parse_quote, Generics, Ident, ItemImpl, ItemTrait, Type, TypeParamBound};

use crate::delegate_components::ast::DelegateEntriesAst;

pub fn define_delegate_component_trait_bounds(
    target_type: &Type,
    delegate_entries: &DelegateEntriesAst,
) -> Punctuated<TypeParamBound, Plus> {
    let mut trait_bounds: Punctuated<TypeParamBound, Plus> = Punctuated::new();

    for component in delegate_entries.all_components() {
        let component_type = &component.component_type;
        let trait_bound: TypeParamBound = parse_quote!(
            DelegateComponent<#component_type, Delegate = #target_type>
        );
        trait_bounds.push(trait_bound);
    }

    trait_bounds
}

pub fn define_delegates_to_trait(
    trait_name: &Ident,
    target_type: &Type,
    target_generics: &Generics,
    delegate_entries: &DelegateEntriesAst,
) -> (ItemTrait, ItemImpl) {
    let trait_bounds = define_delegate_component_trait_bounds(target_type, delegate_entries);

    let mut impl_generics = target_generics.clone();
    impl_generics.params.push(parse_quote!(Components));

    let item_trait = parse_quote! {
        pub trait #trait_name #target_generics: #trait_bounds {}
    };

    let item_impl = parse_quote! {
        impl #impl_generics #trait_name for Components
        where
            Components: #trait_bounds
        {}
    };

    (item_trait, item_impl)
}
