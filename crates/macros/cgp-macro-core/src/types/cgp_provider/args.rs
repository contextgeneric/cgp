use syn::Type;
use syn::parse::{Parse, ParseStream};

use crate::types::keyword::Keyword;
use crate::types::keywords::New;

/// The parsed `#[cgp_provider(...)]` / `#[cgp_new_provider(...)]` attribute
/// argument: an optional component-type override.
///
/// The `new` flag is deliberately *not* parsed from the attribute — declaring
/// the provider struct is controlled by which macro is invoked, not by a keyword
/// in the shared argument grammar. `#[cgp_provider]`'s argument is only the
/// component type; the struct is emitted when `new` is set programmatically by
/// [`#[cgp_new_provider]`](../../../cgp-macro-lib/src/cgp_new_provider.rs) or
/// carried in from [`#[cgp_impl(new …)]`](../cgp_impl/lowered.rs).
#[derive(Clone)]
pub struct ProviderArgs {
    pub new: Option<Keyword<New>>,
    pub component_type: Option<Type>,
}

impl Parse for ProviderArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let component_type = if !input.is_empty() {
            let component_type: Type = input.parse()?;
            Some(component_type)
        } else {
            None
        };

        Ok(ProviderArgs {
            new: None,
            component_type,
        })
    }
}
