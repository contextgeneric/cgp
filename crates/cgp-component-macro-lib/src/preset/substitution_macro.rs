use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

pub fn define_substitution_macro(
    preset_module_name: &Ident,
    macro_name: &Ident,
    substitution: &TokenStream,
) -> TokenStream {
    let local_self = Ident::new(
        &format!("__{preset_module_name}__"),
        preset_module_name.span(),
    );

    quote! {
        #[macro_export]
        macro_rules! #macro_name {
            ( $( $body:tt )* ) => {
                pub struct #local_self;

                replace_with! {
                    [ #substitution ],
                    $( $body )*
                }
            };
        }
    }
}
