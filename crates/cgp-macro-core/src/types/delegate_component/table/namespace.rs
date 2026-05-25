use syn::parse::{Parse, ParseStream};
use syn::{Ident, ItemImpl, ItemTrait, Type, braced, parse_quote};

use crate::traits::PeekKeyword;
use crate::types::delegate_component::{DelegateEntries, EvalDelegateEntry};
use crate::types::generics::{ImplGenerics, TypeGenerics};
use crate::types::keyword::Keyword;
use crate::types::keywords::New;

pub struct NamespaceTable {
    pub impl_generics: ImplGenerics,
    pub new: Option<Keyword<New>>,
    pub namespace_ident: Ident,
    pub namespace_generics: TypeGenerics,
    pub entries: DelegateEntries,
}

pub struct EvaluatedNamespaceTable {
    pub item_impls: Vec<ItemImpl>,
    pub item_trait: Option<ItemTrait>,
}

impl Parse for NamespaceTable {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let impl_generics = input.parse()?;

        let new = if input.peek_keyword::<New>() {
            Some(input.parse()?)
        } else {
            None
        };

        let namespace_ident = input.parse()?;
        let namespace_generics = input.parse()?;

        let entries = {
            let body;
            braced!(body in input);

            body.parse()?
        };

        Ok(Self {
            impl_generics,
            new,
            namespace_ident,
            namespace_generics,
            entries,
        })
    }
}

impl NamespaceTable {
    pub fn eval(&self) -> syn::Result<EvaluatedNamespaceTable> {
        let namespace_ident = &self.namespace_ident;

        let mut namespace_generics = self.namespace_generics.clone();
        namespace_generics.params.push(parse_quote!(__Components__));

        let item_trait: Option<ItemTrait> = if self.new.is_some() {
            let item_trait = parse_quote! {
                pub trait #namespace_ident #namespace_generics {
                    type Provider;
                }
            };

            Some(item_trait)
        } else {
            None
        };

        let mut impl_generics = self.impl_generics.clone();
        impl_generics.params.push(parse_quote!(__Components__));

        let namespace_trait: Type = parse_quote!( #namespace_ident #namespace_generics );
        let table_type: Type = parse_quote!(__Components__);

        let evaluated_entries = self.entries.eval(&table_type)?;

        let mut item_impls: Vec<ItemImpl> = Vec::new();

        for evaluated_entry in evaluated_entries {
            let item_impl =
                evaluated_entry.build_namespace_impl(&namespace_trait, &impl_generics)?;
            item_impls.push(item_impl);
        }

        Ok(EvaluatedNamespaceTable {
            item_impls,
            item_trait,
        })
    }
}
