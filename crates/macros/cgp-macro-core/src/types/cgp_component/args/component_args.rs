use proc_macro2::Span;
use syn::parse::Parse;
use syn::{Error, Ident};

use crate::types::attributes::DeriveDelegateAttributes;
use crate::types::cgp_component::CgpComponentRawArgs;
use crate::types::ident::IdentWithTypeGenerics;

#[derive(Clone)]
pub struct CgpComponentArgs {
    pub context_ident: Ident,
    pub provider_ident: Ident,
    pub component_name: IdentWithTypeGenerics,
    pub derive_delegate_attributes: DeriveDelegateAttributes,
}

impl Parse for CgpComponentArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let raw_args: CgpComponentRawArgs = input.parse()?;

        raw_args.try_into()
    }
}

impl TryFrom<CgpComponentRawArgs> for CgpComponentArgs {
    type Error = Error;

    fn try_from(raw_args: CgpComponentRawArgs) -> Result<Self, Self::Error> {
        let provider_ident = raw_args
            .provider_ident
            .ok_or_else(|| Error::new(Span::call_site(), "`provider_ident` key must be given"))?;

        let context_ident = raw_args
            .context_ident
            .unwrap_or_else(|| Ident::new("__Context__", Span::call_site()));

        let component_name = raw_args.component_name.unwrap_or_else(|| {
            IdentWithTypeGenerics::from(Ident::new(
                &format!("{provider_ident}Component"),
                Span::call_site(),
            ))
        });

        let derive_delegate_attributes = raw_args.derive_delegate_attributes.unwrap_or_default();

        Ok(Self {
            context_ident,
            provider_ident,
            component_name,
            derive_delegate_attributes,
        })
    }
}
