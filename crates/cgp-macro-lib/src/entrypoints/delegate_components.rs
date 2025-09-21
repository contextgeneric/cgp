use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::parse2;

use crate::delegate_components::{define_struct, impl_delegate_components};
use crate::parse::{DelegateComponents, SimpleType, TypeGenerics};

pub fn delegate_components(body: TokenStream) -> syn::Result<TokenStream> {
    let spec: DelegateComponents = parse2(body)?;

    let component_struct = if spec.new_struct {
        let target_type: SimpleType<TypeGenerics> = parse2(spec.target_type.to_token_stream())?;

        let type_generics = target_type.generics.unwrap_or_default().generics;

        let component_struct = define_struct(&target_type.name, &type_generics)?;

        Some(component_struct)
    } else {
        None
    };

    let impl_items =
        impl_delegate_components(&spec.target_type, &spec.target_generics, &spec.entries)?;

    let mut output = quote! {
        #component_struct
    };

    for impl_item in impl_items {
        output.extend(impl_item.to_token_stream());
    }

    Ok(output)
}
