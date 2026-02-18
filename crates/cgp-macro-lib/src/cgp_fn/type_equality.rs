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

    for use_type in specs.iter() {
        let type_equalities = find_type_equalities(use_type, context_type, specs)?;

        if type_equalities.is_empty() {
            bounds.push(parse2(use_type.trait_path.to_token_stream())?);
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
    specs: &[UseTypeSpec],
) -> syn::Result<Vec<(Ident, Type)>> {
    let mut equalities = Vec::new();

    for current_type_ident in current_spec.type_idents.iter() {
        forbid_same_alias(current_type_ident, current_spec, specs)?;

        if let Some(equality) =
            find_type_equality(context_type, current_type_ident, current_spec, specs)?
        {
            equalities.push(equality);
        }
    }

    Ok(equalities)
}

fn forbid_same_alias(
    current_ident: &UseTypeIdent,
    current_spec: &UseTypeSpec,
    specs: &[UseTypeSpec],
) -> syn::Result<()> {
    for spec in specs.iter() {
        if core::ptr::eq(spec, current_spec) {
            // Skip the current spec
            continue;
        }

        for type_ident in spec.type_idents.iter() {
            if current_ident.alias_ident() == type_ident.alias_ident() {
                return Err(syn::Error::new_spanned(
                    &current_ident.type_ident,
                    "Multiple abstract types cannot share the same identifier or alias",
                ));
            }
        }
    }

    Ok(())
}

fn find_type_equality(
    context_type: &TokenStream,
    current_ident: &UseTypeIdent,
    current_spec: &UseTypeSpec,
    specs: &[UseTypeSpec],
) -> syn::Result<Option<(Ident, Type)>> {
    if let Some(equal_target) = current_ident.equals.clone() {
        for spec in specs.iter() {
            if core::ptr::eq(spec, current_spec) {
                // Skip the current spec
                continue;
            }

            for match_use_type in spec.type_idents.iter() {
                let match_type: Type = parse2(match_use_type.alias_ident().to_token_stream())?;
                if match_type == equal_target {
                    let trait_path = &spec.trait_path;
                    let current_type_ident = &current_ident.type_ident;
                    let match_type_ident = &match_use_type.type_ident;

                    let equal_target: Type = parse2(quote! {
                        <#context_type as #trait_path>::#match_type_ident
                    })?;

                    return Ok(Some((current_type_ident.clone(), equal_target)));
                }
            }
        }

        Ok(Some((current_ident.type_ident.clone(), equal_target)))
    } else {
        Ok(None)
    }
}
