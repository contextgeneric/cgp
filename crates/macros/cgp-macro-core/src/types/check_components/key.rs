use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::{Bracket, Comma};
use syn::{Type, bracketed};

/// The components a check entry asserts.
pub enum CheckKey {
    /// One component.
    Single(Type),
    /// Several components.
    Multi(Punctuated<Type, Comma>),
}

impl CheckKey {
    /// The components as a list.
    pub fn to_keys(&self) -> Vec<Type> {
        match self {
            Self::Single(key) => vec![key.clone()],
            Self::Multi(keys) => Vec::from_iter(keys.iter().cloned()),
        }
    }
}

impl Parse for CheckKey {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(Bracket) {
            let content;
            bracketed!(content in input);

            let keys: Punctuated<Type, Comma> = Punctuated::parse_terminated(&content)?;
            Ok(Self::Multi(keys))
        } else {
            let key: Type = input.parse()?;
            Ok(Self::Single(key))
        }
    }
}
