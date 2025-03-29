use quote::ToTokens;
use syn::{parse_quote, Type};

use crate::symbol::symbol_from_string;

pub enum FieldTag {
    Named(String),
    Indexed(usize),
}

impl FieldTag {
    pub fn to_type(&self) -> Type {
        match self {
            Self::Named(name) => symbol_from_string(name),
            Self::Indexed(i) => {
                parse_quote! { Index< #i > }
            }
        }
    }
}

impl ToTokens for FieldTag {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.to_type().to_tokens(tokens);
    }
}
