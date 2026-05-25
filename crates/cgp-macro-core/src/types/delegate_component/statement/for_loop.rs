use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::{Comma, For, Gt, In, Lt};
use syn::{Ident, Type, WhereClause, braced, parse_quote};

use crate::types::delegate_component::{
    EvalDelegateEntry, EvalDelegateKey, EvalDelegateValue, EvaluatedDelegateEntry,
    NormalDelegateMapping,
};
use crate::types::ident_type::IdentType;

#[derive(Debug, Clone)]
pub struct ForDelegateStatement {
    pub for_token: For,
    pub lt: Lt,
    pub key: Ident,
    pub comma: Comma,
    pub value: Ident,
    pub gt: Gt,
    pub in_token: In,
    pub namespace: IdentType,
    pub where_clause: Option<WhereClause>,
    pub mappings: Punctuated<NormalDelegateMapping, Comma>,
}

impl Parse for ForDelegateStatement {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let for_token = input.parse()?;
        let lt = input.parse()?;
        let key = input.parse()?;
        let comma = input.parse()?;
        let value = input.parse()?;
        let gt = input.parse()?;
        let in_token = input.parse()?;
        let namespace = input.parse()?;
        let where_clause = input.parse()?;

        let mappings = {
            let body;
            braced!(body in input);
            Punctuated::parse_terminated(&body)?
        };

        Ok(Self {
            for_token,
            lt,
            key,
            comma,
            value,
            gt,
            in_token,
            namespace,
            where_clause,
            mappings,
        })
    }
}

impl EvalDelegateEntry for ForDelegateStatement {
    fn eval(&self, table_type: &Type) -> syn::Result<Vec<EvaluatedDelegateEntry>> {
        let for_key = &self.key;
        let for_value = &self.value;
        let for_where = &self.where_clause;

        let namespace_ident = &self.namespace.ident;
        let mut namespace_generics = self.namespace.generics.clone();
        namespace_generics
            .generics
            .params
            .push(parse_quote!(#table_type));

        let mut entries = Vec::new();

        for mapping in &self.mappings {
            let keys = mapping.key.eval()?;
            let value_type = mapping.value.eval()?;

            for key in keys {
                let key_type = key.key;
                let value_type = value_type.clone();

                let namespace_trait: Type = {
                    let mut namespace_generics = namespace_generics.clone();
                    namespace_generics.generics.params.push(parse_quote! {
                        Provider = #value_type
                    });

                    parse_quote!( #namespace_ident #namespace_generics )
                };

                let mut generics = key.generics;
                generics.params.push(parse_quote!(#for_key));
                generics.params.push(parse_quote!(#for_value));

                let where_clause = generics.make_where_clause();
                where_clause.predicates.push(parse_quote! {
                    #for_key: #namespace_trait
                });

                if let Some(for_where) = for_where {
                    where_clause.predicates.extend(for_where.predicates.clone());
                }

                let entry = EvaluatedDelegateEntry {
                    table_type: table_type.clone(),
                    generics,
                    key: key_type,
                    value: value_type,
                };

                entries.push(entry);
            }
        }

        Ok(entries)
    }
}
