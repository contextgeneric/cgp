use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::parse::{Parse, ParseStream};
use syn::{Attribute, GenericParam, Generics, ItemImpl, Type, braced};

use crate::macro_core::functions::parse_internal;
use crate::macro_core::traits::ParseOptionalKeyword;
use crate::macro_core::types::delegate_component::{DelegateEntries, ExtractInnerDelegateTables};
use crate::macro_core::types::empty_struct::EmptyStruct;
use crate::macro_core::types::generics::ImplGenerics;
use crate::macro_core::types::ident::IdentWithTypeGenerics;
use crate::macro_core::types::keyword::Keyword;
use crate::macro_core::types::keywords::New;

/// The whole parsed `delegate_components!` body: an optional generic list and
/// `new` keyword, the target type, and its braced table of entries.
pub struct DelegateTable {
    pub attributes: Vec<Attribute>,
    pub impl_generics: ImplGenerics,
    pub new: Option<Keyword<New>>,
    pub table_type: Type,
    pub entries: DelegateEntries,
}

/// The lowered table: the generated impls and any structs (`new` target and
/// lifted inner tables). Its `ToTokens` emits the structs first, then the impls.
pub struct EvaluatedDelegateTable {
    pub item_impls: Vec<ItemImpl>,
    pub item_structs: Vec<EmptyStruct>,
    pub preset_invocations: TokenStream,
}

impl Parse for DelegateTable {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let attributes = input.call(Attribute::parse_outer)?;

        let impl_generics = input.parse()?;

        let new = input.parse_optional_keyword()?;

        let table_type = input.parse()?;

        let entries = {
            let body;
            braced!(body in input);

            body.parse()?
        };

        Ok(Self {
            attributes,
            impl_generics,
            new,
            table_type,
            entries,
        })
    }
}

impl DelegateTable {
    /// Lower the table: emit the `new` struct if present, build every entry's
    /// impl pair, and lift out each nested `UseDelegate` inner table as its own
    /// struct and impls.
    pub fn eval(&self) -> syn::Result<EvaluatedDelegateTable> {
        let mut item_impls = Vec::new();
        let mut item_structs = Vec::new();
        let mut preset_invocations = self
            .entries
            .build_preset_invocations(&self.table_type, &self.impl_generics)?;

        if self.new.is_some() {
            let struct_type: IdentWithTypeGenerics =
                parse_internal(self.table_type.to_token_stream())?;

            // The target's type arguments name each parameter without its kind, so a
            // parameter the table's generic list declares `const` is restored as a
            // const parameter of the struct rather than read as a type parameter.
            let mut generics = struct_type.type_generics.to_generics();
            for param in generics.params.iter_mut() {
                if let GenericParam::Type(type_param) = param
                    && let Some(const_param) =
                        self.impl_generics
                            .generics
                            .params
                            .iter()
                            .find_map(|p| match p {
                                GenericParam::Const(c) if c.ident == type_param.ident => Some(c),
                                _ => None,
                            })
                {
                    let mut const_param = const_param.clone();
                    const_param.eq_token = None;
                    const_param.default = None;
                    *param = GenericParam::Const(const_param);
                }
            }

            item_structs.push(EmptyStruct {
                ident: struct_type.ident,
                generics,
            });
        }

        item_impls.extend(
            self.entries
                .build_impls(&self.table_type, &self.impl_generics)?,
        );

        let inner_tables = self.entries.extract_inner_tables();

        for inner_table in inner_tables {
            item_structs.push(inner_table.build_table_struct());

            item_impls.extend(inner_table.build_impls()?);

            let inner_type = inner_table.build_table_type()?;
            let inner_generics: &Generics = &inner_table.table_generics;
            preset_invocations.extend(
                inner_table
                    .entries
                    .build_preset_invocations(&inner_type, inner_generics)?,
            );
        }

        Ok(EvaluatedDelegateTable {
            item_impls,
            item_structs,
            preset_invocations,
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

        self.preset_invocations.to_tokens(tokens);
    }
}
