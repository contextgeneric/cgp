use proc_macro2::Span;
use quote::ToTokens;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::token::{Comma, Lt, Pound, Where};
use syn::{Attribute, Ident, Item, ItemImpl, ItemTrait, Type, WhereClause, braced, parse2};

use crate::exports::CanUseComponent;
use crate::functions::merge_generics;
use crate::parse_internal;
use crate::types::check_components::{CheckEntries, EvaluatedCheckEntry, TypeWithGenerics};
use crate::types::generics::ImplGenerics;
use crate::types::ident::IdentWithTypeArgs;

pub struct CheckComponentsTable {
    pub check_providers: Option<Punctuated<Type, Comma>>,
    pub impl_generics: ImplGenerics,
    pub trait_name: Ident,
    pub context_type: Type,
    pub where_clause: Option<WhereClause>,
    pub check_entries: CheckEntries,
}

impl CheckComponentsTable {
    pub fn to_items(&self) -> syn::Result<Vec<Item>> {
        let (item_trait, item_impls) = self.eval()?;

        let mut items = vec![item_trait.into()];
        items.extend(item_impls.into_iter().map(Into::into));

        Ok(items)
    }

    pub fn eval(&self) -> syn::Result<(ItemTrait, Vec<ItemImpl>)> {
        let mut item_impls = Vec::new();
        let unit: Type = parse_internal!(());

        let context_type = &self.context_type;
        let trait_name = &self.trait_name;
        let impl_generics = &self.impl_generics;
        let where_clause = &self.where_clause;

        let item_trait: ItemTrait = if self.check_providers.is_some() {
            parse_internal! {
                trait #trait_name <__Component__, __Params__: ?Sized>: IsProviderFor<__Component__, #context_type, __Params__> {}
            }
        } else {
            parse_internal! {
                trait #trait_name <__Component__, __Params__: ?Sized>: #CanUseComponent<__Component__, __Params__> {}
            }
        };

        let evaluated_entries = self.check_entries.eval();

        for entry in evaluated_entries {
            let EvaluatedCheckEntry {
                key: component_type,
                value: component_params,
                span,
            } = entry;

            let self_types = if let Some(check_providers) = &self.check_providers {
                Vec::from_iter(check_providers.iter().cloned())
            } else {
                // Override the span of the context type so that any unsatisfied constraint
                // error is highlighted on the component type instead
                let context_type = override_span(&span, context_type)?;
                vec![context_type]
            };

            let TypeWithGenerics {
                ty: component_param,
                generics: check_generics,
            } = component_params.unwrap_or_else(|| unit.clone().into());

            let generics = merge_generics(&check_generics.generics, &impl_generics.generics);

            let impl_generics = generics.split_for_impl().0;

            for self_type in self_types {
                let item_impl: ItemImpl = parse_internal! {
                    impl #impl_generics
                        #trait_name < #component_type, #component_param >
                        for #self_type
                    #where_clause
                    {}
                };

                item_impls.push(item_impl);
            }
        }

        Ok((item_trait, item_impls))
    }
}

impl Parse for CheckComponentsTable {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut check_providers: Option<Punctuated<Type, Comma>> = None;
        let mut m_check_trait_name: Option<Ident> = None;

        if input.peek(Pound) {
            let attributes = input.call(Attribute::parse_outer)?;

            for attribute in attributes {
                if attribute.path().is_ident("check_providers") {
                    let provider_types: Punctuated<Type, Comma> =
                        attribute.parse_args_with(Punctuated::parse_terminated)?;

                    check_providers
                        .get_or_insert_default()
                        .extend(provider_types);
                } else if attribute.path().is_ident("check_trait") {
                    let check_trait_name: Ident = attribute.parse_args()?;

                    if m_check_trait_name.is_some() {
                        return Err(syn::Error::new(
                            attribute.span(),
                            "Multiple `#[check_trait]` attributes found. Expected at most one.",
                        ));
                    }

                    m_check_trait_name = Some(check_trait_name);
                } else {
                    return Err(syn::Error::new(
                        attribute.span(),
                        format!("Invalid attribute {}", attribute.to_token_stream()),
                    ));
                }
            }
        };

        let impl_generics = if input.peek(Lt) {
            input.parse()?
        } else {
            Default::default()
        };

        let context_type: Type = input.parse()?;

        let trait_name = if let Some(check_trait_name) = m_check_trait_name {
            check_trait_name
        } else {
            derive_check_trait_ident(&context_type, "__Check")?
        };

        let where_clause = if input.peek(Where) {
            Some(input.parse()?)
        } else {
            None
        };

        let content;
        braced!(content in input);

        let entries: CheckEntries = content.parse()?;

        Ok(Self {
            check_providers,
            impl_generics,
            trait_name,
            context_type,
            where_clause,
            check_entries: entries,
        })
    }
}

/// Derive a check trait identifier from a context type by prepending `prefix`
/// to the context type's leading identifier, e.g. `__CheckPerson` or
/// `__CanUsePerson` for the context type `Person`.
pub fn derive_check_trait_ident(context_type: &Type, prefix: &str) -> syn::Result<Ident> {
    let context_type: IdentWithTypeArgs = parse2(context_type.to_token_stream())?;

    Ok(Ident::new(
        &format!("{prefix}{}", context_type.ident),
        context_type.span(),
    ))
}

fn override_span<T>(span: &Span, body: &T) -> syn::Result<T>
where
    T: Parse + ToTokens,
{
    parse2(
        body.to_token_stream()
            .into_iter()
            .map(|mut tree| {
                tree.set_span(*span);
                tree
            })
            .collect(),
    )
}
