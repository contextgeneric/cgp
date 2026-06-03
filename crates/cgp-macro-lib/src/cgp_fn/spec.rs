use cgp_macro_core::types::attributes::{UseProviderAttribute, UseTypeAttributes};
use cgp_macro_core::types::ident::IdentWithTypeArgs;
use syn::{GenericParam, TypeParamBound, WherePredicate};

#[derive(Default)]
pub struct FunctionAttributes {
    pub extend: Vec<TypeParamBound>,
    pub extend_where: Vec<WherePredicate>,
    pub uses: Vec<IdentWithTypeArgs>,
    pub use_type: UseTypeAttributes,
    pub use_provider: Vec<UseProviderAttribute>,
    pub impl_generics: Vec<GenericParam>,
}
