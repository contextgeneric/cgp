use syn::parse::{Parse, ParseStream};
use syn::parse_quote;
use syn::token::At;

use crate::functions::merge_generics;
use crate::types::delegate_component::{EvalDelegateKey, EvaluatedDelegateKey};
use crate::types::generics::ImplGenerics;
use crate::types::path::PathHead;

#[derive(Debug, Clone)]
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
            let mut generics = merge_generics(outer_generics, &inner_generics);
            generics.params.push(parse_quote!(__Wildcard__));

            let prefix = path.to_prefix(parse_quote!(__Wildcard__));
            let key_type = parse_quote!(#prefix);

            let key = EvaluatedDelegateKey {
                generics,
                key: key_type,
            };

            keys.push(key)
        }

        Ok(keys)
    }
}
