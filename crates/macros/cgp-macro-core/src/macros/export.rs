#[macro_export]
macro_rules! export_construct {
    ( $from:ident => $to:ident ) => {
        pub struct $from;

        impl ::quote::ToTokens for $from {
            fn to_tokens(&self, tokens: &mut ::proc_macro2::TokenStream) {
                tokens.extend(::quote::quote! { ::cgp::macro_prelude::$to })
            }
        }
    };
    ( $ident:ident ) => {
        $crate::export_construct! { $ident => $ident }
    };
}

#[macro_export]
macro_rules! export_constructs {
    ( $( $from:ident $( => $to:ident )? ),* $(,)? ) => {
        $( $crate::export_construct! { $from $( => $to )* } )*
    };
}
