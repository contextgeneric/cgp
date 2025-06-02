use proc_macro2::Span;
use quote::quote;
use syn::spanned::Spanned;
use syn::token::{Brace, Eq, For, Impl};
use syn::{
    parse2, Error, Ident, ImplItem, ImplItemConst, ItemImpl, ItemTrait, Path, TraitItem, Visibility,
};

use crate::derive_component::delegate_fn::derive_delegated_fn_impl;
use crate::derive_component::delegate_type::derive_delegate_type_impl;
use crate::parse::DeriveDelegateSpec;

pub fn derive_delegate_impl(
    provider_trait: &ItemTrait,
    spec: &DeriveDelegateSpec,
) -> syn::Result<ItemImpl> {
    let provider_trait_ident = &provider_trait.ident;

    let components_ident = Ident::new("__Components__", Span::call_site());
    let delegate_ident = Ident::new("__Delegate__", Span::call_site());

    let wrapper_ident = &spec.wrapper;
    let use_delegate_params = &spec.params;

    let generics = {
        let mut generics = provider_trait.generics.clone();

        generics.params.push(parse2(quote!( #components_ident ))?);
        generics.params.push(parse2(quote!( #delegate_ident ))?);

        let where_clause = generics.make_where_clause();

        where_clause.predicates.push(parse2(quote! {
            #components_ident: DelegateComponent<
                ( #use_delegate_params ),
                Delegate = #delegate_ident,
            >
        })?);

        let type_generics = provider_trait.generics.split_for_impl().1;

        where_clause.predicates.push(parse2(quote! {
            #delegate_ident : #provider_trait_ident #type_generics
        })?);

        generics
    };

    let (_, type_generics, _) = provider_trait.generics.split_for_impl();

    let trait_path: Path = parse2(quote!( #provider_trait_ident #type_generics ))?;

    let mut impl_items: Vec<ImplItem> = Vec::new();

    for trait_item in provider_trait.items.iter() {
        match trait_item {
            TraitItem::Fn(trait_fn) => {
                let impl_fn = derive_delegated_fn_impl(&trait_fn.sig, &quote!( #delegate_ident ))?;

                impl_items.push(ImplItem::Fn(impl_fn))
            }
            TraitItem::Type(trait_type) => {
                let type_name = &trait_type.ident;

                let type_generics = trait_type.generics.split_for_impl().1;

                let impl_type = derive_delegate_type_impl(
                    trait_type,
                    parse2(quote!(
                        #delegate_ident :: #type_name #type_generics
                    ))?,
                );

                impl_items.push(ImplItem::Type(impl_type));
            }
            TraitItem::Const(trait_item_const) => {
                let const_ident = &trait_item_const.ident;
                let (_, type_generics, _) = trait_item_const.generics.split_for_impl();

                let impl_expr = parse2(quote! {
                    #delegate_ident :: #const_ident #type_generics
                })?;

                let impl_item_const = ImplItemConst {
                    attrs: trait_item_const.attrs.clone(),
                    vis: Visibility::Inherited,
                    defaultness: None,
                    const_token: trait_item_const.const_token,
                    ident: trait_item_const.ident.clone(),
                    generics: trait_item_const.generics.clone(),
                    colon_token: trait_item_const.colon_token,
                    ty: trait_item_const.ty.clone(),
                    eq_token: Eq(Span::call_site()),
                    expr: impl_expr,
                    semi_token: trait_item_const.semi_token,
                };

                impl_items.push(ImplItem::Const(impl_item_const));
            }
            _ => {
                return Err(Error::new(
                    trait_item.span(),
                    format!("unsupported trait item: {trait_item:?}"),
                ));
            }
        }
    }

    let provider_type = parse2(quote!(#wrapper_ident < #components_ident >))?;

    let item = ItemImpl {
        attrs: provider_trait.attrs.clone(),
        defaultness: None,
        unsafety: provider_trait.unsafety,
        impl_token: Impl::default(),
        generics,
        trait_: Some((None, trait_path, For::default())),
        self_ty: Box::new(provider_type),
        brace_token: Brace::default(),
        items: impl_items,
    };

    Ok(item)
}
