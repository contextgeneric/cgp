#[macro_export]
macro_rules! delegate_component {
    (
        $target:ident
            $( < $( $param:ident ),* $(,)? > )?
            $( @markers[ $( $marker:ident ),* $(,)? ] )?
        ;
        $name:ty : $forwarded:ty $(,)?
    ) => {
        impl< $( $( $param ),* )* >
            $crate::traits::delegate_component::DelegateComponent< $name >
            for $target $( < $( $param ),* > )*
        where
            Self: $crate::traits::sync::Async,
        {
            type Delegate = $forwarded;
        }

        $(
            $crate::impl_component_marker!(
                $name : [ $( $marker ),* ]
            );
        )?
    };
}
