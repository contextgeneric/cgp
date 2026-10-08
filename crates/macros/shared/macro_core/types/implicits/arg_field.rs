use syn::token::Mut;
use syn::{Expr, Ident, Stmt, Type};

use crate::macro_core::parse_internal;
use crate::macro_core::types::field::{FieldName, HasFieldBound};
use crate::macro_core::types::getter::{FieldMode, GetFieldExpr, GetFieldWithModeExpr};

#[derive(Clone, Eq, PartialEq)]
pub struct ImplicitArgField {
    pub field_name: Ident,
    pub field_type: Type,
    pub field_mut: Option<Mut>,
    pub field_mode: FieldMode,
    pub arg_type: Type,
}

impl ImplicitArgField {
    pub fn to_has_field_bound(&self) -> syn::Result<HasFieldBound> {
        let field_name = FieldName::from(self.field_name.clone());
        let tag_type = parse_internal!(#field_name);

        Ok(HasFieldBound {
            field_type: self.field_type.clone(),
            field_mut: self.field_mut,
            field_mode: self.field_mode.clone(),
            tag_type,
        })
    }

    /// The field read, converted the way the argument's type requires.
    pub fn to_expr(&self, receiver: &Expr) -> syn::Result<Expr> {
        let get_field_expr = GetFieldWithModeExpr {
            field_mode: self.field_mode.clone(),
            get_field: GetFieldExpr {
                receiver: receiver.clone(),
                field_mut: self.field_mut,
                field_name: self.field_name.clone().into(),
            },
        };

        let expr = parse_internal!( #get_field_expr );
        Ok(expr)
    }

    pub fn to_statement(&self) -> syn::Result<Stmt> {
        let field_name = &self.field_name;
        let arg_type = &self.arg_type;
        let receiver: Expr = parse_internal!(self);
        let get_field_expr = self.to_expr(&receiver)?;

        let statement = parse_internal! {
            let #field_name: #arg_type = #get_field_expr;
        };

        Ok(statement)
    }
}
