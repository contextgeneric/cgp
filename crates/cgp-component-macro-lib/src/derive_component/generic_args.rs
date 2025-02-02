use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{GenericParam, Ident};

pub fn extract_generic_args(
    generic_params: &Punctuated<GenericParam, Comma>,
) -> Punctuated<Ident, Comma> {
    let mut generic_args: Punctuated<Ident, Comma> = Punctuated::new();

    for param in generic_params.iter() {
        match param {
            GenericParam::Type(ty) => {
                generic_args.push(ty.ident.clone());
            }
            GenericParam::Const(arg) => {
                generic_args.push(arg.ident.clone());
            }
            GenericParam::Lifetime(_life) => {
                unimplemented!()
            }
        }
    }

    generic_args
}
