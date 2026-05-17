use quote::quote;
use syn::token::{Brace, For, Impl};
use syn::{GenericParam, Generics, ItemImpl, ItemTrait, Path, Type, parse2};

use crate::derive_component::provider_impl::derive_provider_item_impls;

pub fn derive_redirect_lookup_impl(
    consumer_trait: &ItemTrait,
    provider_trait: &ItemTrait,
) -> syn::Result<ItemImpl> {
    let provider_name = &provider_trait.ident;
    let provider_type_generics = provider_trait.generics.split_for_impl().1;

    let generic_params = extract_type_generics(&consumer_trait.generics)?;

    let mut impl_generics = provider_trait.generics.clone();

    impl_generics
        .params
        .push(parse2(quote! { __Components__ })?);

    impl_generics.params.push(parse2(quote! { __Path__ })?);

    let where_clause = impl_generics.make_where_clause();

    let delegate_constraint = if let Some(generic_params) = &generic_params {
        where_clause.predicates.push(parse2(quote! {
            __Path__: ConcatPath< #generic_params >
        })?);

        quote! {
            DelegateComponent<<__Path__ as ConcatPath< #generic_params >>::Output>
        }
    } else {
        quote! {
            DelegateComponent<__Path__>
        }
    };

    where_clause.predicates.push(parse2(quote! {
        __Components__: #delegate_constraint
    })?);

    let delegate_type = quote! {
        < __Components__ as #delegate_constraint > :: Delegate
    };

    where_clause.predicates.push(parse2(quote! {
        #delegate_type : #provider_name #provider_type_generics
    })?);

    let impl_items = derive_provider_item_impls(provider_trait, &delegate_type)?;

    let self_type = parse2(quote!(RedirectLookup<__Components__, __Path__>))?;

    let trait_path: Path = parse2(quote!( #provider_name #provider_type_generics ))?;

    let item = ItemImpl {
        attrs: provider_trait.attrs.clone(),
        defaultness: None,
        unsafety: provider_trait.unsafety,
        impl_token: Impl::default(),
        generics: impl_generics,
        trait_: Some((None, trait_path, For::default())),
        self_ty: Box::new(self_type),
        brace_token: Brace::default(),
        items: impl_items,
    };

    Ok(item)
}

pub fn extract_type_generics(generics: &Generics) -> syn::Result<Option<Type>> {
    let type_params = generics
        .params
        .iter()
        .filter_map(|param| {
            if let GenericParam::Type(type_param) = param {
                Some(type_param.ident.clone())
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    if type_params.is_empty() {
        Ok(None)
    } else {
        let mut out = quote! { Nil };

        for param in type_params.iter().rev() {
            out = quote! {
                PathCons< #param , #out >
            };
        }

        Ok(Some(parse2(out)?))
    }
}
