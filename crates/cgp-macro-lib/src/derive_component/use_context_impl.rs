use quote::quote;
use syn::spanned::Spanned;
use syn::token::{Brace, For, Impl};
use syn::{parse2, Error, GenericParam, Ident, ImplItem, ItemImpl, ItemTrait, Path, TraitItem};

use crate::derive_component::delegate_fn::derive_delegated_fn_impl;
use crate::derive_component::delegate_type::derive_delegate_type_impl;
use crate::parse::TypeGenerics;

pub fn derive_use_context_impl(
    context_type: &Ident,
    consumer_trait: &ItemTrait,
    provider_trait: &ItemTrait,
) -> syn::Result<ItemImpl> {
    let consumer_trait_ident = &consumer_trait.ident;
    let provider_trait_ident = &provider_trait.ident;

    let provider_generics = TypeGenerics::try_from(&provider_trait.generics)?.generics;

    let consumer_generics = TypeGenerics::try_from(&consumer_trait.generics)?.generics;

    let mut impl_generics = provider_trait.generics.clone();

    let where_clause = impl_generics.make_where_clause();

    where_clause.predicates.push(parse2(quote! {
        #context_type : #consumer_trait_ident #consumer_generics
    })?);

    let mut impl_items: Vec<ImplItem> = Vec::new();

    for trait_item in provider_trait.items.iter() {
        match &trait_item {
            TraitItem::Fn(trait_fn) => {
                let impl_fn = derive_delegated_fn_impl(&trait_fn.sig, &quote!( #context_type ))?;

                impl_items.push(ImplItem::Fn(impl_fn))
            }
            TraitItem::Type(trait_type) => {
                let type_name = &trait_type.ident;

                let type_generics = {
                    let mut type_generics = trait_type.generics.clone();
                    type_generics.where_clause = None;

                    for param in &mut type_generics.params {
                        if let GenericParam::Type(type_param) = param {
                            type_param.bounds.clear();
                        }
                    }

                    type_generics
                };

                let impl_type = derive_delegate_type_impl(
                    trait_type,
                    parse2(quote!(
                        #context_type :: #type_name #type_generics
                    ))?,
                );

                impl_items.push(ImplItem::Type(impl_type));
            }
            _ => {
                return Err(Error::new(
                    trait_item.span(),
                    format!("unsupported trait item: {trait_item:?}"),
                ))
            }
        }
    }

    let trait_path: Path = parse2(quote!( #provider_trait_ident #provider_generics ))?;

    let item = ItemImpl {
        attrs: provider_trait.attrs.clone(),
        defaultness: None,
        unsafety: provider_trait.unsafety,
        impl_token: Impl::default(),
        generics: impl_generics,
        trait_: Some((None, trait_path, For::default())),
        self_ty: Box::new(parse2(quote!(UseContext))?),
        brace_token: Brace::default(),
        items: impl_items,
    };

    Ok(item)
}
