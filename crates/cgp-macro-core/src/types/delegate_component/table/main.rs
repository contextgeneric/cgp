use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::parse::{Parse, ParseStream};
use syn::{ItemImpl, ItemStruct, Type, braced, parse2};

use crate::traits::PeekKeyword;
use crate::types::delegate_component::{DelegateEntries, ExtractInnerDelegateTables};
use crate::types::generics::ImplGenerics;
use crate::types::ident_type::IdentType;
use crate::types::keyword::Keyword;
use crate::types::keywords::New;
use crate::types::provider_struct::ProviderStruct;

pub struct DelegateTable {
    pub impl_generics: ImplGenerics,
    pub new: Option<Keyword<New>>,
    pub table_type: Type,
    pub entries: DelegateEntries,
}

pub struct EvaluatedDelegateTable {
    pub item_impls: Vec<ItemImpl>,
    pub item_structs: Vec<ItemStruct>,
}

impl Parse for DelegateTable {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let impl_generics = input.parse()?;

        let new = if input.peek_keyword::<New>() {
            Some(input.parse()?)
        } else {
            None
        };

        let table_type = input.parse()?;

        let entries = {
            let body;
            braced!(body in input);

            body.parse()?
        };

        Ok(Self {
            impl_generics,
            new,
            table_type,
            entries,
        })
    }
}

impl DelegateTable {
    pub fn eval(&self) -> syn::Result<EvaluatedDelegateTable> {
        let mut item_impls = Vec::new();
        let mut item_structs = Vec::new();

        if self.new.is_some() {
            let struct_type: IdentType = parse2(self.table_type.to_token_stream())?;
            item_structs.push(
                ProviderStruct {
                    ident: struct_type.ident,
                    generics: struct_type.generics.generics,
                }
                .to_item_struct()?,
            );
        }

        item_impls.extend(
            self.entries
                .build_impls(&self.table_type, &self.impl_generics)?,
        );

        let inner_tables = self.entries.extract_inner_tables();

        for inner_table in inner_tables {
            item_structs.push(inner_table.build_table_struct().to_item_struct()?);

            item_impls.extend(inner_table.build_impls()?);
        }

        Ok(EvaluatedDelegateTable {
            item_impls,
            item_structs,
        })
    }
}

impl ToTokens for EvaluatedDelegateTable {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        for item_struct in &self.item_structs {
            item_struct.to_tokens(tokens);
        }

        for item_impl in &self.item_impls {
            item_impl.to_tokens(tokens);
        }
    }
}
