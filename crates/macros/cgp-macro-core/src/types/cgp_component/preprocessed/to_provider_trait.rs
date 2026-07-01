use syn::punctuated::Punctuated;
use syn::visit_mut::VisitMut;
use syn::{Ident, ItemTrait, TraitItem, Type, TypeParamBound};

use crate::exports::IsProviderFor;
use crate::functions::{parse_internal, parse_is_provider_params, to_snake_case_ident};
use crate::types::cgp_component::PreprocessedCgpComponent;
use crate::visitors::{
    ReplaceSelfReceiverVisitor, ReplaceSelfTypeVisitor, ReplaceSelfValueVisitor,
};

impl PreprocessedCgpComponent {
    /// Derive the provider trait from the consumer trait: insert the leading
    /// `Context` parameter, lower supertraits to a `Context` where-bound, set the
    /// `IsProviderFor` supertrait, and rewrite `self`/`Self` to the context.
    pub fn to_provider_trait(&self) -> syn::Result<ItemTrait> {
        let component_name = &self.args.component_name;
        let provider_name = &self.args.provider_ident;
        let consumer_trait = &self.item_trait;
        let context_type_ident = &self.args.context_ident;

        let mut provider_trait = consumer_trait.clone();

        provider_trait.ident = provider_name.clone();

        let context_type: Type = parse_internal!(#context_type_ident);

        // Add generic parameter `Context` to the front of generics
        {
            provider_trait
                .generics
                .params
                .insert(0, parse_internal!(#context_type_ident));
        }

        let local_assoc_types: Vec<Ident> = provider_trait
            .items
            .iter()
            .filter_map(|item| {
                if let TraitItem::Type(assoc_type) = item {
                    Some(assoc_type.ident.clone())
                } else {
                    None
                }
            })
            .collect();

        // Turn the supertrait constraints into `Context` constraints in the `where` clause
        if !provider_trait.supertraits.is_empty() {
            let supertraits = &provider_trait.supertraits;

            provider_trait
                .generics
                .make_where_clause()
                .predicates
                .push(parse_internal! {
                    #context_type_ident : #supertraits
                });
        }

        let is_provider_params = parse_is_provider_params(&consumer_trait.generics)?;

        let provider_supertrait: TypeParamBound = parse_internal! {
            #IsProviderFor< #component_name, #context_type_ident, ( #is_provider_params ) >
        };

        provider_trait.supertraits = Punctuated::from_iter([provider_supertrait]);

        let context_value_ident = to_snake_case_ident(context_type_ident);

        ReplaceSelfTypeVisitor {
            replaced_type: &context_type,
            skip_assoc_types: &local_assoc_types,
        }
        .visit_item_trait_mut(&mut provider_trait);

        ReplaceSelfReceiverVisitor {
            replaced_ident: &context_value_ident,
            replaced_type: &context_type,
        }
        .visit_item_trait_mut(&mut provider_trait);

        ReplaceSelfValueVisitor {
            replaced_ident: &context_value_ident,
        }
        .visit_item_trait_mut(&mut provider_trait);

        Ok(provider_trait)
    }
}
