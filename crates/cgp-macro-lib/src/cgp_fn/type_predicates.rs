use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{Ident, Type, WherePredicate, parse_quote, parse2};

use crate::cgp_fn::{UseTypeIdent, UseTypeSpec};

pub fn derive_use_type_predicates(specs: &[UseTypeSpec]) -> syn::Result<Vec<WherePredicate>> {
    let mut predicates = Vec::new();

    for use_type in specs.iter() {
        let type_equalities = find_type_equalities(use_type, specs)?;

        let trait_path = &use_type.trait_path;
        let mut context_type = use_type.context_type.clone();

        if context_type != parse_quote!(Self)
            && let Some(new_context_type) = find_type_alias(specs, &context_type)?
        {
            context_type = new_context_type;
        }

        if type_equalities.is_empty() {
            predicates.push(parse2(quote! {
                #context_type: #trait_path
            })?);
        } else {
            let mut constraints: Punctuated<TokenStream, Comma> = Punctuated::new();

            for (alias_ident, equal_target) in type_equalities.into_iter() {
                constraints.push(quote! {
                    #alias_ident = #equal_target
                });
            }

            predicates.push(parse2(quote! {
                #context_type: #trait_path < #constraints >
            })?);
        }
    }

    Ok(predicates)
}

fn find_type_alias(specs: &[UseTypeSpec], context_type: &Type) -> syn::Result<Option<Type>> {
    let Ok(context_ident) = parse2::<Ident>(context_type.to_token_stream()) else {
        return Ok(None);
    };

    for spec in specs {
        for ident in spec.type_idents.iter() {
            if ident.alias_ident() == &context_ident {
                let new_context_type = &spec.context_type;
                let type_ident = &ident.type_ident;
                let trait_path = &spec.trait_path;

                let new_type = parse2(quote! {
                    <#new_context_type as #trait_path>::#type_ident
                })?;

                return Ok(Some(new_type));
            }
        }
    }

    Ok(None)
}

pub fn find_type_equalities(
    current_spec: &UseTypeSpec,
    specs: &[UseTypeSpec],
) -> syn::Result<Vec<(Ident, Type)>> {
    let mut equalities = Vec::new();

    for current_type_ident in current_spec.type_idents.iter() {
        forbid_same_alias(current_type_ident, current_spec, specs)?;

        if let Some(equality) = find_type_equality(current_type_ident, current_spec, specs)? {
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
                    let context_type = &spec.context_type;

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
