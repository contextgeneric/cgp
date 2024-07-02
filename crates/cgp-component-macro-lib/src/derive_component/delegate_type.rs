use syn::token::Eq;
use syn::{ImplItemType, TraitItemType, Type, Visibility};

pub fn derive_delegate_type_impl(trait_type: &TraitItemType, delegated_type: Type) -> ImplItemType {
    ImplItemType {
        attrs: trait_type.attrs.clone(),
        vis: Visibility::Inherited,
        defaultness: None,
        type_token: trait_type.type_token,
        ident: trait_type.ident.clone(),
        generics: trait_type.generics.clone(),
        eq_token: Eq::default(),
        ty: delegated_type,
        semi_token: trait_type.semi_token,
    }
}
