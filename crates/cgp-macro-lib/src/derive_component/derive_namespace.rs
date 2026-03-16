use quote::{ToTokens, quote};
use syn::{Ident, ItemImpl, Type, parse2};

use crate::attributes::UseNamespaceAttribute;

pub fn derive_namespace_impls(
    attributes: &[UseNamespaceAttribute],
    component_name: &Ident,
) -> syn::Result<Vec<ItemImpl>> {
    let mut out = Vec::new();

    for attribute in attributes {
        out.push(derive_namespace_impl(attribute, component_name)?);
    }

    Ok(out)
}

pub fn derive_namespace_impl(
    attribute: &UseNamespaceAttribute,
    component_name: &Ident,
) -> syn::Result<ItemImpl> {
    let namespace = &attribute.namespace;
    let mut paths = Vec::from_iter(attribute.path.iter().map(|path| path.path_type.clone()));
    paths.push(parse2(component_name.to_token_stream())?);

    let path = path_to_product(&paths)?;

    let out = quote! {
        impl<__Components__> #namespace < __Components__ > for #component_name
        {
            type Provider = RedirectLookup< __Components__, #path >;
        }
    };

    parse2(out)
}

pub fn path_to_product(paths: &[Type]) -> syn::Result<Type> {
    let mut out = quote! { PathNil };

    for path in paths.iter().rev() {
        out = quote! {
            PathCons< #path , #out >
        };
    }

    parse2(out)
}
