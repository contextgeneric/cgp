use syn::Ident;
use syn::parse::ParseBuffer;

pub trait IsKeyword {
    const IDENT: &'static str;
}

pub trait PeekKeyword {
    fn peek_keyword<K: IsKeyword>(&self) -> bool;
}

impl<'a> PeekKeyword for ParseBuffer<'a> {
    fn peek_keyword<K: IsKeyword>(&self) -> bool {
        if let Ok(ident) = self.fork().parse::<Ident>()
            && ident == K::IDENT
        {
            true
        } else {
            false
        }
    }
}
