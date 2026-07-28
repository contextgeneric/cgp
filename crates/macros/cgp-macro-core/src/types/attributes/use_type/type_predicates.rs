use proc_macro2::TokenStream;
use quote::quote;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::visit_mut::VisitMut;
use syn::{Ident, Type, WherePredicate};

use crate::functions::parse_internal;
use crate::types::attributes::{UseTypeAttribute, UseTypeIdent};
use crate::visitors::SubstituteAbstractTypes;

/// Derive the impl-side `where` predicates a set of `#[use_type]` specs
/// contributes: one `Context: Trait` bound per spec, carrying any type-equality
/// (`= T`) pins as associated-type bindings. The specs' contexts must already be
/// grounded (see [`UseTypeAttributes::grounded_specs`]), so this reads
/// `use_type.context_type` directly rather than re-resolving aliases.
pub fn derive_use_type_predicates(specs: &[UseTypeAttribute]) -> syn::Result<Vec<WherePredicate>> {
    let mut predicates = Vec::new();

    for use_type in specs.iter() {
        let type_equalities = find_type_equalities(use_type, specs);

        let trait_path = &use_type.trait_path;
        let context_type = &use_type.context_type;

        if type_equalities.is_empty() {
            predicates.push(parse_internal! {
                #context_type: #trait_path
            });
        } else {
            let mut constraints: Punctuated<TokenStream, Comma> = Punctuated::new();

            for (alias_ident, equal_target) in type_equalities.into_iter() {
                constraints.push(quote! {
                    #alias_ident = #equal_target
                });
            }

            predicates.push(parse_internal! {
                #context_type: #trait_path < #constraints >
            });
        }
    }

    Ok(predicates)
}

/// Reject two imports that resolve to the same bare identifier or alias, across
/// every spec *and within a single braced list*. A shared alias would make the
/// substitution silently pick the first match and drop the rest, so it is an
/// error regardless of which host macro drives the import.
pub fn forbid_duplicate_aliases(specs: &[UseTypeAttribute]) -> syn::Result<()> {
    let idents: Vec<&UseTypeIdent> = specs
        .iter()
        .flat_map(|spec| spec.type_idents.iter())
        .collect();

    for (index, current) in idents.iter().enumerate() {
        for other in idents.iter().skip(index + 1) {
            if current.alias_ident() == other.alias_ident() {
                return Err(syn::Error::new_spanned(
                    &other.type_ident,
                    "Multiple abstract types cannot share the same identifier or alias",
                ));
            }
        }
    }

    Ok(())
}

fn find_type_equalities(
    current_spec: &UseTypeAttribute,
    specs: &[UseTypeAttribute],
) -> Vec<(Ident, Type)> {
    current_spec
        .type_idents
        .iter()
        .filter_map(|current_type_ident| find_type_equality(current_type_ident, specs))
        .collect()
}

/// Ground one `= T` pin: rewrite every imported alias appearing *anywhere
/// inside* the pin's right-hand side into its fully-qualified projection, so
/// `{Transaction = Tx<Db>}` grounds its nested `Db` exactly as
/// `{HashedPassword = Password}` grounds a bare one. Substituting through the
/// shared visitor rather than comparing the whole type is what makes the two
/// cases one rule — the right-hand side is an ordinary type, and an alias is
/// resolved wherever it occurs in it.
///
/// The pinned alias itself is excluded from the substitution set, so a
/// degenerate self-pin (`{Foo = Foo}`) stays the unresolved-name error it
/// already was rather than silently becoming a vacuous bound.
fn find_type_equality(
    current_ident: &UseTypeIdent,
    specs: &[UseTypeAttribute],
) -> Option<(Ident, Type)> {
    let mut equal_target = current_ident.equals.clone()?;

    let others = specs_excluding_alias(specs, current_ident.alias_ident());
    SubstituteAbstractTypes::new(&others).visit_type_mut(&mut equal_target);

    Some((current_ident.type_ident.clone(), equal_target))
}

/// The grounded specs with one alias dropped, for substituting inside that
/// alias's own equality pin.
fn specs_excluding_alias(specs: &[UseTypeAttribute], alias: &Ident) -> Vec<UseTypeAttribute> {
    specs
        .iter()
        .map(|spec| {
            let mut spec = spec.clone();
            spec.type_idents
                .retain(|type_ident| type_ident.alias_ident() != alias);
            spec
        })
        .collect()
}
