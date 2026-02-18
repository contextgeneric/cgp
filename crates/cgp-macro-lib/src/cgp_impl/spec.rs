use syn::parse::discouraged::Speculative;
use syn::parse::{Parse, ParseStream};
use syn::token::Colon;
use syn::{Ident, Type};

pub struct ImplProviderSpec {
    pub new_struct: bool,
    pub provider_type: Type,
    pub component_type: Option<Type>,
}

impl Parse for ImplProviderSpec {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let new_struct = {
            let fork = input.fork();
            let new_ident: Option<Ident> = fork.parse().ok();
            match new_ident {
                Some(new_ident) if new_ident == "new" => {
                    input.advance_to(&fork);
                    true
                }
                _ => false,
            }
        };

        let provider_type = input.parse()?;

        let component_type = if let Some(_colon) = input.parse::<Option<Colon>>()? {
            let component_type: Type = input.parse()?;
            Some(component_type)
        } else {
            None
        };

        Ok(ImplProviderSpec {
            new_struct,
            provider_type,
            component_type,
        })
    }
}
