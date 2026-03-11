use quote::quote;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{GenericParam, Generics, Ident, ItemImpl, ItemTrait, Type, parse2};

pub fn derive_redirect_lookup_impl(
    context_type: &Ident,
    consumer_trait: &ItemTrait,
    provider_trait: &ItemTrait,
) -> syn::Result<ItemImpl> {
    let generic_params = extract_type_generics(&provider_trait.generics)?;

    let mut impl_generics = provider_trait.generics.clone();

    impl_generics
        .params
        .push(parse2(quote! { __Components__ })?);
    impl_generics.params.push(parse2(quote! { __Path__ })?);

    if let Some(generic_params) = generic_params {
        let where_clause = impl_generics.make_where_clause();
        where_clause.predicates.push(parse2(quote! {
            __Path__: AppendProduct< ( #generic_params ) >
        })?);

        where_clause.predicates.push(parse2(quote! {
            __Components__: DelegateComponent<<__Path__ as AppendProduct< ( #generic_params ) >>::Output>,
        })?);
    } else {
        let where_clause = impl_generics.make_where_clause();
        where_clause.predicates.push(parse2(quote! {
            __Components__: DelegateComponent<__Path__>,
        })?);
    }

    todo!()
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
