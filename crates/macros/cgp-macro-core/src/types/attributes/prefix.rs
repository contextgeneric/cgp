use syn::ItemImpl;
use syn::parse::{Parse, ParseStream};
use syn::token::In;

use crate::exports::RedirectLookup;
use crate::functions::parse_internal;
use crate::types::ident::{IdentWithTypeGenerics, PathWithTypeArgs};
use crate::types::path::UniPath;

/// One `#[prefix]` registration.
#[derive(Clone)]
pub struct PrefixAttribute {
    /// The path the component registers under.
    pub path: UniPath,
    /// The `in` keyword.
    pub _in_token: In,
    /// The namespace the component registers into.
    pub namespace: PathWithTypeArgs,
}

impl PrefixAttribute {
    /// Emits the namespace impl that redirects the component's lookup.
    pub fn to_namespace_impl(
        &self,
        component_name: &IdentWithTypeGenerics,
    ) -> syn::Result<ItemImpl> {
        let mut namespace = self.namespace.clone();
        namespace
            .type_args
            .args
            .push(parse_internal!(__Components__));

        let mut path = self.path.clone();
        path.append_type(parse_internal!(#component_name));

        let mut type_generics = component_name.type_generics.clone();
        // Insert the components table as the leading generic. Position 0 sits
        // ahead of any lifetime, which the `parse_internal! { impl #type_generics
        // … }` re-parse below normalizes (`syn` re-emits lifetimes first). See
        // cgp-knowledge-base/cgp/implementation/README.md, "Generic-parameter insertion and
        // lifetime ordering".
        type_generics
            .params
            .insert(0, parse_internal!(__Components__));

        let item_impl = parse_internal! {
            impl #type_generics #namespace for #component_name
            {
                type Delegate = #RedirectLookup< __Components__, #path >;
            }
        };

        Ok(item_impl)
    }
}

impl Parse for PrefixAttribute {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let path = input.parse()?;
        let in_token = input.parse()?;
        let namespace = input.parse()?;

        Ok(PrefixAttribute {
            namespace,
            _in_token: in_token,
            path,
        })
    }
}
