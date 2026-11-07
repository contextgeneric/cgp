use syn::token::Mut;
use syn::{Ident, Stmt, Type};

use crate::parse_internal;
use crate::types::field::{FieldName, HasFieldBound};
use crate::types::getter::{FieldMode, GetFieldExpr, GetFieldWithModeExpr};

/// One `#[implicit]` argument parsed into the field it reads.
#[derive(Clone, Eq, PartialEq)]
pub struct ImplicitArgField {
    /// The field's name.
    pub field_name: Ident,
    /// The field's type.
    pub field_type: Type,
    /// The `mut` of a `&mut` argument.
    pub field_mut: Option<Mut>,
    /// How the field is read.
    pub field_mode: FieldMode,
    /// The argument's type.
    pub arg_type: Type,
}

impl ImplicitArgField {
    /// Builds the `HasField` bound the argument requires.
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

    /// Builds the statement that reads the field into the argument.
    pub fn to_statement(&self) -> syn::Result<Stmt> {
        let field_name = &self.field_name;
        let arg_type = &self.arg_type;

        let get_field_expr = GetFieldWithModeExpr {
            field_mode: self.field_mode.clone(),
            get_field: GetFieldExpr {
                receiver: parse_internal!(self),
                field_mut: self.field_mut,
                field_name: self.field_name.clone().into(),
            },
        };

        let statement = parse_internal! {
            let #field_name: #arg_type = #get_field_expr;
        };

        Ok(statement)
    }
}
