use proc_macro2::{Span, TokenStream};
use quote::{ToTokens, quote};
use syn::{Ident, parse2};

use crate::delegate_components::{
    DelegateNamespaceAttribute, define_struct, impl_delegate_components, parse_delegate_attributes,
};
use crate::parse::{DelegateComponents, SimpleType, TypeGenerics};

pub fn delegate_components(body: TokenStream) -> syn::Result<TokenStream> {
    let spec: DelegateComponents = parse2(body)?;

    let target_type = &spec.target_type;
    let target_generics = &spec.target_generics;

    let mut output = TokenStream::new();

    if spec.new_struct {
        let target_type: SimpleType<TypeGenerics> = parse2(target_type.to_token_stream())?;

        let type_generics = target_type.generics.unwrap_or_default().generics;

        let component_struct = define_struct(&target_type.name, &type_generics)?;

        output.extend(component_struct.to_token_stream());
    }

    let attributes = parse_delegate_attributes(spec.attributes)?;

    if let Some(DelegateNamespaceAttribute { namespace }) = attributes.use_namespace {
        let namespace =
            namespace.unwrap_or_else(|| Ident::new("DefaultNamespace", Span::call_site()));

        let mut generics = target_generics.generics.clone();
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

        output.extend(namespace_impl);

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

        output.extend(is_provider_for_impl);
    }

    let impl_items = impl_delegate_components(&target_type, &target_generics, &spec.entries)?;

    output.extend(impl_items);

    Ok(output)
}
