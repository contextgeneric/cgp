/// Quasi-quote tokens and parse them into an inferred `syn` type via the
/// `parse_internal` function. Expands to a `?` expression, so it must be called
/// inside a `syn::Result`-returning function.
#[macro_export]
macro_rules! parse_internal {
    ( $($body:tt)* ) => {
        $crate::functions::parse_internal(
            $crate::vendor::quote!( $( $body )* ))?
    }
}

pub use parse_internal;
