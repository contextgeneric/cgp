use syn::braced;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::{Brace, Comma, Dot};

use crate::types::generics::ImplGenerics;
use crate::types::path::{PathElement, UniPath};

#[derive(Debug, Clone)]
pub enum PathHead {
    Type(ImplGenerics, Box<PathElement>, Box<PathHead>),
    Group(Punctuated<PathHead, Comma>),
    End,
}

impl PathHead {
    pub fn into_paths(&self) -> Vec<(ImplGenerics, UniPath)> {
        match self {
            Self::Type(generics, path_element, tail) => {
                let tail_paths = tail.into_paths();
                let mut out_paths = Vec::new();

                for (tail_generics, mut tail_path) in tail_paths {
                    let mut generics = generics.clone();
                    generics.params.extend(tail_generics.params.iter().cloned());
                    tail_path.elements.insert(0, path_element.as_ref().clone());
                    out_paths.push((generics, tail_path))
                }

                out_paths
            }
            Self::Group(path_heads) => path_heads
                .iter()
                .flat_map(|path| path.into_paths())
                .collect(),
            Self::End => {
                vec![(ImplGenerics::default(), UniPath::default())]
            }
        }
    }
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
