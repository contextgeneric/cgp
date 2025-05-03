use alloc::boxed::Box;
use alloc::vec;
use alloc::vec::Vec;

use quote::{quote, ToTokens};
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{parse2, ImplItem, ImplItemType, ItemImpl, Path, Type};

use crate::delegate_components::merge_generics::merge_generics;
use crate::parse::{DelegateComponentEntry, DelegateComponentName, ImplGenerics};

pub fn impl_delegate_components<T>(
    target_type: &Type,
    target_generics: &ImplGenerics,
    delegate_entries: &Punctuated<DelegateComponentEntry<T>, Comma>,
) -> syn::Result<Vec<ItemImpl>>
where
    T: ToTokens,
{
    let mut components = Vec::new();

    for entry in delegate_entries.iter() {
        let source = &entry.source;
        for component in entry.components.iter() {
            let mut impls =
                impl_delegate_component(target_type, target_generics, component, source)?;
            components.append(&mut impls);
        }
    }

    Ok(components)
}

pub fn impl_delegate_component<T>(
    target_type: &Type,
    target_generics: &ImplGenerics,
    component: &DelegateComponentName<T>,
    source: &Type,
) -> syn::Result<Vec<ItemImpl>>
where
    T: ToTokens,
{
    let component_type = &component.component_type;

    let delegate_trait_path: Path = parse2(quote!(DelegateComponent < #component_type >))?;

    let delegate_type: ImplItemType = parse2(quote!(type Delegate = #source;))?;

    let delegate_generics = merge_generics(
        &target_generics.generics,
        &component.component_generics.generics,
    );

    let is_provider_generics = {
        let mut generics = delegate_generics.clone();

        generics.params.push(parse2(quote!(__Context__))?);
        generics.params.push(parse2(quote!(__Params__))?);

        let where_clause = generics.make_where_clause();
        where_clause.predicates.push(parse2(
            quote!( #source : IsProviderFor< #component_type, __Context__, __Params__ > ),
        )?);

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
        parse2(quote!( IsProviderFor< #component_type, __Context__, __Params__ > ))?;

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

    Ok(vec![delegate_impl, is_provider_impl])
}
