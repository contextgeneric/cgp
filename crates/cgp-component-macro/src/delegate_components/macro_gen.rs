use proc_macro2::TokenStream;
use quote::quote;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::Ident;

use crate::delegate_components::ast::ComponentAst;

pub fn generate_with_components_macro(
    macro_name: &Ident,
    components: &Punctuated<ComponentAst, Comma>,
) -> TokenStream {
    quote! {
        #[macro_export]
        macro_rules! {
            #macro_name {
                (
                    @remaining(  )
                    @out( $( $out:tt )* )
                ) => {
                    $( $out )*
                }

                (
                    @remaining( @components $( $remaining:tt )* )
                    @out( $( $out:tt )* )
                ) => {
                    crate:: #macro_name ! {
                        @remaining( $( $remaining )* )
                        @out(
                            $( $out )*
                            [ #components ]
                        )
                    }
                }

                (
                    @remaining( $current:tt $( $remaining:tt )* )
                    @out( $( $out:tt )* )
                ) => {
                    crate:: #macro_name ! {
                        @remaining( $( $remaining )* )
                        @out( $( $out )* $current )
                    }
                }

                ( $( $remaining:tt )* ) => {
                    crate:: #macro_name ! {
                        @remaining( $( $remaining )* )
                        @out(  )
                    }
                }
            }
        }
    }
}
