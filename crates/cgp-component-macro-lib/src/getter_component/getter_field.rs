use syn::token::Mut;
use syn::{Ident, Type};

pub struct GetterField {
    pub field_name: Ident,
    pub provider_type: Type,
    pub field_mut: Option<Mut>,
}
