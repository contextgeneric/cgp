use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::{Bracket, Comma};
use syn::{Type, bracketed};

pub enum CheckKey {
    Single(Type),
    Multi(Punctuated<Type, Comma>),
}

impl CheckKey {
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
