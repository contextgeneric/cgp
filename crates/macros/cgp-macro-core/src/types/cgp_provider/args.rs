use syn::Type;
use syn::parse::{Parse, ParseStream};

use crate::traits::ParseOptionalKeyword;
use crate::types::keyword::Keyword;
use crate::types::keywords::New;

#[derive(Clone)]
pub struct ProviderArgs {
    pub new: Option<Keyword<New>>,
    pub component_type: Option<Type>,
}

impl Parse for ProviderArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let new = input.parse_optional_keyword()?;

        let component_type = if !input.is_empty() {
            let component_type: Type = input.parse()?;
            Some(component_type)
        } else {
            None
        };

        Ok(ProviderArgs {
            new,
            component_type,
        })
    }
}
