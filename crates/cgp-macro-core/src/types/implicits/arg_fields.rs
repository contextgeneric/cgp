use syn::punctuated::Punctuated;
use syn::token::Plus;
use syn::{Block, TypeParamBound, parse_quote};

use crate::types::implicits::ImplicitArgField;

#[derive(Default)]
pub struct ImplicitArgFields {
    pub fields: Vec<ImplicitArgField>,
}

impl ImplicitArgFields {
    pub fn new(fields: Vec<ImplicitArgField>) -> Self {
        Self { fields }
    }
}

impl ImplicitArgFields {
    pub fn to_type_param_bounds(&self) -> syn::Result<Punctuated<TypeParamBound, Plus>> {
        let mut constraints: Punctuated<TypeParamBound, Plus> = Punctuated::new();

        for field in &self.fields {
            let constraint = field.to_has_field_bound()?;
            constraints.push(parse_quote!(#constraint));
        }

        Ok(constraints)
    }

    pub fn prepend_to_block(&self, block: &mut Block) -> syn::Result<()> {
        let block_statements = core::mem::take(&mut block.stmts);

        for field in &self.fields {
            let statement = field.to_statement()?;
            block.stmts.push(statement);
        }

        block.stmts.extend(block_statements);

        Ok(())
    }
}
