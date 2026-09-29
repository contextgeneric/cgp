use proc_macro2::Span;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::{Comma, Gt, Lt, Paren};
use syn::{Error, Ident, ItemImpl, ItemTrait, Path, parenthesized};

use crate::exports::DelegateComponent;
use crate::functions::{parse_internal, trait_items_to_delegated_impl_items};

#[derive(Clone)]
pub struct DeriveDelegateAttribute {
    pub wrapper: Ident,
    pub params: Punctuated<Ident, Comma>,
}

impl DeriveDelegateAttribute {
    pub fn to_provider_impl(&self, provider_trait: &ItemTrait) -> syn::Result<ItemImpl> {
        let provider_trait_ident = &provider_trait.ident;

        let components_ident = Ident::new("__Components__", Span::call_site());
        let delegate_ident = Ident::new("__Delegate__", Span::call_site());

        let wrapper_ident = &self.wrapper;
        let use_delegate_params = &self.params;

        let mut generics = provider_trait.generics.clone();

        generics.params.push(parse_internal!( #components_ident ));
        generics.params.push(parse_internal!( #delegate_ident ));

        let where_clause = generics.make_where_clause();

        where_clause.predicates.push(parse_internal! {
            #components_ident: #DelegateComponent<
                ( #use_delegate_params ),
                Delegate = #delegate_ident,
            >
        });

        let type_generics = provider_trait.generics.split_for_impl().1;

        where_clause.predicates.push(parse_internal! {
            #delegate_ident : #provider_trait_ident #type_generics
        });

        let type_generics = provider_trait.generics.split_for_impl().1;

        let impl_items = trait_items_to_delegated_impl_items(
            &provider_trait.items,
            &parse_internal!( #delegate_ident ),
            &parse_internal!( #provider_trait_ident #type_generics ),
        )?;

        let provider_type = parse_internal! {
            #wrapper_ident < #components_ident >
        };

        let trait_path: Path = parse_internal! {
            #provider_trait_ident #type_generics
        };

        let item = ItemImpl {
            attrs: provider_trait.attrs.clone(),
            defaultness: None,
            unsafety: provider_trait.unsafety,
            impl_token: Default::default(),
            generics,
            trait_: Some((None, trait_path, Default::default())),
            self_ty: Box::new(provider_type),
            brace_token: Default::default(),
            items: impl_items,
        };

        Ok(item)
    }
}

impl Parse for DeriveDelegateAttribute {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let wrapper: Ident = input.parse()?;

        let _: Lt = input.parse()?;

        let idents = if input.peek(Paren) {
            let body;
            parenthesized!(body in input);
            let idents = Punctuated::parse_terminated(&body)?;
            if idents.is_empty() {
                return Err(Error::new(
                    body.span(),
                    "expect non-empty tuple list of identifiers in use_delegate_spec",
                ));
            }

            idents
        } else {
            let ident: Ident = input.parse()?;
            Punctuated::from_iter([ident])
        };

        let _: Gt = input.parse()?;
        Ok(Self {
            wrapper,
            params: idents,
        })
    }
}
