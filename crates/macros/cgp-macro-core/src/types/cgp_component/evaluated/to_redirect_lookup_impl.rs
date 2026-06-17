use quote::quote;
use syn::{GenericParam, Generics, ItemImpl, Path};

use crate::exports::{ConcatPath, DelegateComponent, RedirectLookup};
use crate::functions::{parse_internal, provider_trait_to_impl_items};
use crate::types::cgp_component::EvaluatedCgpComponent;
use crate::types::path::{PathElement, UniPath};
use crate::types::provider_impl::ItemProviderImpl;

impl EvaluatedCgpComponent {
    pub fn to_redirect_lookup_impl(&self) -> syn::Result<ItemProviderImpl> {
        let consumer_trait = &self.consumer_trait;
        let provider_trait = &self.provider_trait;

        let provider_name = &provider_trait.ident;
        let provider_type_generics = provider_trait.generics.split_for_impl().1;

        let generic_params = generic_params_to_path(&consumer_trait.generics)?;

        let mut impl_generics = provider_trait.generics.clone();

        impl_generics.params.push(parse_internal!(__Components__));

        impl_generics.params.push(parse_internal!(__Path__));

        let where_clause = impl_generics.make_where_clause();

        let delegate_constraint = if let Some(generic_params) = &generic_params {
            where_clause.predicates.push(parse_internal! {
                __Path__: #ConcatPath< #generic_params >
            });

            quote! {
                #DelegateComponent<<__Path__ as #ConcatPath< #generic_params >>::Output>
            }
        } else {
            quote! {
                #DelegateComponent<__Path__>
            }
        };

        where_clause.predicates.push(parse_internal! {
            __Components__: #delegate_constraint
        });

        let delegate_type = parse_internal! {
            < __Components__ as #delegate_constraint > :: Delegate
        };

        where_clause.predicates.push(parse_internal! {
            #delegate_type : #provider_name #provider_type_generics
        });

        let impl_items = provider_trait_to_impl_items(provider_trait, &delegate_type)?;

        let self_type = parse_internal!(#RedirectLookup<__Components__, __Path__>);

        let trait_path: Path = parse_internal!( #provider_name #provider_type_generics );

        let item_impl = ItemImpl {
            attrs: provider_trait.attrs.clone(),
            defaultness: None,
            unsafety: provider_trait.unsafety,
            impl_token: Default::default(),
            generics: impl_generics,
            trait_: Some((None, trait_path, Default::default())),
            self_ty: Box::new(self_type),
            brace_token: Default::default(),
            items: impl_items,
        };

        Ok(ItemProviderImpl {
            component_type: self.args.component_name.to_type(),
            item_impl,
        })
    }
}

fn generic_params_to_path(generics: &Generics) -> syn::Result<Option<UniPath>> {
    let type_params = generics
        .params
        .iter()
        .filter_map(|param| {
            if let GenericParam::Type(type_param) = param {
                Some(type_param.ident.clone())
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    if type_params.is_empty() {
        Ok(None)
    } else {
        let mut path = UniPath::default();
        for param in type_params {
            path.elements
                .push(PathElement::Type(parse_internal!(#param)))
        }

        Ok(Some(path))
    }
}
