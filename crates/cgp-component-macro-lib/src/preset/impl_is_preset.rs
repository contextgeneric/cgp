use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{parse_quote, Generics, Ident, ItemImpl, Type};

use crate::delegate_components::ast::{ComponentAst, DelegateEntriesAst};

pub fn derive_impl_components_is_preset(
    trait_name: &Ident,
    preset_type: &Type,
    preset_generics: &Generics,
    delegate_entries: &DelegateEntriesAst,
) -> TokenStream {
    let mut out = TokenStream::new();

    for entry in delegate_entries.entries.iter() {
        for component in entry.components.iter() {
            let impl_is_preset =
                impl_component_is_preset(trait_name, preset_type, preset_generics, component);
            impl_is_preset.to_tokens(&mut out);
        }
    }

    out
}

pub fn impl_component_is_preset(
    trait_name: &Ident,
    _preset_type: &Type,
    _preset_generics: &Generics,
    component: &ComponentAst,
) -> ItemImpl {
    let component_type = &component.component_type;

    // FIXME: The preset generic would be absent if the if it is used as part of the
    // component name's generic.
    // let generics = merge_generics(preset_generics, &component.component_generics);

    let mut generics = component.component_generics.clone();
    generics.params.push(parse_quote!(T: Sized));

    let impl_generics = generics.split_for_impl().0;

    parse_quote! {
        impl #impl_generics #trait_name < #component_type > for T {}
    }
}
