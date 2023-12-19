#[macro_export]
macro_rules! impl_component_marker {
    ( $name:ty: [ $( $marker_trait:ident ),* $(,)? ]
    ) => {
        $(
            impl<T> $marker_trait< $name > for T {}
        )*
    };
}
