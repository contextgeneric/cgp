use cgp_macro_core::types::attributes::{UseProviderAttribute, UseTypeAttribute};
use cgp_macro_core::types::ident::IdentWithTypeArgs;
use syn::token::Mut;
use syn::{GenericParam, Ident, Type, TypeParamBound, WherePredicate};

use crate::derive_getter::FieldMode;

#[derive(Clone, Eq, PartialEq)]
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
    pub extend_where: Vec<WherePredicate>,
    pub uses: Vec<IdentWithTypeArgs>,
    pub use_type: Vec<UseTypeAttribute>,
    pub use_provider: Vec<UseProviderAttribute>,
    pub impl_generics: Vec<GenericParam>,
}
