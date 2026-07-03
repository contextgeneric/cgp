use proc_macro2::Span;
use syn::Attribute;
use syn::parse::{Parse, ParseStream};
use syn::token::At;

use crate::functions::merge_generics;
use crate::parse_internal;
use crate::types::delegate_component::{EvalDelegateKey, EvaluatedDelegateKey};
use crate::types::generics::ImplGenerics;
use crate::types::path::{PathElement, PathHead};

/// The `@`-prefixed open key. It lowers each path to a prefix type terminated by
/// a `__Wildcard__` generic, into which the dispatch parameter slots at lookup
/// time; a brace group in the path expands to one key per element.
#[derive(Debug, Clone)]
pub struct PathDelegateKey {
    pub attributes: Vec<Attribute>,
    pub generics: ImplGenerics,
    pub at: At,
    pub path: PathHead,
}

impl Parse for PathDelegateKey {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let attributes = input.call(Attribute::parse_outer)?;

        let generics = input.parse()?;
        let at = input.parse()?;
        let path = input.parse()?;

        Ok(Self {
            attributes,
            generics,
            at,
            path,
        })
    }
}

impl EvalDelegateKey for PathDelegateKey {
    fn eval(&self) -> syn::Result<Vec<EvaluatedDelegateKey>> {
        let paths = self.path.into_paths();
        let outer_generics = &self.generics;
        let mut keys = Vec::new();

        for (inner_generics, path) in paths {
            let mut generics = merge_generics(outer_generics, &inner_generics);
            generics.params.push(parse_internal!(__Wildcard__));

            // Span the entry on the path segments the user wrote, not the
            // synthesized `PathCons<..>` key type (whose first token is a
            // `call_site`-spanned `PathCons`). `join` widens the span to the whole
            // path on toolchains that support it; where it does not (stable), the
            // fallback keeps the leaf segment — the component of a namespace path
            // or the dispatch key of an `@Component.key` entry — since that is the
            // discriminating segment among entries sharing a prefix.
            let span = path
                .elements
                .iter()
                .map(PathElement::span)
                .reduce(|acc, next| acc.join(next).unwrap_or(next))
                .unwrap_or_else(Span::call_site);

            let prefix = path.to_prefix(parse_internal!(__Wildcard__));
            let key_type = parse_internal!(#prefix);

            let key = EvaluatedDelegateKey {
                generics,
                key: key_type,
                span,
            };

            keys.push(key)
        }

        Ok(keys)
    }
}
