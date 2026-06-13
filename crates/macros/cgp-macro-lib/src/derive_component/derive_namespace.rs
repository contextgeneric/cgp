use cgp_macro_core::exports::RedirectLookup;
use cgp_macro_core::types::attributes::PrefixAttribute;
use cgp_macro_core::types::ident::IdentWithTypeGenerics;
use quote::quote;
use syn::{ItemImpl, parse_quote, parse2};

pub fn derive_prefix_impls(
    attributes: &[PrefixAttribute],
    component_name: &IdentWithTypeGenerics,
) -> syn::Result<Vec<ItemImpl>> {
    let mut out = Vec::new();

    for attribute in attributes {
        out.push(derive_prefix_impl(attribute, component_name)?);
    }

    Ok(out)
}

pub fn derive_prefix_impl(
    attribute: &PrefixAttribute,
    component_name: &IdentWithTypeGenerics,
) -> syn::Result<ItemImpl> {
    let mut namespace = attribute.namespace.clone();
    namespace.type_args.make_args().push(parse_quote!(__Components__));

    let mut path = attribute.path.clone();
    path.append_type(parse_quote!(#component_name));

    let mut type_generics = component_name.type_generics.clone();
    type_generics.params.insert(0, parse_quote!(__Components__));

    let out = quote! {
        impl #type_generics #namespace for #component_name
        {
            type Delegate = #RedirectLookup< __Components__, #path >;
        }
    };

    parse2(out)
}
