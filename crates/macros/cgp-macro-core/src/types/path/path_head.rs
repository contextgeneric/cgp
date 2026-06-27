use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::{Brace, Bracket, Comma, Dot};
use syn::{braced, bracketed};

use crate::types::generics::ImplGenerics;
use crate::types::path::{PathElementWithGenerics, UniPath};

#[derive(Debug, Clone)]
pub enum PathHead {
    Type(Box<PathElementWithGenerics>, Box<PathHead>),
    Nested(Punctuated<PathHead, Comma>),
    Group(Punctuated<PathElementWithGenerics, Comma>, Box<PathHead>),
    End,
}

impl PathHead {
    pub fn into_paths(&self) -> Vec<(ImplGenerics, UniPath)> {
        match self {
            Self::Type(path_element, tail) => {
                let generics = &path_element.generics;
                let element = &path_element.element;

                let tail_paths = tail.into_paths();
                let mut out_paths = Vec::new();

                for (tail_generics, mut tail_path) in tail_paths {
                    let mut generics = generics.clone();
                    generics.params.extend(tail_generics.params.iter().cloned());
                    tail_path.elements.insert(0, element.clone());
                    out_paths.push((generics, tail_path))
                }

                out_paths
            }
            Self::Nested(path_heads) => path_heads
                .iter()
                .flat_map(|path| path.into_paths())
                .collect(),
            Self::Group(path_elements, tail) => {
                let tail_paths = tail.into_paths();
                let mut out_paths = Vec::new();

                for path_element in path_elements {
                    for (tail_generics, tail_path) in &tail_paths {
                        let mut generics = path_element.generics.clone();
                        generics.params.extend(tail_generics.params.iter().cloned());

                        let mut path = tail_path.clone();
                        path.elements.insert(0, path_element.element.clone());
                        out_paths.push((generics, path));
                    }
                }

                out_paths
            }
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

            Ok(Self::Nested(group))
        } else if input.peek(Bracket) {
            let body;
            bracketed!(body in input);

            let group = Punctuated::parse_terminated(&body)?;

            let rest_path = if input.peek(Dot) {
                let _: Dot = input.parse()?;
                Box::new(Self::parse(input)?)
            } else {
                Box::new(Self::End)
            };

            Ok(Self::Group(group, rest_path))
        } else {
            let path_element = input.parse()?;

            let rest_path = if input.peek(Dot) {
                let _: Dot = input.parse()?;
                Box::new(Self::parse(input)?)
            } else {
                Box::new(Self::End)
            };

            Ok(Self::Type(Box::new(path_element), rest_path))
        }
    }
}
