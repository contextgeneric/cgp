use syn::Block;

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
