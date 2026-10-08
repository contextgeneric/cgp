use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::token::Comma;
use syn::{Attribute, Error, Type};

#[derive(Clone)]
pub enum CheckParamsAttribute {
    Default,
    Skip,
    Multi(Punctuated<Type, Comma>),
}

impl CheckParamsAttribute {
    pub fn merge(&self, other: &Self) -> syn::Result<Self> {
        let res = match (self, other) {
            (Self::Default, other) => other.clone(),
            (other, Self::Default) => other.clone(),
            (Self::Skip, Self::Skip) => Self::Skip,
            (Self::Multi(params_a), Self::Multi(params_b)) => Self::Multi(Punctuated::from_iter(
                params_a.iter().chain(params_b.iter()).cloned(),
            )),
            (Self::Skip, Self::Multi(params)) | (Self::Multi(params), Self::Skip) => {
                return Err(Error::new(
                    params.span(),
                    "cannot combine #[skip_check] with #[check_params]",
                ));
            }
        };

        Ok(res)
    }

    pub fn parse_attributes(attributes: &[Attribute]) -> syn::Result<Self> {
        if attributes.is_empty() {
            return Ok(Self::Default);
        }

        if attributes.len() > 1 {
            return Err(Error::new(
                attributes[1].span(),
                "Expected at most one `#[check_params]` or `#[skip_check]` attribute",
            ));
        }

        let attribute = &attributes[0];

        if attribute.path().is_ident("check_params") {
            let params = attribute.parse_args_with(Punctuated::parse_terminated)?;
            Ok(CheckParamsAttribute::Multi(params))
        } else if attribute.path().is_ident("skip_check") {
            attribute.meta.require_path_only().map_err(|_| {
                Error::new(
                    attribute.span(),
                    "`#[skip_check]` does not take any arguments",
                )
            })?;

            Ok(CheckParamsAttribute::Skip)
        } else {
            Err(Error::new(
                attribute.span(),
                "Expected either `#[skip_check]` or `#[check_params]` attribute for specifying the check generics",
            ))
        }
    }
}
