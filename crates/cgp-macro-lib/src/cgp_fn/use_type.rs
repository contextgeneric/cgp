use syn::parse::{Parse, ParseStream};
use syn::token::{Brace, Colon, Comma, Gt, Lt};
use syn::{Ident, braced};

use crate::parse::SimpleType;

pub struct UseTypeSpec {
    pub trait_path: SimpleType,
    pub type_idents: Vec<Ident>,
}

impl Parse for UseTypeSpec {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let turbofish = input.peek(Lt);

        if turbofish {
            let _: Lt = input.parse()?;
        }

        let trait_path: SimpleType = input.parse()?;

        if turbofish {
            let _: Gt = input.parse()?;
        }

        let _: Colon = input.parse()?;
        let _: Colon = input.parse()?;

        let type_idents: Vec<Ident> = if input.peek(Brace) {
            let content;
            braced!(content in input);
            content
                .parse_terminated(Ident::parse, Comma)?
                .into_iter()
                .collect()
        } else {
            let ident: Ident = input.parse()?;
            vec![ident]
        };

        Ok(Self {
            trait_path,
            type_idents,
        })
    }
}
