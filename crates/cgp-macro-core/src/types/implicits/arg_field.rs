use syn::token::Mut;
use syn::{Ident, Type};

use crate::types::getter::FieldMode;

#[derive(Clone, Eq, PartialEq)]
pub struct ImplicitArgField {
    pub field_name: Ident,
    pub field_type: Type,
    pub field_mut: Option<Mut>,
    pub field_mode: FieldMode,
    pub arg_type: Type,
}
