use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, ItemImpl, ItemStruct, ItemTrait, parse2};

use crate::cgp_namespace::spec::NamespaceSpec;

pub fn cgp_namespace(body: TokenStream) -> syn::Result<TokenStream> {
    let spec: NamespaceSpec = parse2(body)?;

    let mut out = TokenStream::new();

    let namespace_ident = &spec.namespace_ident;

    let namespace_trait: ItemTrait = parse2(quote! {
        pub trait #namespace_ident< __Components__ > {
            type Provider;
        }
    })?;

    out.extend(quote! {
        #namespace_trait
    });

    if let Some(parent_namespace_ident) = spec.parent_namespace_ident {
        let namespace_struct_ident = Ident::new(
            &format!("__{}Components", namespace_ident),
            namespace_ident.span(),
        );

        let namespace_struct: ItemStruct = parse2(quote! {
            pub struct #namespace_struct_ident;
        })?;

        let item_impl: ItemImpl = parse2(quote! {
            impl<__Context__, __Components__, __Provider__>
                #namespace_ident< __Components__ >
                for __Context__
            where
                __Context__: #parent_namespace_ident< __Components__, Provider = __Provider__ >
                    + #parent_namespace_ident< #namespace_struct_ident >,
            {
                type Provider = __Provider__;
            }
        })?;

        out.extend(quote! {
            #namespace_struct

            #item_impl
        })
    }

    for entry in spec.entries.into_iter() {
        let value = entry.value;
        for path in entry.keys.paths.into_iter() {
            let path_type = path.path_type;

            let mut generics = path.generics.generics;
            generics.params.push(parse2(quote!(__Components__))?);

            let impl_generics = generics.split_for_impl().0;

            let item_impl: ItemImpl = parse2(quote! {
                impl #impl_generics
                    #namespace_ident< __Components__ >
                    for #path_type
                {
                    type Provider = RedirectLookup<
                        __Components__,
                        #value,
                    >;
                }
            })?;

            out.extend(quote! {
                #item_impl
            })
        }
    }

    Ok(out)
}
