use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{Ident, Type, TypeParamBound, parse2};

use crate::cgp_fn::{UseTypeIdent, UseTypeSpec};

pub fn derive_use_type_trait_bounds(
    context_type: &TokenStream,
    specs: &[UseTypeSpec],
) -> syn::Result<Vec<TypeParamBound>> {
    let mut bounds = Vec::new();

    let mut past_specs = Vec::new();

    for use_type in specs.iter() {
        let type_equalities = find_type_equalities(use_type, context_type, &past_specs)?;

        if type_equalities.is_empty() {
            bounds.push(parse2(use_type.trait_path.to_token_stream())?);
            past_specs.push(use_type);
        } else {
            let mut constraints: Punctuated<TokenStream, Comma> = Punctuated::new();

            for (alias_ident, equal_target) in type_equalities.into_iter() {
                constraints.push(quote! {
                    #alias_ident = #equal_target
                });
            }

            let trait_path = &use_type.trait_path;
            let bound = quote! {
                #trait_path < #constraints >
            };

            bounds.push(parse2(bound)?);
        }
    }

    Ok(bounds)
}

pub fn find_type_equalities(
    current_spec: &UseTypeSpec,
    context_type: &TokenStream,
    past_specs: &[&UseTypeSpec],
) -> syn::Result<Vec<(Ident, Type)>> {
    let mut equalities = Vec::new();

    for type_ident in current_spec.type_idents.iter() {
        if let Some(equality) = find_type_equality(context_type, type_ident, past_specs)? {
            equalities.push(equality);
        }
    }

    Ok(equalities)
}

pub fn find_type_equality(
    context_type: &TokenStream,
    current_ident: &UseTypeIdent,
    past_specs: &[&UseTypeSpec],
) -> syn::Result<Option<(Ident, Type)>> {
    for past_spec in past_specs.iter() {
        for type_ident in past_spec.type_idents.iter() {
            let current_alias = current_ident.alias_ident();

            if current_alias == type_ident.alias_ident() {
                if current_ident.as_alias.is_none() || type_ident.as_alias.is_none() {
                    return Err(syn::Error::new_spanned(
                        &current_ident.type_ident,
                        "Multiple abstract types with the same name must be aliased explicitly with `as`",
                    ));
                }

                let trait_path = &past_spec.trait_path;
                let type_ident = &current_ident.type_ident;

                let equal_target: Type = parse2(quote! {
                    <#context_type as #trait_path>::#type_ident
                })?;

                return Ok(Some((current_alias.clone(), equal_target)));
            }
        }
    }

    Ok(None)
}
