use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

pub fn define_substitution_macro(macro_name: &Ident, substitution: &TokenStream) -> TokenStream {
    quote! {
        #[macro_export]
        macro_rules! #macro_name {
            ( $( $body:tt )* ) => {
                replace_with! {
                    [ #substitution ],
                    $( $body )*
                }
            };
        }

        pub use #macro_name;
    }
}
