use quote::quote;
use syn::{Ident, ItemImpl, parse_quote, parse2};

use crate::attributes::PrefixAttribute;

pub fn derive_namespace_impls(
    attributes: &[PrefixAttribute],
    component_name: &Ident,
) -> syn::Result<Vec<ItemImpl>> {
    let mut out = Vec::new();

    for attribute in attributes {
        out.push(derive_namespace_impl(attribute, component_name)?);
    }

    Ok(out)
}

pub fn derive_namespace_impl(
    attribute: &PrefixAttribute,
    component_name: &Ident,
) -> syn::Result<ItemImpl> {
    let namespace = &attribute.namespace;
    let mut path = attribute.path.clone();
    path.append_type(parse_quote!(#component_name));

    let out = quote! {
        impl<__Components__> #namespace < __Components__ > for #component_name
        {
            type Delegate = RedirectLookup< __Components__, #path >;
        }
    };

    parse2(out)
}
