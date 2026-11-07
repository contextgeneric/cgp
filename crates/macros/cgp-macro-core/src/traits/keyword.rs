use syn::Ident;
use syn::parse::ParseBuffer;

use crate::types::keyword::Keyword;

/// A marker for a keyword the macros parse.
pub trait IsKeyword {
    /// The keyword's spelling.
    const IDENT: &'static str;
}

/// Peeks whether the next token is a keyword.
pub trait PeekKeyword {
    /// Returns whether the next token is the keyword.
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

/// Parses a keyword when it is present.
pub trait ParseOptionalKeyword {
    /// Parses the keyword, or returns `None`.
    fn parse_optional_keyword<K: IsKeyword>(&self) -> syn::Result<Option<Keyword<K>>>;
}

impl<'a> ParseOptionalKeyword for ParseBuffer<'a> {
    fn parse_optional_keyword<K: IsKeyword>(&self) -> syn::Result<Option<Keyword<K>>> {
        let keyword = if self.peek_keyword::<K>() {
            Some(self.parse()?)
        } else {
            None
        };

        Ok(keyword)
    }
}
