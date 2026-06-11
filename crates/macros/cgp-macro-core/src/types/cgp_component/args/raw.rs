use syn::Ident;

use crate::types::ident::IdentWithTypeGenerics;

pub struct CgpComponentRawArgs {
    pub context: Option<Ident>,
    pub provider_ident: Option<Ident>,
    pub component_name: IdentWithTypeGenerics,
}
