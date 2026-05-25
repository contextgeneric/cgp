use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{Generics, ItemImpl, Type};

use crate::types::delegate_component::{
    DelegateMapping, DelegateStatement, EvalDelegateEntries, ExtractInnerDelegateTables,
    InnerDelegateTable,
};

#[derive(Debug, Clone)]
pub struct DelegateEntries {
    pub statements: Vec<DelegateStatement>,
    pub entries: Punctuated<DelegateMapping, Comma>,
}

impl Parse for DelegateEntries {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut statements = Vec::new();

        while DelegateStatement::peek_statement(input) {
            let statement = input.parse()?;
            statements.push(statement);
        }

        let entries = Punctuated::parse_terminated(input)?;

        Ok(Self {
            statements,
            entries,
        })
    }
}

impl EvalDelegateEntries for DelegateEntries {
    fn eval_entries(
        &self,
        table_type: &Type,
    ) -> syn::Result<Vec<crate::types::delegate_component::EvaluatedDelegateEntry>> {
        let mut evaluated_entries = Vec::new();

        for statement in &self.statements {
            evaluated_entries.extend(statement.eval_entries(table_type)?);
        }

        for entry in &self.entries {
            evaluated_entries.extend(entry.eval_entries(table_type)?);
        }

        Ok(evaluated_entries)
    }
}

impl DelegateEntries {
    pub fn build_impls(
        &self,
        table_type: &Type,
        outer_generics: &Generics,
    ) -> syn::Result<Vec<ItemImpl>> {
        let mut item_impls = Vec::new();

        let evaluated_entries = self.eval_entries(table_type)?;

        for evaluated_entry in evaluated_entries {
            let delegate_component_impl =
                evaluated_entry.build_delegate_component_impl(outer_generics)?;

            let is_provider_impl = evaluated_entry.build_is_provider_for_impl(outer_generics)?;

            item_impls.push(delegate_component_impl);
            item_impls.push(is_provider_impl);
        }

        Ok(item_impls)
    }
}

impl ExtractInnerDelegateTables for DelegateEntries {
    fn extract_inner_tables(&self) -> Vec<InnerDelegateTable> {
        self.entries
            .iter()
            .flat_map(|entry| entry.extract_inner_tables())
            .collect()
    }
}
