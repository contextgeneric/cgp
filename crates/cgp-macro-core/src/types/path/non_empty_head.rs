use core::ops::Deref;

use syn::parse::{Parse, ParseStream};

use crate::types::PathHead;

#[derive(Debug, Clone)]
pub struct NonEmptyPathHead {
    pub path_head: PathHead,
}

impl Deref for NonEmptyPathHead {
    type Target = PathHead;

    fn deref(&self) -> &Self::Target {
        &self.path_head
    }
}

impl Parse for NonEmptyPathHead {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let path_head = input.parse()?;

        if let PathHead::End = path_head {
            return Err(syn::Error::new(
                input.span(),
                "Expected at least one path element",
            ));
        }

        Ok(Self { path_head })
    }
}
