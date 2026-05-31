use cgp_macro_core::types::attributes::{UseProviderAttribute, UseTypeAttribute};
use cgp_macro_core::types::ident::IdentWithTypeArgs;
use syn::{GenericParam, TypeParamBound, WherePredicate};

#[derive(Default)]
pub struct FunctionAttributes {
    pub extend: Vec<TypeParamBound>,
    pub extend_where: Vec<WherePredicate>,
    pub uses: Vec<IdentWithTypeArgs>,
    pub use_type: Vec<UseTypeAttribute>,
    pub use_provider: Vec<UseProviderAttribute>,
    pub impl_generics: Vec<GenericParam>,
}
