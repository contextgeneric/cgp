use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::parse2;

use crate::delegate_components::{define_struct, impl_delegate_components};
use crate::parse::{DelegateComponents, SimpleType, TypeGenerics};

pub fn delegate_components(body: TokenStream) -> syn::Result<TokenStream> {
    let spec: DelegateComponents = parse2(body)?;

    let target_type = &spec.target_type;
    let target_generics = &spec.target_generics;

    let mut output = TokenStream::new();

    if spec.new_struct {
        let target_type: SimpleType<TypeGenerics> = parse2(target_type.to_token_stream())?;

        let type_generics = target_type.generics.unwrap_or_default().generics;

        let component_struct = define_struct(&target_type.name, &type_generics)?;

        output.extend(component_struct.to_token_stream());
    }

    let impl_items = impl_delegate_components(target_type, target_generics, &spec.entries)?;

    output.extend(impl_items);

    Ok(output)
}
