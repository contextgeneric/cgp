use syn::punctuated::Punctuated;
use syn::token::Plus;
use syn::{Block, ImplItem, TypeParamBound, parse_quote};

use crate::functions::extract_and_parse_implicit_args;
use crate::traits::ToTypeParamBounds;
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

impl ToTypeParamBounds for ImplicitArgFields {
    fn to_type_param_bounds(&self) -> syn::Result<Punctuated<TypeParamBound, Plus>> {
        let mut constraints: Punctuated<TypeParamBound, Plus> = Punctuated::new();

        for field in &self.fields {
            let constraint = field.to_has_field_bound()?;
            constraints.push(parse_quote!(#constraint));
        }

        Ok(constraints)
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

    pub fn extract_from_impl_items(impl_items: &mut [ImplItem]) -> syn::Result<Self> {
        let mut all_fields = Vec::new();

        for item in impl_items {
            if let ImplItem::Fn(method) = item {
                let implicit_args = extract_and_parse_implicit_args(&mut method.sig.inputs)?;
                implicit_args.prepend_to_block(&mut method.block)?;

                for implicit_arg in implicit_args.fields {
                    if !all_fields.contains(&implicit_arg) {
                        all_fields.push(implicit_arg);
                    }
                }
            }
        }

        Ok(ImplicitArgFields { fields: all_fields })
    }
}
