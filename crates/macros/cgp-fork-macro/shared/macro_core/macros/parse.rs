/// Quasi-quote tokens and parse them into an inferred `syn` type via the
/// `parse_internal` function. Expands to a `?` expression, so it must be called
/// inside a `syn::Result`-returning function.
macro_rules! parse_internal {
    ( $($body:tt)* ) => {
        $crate::macro_core::functions::parse_internal(
            $crate::macro_core::vendor::quote!( $( $body )* ))?
    }
}

pub(crate) use parse_internal;
