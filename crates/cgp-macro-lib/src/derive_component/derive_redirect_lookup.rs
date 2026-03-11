use quote::quote;
use syn::punctuated::Punctuated;
use syn::token::{Brace, Comma, For, Impl};
use syn::{GenericParam, Generics, Ident, ItemImpl, ItemTrait, Path, Type, parse2};

use crate::derive_component::provider_impl::derive_provider_item_impls;

pub fn derive_redirect_lookup_impl(
    context_type: &Ident,
    consumer_trait: &ItemTrait,
    provider_trait: &ItemTrait,
) -> syn::Result<ItemImpl> {
    let provider_name = &provider_trait.ident;
    let provider_type_generics = provider_trait.generics.split_for_impl().1;

    let generic_params = extract_type_generics(&provider_trait.generics)?;

    let mut impl_generics = provider_trait.generics.clone();

    impl_generics
        .params
        .push(parse2(quote! { __Components__ })?);

    impl_generics.params.push(parse2(quote! { __Path__ })?);

    let where_clause = impl_generics.make_where_clause();

    let delegate_constraint = if let Some(generic_params) = &generic_params {
        where_clause.predicates.push(parse2(quote! {
            __Path__: AppendProduct< ( #generic_params ) >
        })?);

        quote! {
            DelegateComponent<<__Path__ as AppendProduct< ( #generic_params ) >>::Output>
        }
    } else {
        quote! {
            DelegateComponent<__Path__>
        }
    };

    where_clause.predicates.push(parse2(quote! {
        __Components__: #delegate_constraint,
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
    let mut params = Punctuated::<Ident, Comma>::new();

    for param in generics.params.iter() {
        if let GenericParam::Type(type_param) = param {
            params.push(type_param.ident.clone());
        }
    }

    if params.is_empty() {
        Ok(None)
    } else {
        let params = parse2(quote! {
            ( #params )
        })?;

        Ok(Some(params))
    }
}
