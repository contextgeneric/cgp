use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

pub fn define_substitution_macro(
    macro_name: &Ident,
    substitute_ident: &Ident,
    substitution: &TokenStream,
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
                @remaining( @ #substitute_ident $( $remaining:tt )* )
                @out( $( $out:tt )* )
                @stack( $( $stack:tt )* )
            ) => {
                $crate:: #macro_name ! {
                    @remaining( $( $remaining )* )
                    @out(
                        $( $out )*
                        [ #substitution ]
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
