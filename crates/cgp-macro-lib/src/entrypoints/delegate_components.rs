use cgp_macro_core::types::delegate_component::DelegateTable;
use cgp_macro_core::types::generics::TypeGenerics;
use cgp_macro_core::types::provider_struct::ProviderStruct;
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::parse2;

use crate::delegate_components::impl_delegate_components;
use crate::parse::{DelegateComponents, SimpleType};

pub fn delegate_components(body: TokenStream) -> syn::Result<TokenStream> {
    let table: DelegateTable = parse2(body.clone())?;

    let spec: DelegateComponents = parse2(body)?;

    let target_type = &spec.target_type;
    let target_generics = &spec.target_generics;

    let mut output = TokenStream::new();

    if spec.new_struct {
        let target_type: SimpleType<TypeGenerics> = parse2(target_type.to_token_stream())?;

        let type_generics = target_type.generics.unwrap_or_default().generics;

        let component_struct = ProviderStruct {
            ident: target_type.name,
            generics: type_generics,
        }
        .to_item_struct()?;

        output.extend(component_struct.to_token_stream());
    }

    let impl_items = impl_delegate_components(target_type, target_generics, &spec.entries)?;

    output.extend(impl_items);

    Ok(output)
}
