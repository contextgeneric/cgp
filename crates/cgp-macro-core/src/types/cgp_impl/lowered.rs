use syn::{ItemImpl, Type};

use crate::types::cgp_impl::ImplArgs;
use crate::types::ident::IdentWithTypeArgs;

pub struct LoweredCgpImpl {
    pub args: ImplArgs,
    pub item_impl: ItemImpl,
    pub context_type: Type,
    pub consumer_trait_path: IdentWithTypeArgs,
}
