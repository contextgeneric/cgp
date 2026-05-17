use syn::braced;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::{Brace, Comma, Dot};

use crate::types::{ImplGenerics, PathElement};

pub enum PathHead {
    Type(ImplGenerics, Box<PathElement>, Box<PathHead>),
    Group(Punctuated<PathHead, Comma>),
    End,
}

impl Parse for PathHead {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.is_empty() {
            Ok(Self::End)
        } else if input.peek(Brace) {
            let body;
            braced!(body in input);

            let group = Punctuated::parse_terminated(&body)?;

            Ok(Self::Group(group))
        } else {
            let generics = input.parse()?;

            let path_type: PathElement = input.parse()?;

            let rest_path = if input.peek(Dot) {
                let _: Dot = input.parse()?;
                Box::new(Self::parse(input)?)
            } else {
                Box::new(Self::End)
            };

            Ok(Self::Type(generics, Box::new(path_type), rest_path))
        }
    }
}
