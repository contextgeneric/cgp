use syn::parse::Parse;
use syn::parse::ParseStream;
use syn::punctuated::Punctuated;
use syn::token::{Comma, Gt, Lt};
use syn::{parse_quote, Generics, Type};

pub fn extract_generic_type_params(generics: &Generics) -> Punctuated<Type, Comma> {
    let type_generics = generics.split_for_impl().1;
    let extracted: ExtractGenericTypeParams = parse_quote!(#type_generics);

    extracted.type_params
}

pub struct ExtractGenericTypeParams {
    pub type_params: Punctuated<Type, Comma>,
}

impl Parse for ExtractGenericTypeParams {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let _: Lt = input.parse()?;

        let type_params = input.parse_terminated(Type::parse, Comma)?;

        let _: Gt = input.parse()?;

        Ok(Self { type_params })
    }
}
