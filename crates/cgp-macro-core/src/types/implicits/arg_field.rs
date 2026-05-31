use syn::token::Mut;
use syn::{Ident, Stmt, Type, parse_quote};

use crate::types::getter::{FieldMode, GetFieldExpr, GetFieldWithModeExpr};

#[derive(Clone, Eq, PartialEq)]
pub struct ImplicitArgField {
    pub field_name: Ident,
    pub field_type: Type,
    pub field_mut: Option<Mut>,
    pub field_mode: FieldMode,
    pub arg_type: Type,
}

impl ImplicitArgField {
    pub fn to_statement(&self) -> syn::Result<Stmt> {
        let field_name = &self.field_name;
        let arg_type = &self.arg_type;

        let get_field_expr = GetFieldWithModeExpr {
            field_mode: self.field_mode.clone(),
            get_field: GetFieldExpr {
                receiver: parse_quote!(self),
                field_mut: self.field_mut,
                field_name: self.field_name.clone().into(),
            },
        };

        let statement = parse_quote! {
            let #field_name: #arg_type = #get_field_expr;
        };

        Ok(statement)
    }
}
