use syn::token::{Brace, For, Impl};
use syn::{ItemImpl, Path, Type};

use crate::functions::{parse_internal, trait_items_to_delegated_impl_items};
use crate::types::cgp_component::PreprocessedCgpComponent;
use crate::types::generics::TypeGenerics;

impl PreprocessedCgpComponent {
    /// Build the consumer blanket impl: any context implementing the provider
    /// trait for itself gets the consumer trait, forwarding each method to it.
    pub fn to_consumer_item_impl(&self) -> syn::Result<ItemImpl> {
        let consumer_trait = &self.item_trait;
        let provider_ident = &self.args.provider_ident;
        let context_type_ident = &self.args.context_ident;

        let consumer_name = &consumer_trait.ident;

        let consumer_type_generics = TypeGenerics::try_from(&consumer_trait.generics)?;

        let provider_trait_path: Type = {
            let mut provider_type_generics = consumer_type_generics.clone();
            provider_type_generics
                .generics
                .params
                .insert(0, parse_internal!(#context_type_ident));

            parse_internal!(#provider_ident #provider_type_generics)
        };

        let generics_for_impl = {
            let mut generics = consumer_trait.generics.clone();

            generics
                .params
                .insert(0, parse_internal!(#context_type_ident));

            let where_clause = generics.make_where_clause();

            if !consumer_trait.supertraits.is_empty() {
                let supertrait_constraints = consumer_trait.supertraits.clone();
                where_clause.predicates.push(parse_internal! {
                    #context_type_ident : #supertrait_constraints
                });
            }

            where_clause.predicates.push(parse_internal! {
                #context_type_ident : #provider_trait_path
            });

            generics
        };

        let impl_items = trait_items_to_delegated_impl_items(
            &consumer_trait.items,
            &parse_internal!(#context_type_ident),
            &provider_trait_path,
        )?;

        let consumer_trait_path: Path = parse_internal!( #consumer_name #consumer_type_generics );

        let item_impl = ItemImpl {
            attrs: consumer_trait.attrs.clone(),
            defaultness: None,
            unsafety: consumer_trait.unsafety,
            impl_token: Impl::default(),
            generics: generics_for_impl,
            trait_: Some((None, consumer_trait_path, For::default())),
            self_ty: Box::new(parse_internal!(#context_type_ident)),
            brace_token: Brace::default(),
            items: impl_items,
        };

        Ok(item_impl)
    }
}
