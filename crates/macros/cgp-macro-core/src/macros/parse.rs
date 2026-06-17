#[macro_export]
macro_rules! parse_internal {
    ( $($body:tt)* ) => {
        $crate::functions::parse_internal(
            $crate::vendor::quote!( $( $body )* ))?
    }
}

pub use parse_internal;
