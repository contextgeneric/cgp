/// Declare a custom-keyword marker: a zero-sized struct plus its `IsKeyword` impl
/// carrying the keyword's spelling, which the parsers peek against.
macro_rules! define_keyword {
    ( $struct_ident:ident, $value:literal ) => {
        pub struct $struct_ident;

        impl $crate::macro_core::traits::IsKeyword for $struct_ident {
            const IDENT: &'static str = $value;
        }
    };
}

pub(crate) use define_keyword;
