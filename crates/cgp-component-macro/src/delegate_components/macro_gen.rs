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
        macro_rules! #macro_name {
            (
                @remaining(  )
                @out( $( $out:tt )* )
                @stack( )
            ) => {
                $( $out )*
            };

            (
                @remaining(  )
                @out( $( $out:tt )* )
                @stack(
                    @layer {
                        @front( $( $front:tt )* )
                        @remaining( $( $remaining:tt )* )
                    }
                    $( $stack:tt )*
                )
            ) => {
                $crate:: #macro_name ! {
                    @remaining( $( $remaining )* )
                    @out(
                        $( $front )* { $( $out )* }
                    )
                    @stack(
                        $( $stack )*
                    )
                }
            };

            (
                @remaining(  )
                @out( $( $out:tt )* )
                @stack(
                    @layer [
                        @front( $( $front:tt )* )
                        @remaining( $( $remaining:tt )* )
                    ]
                    $( $stack:tt )*
                )
            ) => {
                $crate:: #macro_name ! {
                    @remaining( $( $remaining )* )
                    @out(
                        $( $front )* [ $( $out )* ]
                    )
                    @stack(
                        $( $stack )*
                    )
                }
            };

            (
                @remaining( @ components $( $remaining:tt )* )
                @out( $( $out:tt )* )
                @stack( $( $stack:tt )* )
            ) => {
                $crate:: #macro_name ! {
                    @remaining( $( $remaining )* )
                    @out(
                        $( $out )*
                        [ #components ]
                    )
                    @stack( $( $stack )* )
                }
            };


            (
                @remaining( { $( $inner:tt )* } $( $outer:tt )* )
                @out( $( $out:tt )* )
                @stack( $( $stack:tt )* )
            ) => {
                $crate:: #macro_name ! {
                    @remaining( $( $inner )* )
                    @out( )
                    @stack(
                        @layer {
                            @front( $( $out )* )
                            @remaining( $( $outer )* )
                        }
                        $( $stack )*
                    )
                }
            };

            (
                @remaining( [ $( $inner:tt )* ] $( $outer:tt )* )
                @out( $( $out:tt )* )
                @stack( $( $stack:tt )* )
            ) => {
                $crate:: #macro_name ! {
                    @remaining( $( $inner )* )
                    @out( )
                    @stack(
                        @layer [
                            @front( $( $out )* )
                            @remaining( $( $outer )* )
                        ]
                        $( $stack )*
                    )
                }
            };

            (
                @remaining( $current:tt $( $remaining:tt )* )
                @out( $( $out:tt )* )
                @stack( $( $stack:tt )* )
            ) => {
                $crate:: #macro_name ! {
                    @remaining( $( $remaining )* )
                    @out( $( $out )* $current )
                    @stack( $( $stack )* )
                }
            };

            ( $( $remaining:tt )* ) => {
                $crate:: #macro_name ! {
                    @remaining( $( $remaining )* )
                    @out(  )
                    @stack(  )
                }
            };
        }
    }
}
