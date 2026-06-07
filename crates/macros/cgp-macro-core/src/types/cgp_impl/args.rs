use syn::Type;
use syn::parse::{Parse, ParseStream};
use syn::token::Colon;

use crate::traits::ParseOptionalKeyword;
use crate::types::keyword::Keyword;
use crate::types::keywords::New;

#[derive(Clone)]
pub struct ImplArgs {
    pub new: Option<Keyword<New>>,
    pub provider_type: Type,
    pub component_type: Option<Type>,
}

impl Parse for ImplArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let new = input.parse_optional_keyword()?;

        let provider_type = input.parse()?;

        let component_type = if let Some(_colon) = input.parse::<Option<Colon>>()? {
            let component_type: Type = input.parse()?;
            Some(component_type)
        } else {
            None
        };

        Ok(ImplArgs {
            new,
            provider_type,
            component_type,
        })
    }
}
