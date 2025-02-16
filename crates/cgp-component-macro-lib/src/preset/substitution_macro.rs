use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

pub fn define_substitution_macro(
    preset_module_name: &Ident,
    macro_name: &Ident,
    substitution: &TokenStream,
) -> TokenStream {
    quote! {
        #[macro_export]
        macro_rules! #macro_name {
            ( $( $body:tt )* ) => {
                use #preset_module_name ::components::*;

                replace_with! {
                    [ #substitution ],
                    $( $body )*
                }
            };
        }
    }
}
