#[macro_export]
macro_rules! parse_internal {
    ( $($body:tt)* ) => {
        $crate::function::parse_internal(
            $crate::vendor::quote!( $( $body )* ))
    }
}
