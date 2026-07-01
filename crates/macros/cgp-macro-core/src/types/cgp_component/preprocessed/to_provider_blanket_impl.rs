use proc_macro2::Span;
use quote::quote;
use syn::punctuated::Punctuated;
use syn::token::{Brace, For, Impl, Plus};
use syn::{Ident, ItemImpl, ItemTrait, Path, TypeParamBound};

use crate::exports::{DelegateComponent, IsProviderFor};
use crate::functions::{parse_internal, parse_is_provider_params, provider_trait_to_impl_items};
use crate::types::cgp_component::PreprocessedCgpComponent;

impl PreprocessedCgpComponent {
    /// Build the provider trait together with its blanket impl for `__Provider__`,
    /// which inherits the provider trait from whatever `DelegateComponent` names.
    /// Returns both so they share one construction of the provider trait.
    pub fn to_provider_trait_and_blanket_impl(&self) -> syn::Result<(ItemTrait, ItemImpl)> {
        let consumer_trait = &self.item_trait;
        let context_type = &self.args.context_ident;
        let component_name = &self.args.component_name;
        let provider_name = &self.args.provider_ident;

        let provider_trait = self.to_provider_trait()?;

        let provider_type = Ident::new("__Provider__", Span::call_site());

        let delegate_constraint = quote! {
            #DelegateComponent< #component_name >
        };

        let delegate_type = parse_internal! {
            < #provider_type as #delegate_constraint > :: Delegate
        };

        let provider_type_generics = provider_trait.generics.split_for_impl().1;

        let impl_generics = {
            let mut impl_generics = provider_trait.generics.clone();

            impl_generics
                .params
                .insert(0, parse_internal!(#provider_type));

            {
                let is_provider_params = parse_is_provider_params(&consumer_trait.generics)?;

                let mut delegate_constraints: Punctuated<TypeParamBound, Plus> =
                    Punctuated::default();

                delegate_constraints.push(parse_internal(delegate_constraint)?);

                delegate_constraints.push(parse_internal! {
                    #IsProviderFor< #component_name, #context_type, ( #is_provider_params ) >
                });

                let provider_constraint: TypeParamBound = parse_internal! {
                    #provider_name #provider_type_generics
                };

                let where_clause = impl_generics.make_where_clause();

                where_clause.predicates.push(parse_internal! {
                    #provider_type : #delegate_constraints
                });

                where_clause.predicates.push(parse_internal! {
                    #delegate_type : #provider_constraint
                });
            }

            impl_generics
        };

        let impl_items = provider_trait_to_impl_items(&provider_trait, &delegate_type)?;

        let trait_path: Path = parse_internal!( #provider_name #provider_type_generics );

        let provider_blanket_impl = ItemImpl {
            attrs: provider_trait.attrs.clone(),
            defaultness: None,
            unsafety: provider_trait.unsafety,
            impl_token: Impl::default(),
            generics: impl_generics,
            trait_: Some((None, trait_path, For::default())),
            self_ty: Box::new(parse_internal!(#provider_type)),
            brace_token: Brace::default(),
            items: impl_items,
        };

        Ok((provider_trait, provider_blanket_impl))
    }
}
