use syn::Path;
use syn::parse::{Parse, ParseStream};

use crate::macro_core::types::keyword::Keyword;
use crate::macro_core::types::keywords::Preset;

/// `preset Path` — one entry that expands, through `Path::with_components!`,
/// into a `DelegateComponent` impl for every component the preset carries.
#[derive(Debug, Clone)]
pub struct PresetDelegateEntry {
    pub preset_token: Keyword<Preset>,
    pub path: Path,
}

impl Parse for PresetDelegateEntry {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            preset_token: input.parse()?,
            path: input.parse()?,
        })
    }
}
