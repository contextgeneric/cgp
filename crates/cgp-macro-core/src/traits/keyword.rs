use syn::Ident;
use syn::parse::ParseBuffer;

use crate::types::keyword::Keyword;

pub trait IsKeyword {
    const IDENT: &'static str;
}

pub trait PeekKeyword<K> {
    fn peek(&self, keyword: K) -> bool;
}

impl<'a, K: IsKeyword> PeekKeyword<Keyword<K>> for ParseBuffer<'a> {
    fn peek(&self, _keyword: Keyword<K>) -> bool {
        if let Ok(ident) = self.fork().parse::<Ident>()
            && ident == K::IDENT
        {
            true
        } else {
            false
        }
    }
}
