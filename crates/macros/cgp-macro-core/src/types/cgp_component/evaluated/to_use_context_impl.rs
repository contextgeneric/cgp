use quote::quote;
use syn::{ItemImpl, Path, Type, parse_quote, parse2};

use crate::exports::UseContext;
use crate::functions::trait_items_to_delegated_impl_items;
use crate::types::cgp_component::EvaluatedCgpComponent;
use crate::types::provider_impl::ItemProviderImpl;

impl EvaluatedCgpComponent {
    pub fn to_use_context_impl(&self) -> syn::Result<ItemProviderImpl> {
        let component_name = &self.args.component_name;
        let context_type_ident = &self.args.context_ident;
        let consumer_trait = &self.consumer_trait;
        let provider_trait = &self.provider_trait;

        let consumer_trait_ident = &consumer_trait.ident;
        let provider_trait_ident = &provider_trait.ident;

        let provider_generics = provider_trait.generics.split_for_impl().1;

        let consumer_trait_generics = consumer_trait.generics.split_for_impl().1;

        let consumer_trait_path: Type =
            parse_quote!(#consumer_trait_ident #consumer_trait_generics);

        let mut impl_generics = provider_trait.generics.clone();

        impl_generics
            .make_where_clause()
            .predicates
            .push(parse2(quote! {
                #context_type_ident : #consumer_trait_ident #consumer_trait_generics
            })?);

        let impl_items = trait_items_to_delegated_impl_items(
            &provider_trait.items,
            &parse_quote!(#context_type_ident),
            &consumer_trait_path,
        )?;

        let provider_trait_path: Path = parse2(quote!( #provider_trait_ident #provider_generics ))?;

        let item_impl = ItemImpl {
            attrs: provider_trait.attrs.clone(),
            defaultness: None,
            unsafety: provider_trait.unsafety,
            impl_token: Default::default(),
            generics: impl_generics,
            trait_: Some((None, provider_trait_path, Default::default())),
            self_ty: Box::new(parse_quote!(#UseContext)),
            brace_token: Default::default(),
            items: impl_items,
        };

        Ok(ItemProviderImpl {
            component_type: parse_quote!(#component_name),
            item_impl,
        })
    }
}
