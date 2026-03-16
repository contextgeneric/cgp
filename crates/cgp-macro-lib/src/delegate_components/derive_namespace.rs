use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Generics, Ident, Type, parse2};

pub fn derive_namespace_delegate(
    namespace: Option<Ident>,
    target_type: &Type,
    target_generics: &Generics,
) -> syn::Result<TokenStream> {
    let namespace = namespace.unwrap_or_else(|| Ident::new("DefaultNamespace", Span::call_site()));

    let mut generics = target_generics.clone();
    generics.params.push(parse2(quote! { __Component__ })?);

    let impl_generics = generics.split_for_impl().0;

    let namespace_impl = quote! {
        impl #impl_generics
            DelegateComponent<__Component__>
            for #target_type
        where
            __Component__: #namespace< #target_type >,
        {
            type Delegate = < __Component__ as #namespace< #target_type >>::Provider;
        }
    };

    let mut generics = generics.clone();
    generics.params.push(parse2(quote! { __Context__ })?);
    generics.params.push(parse2(quote! { __Params__ })?);

    let impl_generics = generics.split_for_impl().0;

    let is_provider_for_impl = quote! {
        impl #impl_generics
            IsProviderFor<__Component__, __Context__, __Params__>
            for #target_type
        where
            __Component__: #namespace< #target_type >,
            < __Component__ as #namespace< #target_type >>::Provider: IsProviderFor<__Component__, __Context__, __Params__>,
        {
        }
    };

    Ok(quote! {
        #namespace_impl
        #is_provider_for_impl
    })
}
