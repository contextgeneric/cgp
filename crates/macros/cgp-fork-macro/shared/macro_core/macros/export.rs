macro_rules! export_construct {
    ( $from:ident => $to:ident ) => {
        pub struct $from;

        impl ::quote::ToTokens for $from {
            fn to_tokens(&self, tokens: &mut ::proc_macro2::TokenStream) {
                tokens.extend(::quote::quote! { ::cgp_fork::macro_prelude::$to })
            }
        }
    };
    ( $ident:ident ) => {
        $crate::macro_core::export_construct! { $ident => $ident }
    };
}

macro_rules! export_constructs {
    ( $( $from:ident $( => $to:ident )? ),* $(,)? ) => {
        $( $crate::macro_core::export_construct! { $from $( => $to )* } )*
    };
}

pub(crate) use export_construct;
pub(crate) use export_constructs;
