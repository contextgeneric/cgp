use syn::parse::{Parse, ParseStream};
use syn::token::At;
use syn::{Type, parse_quote};

use crate::functions::merge_generics;
use crate::types::delegate_component::{EvalDelegateKey, EvaluatedDelegateKey};
use crate::types::generics::ImplGenerics;
use crate::types::path::PathHead;

pub struct PathDelegateKey {
    pub generics: ImplGenerics,
    pub at: At,
    pub path: PathHead,
}

impl Parse for PathDelegateKey {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let generics = input.parse()?;
        let at = input.parse()?;
        let path = input.parse()?;

        Ok(Self { generics, at, path })
    }
}

impl EvalDelegateKey for PathDelegateKey {
    fn eval(&self) -> syn::Result<Vec<EvaluatedDelegateKey>> {
        let paths = self.path.into_paths();
        let outer_generics = &self.generics;
        let mut keys = Vec::new();

        for (inner_generics, path) in paths {
            let generics = merge_generics(outer_generics, &inner_generics);
            let path_type: Type = parse_quote!(#path);

            let key = EvaluatedDelegateKey {
                generics,
                key: path_type,
            };

            keys.push(key)
        }

        Ok(keys)
    }
}
