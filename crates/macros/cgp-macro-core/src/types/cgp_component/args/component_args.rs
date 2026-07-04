use proc_macro2::Span;
use syn::parse::Parse;
use syn::{Error, Ident};

use crate::types::cgp_component::CgpComponentRawArgs;
use crate::types::ident::IdentWithTypeGenerics;

/// The `#[cgp_component]` attribute args with defaults applied: the context type
/// identifier (`__Context__` by default), the required provider trait name, and
/// the component marker name (`{Provider}Component` by default).
#[derive(Clone)]
pub struct CgpComponentArgs {
    pub context_ident: Ident,
    pub provider_ident: Ident,
    pub component_name: IdentWithTypeGenerics,
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
            .ok_or_else(|| Error::new(Span::call_site(), "the `provider` key must be given"))?;

        let context_ident = raw_args
            .context_ident
            .unwrap_or_else(|| Ident::new("__Context__", Span::call_site()));

        // Span the derived marker on the provider identifier the user wrote, not
        // `Span::call_site()`. A `call_site` span here covers the whole
        // `#[cgp_component(..)]` attribute, so the generated `struct {Provider}Component`
        // reports its definition on a range that also contains the `cgp_component`
        // macro path — and an editor's go-to-definition on a delegate key naming
        // this component then offers both the component and the macro as targets
        // (visible cross-crate, where navigation relies on this span). Deriving the
        // ident from `provider_ident.span()` points it at a single user token, the
        // way `derive_check_trait_ident` spans its derived check trait.
        let component_name = raw_args.component_name.unwrap_or_else(|| {
            IdentWithTypeGenerics::from(Ident::new(
                &format!("{provider_ident}Component"),
                provider_ident.span(),
            ))
        });

        Ok(Self {
            context_ident,
            provider_ident,
            component_name,
        })
    }
}
