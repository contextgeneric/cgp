use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{parse_quote, ItemTrait, ReturnType, TraitItem, Type};

pub fn impl_async(item: TokenStream) -> TokenStream {
    match syn::parse2::<ItemTrait>(item.clone()) {
        Ok(mut target_trait) => {
            for trait_item in target_trait.items.iter_mut() {
                if let TraitItem::Fn(trait_fn) = trait_item {
                    if trait_fn.sig.asyncness.is_some() {
                        let return_type: Type = match &trait_fn.sig.output {
                            ReturnType::Default => {
                                parse_quote!(())
                            }
                            ReturnType::Type(_, return_type) => return_type.as_ref().clone(),
                        };

                        let impl_return: ReturnType = parse_quote! {
                            -> impl ::core::future::Future<Output = #return_type>
                        };

                        trait_fn.sig.output = impl_return;
                        trait_fn.sig.asyncness = None;
                    }
                }
            }

            target_trait.to_token_stream()
        }
        _ => item,
    }
}
