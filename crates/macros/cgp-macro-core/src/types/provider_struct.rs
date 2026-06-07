use quote::quote;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{GenericParam, Generics, Ident, ItemStruct, Type, parse2};

pub struct ProviderStruct {
    pub ident: Ident,
    pub generics: Generics,
}

impl ProviderStruct {
    pub fn to_item_struct(&self) -> syn::Result<ItemStruct> {
        let struct_ident = &self.ident;
        let struct_generics = &self.generics;

        if struct_generics.params.is_empty() {
            parse2(quote! {
                pub struct #struct_ident;
            })
        } else {
            let mut generic_params = struct_generics.params.clone();
            let mut phantom_params: Punctuated<Type, Comma> = Default::default();

            for param in generic_params.iter_mut() {
                match param {
                    GenericParam::Type(type_param) => {
                        type_param.colon_token = None;
                        type_param.bounds.clear();

                        let type_ident = &type_param.ident;
                        phantom_params.push(parse2(quote!( #type_ident ))?);
                    }
                    GenericParam::Lifetime(life_param) => {
                        life_param.colon_token = None;
                        life_param.bounds.clear();

                        let lifetime = &life_param.lifetime;
                        phantom_params.push(parse2(quote!( Life<#lifetime> ))?);
                    }
                    _ => {}
                }
            }

            parse2(quote! {
                pub struct #struct_ident < #generic_params > (
                    pub ::core::marker::PhantomData<( #phantom_params )>
                );
            })
        }
    }
}
