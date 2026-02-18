use syn::token::Mut;
use syn::{Ident, Type, TypeParamBound};

use crate::cgp_fn::UseTypeSpec;
use crate::derive_getter::FieldMode;
use crate::parse::SimpleType;

pub struct ImplicitArgField {
    pub field_name: Ident,
    pub field_type: Type,
    pub field_mut: Option<Mut>,
    pub field_mode: FieldMode,
    pub arg_type: Type,
}

#[derive(Default)]
pub struct FunctionAttributes {
    pub extend: Vec<TypeParamBound>,
    pub uses: Vec<SimpleType>,
    pub use_type: Vec<UseTypeSpec>,
}
