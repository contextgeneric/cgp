use quote::{ToTokens, quote_spanned};
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{GenericParam, Generics, Ident, ItemStruct, Type, parse_quote};

use crate::exports::Life;

pub struct EmptyStruct {
    pub ident: Ident,
    pub generics: Generics,
    /// The doc line emitted above the struct, so a crate that enables
    /// `missing_docs` does not report the generated item.
    pub doc: Option<String>,
}

impl EmptyStruct {
    pub fn to_item_struct(&self) -> ItemStruct {
        parse_quote!(#self)
    }
}

impl ToTokens for EmptyStruct {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let struct_ident = &self.ident;
        let struct_generics = &self.generics;
        let struct_doc = self
            .doc
            .as_ref()
            .map(|doc| quote_spanned! { struct_ident.span() => #[doc = #doc] });

        // Stamp the synthesized `pub struct … ;` tokens with the struct ident's
        // span rather than the macro `call_site`, so a redefinition error
        // (`E0428`) when two providers share a name points at the offending name
        // (e.g. the `Foo` in `#[cgp_impl(new Foo)]`) instead of the whole macro
        // invocation.
        let span = struct_ident.span();

        if struct_generics.params.is_empty() {
            tokens.extend(quote_spanned! { span =>
                #struct_doc
                pub struct #struct_ident;
            });
        } else {
            let mut generic_params = struct_generics.params.clone();
            let mut phantom_params: Punctuated<Type, Comma> = Default::default();

            for param in generic_params.iter_mut() {
                match param {
                    GenericParam::Type(type_param) => {
                        type_param.colon_token = None;
                        type_param.bounds.clear();

                        let type_ident = &type_param.ident;
                        phantom_params.push(parse_quote!( #type_ident ));
                    }
                    GenericParam::Lifetime(life_param) => {
                        life_param.colon_token = None;
                        life_param.bounds.clear();

                        let lifetime = &life_param.lifetime;
                        phantom_params.push(parse_quote!( #Life<#lifetime> ));
                    }
                    _ => {}
                }
            }

            // Emit `PhantomData<T>` for a single parameter and `PhantomData<()>`
            // when none survive (all consts), reserving the tuple form for two or
            // more. A single-element `PhantomData<(T)>` is a parenthesized type,
            // not a tuple, and now that the struct carries the user's span (above)
            // it would trip the `unused_parens` lint in the caller's crate.
            let phantom_type: Type = if phantom_params.len() == 1 {
                phantom_params.into_iter().next().unwrap()
            } else {
                parse_quote!( ( #phantom_params ) )
            };

            tokens.extend(quote_spanned! { span =>
                #struct_doc
                pub struct #struct_ident < #generic_params > (
                    pub ::core::marker::PhantomData< #phantom_type >
                );
            })
        }
    }
}
