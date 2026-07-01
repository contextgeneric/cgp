/// Declare a custom-keyword marker: a zero-sized struct plus its `IsKeyword` impl
/// carrying the keyword's spelling, which the parsers peek against.
#[macro_export]
macro_rules! define_keyword {
    ( $struct_ident:ident, $value:literal ) => {
        pub struct $struct_ident;

        impl $crate::traits::IsKeyword for $struct_ident {
            const IDENT: &'static str = $value;
        }
    };
}
